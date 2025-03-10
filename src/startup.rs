use crate::settings::Settings;
use anyhow::Result;
use axum::{Json, Router, body::Body, routing::get};
use http::Request;
use serde_json::{Value, json};
use std::sync::Arc;
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub async fn run_server(
    listener: TcpListener,
    db: Surreal<Client>,
    config: Settings,
) -> Result<()> {
    let config_state = Arc::new(config);
    let db_state = Arc::new(db);

    let cors_layer = CorsLayer::permissive();
    let trace_layer = TraceLayer::new_for_http().make_span_with(
        |_request: &Request<Body>| tracing::debug_span!("http request", request_id=%Uuid::now_v7()),
    );
    let app = Router::new()
        .route("/", get(server_info))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(config_state)
        .with_state(db_state);

    axum::serve(listener, app).await?;

    Ok(())
}

#[tracing::instrument]
pub async fn server_info() -> Json<Value> {
    tracing::info!("example");
    tracing::warn!("things happened!");
    Json(json!({ "version": 1 }))
}
