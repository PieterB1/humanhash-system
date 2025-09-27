use anyhow::Result;
use axum::{routing::{get, post}, Router, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber;
use reqwest::Client;
use secp256k1::{Keypair, Message, Secp256k1, SecretKey, XOnlyPublicKey};
use sha2::{Digest, Sha256};

mod zkp;

#[derive(Clone, Deserialize)]
struct Config {
    host: String,
    port: u16,
    lnd_host: String,
    lnd_macaroon_path: String,
    lnd_tls_cert_path: String,
    oracle_provider: String,
    api_endpoint: String,
    oracle_pubkey: String,
    kyc_endpoint: String,
    payment_endpoint: String,
}

#[derive(Deserialize)]
struct KycRequest {
    #[serde(rename = "humanHashId")]
    human_hash_id: String,
    proof: Option<String>,
}

#[derive(Deserialize)]
struct VerifyZkpRequest {
    proof: String,
    verifying_key: String,
    human_hash_id: String,
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

#[derive(Serialize)]
struct ZkpResponse {
    proof: String,
    verifying_key: String,
}

#[derive(Serialize)]
struct VerifyZkpResponse {
    valid: bool,
}

async fn query_kyc(State(config): State<Config>, Json(payload): Json<KycRequest>) -> Result<(StatusCode, Json<OracleAttestation>), StatusCode> {
    info!("KYC query for HumanHash ID: {}", payload.human_hash_id);

    let secp = Secp256k1::new();
    let secret_key = match SecretKey::from_slice(&hex::decode("33d0fe452d329ae213c531dfda4582300742cfe7ec6a36b43e6eaa2c1564ea42").unwrap()) {
        Ok(sk) => sk,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let keypair = Keypair::from_secret_key(&secp, &secret_key);

    let config_pubkey = match hex::decode(&config.oracle_pubkey) {
        Ok(pubkey_bytes) => match XOnlyPublicKey::from_slice(&pubkey_bytes) {
            Ok(pubkey) => pubkey,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let outcome = format!("biometric_hash_{}", payload.human_hash_id);
    let mut hasher = Sha256::new();
    hasher.update(outcome.as_bytes());
    let message = Message::from_digest_slice(&hasher.finalize()).expect("32 bytes hash required");

    let signature = secp.sign_schnorr(&message, &keypair);
    let serialized_sig = signature.serialize();

    let verified = secp.verify_schnorr(&signature, &message, &config_pubkey).is_ok();
    if !verified {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((StatusCode::OK, Json(OracleAttestation {
        oracle_id: "humanhash-oracle-001".to_string(),
        provider: config.oracle_provider.clone(),
        data_source: "humanhash-verification".to_string(),
        verification_result: true,
        confidence: 0.95,
        signature: hex::encode(serialized_sig),
        dlc_outcome: Some(outcome),
    })))
}

async fn zkp(State(config): State<Config>, Json(payload): Json<KycRequest>) -> Result<(StatusCode, Json<ZkpResponse>), StatusCode> {
    let secp = Secp256k1::new();
    let secret_key = match SecretKey::from_slice(&hex::decode("33d0fe452d329ae213c531dfda4582300742cfe7ec6a36b43e6eaa2c1564ea42").unwrap()) {
        Ok(sk) => sk,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let config_pubkey = match hex::decode(&config.oracle_pubkey) {
        Ok(pubkey_bytes) => match XOnlyPublicKey::from_slice(&pubkey_bytes) {
            Ok(pubkey) => pubkey,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let outcome = format!("biometric_hash_{}", payload.human_hash_id);
    let mut hasher = Sha256::new();
    hasher.update(outcome.as_bytes());
    let message = Message::from_digest_slice(&hasher.finalize()).expect("32 bytes hash required");
    let signature = secp.sign_schnorr(&message, &keypair);
    let serialized_sig = signature.serialize();

    let (proof, vk) = match zkp::generate_zkp(outcome.as_bytes(), &serialized_sig, &config_pubkey.serialize()) {
        Ok((proof, vk)) => (proof, vk),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((StatusCode::OK, Json(ZkpResponse { proof, verifying_key: vk })))
}

async fn verify_zkp(State(config): State<Config>, Json(payload): Json<VerifyZkpRequest>) -> Result<(StatusCode, Json<VerifyZkpResponse>), StatusCode> {
    let secp = Secp256k1::new();
    let config_pubkey = match hex::decode(&config.oracle_pubkey) {
        Ok(pubkey_bytes) => match XOnlyPublicKey::from_slice(&pubkey_bytes) {
            Ok(pubkey) => pubkey,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let outcome = format!("biometric_hash_{}", payload.human_hash_id);
    let mut hasher = Sha256::new();
    hasher.update(outcome.as_bytes());
    let message = Message::from_digest_slice(&hasher.finalize()).expect("32 bytes hash required");
    let signature = secp.sign_schnorr(&message, &Keypair::from_secret_key(&secp, &SecretKey::from_slice(&hex::decode("33d0fe452d329ae213c531dfda4582300742cfe7ec6a36b43e6eaa2c1564ea42").unwrap()).unwrap()));
    let serialized_sig = signature.serialize();

    let valid = match zkp::verify_zkp(&payload.proof, &payload.verifying_key, outcome.as_bytes(), &serialized_sig, &config_pubkey.serialize()) {
        Ok(valid) => valid,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((StatusCode::OK, Json(VerifyZkpResponse { valid })))
}

async fn payment_handler(State(config): State<Config>, Json(_payload): Json<serde_json::Value>) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let tls_cert = match fs::read(&config.lnd_tls_cert_path) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let certificate = match reqwest::Certificate::from_pem(&tls_cert) {
        Ok(cert) => cert,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let client = match Client::builder().add_root_certificate(certificate).build() {
        Ok(client) => client,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let macaroon = match fs::read(&config.lnd_macaroon_path) {
        Ok(data) => hex::encode(data),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let payment_request = serde_json::json!({ "payment_request": "temp-invoice" });
    let response = match client
        .post(format!("{}/v1/payments", config.lnd_host))
        .header("Grpc-Metadata-macaroon", &macaroon)
        .json(&payment_request)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if response.status().is_success() {
        let json = match response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        Ok((StatusCode::OK, Json(json)))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn health(State(_config): State<Config>) -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { status: "healthy".to_string() }))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let config_str = fs::read_to_string("oracle_config.json")?;
    let config: Config = serde_json::from_str(&config_str)?;
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    info!("Starting Oracle server on {}", addr);

    let app = Router::new()
        .route(&config.kyc_endpoint, post(query_kyc))
        .route(&config.payment_endpoint, post(payment_handler))
        .route("/oracle/zkp", post(zkp))
        .route("/oracle/verify_zkp", post(verify_zkp))
        .route("/health", get(health))
        .with_state(config);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service()
    ).await?;

    Ok(())
}