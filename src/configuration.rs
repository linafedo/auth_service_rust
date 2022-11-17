use std::io::Error;
use serde::Deserialize;
use config::ConfigError;
use std::env::current_dir;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub application: AppConfig,
    pub database: DatabaseSettings,
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
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
        "postgres://{}:{}@{}:{}",
        self.username, self.password, self.host, self.port
        )
    }
}
