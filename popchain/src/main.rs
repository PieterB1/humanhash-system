use axum::{routing::{post, get}, Router, Json, extract::State, http::StatusCode, Server};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use reqwest::Client;
use rand::Rng;
use sha2::{Digest, Sha256};

#[derive(Clone, Deserialize)]
struct Config {
    lnd_host: String,
    lnd_macaroon_path: String,
    lnd_tls_cert_path: String,
    port: u16,
    ledger_endpoint: String,
}

#[derive(Deserialize)]
struct LedgerRequest {
    human_hash_id: String,
    biometric_data: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct AttestationResponse {
    attestation_id: String,
    human_hash_id: String,
    biometric_proof: ZKProof,
    transaction_hash: String,
    timestamp: u64,
    expires_at: u64,
}

#[derive(Serialize)]
struct ZKProof {
    circuit: String,
    proof: String,
    public_inputs: Vec<String>,
    verification_key: String,
}

async fn write_ledger(State(config): State<Config>, Json(payload): Json<LedgerRequest>) -> Result<Json<AttestationResponse>, StatusCode> {
    println!("Received identity commitment: {}", payload.human_hash_id);
    let biometric_hash = hash_biometric_data(&payload.biometric_data);
    let zk_proof = generate_zk_proof(&payload.human_hash_id, &biometric_hash);
    
    match get_lnd_info(config.clone()).await {
        Ok(info) => println!("LND connection for commitment: {}", info),
        Err(e) => {
            eprintln!("LND connection failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let transaction_hash = format!("tx_{}", generate_nonce());
    let attestation_id = generate_attestation_id();
    
    let response = AttestationResponse {
        attestation_id,
        human_hash_id: payload.human_hash_id,
        biometric_proof: zk_proof,
        transaction_hash,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        expires_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() + 365 * 24 * 60 * 60,
    };
    
    Ok(Json(response))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

async fn get_lnd_info(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    println!("Attempting to read TLS cert at: {}", config.lnd_tls_cert_path);
    let tls_cert = fs::read(&config.lnd_tls_cert_path)?;
    println!("Successfully read TLS cert (length: {} bytes)", tls_cert.len());

    println!("Attempting to read macaroon at: {}", config.lnd_macaroon_path);
    let macaroon = fs::read(&config.lnd_macaroon_path)?;
    println!("Successfully read macaroon (length: {} bytes)", macaroon.len());
    let macaroon = hex::encode(macaroon);

    let client = Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(&tls_cert)?)
        .build()?;
    
    let rest_endpoint = format!("{}/v1/getinfo", config.lnd_host);
    println!("Attempting to connect to LND REST API at: {}", rest_endpoint);
    let response = client
        .get(&rest_endpoint)
        .header("Grpc-Metadata-macaroon", macaroon)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("Successfully connected to LND REST API");
        Ok(format!("LND Identity Pubkey: {}", json["identity_pubkey"].as_str().unwrap_or("unknown")))
    } else {
        Err(format!("Failed to connect to LND REST API: {}", response.status()).into())
    }
}

fn hash_biometric_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn generate_zk_proof(human_hash_id: &str, biometric_hash: &str) -> ZKProof {
    ZKProof {
        circuit: "biometric_verification".to_string(),
        proof: format!("proof_{}", generate_nonce()),
        public_inputs: vec![human_hash_id.to_string(), biometric_hash.to_string()],
        verification_key: "ver_key_123".to_string(),
    }
}

fn generate_attestation_id() -> String {
    format!("att_{}_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), generate_nonce())
}

fn generate_nonce() -> String {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        &fs::read_to_string("popchain_config.json")?
    )?;
    println!("Loaded config for LND at {}", config.lnd_host);
    match get_lnd_info(config.clone()).await {
        Ok(info) => println!("LND connection successful: {}", info),
        Err(e) => eprintln!("LND connection failed: {}", e),
    }
    let app = Router::new()
        .route(&config.ledger_endpoint, post(write_ledger))
        .route("/health", get(health))
        .with_state(config.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Starting PoPChain server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
