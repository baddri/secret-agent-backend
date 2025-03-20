use crate::{
    routes::{
        common::{health_check, server_info},
        ws::ws_handler,
    },
    settings::Settings,
};
use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::MatchedPath,
    http::Request,
    response::Response,
    routing::{any, get},
};
use std::{sync::Arc, time::Duration};
use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::Span;
use uuid::Uuid;

use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};

pub fn build_app(db: Surreal<Client>, config: Settings) -> Result<Router> {
    let config_state = Arc::new(config);
    let db_state = Arc::new(db);

    let cors_layer = CorsLayer::permissive();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let method = request.method();
            let uri = request.uri();
            let request_id = Uuid::now_v7();

            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(|matched_path| matched_path.as_str());

            tracing::debug_span!("http", %method, %uri, matched_path, %request_id)
        })
        .on_request(|_req: &Request<_>, _span: &Span| {
            tracing::info!("<- HTTP Request");
        })
        .on_response(|res: &Response, latency: Duration, _span: &Span| {
            let res_status = res.status().to_string();

            if res.status().is_redirection() {
                let location = res.headers().get("Location").unwrap().to_str().unwrap();
                tracing::info!("{} to {}", res_status, location);
                return;
            }
            if !res.status().is_server_error() {
                tracing::info!("-> {} in {}ms", res_status, latency.as_millis());
                return;
            }
            tracing::error!("{}", res_status);
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                let err_message = match error {
                    ServerErrorsFailureClass::Error(message) => message,
                    _ => "snap! something break".into(),
                };
                tracing::error!(err_message);
            },
        );

    let app = Router::new()
        .route("/", get(server_info))
        .route("/ws", any(ws_handler))
        .route("/health", get(health_check))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(db_state)
        .with_state(config_state);

    Ok(app)
}
