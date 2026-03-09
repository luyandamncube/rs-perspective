// server-rs\src\app.rs
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::handlers::health;

pub fn build_router() -> Router {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    Router::new()
        .route("/health", get(health))
        .fallback_service(ServeDir::new(static_dir).append_index_html_on_directories(true))
}