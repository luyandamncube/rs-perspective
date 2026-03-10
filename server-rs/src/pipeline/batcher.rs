// server-rs\src\pipeline\batcher.rs
use anyhow::Result;
use perspective::client::Table;
use tokio::{
    sync::mpsc::Receiver,
    time::{self, MissedTickBehavior},
};
// use tracing::debug;
use tracing::info;

use crate::{
    models::Tick,
    pipeline::flush_policy::FlushPolicy,
    sinks::perspective::apply_rows,
    transforms::demo_rows::ticks_to_demo_rows,
};

pub async fn run_batcher(
    mut rx: Receiver<Tick>,
    table: Table,
    policy: FlushPolicy,
) -> Result<()> {
    let mut interval = time::interval(policy.flush_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut buffer: Vec<Tick> = Vec::with_capacity(policy.max_batch_size);

    loop {
        tokio::select! {
            maybe_tick = rx.recv() => {
                match maybe_tick {
                    Some(tick) => {
                        buffer.push(tick);

                        if buffer.len() >= policy.max_batch_size {
                            flush_buffer(&table, &mut buffer).await?;
                        }
                    }
                    None => {
                        if !buffer.is_empty() {
                            flush_buffer(&table, &mut buffer).await?;
                        }
                        break;
                    }
                }
            }
            _ = interval.tick() => {
                if !buffer.is_empty() {
                    flush_buffer(&table, &mut buffer).await?;
                }
            }
        }
    }

    Ok(())
}

async fn flush_buffer(table: &Table, buffer: &mut Vec<Tick>) -> Result<()> {
    let batch = std::mem::take(buffer);
    let tick_count = batch.len();
    let rows = ticks_to_demo_rows(&batch)?;
    let row_count = rows.len();

    apply_rows(table, &rows).await?;

    info!(tick_count, row_count, "flushed batch to perspective");
    Ok(())
}