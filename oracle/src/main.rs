use axum::{routing::{post, get}, Router, Json, extract::State, http::StatusCode, Server};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use reqwest::Client;
use secp256k1::{Keypair, Message, Secp256k1, SecretKey, XOnlyPublicKey};
use sha2::{Sha256, Digest};

#[derive(Clone, Deserialize)]
struct Config {
    #[allow(dead_code)]
    lnd_host: String,
    #[allow(dead_code)]
    lnd_macaroon_path: String,
    #[allow(dead_code)]
    lnd_tls_cert_path: String,
    oracle_provider: String,
    #[allow(dead_code)]
    api_endpoint: String,
    oracle_pubkey: String,
    port: u16,
    kyc_endpoint: String,
    #[allow(dead_code)]
    payment_endpoint: String,
}

#[derive(Deserialize)]
struct KycRequest {
    human_hash_id: String,
    #[allow(dead_code)]
    proof: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct OracleAttestation {
    oracle_id: String,
    provider: String,
    data_source: String,
    verification_result: bool,
    confidence: f32,
    signature: String,
    dlc_outcome: Option<String>,
}

#[allow(dead_code)]
async fn pay_invoice(config: &Config, invoice: &str) -> Result<(), StatusCode> {
    let tls_cert = match fs::read(&config.lnd_tls_cert_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read TLS cert: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let certificate = match reqwest::Certificate::from_pem(&tls_cert) {
        Ok(cert) => cert,
        Err(e) => {
            eprintln!("Failed to parse TLS cert: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let client = match Client::builder().add_root_certificate(certificate).build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to build reqwest client: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    let macaroon = match fs::read(&config.lnd_macaroon_path) {
        Ok(data) => hex::encode(data),
        Err(e) => {
            eprintln!("Failed to read macaroon: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    let payment_request = serde_json::json!({ "payment_request": invoice });
    let response = match client
        .post(format!("{}/v1/payments", config.lnd_host))
        .header("Grpc-Metadata-macaroon", &macaroon)
        .json(&payment_request)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to send payment request: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    if response.status().is_success() {
        let json = match response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to parse payment response: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        println!("Successfully paid invoice: {:?}", json);
        Ok(())
    } else {
        eprintln!("Failed to pay invoice: {}", response.status());
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn kyc(State(config): State<Config>, Json(payload): Json<KycRequest>) -> Result<Json<OracleAttestation>, StatusCode> {
    println!("Received KYC request for user: {}", payload.human_hash_id);
    
    let secp = Secp256k1::new();
    // Load secret key for testing
    let secret_key = match SecretKey::from_slice(&hex::decode("33d0fe452d329ae213c531dfda4582300742cfe7ec6a36b43e6eaa2c1564ea42").unwrap()) {
        Ok(sk) => sk,
        Err(e) => {
            eprintln!("Failed to parse secret key: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    
    // Parse the configured public key
    let config_pubkey = match hex::decode(&config.oracle_pubkey) {
        Ok(pubkey_bytes) => match XOnlyPublicKey::from_slice(&pubkey_bytes) {
            Ok(pubkey) => pubkey,
            Err(e) => {
                eprintln!("Failed to parse oracle public key: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        Err(e) => {
            eprintln!("Invalid oracle public key hex: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Mock biometric hash outcome (replace with MegaMatcher SDK output)
    let outcome = format!("biometric_hash_{}", payload.human_hash_id);
    
    // Hash the outcome with SHA256 (BIP340)
    let mut hasher = Sha256::new();
    hasher.update(outcome.as_bytes());
    let message = Message::from_digest_slice(&hasher.finalize()).expect("32 bytes hash required");
    
    // Generate Schnorr signature
    let signature = secp.sign_schnorr(&message, &keypair);
    let serialized_sig = signature.serialize();
    
    // Verify the signature with the configured public key
    let verified = secp.verify_schnorr(&signature, &message, &config_pubkey).is_ok();
    if !verified {
        eprintln!("Failed to verify Schnorr signature with configured public key");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let attestation = OracleAttestation {
        oracle_id: "humanhash-oracle-001".to_string(),
        provider: "HumanhashOracle".to_string(),
        data_source: "humanhash-verification".to_string(),
        verification_result: true,
        confidence: 0.95,
        signature: hex::encode(serialized_sig),
        dlc_outcome: Some(outcome),
    };
    Ok(Json(attestation))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(
        &fs::read_to_string("oracle_config.json")?
    )?;
    println!("Starting Oracle server with provider: {}", config.oracle_provider);

    let app = Router::new()
        .route(&config.kyc_endpoint, post(kyc))
        .route("/health", get(health))
        .with_state(config.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Starting Oracle server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
