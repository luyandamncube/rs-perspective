// server-rs\src\ticks.rs
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub size: u64,
    pub ts: String,
}

pub fn parse_tick(bytes: &[u8]) -> Result<Tick> {
    let tick: Tick = serde_json::from_slice(bytes)?;

    if tick.symbol.trim().is_empty() {
        return Err(anyhow!("tick symbol cannot be empty"));
    }

    if !tick.price.is_finite() || tick.price <= 0.0 {
        return Err(anyhow!("tick price must be finite and > 0"));
    }

    if tick.size == 0 {
        return Err(anyhow!("tick size must be > 0"));
    }

    if tick.ts.trim().is_empty() {
        return Err(anyhow!("tick ts cannot be empty"));
    }

    Ok(tick)
}

pub fn tick_to_row(tick: &Tick) -> Value {
    json!({
        "symbol": tick.symbol,
        "price": tick.price,
        "size": tick.size,
        "ts": tick.ts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tick_accepts_valid_json() {
        let payload = br#"{
            "symbol": "AAPL",
            "price": 101.25,
            "size": 50,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        let tick = parse_tick(payload).unwrap();

        assert_eq!(tick.symbol, "AAPL");
        assert_eq!(tick.price, 101.25);
        assert_eq!(tick.size, 50);
        assert_eq!(tick.ts, "2026-03-09T06:30:00Z");
    }

    #[test]
    fn parse_tick_rejects_malformed_json() {
        let payload = br#"{"symbol":"AAPL","price":"bad","size":50,"ts":"2026-03-09T06:30:00Z"}"#;

        let err = parse_tick(payload).unwrap_err().to_string();
        assert!(!err.is_empty());
    }

    #[test]
    fn parse_tick_rejects_non_positive_price() {
        let payload = br#"{
            "symbol": "AAPL",
            "price": 0.0,
            "size": 50,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        let err = parse_tick(payload).unwrap_err().to_string();
        assert!(err.contains("price"));
    }

    #[test]
    fn parse_tick_rejects_zero_size() {
        let payload = br#"{
            "symbol": "AAPL",
            "price": 101.25,
            "size": 0,
            "ts": "2026-03-09T06:30:00Z"
        }"#;

        let err = parse_tick(payload).unwrap_err().to_string();
        assert!(err.contains("size"));
    }

    #[test]
    fn tick_to_row_contains_expected_keys() {
        let tick = Tick {
            symbol: "MSFT".to_string(),
            price: 250.75,
            size: 100,
            ts: "2026-03-09T06:31:00Z".to_string(),
        };

        let row = tick_to_row(&tick);

        assert_eq!(row["symbol"], "MSFT");
        assert_eq!(row["price"], 250.75);
        assert_eq!(row["size"], 100);
        assert_eq!(row["ts"], "2026-03-09T06:31:00Z");
    }
}
