use serde::Deserialize;
use config::ConfigError;
use std::env::current_dir;
use secrecy::{ExposeSecret, Secret};
use crate::telemetry::Level;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log: Log
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub application: AppConfig,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Log {
    pub level: Option<Level>
}

impl Config {
    pub fn load() -> Result<Config, ConfigError> {
        let base_path = current_dir().map_err(|_| {
            ConfigError::Message("Filled to getting the current directory".to_string())
        })?;
        let config_dir = base_path.join("config");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("configuration.yaml")))
            .build()?;

        config.try_deserialize::<Config>()
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
        "postgres://{}:{}@{}:{}",
        self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}
