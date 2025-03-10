use config::Config;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app: AppSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub namespace: String,
}

impl DatabaseSettings {
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub log: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config = Config::builder()
        .add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .unwrap();

    config.try_deserialize()
}
