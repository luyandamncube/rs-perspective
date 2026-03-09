// server-rs\tests\health.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use perspective::server::Server;
use server_rs::app::build_router;
use tower::util::ServiceExt;

#[tokio::test]
async fn health_returns_ok() {
    let app = build_router(Server::new(None));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}