// server-rs\src\main.rs
use std::net::SocketAddr;

use server_rs::{
    app::build_router,
    config::AppConfig,
    nats_consumer::run_nats_consumer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = AppConfig::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));

    let nats_url = cfg.nats_url.clone();
    let nats_subject = cfg.nats_subject.clone();

    tokio::spawn(async move {
        if let Err(err) = run_nats_consumer(nats_url, nats_subject).await {
            tracing::error!(error = %err, "NATS consumer task failed");
        }
    });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("server-rs listening on http://{addr}");
    tracing::info!("HTTP server ready");

    axum::serve(
        listener,
        build_router().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}