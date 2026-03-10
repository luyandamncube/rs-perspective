// server-rs\src\nats_consumer.rs
use anyhow::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use tracing::{error, info, warn};

use crate::models::{parse_tick, Tick};

pub async fn run_nats_consumer(
    nats_url: String,
    subject: String,
    tx: Sender<Tick>,
) -> Result<()> {
    info!("connecting to NATS at {}", nats_url);

    let client = async_nats::connect(&nats_url).await?;
    let mut subscriber = client.subscribe(subject.clone()).await?;

    info!("subscribed to NATS subject '{}'", subject);

    while let Some(message) = subscriber.next().await {
        match parse_tick(message.payload.as_ref()) {
            Ok(tick) => {
                tracing::info!(symbol = %tick.symbol, "tick received from NATS");
                if tx.send(tick).await.is_err() {
                    error!("tick channel closed; stopping consumer");
                    break;
                }
            }
            Err(err) => {
                warn!(subject = %subject, error = %err, "invalid tick skipped");
            }
        }
    }

    error!("NATS subscription stream ended unexpectedly");
    Ok(())
}