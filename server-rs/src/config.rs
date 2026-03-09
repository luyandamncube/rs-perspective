// server-rs\src\config.rs
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub nats_url: String,
    pub nats_subject: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            http_port: env::var("HTTP_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            nats_url: env::var("NATS_URL")
                .unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string()),
            nats_subject: env::var("NATS_SUBJECT")
                .unwrap_or_else(|_| "ticks".to_string()),
        }
    }
}