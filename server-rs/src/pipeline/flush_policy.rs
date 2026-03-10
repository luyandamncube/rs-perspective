// server-rs\src\pipeline\flush_policy.rs
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FlushPolicy {
    pub flush_interval: Duration,
    pub max_batch_size: usize,
}

impl FlushPolicy {
    pub fn new(flush_interval: Duration, max_batch_size: usize) -> Self {
        Self {
            flush_interval,
            max_batch_size,
        }
    }
}