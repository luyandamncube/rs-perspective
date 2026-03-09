// server-rs\src\main.rs
use std::net::SocketAddr;

use server_rs::{
    app::build_router,
    config::AppConfig,
    nats_consumer::run_nats_consumer,
    perspective::PerspectiveState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = AppConfig::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));

    let perspective = PerspectiveState::bootstrap().await?;
    tracing::info!("Perspective state initialized");

    let nats_url = cfg.nats_url.clone();
    let nats_subject = cfg.nats_subject.clone();
    let table = perspective.table.clone();
    let server = perspective.server.clone();

    tokio::spawn(async move {
        if let Err(err) = run_nats_consumer(nats_url, nats_subject, table).await {
            tracing::error!(error = %err, "NATS consumer task failed");
        }
    });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("server-rs listening on http://{addr}");

    axum::serve(
        listener,
        build_router(server).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}