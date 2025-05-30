use axum::{routing::get, Router, extract::State, middleware::Next, response::Response};
use axum::body::Body;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use tracing::{info, debug, error, trace};
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;
use std::sync::Arc;
use axum::http::Request;

#[derive(Clone)]
struct AppState {
    request_count: Arc<Mutex<u32>>,
    active_connections: Arc<Mutex<u32>>,
}

async fn log_connection(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next
) -> Response {
    {
        let mut active = state.active_connections.lock().await;
        *active += 1;
        trace!("New connection opened, active connections: {}", *active);
    }
    let response = next.run(req).await;
    {
        let mut active = state.active_connections.lock().await;
        *active -= 1;
        trace!("Connection closed, active connections: {}", *active);
    }
    response
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
        active_connections: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/test", get(|State(state): State<AppState>| async move {
            trace!("Received /test request");
            info!("Handling /test request");
            debug!("Processing request");
            let mut count = state.request_count.lock().await;
            *count += 1;
            debug!("Request count: {}", *count);
            let active = *state.active_connections.lock().await;
            debug!("Active connections: {}", active);
            let response = "OK";
            debug!("Sending response: {}", response);
            trace!("Completed /test request");
            response
        }))
        .layer(axum::middleware::from_fn_with_state(state.clone(), log_connection))
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
