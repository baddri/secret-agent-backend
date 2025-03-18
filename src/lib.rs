pub mod error;
pub mod routes;
pub mod settings;
pub mod startup;
pub mod utils;

use std::sync::Arc;

use axum::{extract::FromRequest, response::IntoResponse};
use error::AppError;
pub use settings::*;

use anyhow::Result;
use secrecy::ExposeSecret;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use tracing::{Level, Subscriber, subscriber::set_global_default};
use tracing_subscriber::{
    Registry,
    filter::Targets,
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

pub type DbState = Arc<Surreal<Client>>;

pub async fn start_database(config: &DatabaseSettings) -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(config.get_url()).await?;

    db.signin(Root {
        username: config.username.as_str(),
        password: config.password.expose_secret(),
    })
    .await?;

    db.use_ns(&config.namespace).use_db(&config.name).await?;
    Ok(db)
}

pub fn get_subscriber<Sink>(
    default_level: Level,
    sink: Sink,
) -> impl Subscriber + Send + Sync + 'static
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    let filter = Targets::new()
        .with_target("tower_http", Level::WARN)
        .with_target("surrealdb", Level::WARN)
        .with_target("axum", Level::WARN)
        .with_default(default_level);

    let format = fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .pretty();

    let fmt_layer = fmt::layer().event_format(format).with_writer(sink);

    Registry::default().with(filter).with(fmt_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
