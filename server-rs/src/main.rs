// server-rs\src\main.rs
use std::net::SocketAddr;

use tokio::sync::mpsc;

use server_rs::{
    app::build_router,
    config::AppConfig,
    ingest::nats_consumer::run_nats_consumer,
    pipeline::{batcher::run_batcher, flush_policy::FlushPolicy},
    sinks::perspective::PerspectiveState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt().with_env_filter("info").init();
    tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()),
    )
    .init();

    let cfg = AppConfig::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));

    let perspective = PerspectiveState::bootstrap(cfg.perspective_table_limit).await?;
    tracing::info!("Perspective state initialized");

    let (tx, rx) = mpsc::channel(cfg.queue_capacity);

    let nats_url = cfg.nats_url.clone();
    let nats_subject = cfg.nats_subject.clone();

    tokio::spawn(async move {
        if let Err(err) = run_nats_consumer(nats_url, nats_subject, tx).await {
            tracing::error!(error = %err, "NATS consumer task failed");
        }
    });

    let table = perspective.table.clone();
    let policy = FlushPolicy::new(cfg.batch_flush_interval(), cfg.max_batch_size);

    tokio::spawn(async move {
        if let Err(err) = run_batcher(rx, table, policy).await {
            tracing::error!(error = %err, "batcher task failed");
        }
    });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("server-rs listening on http://{addr}");

    axum::serve(
        listener,
        build_router(perspective.server).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}