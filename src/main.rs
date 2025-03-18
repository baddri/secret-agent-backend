use std::str::FromStr;

use secret_agent::{get_subscriber, init_subscriber, settings, start_database, startup, utils};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::load_env();

    let config = settings::get_configuration().expect("failed to read configuration!");

    let log_level = Level::from_str(&config.app.log).unwrap();
    let subscriber = get_subscriber(log_level, std::io::stdout);
    init_subscriber(subscriber);

    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to create listener!");

    let db = start_database(&config.database).await?;

    let app = startup::build_app(db, config)?;
    axum::serve(listener, app).await?;

    Ok(())
}
