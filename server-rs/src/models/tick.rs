// server-rs\src\models\ticks.rs
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub size: u64,
    pub ts: String,
}

impl Tick {
    pub fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(anyhow!("tick symbol cannot be empty"));
        }

        if !self.price.is_finite() || self.price <= 0.0 {
            return Err(anyhow!("tick price must be finite and > 0"));
        }

        if self.size == 0 {
            return Err(anyhow!("tick size must be > 0"));
        }

        if self.ts.trim().is_empty() {
            return Err(anyhow!("tick ts cannot be empty"));
        }

        Ok(())
    }
}

pub fn parse_tick(bytes: &[u8]) -> Result<Tick> {
    let tick: Tick = serde_json::from_slice(bytes)?;
    tick.validate()?;
    Ok(tick)
}

// ------------------------------------------------
// UNIT TESTS
// ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tick_accepts_valid_json() {
        let payload = br#"{
            "symbol": "AAPL.N",
            "price": 101.25,
            "size": 50,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        let tick = parse_tick(payload).unwrap();

        assert_eq!(tick.symbol, "AAPL.N");
        assert_eq!(tick.price, 101.25);
        assert_eq!(tick.size, 50);
        assert_eq!(tick.ts, "2026-03-09T06:30:00Z");
    }

    #[test]
    fn parse_tick_rejects_empty_symbol() {
        let payload = br#"{
            "symbol": "",
            "price": 101.25,
            "size": 50,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        assert!(parse_tick(payload).is_err());
    }

    #[test]
    fn parse_tick_rejects_invalid_price() {
        let payload = br#"{
            "symbol": "AAPL.N",
            "price": 0.0,
            "size": 50,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        assert!(parse_tick(payload).is_err());
    }

    #[test]
    fn parse_tick_rejects_zero_size() {
        let payload = br#"{
            "symbol": "AAPL.N",
            "price": 101.25,
            "size": 0,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        assert!(parse_tick(payload).is_err());
    }
}