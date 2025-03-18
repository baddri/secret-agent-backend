use axum::body::Body;
use http::{Request, StatusCode};
use secret_agent::utils::test::{BodyToJson, spawn_server};
use tower::ServiceExt;

#[tokio::test]
async fn health_check_should_success() {
    let server = spawn_server().await;

    let res = server
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.json().await;

    assert_eq!(body.get("database").unwrap(), true);
}

#[tokio::test]
async fn server_info_should_success() {
    let server = spawn_server().await;
    let res = server
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.json().await;

    assert_eq!(body.get("version").unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(body.get("authors").unwrap(), env!("CARGO_PKG_AUTHORS"));
}
