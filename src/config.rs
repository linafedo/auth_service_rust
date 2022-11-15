use dotenv::{dotenv};
use std::io::ErrorKind;
use std::io::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

pub struct Config {
    pub app_config: AppConfig
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        dotenv::dotenv().expect("Failed to read .env file");
        return match envy::from_env::<AppConfig>() {
            Ok(app_config) => {
                println!("Successful get config {app_config:?}");
                Ok(Config{app_config})
            },
            Err(_) => {
                Err(Error::new(ErrorKind::Other, "Building config was failed"))
            },
        }
    }
}
