// server-rs\src\nats_consumer.rs
use anyhow::Result;
use futures_util::StreamExt;
use tracing::{error, info, warn};

use crate::ticks::{parse_tick, tick_to_row};

pub async fn run_nats_consumer(nats_url: String, subject: String) -> Result<()> {
    info!("connecting to NATS at {}", nats_url);

    let client = async_nats::connect(&nats_url).await?;
    let mut subscriber = client.subscribe(subject.clone()).await?;

    info!("subscribed to NATS subject '{}'", subject);

    while let Some(message) = subscriber.next().await {
        match parse_tick(message.payload.as_ref()) {
            Ok(tick) => {
                let row = tick_to_row(&tick);
                info!(subject = %subject, tick = %row, "validated tick received");
            }
            Err(err) => {
                warn!(subject = %subject, error = %err, "invalid tick skipped");
            }
        }
    }

    error!("NATS subscription stream ended unexpectedly");
    Ok(())
}