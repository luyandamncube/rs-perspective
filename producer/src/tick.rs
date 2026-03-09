// producer\src\tick.rs
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub size: u64,
    pub ts: DateTime<Utc>,
}

pub fn sample_tick(symbol: &str) -> Tick {
    Tick {
        symbol: symbol.to_string(),
        price: 100.0,
        size: 10,
        ts: Utc::now(),
    }
}

pub fn next_tick(prev_price: f64, symbol: &str) -> Tick {
    let mut rng = rand::thread_rng();

    let drift = rng.gen_range(-0.75..=0.75);
    let next_price = (prev_price + drift).max(0.01);
    let size = rng.gen_range(1..=1_000);

    Tick {
        symbol: symbol.to_string(),
        price: (next_price * 100.0).round() / 100.0,
        size,
        ts: Utc::now(),
    }
}