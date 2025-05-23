use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use bitcoin::network::constants::Network;
use rust_lightning::ln::channelmanager::ChannelManager;

async fn write_to_ledger(data: &str, channel: Option<&str>) -> Option<String> { Some("tx-abcdef123456".to_string()) }
async fn compute_merkle_root(data: &str) -> Option<String> { Some("merkleroot-987654".to_string()) }

struct StateChannel {
    id: String,
    balance: u64,
    transactions: Vec<String>,
}

async fn open_channel(user_id: &str, amount: u64) -> Option<String> {
    let channel = StateChannel {
        id: format!("channel-{}", rand::random::<u64>()),
        balance: amount,
        transactions: vec![],
    };
    let _ = ChannelManager::new(Network::Testnet).open_channel(user_id, amount).await;
    Some(channel.id)
}

async fn write_offchain(channel_id: &str, data: &str) -> Option<String> {
    let tx_id = write_to_ledger(data, Some(channel_id)).await?;
    Some(tx_id)
}

async fn close_channel(channel_id: &str) -> Option<String> {
    let merkle_root = compute_merkle_root(channel_id).await?;
    let _ = commit_to_bitcoin(&merkle_root).await;
    Some(merkle_root)
}

async fn commit_to_bitcoin(merkle_root: &str) -> Option<()> { Some(()) }

#[derive(Deserialize)]
struct LedgerWriteRequest {
    data: String,
    channel: Option<String>,
}

#[derive(Serialize)]
struct LedgerWriteResponse {
    tx_id: String,
    merkle_root: String,
    timestamp: String,
}

#[post("/ledger/write")]
async fn write_ledger(req: web::Json<LedgerWriteRequest>) -> HttpResponse {
    let tx_id = if let Some(channel) = &req.channel {
        write_offchain(channel, &req.data).await.unwrap_or("tx-abcdef123456".to_string())
    } else {
        write_to_ledger(&req.data, None).await.unwrap_or("tx-abcdef123456".to_string())
    };
    let merkle_root = compute_merkle_root(&req.data).await.unwrap_or("merkleroot-987654".to_string());
    HttpResponse::Ok().json(LedgerWriteResponse {
        tx_id,
        merkle_root,
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Popchain service running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(write_ledger)
            .service(health)
    })
    .bind("0.0.0.0:3002")?
    .run()
    .await
}
