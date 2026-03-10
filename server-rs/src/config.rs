// server-rs\src\config.rs
use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub nats_url: String,
    pub nats_subject: String,
    pub batch_flush_ms: u64,
    pub max_batch_size: usize,
    pub queue_capacity: usize,
    pub perspective_table_limit: Option<u32>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let http_port = env::var("HTTP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        let nats_url = env::var("NATS_URL")
            .unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());

        let nats_subject = env::var("NATS_SUBJECT")
            .unwrap_or_else(|_| "ticks".to_string());

        let batch_flush_ms = env::var("BATCH_FLUSH_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(50);

        let max_batch_size = env::var("MAX_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(256);

        let queue_capacity = env::var("QUEUE_CAPACITY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(4096);

        let perspective_table_limit = env::var("PERSPECTIVE_TABLE_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok());

        Self {
            http_port,
            nats_url,
            nats_subject,
            batch_flush_ms,
            max_batch_size,
            queue_capacity,
            perspective_table_limit,
        }
    }

    pub fn batch_flush_interval(&self) -> Duration {
        Duration::from_millis(self.batch_flush_ms)
    }
}