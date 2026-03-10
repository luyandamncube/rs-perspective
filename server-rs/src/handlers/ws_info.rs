// server-rs\src\handlers\ws_info.rs
use axum::http::StatusCode;

pub async fn ws_info() -> (StatusCode, &'static str) {
    (StatusCode::OK, "/ws")
}