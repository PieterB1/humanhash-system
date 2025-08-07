use axum::{routing::post, Json, Router};
   use reqwest::blocking::get;
   use serde::{Deserialize, Serialize};
   use sha2::{Digest, Sha256};
   use uuid::Uuid;
   use chrono::Utc;
   use std::fs;
   use tracing::{info, error};
   use tracing_subscriber::{fmt, EnvFilter};

   #[derive(Serialize, Deserialize)]
   struct OracleRequest {
       user_id: String,
   }

   #[derive(Serialize, Deserialize)]
   struct OracleResponse {
       status: String,
       sequence_code: String,
   }

   async fn kyc_oracle(Json(req): Json<OracleRequest>) -> Json<OracleResponse> {
       info!("Processing KYC oracle request for user_id: {}", req.user_id);
       
       // Load Bitcoin wallet data for sureBits
       let wallet_data = match fs::read("/Users/pieterwjbouwer/bitcoin/testnet3/wallets/testwallet/wallet.dat") {
           Ok(data) => data,
           Err(e) => {
               error!("Failed to read wallet.dat: {}", e);
               panic!("Failed to read wallet.dat");
           }
       };
       
       // Query sureBits API (placeholder endpoint)
       let response = get("https://surebits.oracle/api/feed")
           .query(&[("user_id", &req.user_id)])
           .send()
           .map_err(|e| {
               error!("sureBits error: {}", e);
               format!("sureBits error: {}", e)
           })
           .and_then(|r| r.text().map_err(|e| {
               error!("sureBits response error: {}", e);
               format!("sureBits response error: {}", e)
           }))
           .unwrap_or("Error".to_string());
       
       // Generate unique sequence code
       let sequence_code = generate_sequence_code("QUERY");
       
       // Log to PoPChain
       log_to_popchain(&response, &sequence_code);
       
       info!("KYC oracle response: {}, sequence_code: {}", response, sequence_code);
       
       Json(OracleResponse {
           status: response,
           sequence_code,
       })
   }

   fn log_to_popchain(data: &str, sequence_code: &str) {
       // Placeholder for PoPChain logging
       println!("Logged to PoPChain: data={}, sequence_code={}", data, sequence_code);
   }

   fn generate_sequence_code(action: &str) -> String {
       let uuid = Uuid::new_v4();
       let timestamp = Utc::now().timestamp();
       let mut hasher = Sha256::new();
       hasher.update(format!("{}{}", uuid, timestamp));
       let hash = hex::encode(hasher.finalize());
       format!("TX-{}-{}-{}-{}", action, uuid, timestamp, &hash[..8])
   }

   #[tokio::main]
   async fn main() {
       fmt()
           .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
           .with_thread_ids(true)
           .init();
       
       let app = Router::new()
           .route("/oracle/kyc", post(kyc_oracle));
       
       info!("Starting oracle service on 0.0.0.0:3003");
       axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
           .serve(app.into_make_service())
           .await
           .unwrap();
   }
