// server-rs\src\pipeline\backpressure.rs
#[derive(Debug, Clone, Copy)]
pub enum BackpressurePolicy {
    Block,
}