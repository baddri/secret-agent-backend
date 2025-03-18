use crate::utils::load_env;
use async_trait::async_trait;
use axum::Router;
use axum::{body::Body, response::Response};
use http_body_util::BodyExt;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::env;
use tracing::Level;
use uuid::Uuid;

use crate::{
    get_configuration, get_subscriber, init_subscriber, start_database, startup::build_app,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_level = Level::INFO;

    if env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(default_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(default_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_server() -> Router {
    load_env();

    Lazy::force(&TRACING);

    let mut config = get_configuration().expect("failed to get configuration.");
    config.database.namespace = "tests".to_string();
    config.database.name = Uuid::now_v7().to_string();

    let db = start_database(&config.database)
        .await
        .expect("failed to start database");
    let app = build_app(db, config).expect("failed to build router");
    app
}

#[async_trait]
pub trait BodyToJson {
    async fn json(self) -> Value;
}

#[async_trait]
impl BodyToJson for Response<Body> {
    /// Return `serde_json::Value` of Body
    async fn json(self) -> Value {
        let body = self.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        body
    }
}
