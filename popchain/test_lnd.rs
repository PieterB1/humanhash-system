use std::fs;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lnd_host = "https://localhost:8081";
    let tls_cert_path = "/Users/pieterwjbouwer/lnd-test/tls.cert";
    let macaroon_path = "/Users/pieterwjbouwer/lnd-test/data/chain/bitcoin/testnet/admin.macaroon";

    println!("Attempting to read TLS cert at: {}", tls_cert_path);
    let tls_cert = fs::read(tls_cert_path)?;
    println!("Successfully read TLS cert (length: {} bytes)", tls_cert.len());

    println!("Attempting to read macaroon at: {}", macaroon_path);
    let macaroon = fs::read(macaroon_path)?;
    println!("Successfully read macaroon (length: {} bytes)", macaroon.len());
    let macaroon = hex::encode(macaroon);

    let client = Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(&tls_cert)?)
        .build()?;
    
    let rest_endpoint = format!("{}/v1/getinfo", lnd_host);
    println!("Attempting to connect to LND REST API at: {}", rest_endpoint);
    let response = client
        .get(&rest_endpoint)
        .header("Grpc-Metadata-macaroon", macaroon)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("Successfully connected to LND REST API");
        println!("LND Identity Pubkey: {}", json["identity_pubkey"].as_str().unwrap_or("unknown"));
        Ok(())
    } else {
        Err(format!("Failed to connect to LND REST API: {}", response.status()).into())
    }
}
