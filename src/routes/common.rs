use crate::DbState;
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

#[tracing::instrument]
pub async fn server_info() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    Json(json!({
        "authors": authors,
        "version": version,
    }))
}

#[tracing::instrument(skip_all)]
pub async fn health_check(State(db): State<DbState>) -> impl IntoResponse {
    let db_status = db.health().await.is_ok();
    Json(json!({
        "database": db_status,
    }))
}
