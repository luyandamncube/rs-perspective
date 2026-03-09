// server-rs\src\nats_consumer.rs
use anyhow::Result;
use futures_util::StreamExt;
use perspective::client::Table;
use tracing::{error, info, warn};

use crate::{
    perspective::apply_tick,
    ticks::{parse_tick, tick_to_row},
};

pub async fn run_nats_consumer(nats_url: String, subject: String, table: Table) -> Result<()> {
    info!("connecting to NATS at {}", nats_url);

    let client = async_nats::connect(&nats_url).await?;
    let mut subscriber = client.subscribe(subject.clone()).await?;

    info!("subscribed to NATS subject '{}'", subject);

    while let Some(message) = subscriber.next().await {
        match parse_tick(message.payload.as_ref()) {
            Ok(tick) => {
                apply_tick(&table, &tick).await?;
                let row = tick_to_row(&tick);
                info!(subject = %subject, tick = %row, "validated tick applied");
            }
            Err(err) => {
                warn!(subject = %subject, error = %err, "invalid tick skipped");
            }
        }
    }

    error!("NATS subscription stream ended unexpectedly");
    Ok(())
}