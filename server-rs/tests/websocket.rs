// server-rs\tests\websocket.rs
use axum::body::Body;
use axum::http::{Request, StatusCode, Version};
use perspective::server::Server;
use server_rs::app::build_router;
use tower::util::ServiceExt;

#[tokio::test]
async fn websocket_route_is_present() {
    let perspective_server = Server::new(None);
    let app = build_router(perspective_server);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/ws")
                .version(Version::HTTP_11)
                .header("host", "localhost")
                .header("connection", "upgrade")
                .header("upgrade", "websocket")
                .header("sec-websocket-version", "13")
                .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::SWITCHING_PROTOCOLS
            || response.status() == StatusCode::UPGRADE_REQUIRED,
        "expected 101 or 426, got {}",
        response.status()
    );
}