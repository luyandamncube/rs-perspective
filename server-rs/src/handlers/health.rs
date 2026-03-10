// server-rs\src\handlers\health.rs
use axum::http::StatusCode;

pub async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}