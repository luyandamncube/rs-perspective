// server-rs\tests\static_assets.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use perspective::server::Server;
use server_rs::app::build_router;
use tower::util::ServiceExt;

#[tokio::test]
async fn index_html_is_served() {
    let app = build_router(Server::new(None));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn streaming_js_is_served() {
    let app = build_router(Server::new(None));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/streaming.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn missing_static_asset_returns_not_found() {
    let app = build_router(Server::new(None));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}