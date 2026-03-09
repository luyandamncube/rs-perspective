// server-rs\src\handlers.rs
pub async fn health() -> &'static str {
    "ok"
}

pub async fn ws_info() -> &'static str {
    "Perspective websocket endpoint is available at /ws"
}