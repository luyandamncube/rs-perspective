// producer\src\lib.rs
pub mod tick;

use anyhow::Result;
use tick::Tick;

pub fn encode_tick_json(tick: &Tick) -> Result<String> {
    Ok(serde_json::to_string(tick)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tick::{next_tick, sample_tick};

    #[test]
    fn sample_tick_has_expected_symbol_and_positive_values() {
        let tick = sample_tick("AAPL");

        assert_eq!(tick.symbol, "AAPL");
        assert!(tick.price.is_finite());
        assert!(tick.price > 0.0);
        assert!(tick.size > 0);
    }

    #[test]
    fn next_tick_preserves_symbol_and_positive_values() {
        let tick = next_tick(100.0, "MSFT");

        assert_eq!(tick.symbol, "MSFT");
        assert!(tick.price.is_finite());
        assert!(tick.price > 0.0);
        assert!(tick.size > 0);
    }

    #[test]
    fn tick_serializes_to_valid_json_with_required_keys() {
        let tick = sample_tick("NVDA");
        let json = encode_tick_json(&tick).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["symbol"], "NVDA");
        assert!(value.get("price").is_some());
        assert!(value.get("size").is_some());
        assert!(value.get("ts").is_some());
    }

    #[test]
    fn serialized_tick_round_trips() {
        let tick = sample_tick("SPY");
        let json = encode_tick_json(&tick).unwrap();
        let decoded: Tick = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.symbol, tick.symbol);
        assert_eq!(decoded.size, tick.size);
        assert_eq!(decoded.price, tick.price);
    }
}