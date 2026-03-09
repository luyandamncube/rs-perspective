// server-rs\src\main.rs
use std::net::SocketAddr;

use server_rs::{app::build_router, config::AppConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = AppConfig::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("server-rs listening on http://{addr}");

    axum::serve(
        listener,
        build_router().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}