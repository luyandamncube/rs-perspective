// producer\src\main.rs
use anyhow::Result;
use std::env;
use std::time::Duration;

use producer::encode_tick_json;
use producer::tick::next_tick;

const DEMO_SYMBOLS: &[&str] = &[
    "AAPL.N",
    "AMZN.N",
    "QQQ.N",
    "NVDA.N",
    "TSLA.N",
    "FB.N",
    "MSFT.N",
    "TLT.N",
    "XIV.N",
    "YY.N",
    "CSCO.N",
    "GOOGL.N",
    "PCLN.N",
];

#[tokio::main]
async fn main() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let nats_subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "ticks".to_string());

    // Optional:
    // TICK_SYMBOL=AAPL.N -> only publish that symbol
    let single_symbol = env::var("TICK_SYMBOL").ok();

    // Example:
    // TICK_INTERVAL=250 cargo run -p producer
    let tick_interval_ms: u64 = env::var("TICK_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(250);

    println!("Connecting to NATS at {nats_url} ...");
    let client = async_nats::connect(&nats_url).await?;
    println!("Connected. Publishing ticks to subject '{nats_subject}'.");
    println!("Tick interval: {tick_interval_ms} ms");

    let symbols: Vec<&str> = if let Some(symbol) = single_symbol.as_deref() {
        vec![symbol]
    } else {
        DEMO_SYMBOLS.to_vec()
    };

    let mut last_prices = vec![100.0_f64; symbols.len()];

    loop {
        for (idx, symbol) in symbols.iter().enumerate() {
            let tick = next_tick(last_prices[idx], symbol);
            last_prices[idx] = tick.price;

            let payload = encode_tick_json(&tick)?;
            client
                .publish(nats_subject.clone(), payload.clone().into())
                .await?;

            println!("published: {payload}");
        }

        tokio::time::sleep(Duration::from_millis(tick_interval_ms)).await;
    }
}