// server-rs\src\app.rs
use axum::{routing::get, Router};
use perspective::server::Server;
use tower_http::services::ServeDir;

use crate::handlers::{health, ws_info};

pub fn build_router(perspective_server: Server) -> Router {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    Router::new()
        .route("/health", get(health))
        .route("/ws-info", get(ws_info))
        .route("/ws", perspective::axum::websocket_handler())
        .with_state(perspective_server)
        .fallback_service(ServeDir::new(static_dir).append_index_html_on_directories(true))
}