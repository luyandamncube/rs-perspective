// server-rs\tests\nats_integration.rs
use std::net::SocketAddr;
use std::time::Duration;

use reqwest::StatusCode;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

use server_rs::app::build_router;

#[tokio::test]
async fn server_starts_with_nats_and_stays_healthy_after_tick_publish() {
    let nats = GenericImage::new("nats", "2.10")
        .with_exposed_port(4222.tcp())
        .with_wait_for(WaitFor::millis(1500))
        .start()
        .await
        .expect("failed to start NATS container");

    let nats_port = nats
        .get_host_port_ipv4(4222.tcp())
        .await
        .expect("failed to resolve mapped NATS port");

    let nats_url = format!("nats://127.0.0.1:{nats_port}");

    let consumer_nats_url = nats_url.clone();
    let consumer_subject = "ticks".to_string();
    tokio::spawn(async move {
        if let Err(err) =
            server_rs::nats_consumer::run_nats_consumer(consumer_nats_url, consumer_subject).await
        {
            panic!("NATS consumer task failed: {err}");
        }
    });

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("failed to bind test listener");

    let addr: SocketAddr = listener.local_addr().expect("failed to resolve listener address");

    let server = tokio::spawn(async move {
        axum::serve(
            listener,
            build_router().into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .expect("test HTTP server failed");
    });

    let http_base = format!("http://{addr}");
    let client = reqwest::Client::new();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let health = client
        .get(format!("{http_base}/health"))
        .send()
        .await
        .expect("failed to call /health");

    assert_eq!(health.status(), StatusCode::OK);
    assert_eq!(health.text().await.expect("failed to read health body"), "ok");

    let publisher = async_nats::connect(&nats_url)
        .await
        .expect("failed to connect publisher to test NATS");

    let payload = serde_json::json!({
        "symbol": "AAPL",
        "price": 101.25,
        "size": 50,
        "ts": "2026-03-09T06:30:00Z"
    })
    .to_string();

    publisher
        .publish("ticks", payload.into())
        .await
        .expect("failed to publish test tick");

    publisher.flush().await.expect("failed to flush published tick");

    tokio::time::sleep(Duration::from_millis(500)).await;

    let health_after = client
        .get(format!("{http_base}/health"))
        .send()
        .await
        .expect("failed to call /health after publish");

    assert_eq!(health_after.status(), StatusCode::OK);
    assert_eq!(
        health_after
            .text()
            .await
            .expect("failed to read post-publish health body"),
        "ok"
    );

    server.abort();
}