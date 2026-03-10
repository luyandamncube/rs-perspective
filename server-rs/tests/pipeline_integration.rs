// server-rs\tests\pipeline_integration.rs
use std::time::Duration;

use tokio::sync::mpsc;

use server_rs::{
    models::Tick,
    pipeline::{batcher::run_batcher, flush_policy::FlushPolicy},
    sinks::perspective::PerspectiveState,
};

#[tokio::test]
async fn batcher_flushes_ticks_into_perspective_table() {
    let perspective = PerspectiveState::bootstrap(Some(5000))
        .await
        .expect("failed to bootstrap perspective state");

    let (tx, rx) = mpsc::channel(128);

    let table = perspective.table.clone();
    let policy = FlushPolicy::new(Duration::from_millis(25), 256);

    let batcher = tokio::spawn(async move {
        if let Err(err) = run_batcher(rx, table, policy).await {
            panic!("batcher task failed: {err}");
        }
    });

    let tick = Tick {
        symbol: "AAPL.N".to_string(),
        price: 101.25,
        size: 50,
        ts: "2026-03-09T08:30:00Z".to_string(),
    };

    tx.send(tick)
        .await
        .expect("failed to send tick to batcher");

    tokio::time::sleep(Duration::from_millis(100)).await;

    let size = perspective
        .table
        .size()
        .await
        .expect("failed to read perspective table size");

    assert_eq!(size, 9);

    drop(tx);
    batcher.abort();
}