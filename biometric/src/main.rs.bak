use axum::{routing::get, Router, extract::State};
use std::net::SocketAddr;
use tokio::sync::Mutex;
use tracing::{info, debug, error, trace};
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    request_count: Arc<Mutex<u32>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")))
        .with_thread_ids(true)
        .init();
    info!("Initializing server");

    let state = AppState {
        request_count: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/test", get(|State(state): State<AppState>| async move {
            trace!("Received /test request");
            info!("Handling /test request");
            debug!("Processing request");
            let mut count = state.request_count.lock().await;
            *count += 1;
            debug!("Request count: {}", *count);
            let response = "OK";
            debug!("Sending response: {}", response);
            trace!("Completed /test request");
            response
        }))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    info!("Binding to {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server bound to {}", addr);

    debug!("Starting server loop");
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("Server error: {:?}", e);
    }
    info!("Server shutdown complete");

    Ok(())
}
