// server-rs\src\transforms\demo_rows.rs
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::models::{DemoRow, Tick};

const CLIENTS: &[&str] = &[
    "Homer", "Marge", "Bart", "Lisa", "Maggie", "Moe", "Lenny", "Carl", "Krusty",
];

fn parse_ts_to_epoch_millis(ts: &str) -> Result<i64> {
    let dt = DateTime::parse_from_rfc3339(ts)?;
    Ok(dt.with_timezone(&Utc).timestamp_millis())
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn hash64(input: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037; // FNV-1a offset basis
    for b in input.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

fn jitter_01(seed: &str) -> f64 {
    let h = hash64(seed);
    (h as f64) / (u64::MAX as f64)
}

fn jitter_range(seed: &str, min: f64, max: f64) -> f64 {
    min + (max - min) * jitter_01(seed)
}

pub fn tick_to_demo_rows(tick: &Tick) -> Result<Vec<DemoRow>> {
    let last_update = parse_ts_to_epoch_millis(&tick.ts)?;

    // Keep the symbol anchored to the real incoming tick, but with much smaller
    // visual ranges than before so the grid looks closer to the reference demo.
    let symbol_center_chg = (tick.price - 100.0) * 0.35;
    let symbol_base_bid = 99.0 + ((tick.price - 100.0) * 0.25);
    let symbol_base_vol = 100.0 + ((tick.size % 1000) as f64 / 1000.0) * 4.0;

    let rows = CLIENTS
        .iter()
        .map(|client| {
            let seed_base = format!("{}|{}|{}", tick.symbol, client, tick.ts);

            // Smaller and more varied per-client moves.
            let chg_jitter = jitter_range(&format!("{seed_base}|chg"), -1.25, 1.25);
            let bid_jitter = jitter_range(&format!("{seed_base}|bid"), -0.65, 0.65);
            let spread = jitter_range(&format!("{seed_base}|spread"), 0.25, 0.95);
            let vol_jitter = jitter_range(&format!("{seed_base}|vol"), -3.0, 8.0);

            let chg = round2(symbol_center_chg + chg_jitter);
            let bid = round2((symbol_base_bid + bid_jitter).max(0.01));
            let ask = round2((bid + spread).max(bid + 0.01));
            let vol = round2((symbol_base_vol + vol_jitter).max(1.0));

            DemoRow {
                name: tick.symbol.clone(),
                client: (*client).to_string(),
                last_update,
                chg,
                bid,
                ask,
                vol,
            }
        })
        .collect();

    Ok(rows)
}

pub fn ticks_to_demo_rows(ticks: &[Tick]) -> Result<Vec<DemoRow>> {
    let mut out = Vec::with_capacity(ticks.len() * CLIENTS.len());

    for tick in ticks {
        out.extend(tick_to_demo_rows(tick)?);
    }

    Ok(out)
}

pub fn demo_rows_to_value(rows: &[DemoRow]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| {
                json!({
                    "name": row.name,
                    "client": row.client,
                    "lastUpdate": row.last_update,
                    "chg": row.chg,
                    "bid": row.bid,
                    "ask": row.ask,
                    "vol": row.vol,
                })
            })
            .collect(),
    )
}

// ------------------------------------------------
// UNIT TESTS
// ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Tick;

    #[test]
    fn tick_maps_to_all_clients() {
        let tick = Tick {
            symbol: "MSFT.N".to_string(),
            price: 100.50,
            size: 162,
            ts: "2026-03-09T14:57:12.786Z".to_string(),
        };

        let rows = tick_to_demo_rows(&tick).unwrap();

        assert_eq!(rows.len(), CLIENTS.len());
        assert!(rows.iter().all(|r| r.name == "MSFT.N"));
    }

    #[test]
    fn ticks_map_to_expected_total_rows() {
        let ticks = vec![
            Tick {
                symbol: "AAPL.N".to_string(),
                price: 101.0,
                size: 10,
                ts: "2026-03-09T10:00:00Z".to_string(),
            },
            Tick {
                symbol: "NVDA.N".to_string(),
                price: 102.0,
                size: 20,
                ts: "2026-03-09T10:00:01Z".to_string(),
            },
        ];

        let rows = ticks_to_demo_rows(&ticks).unwrap();
        assert_eq!(rows.len(), ticks.len() * CLIENTS.len());
    }

    #[test]
    fn demo_rows_to_value_contains_expected_keys() {
        let tick = Tick {
            symbol: "AAPL.N".to_string(),
            price: 101.25,
            size: 50,
            ts: "2026-03-09T08:30:00Z".to_string(),
        };

        let rows = tick_to_demo_rows(&tick).unwrap();
        let value = demo_rows_to_value(&rows);

        let arr = value.as_array().expect("expected array");
        assert!(!arr.is_empty());

        let first = arr[0].as_object().expect("expected object row");
        assert!(first.contains_key("name"));
        assert!(first.contains_key("client"));
        assert!(first.contains_key("lastUpdate"));
        assert!(first.contains_key("chg"));
        assert!(first.contains_key("bid"));
        assert!(first.contains_key("ask"));
        assert!(first.contains_key("vol"));
    }

    #[test]
    fn rows_vary_across_clients_for_same_tick() {
        let tick = Tick {
            symbol: "AAPL.N".to_string(),
            price: 100.75,
            size: 123,
            ts: "2026-03-09T08:30:00Z".to_string(),
        };

        let rows = tick_to_demo_rows(&tick).unwrap();

        let unique_chg_count = rows
            .iter()
            .map(|r| r.chg.to_bits())
            .collect::<std::collections::HashSet<_>>()
            .len();

        assert!(unique_chg_count > 1, "expected client rows to vary");
    }
}