// producer\src\main.rs
use anyhow::Result;
use std::env;
use std::time::Duration;

use producer::encode_tick_json;
use producer::tick::next_tick;

#[tokio::main]
async fn main() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let nats_subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "ticks".to_string());
    let symbol = env::var("TICK_SYMBOL").unwrap_or_else(|_| "AAPL".to_string());

    println!("Connecting to NATS at {nats_url} ...");
    let client = async_nats::connect(&nats_url).await?;
    println!("Connected. Publishing ticks to subject '{nats_subject}'.");

    let mut last_price = 100.0_f64;

    loop {
        let tick = next_tick(last_price, &symbol);
        last_price = tick.price;

        let payload = encode_tick_json(&tick)?;
        client
            .publish(nats_subject.clone(), payload.clone().into())
            .await?;

        println!("published: {payload}");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}