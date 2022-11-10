use dotenv::{dotenv};
use std::env;
use dotenvy::var;
use serde::Deserialize;
use serde;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub database_url: String,
    #[serde(default="default_port")]
    pub port: u16
}

fn default_port() -> u16 {
    8000
}

pub fn get_env_config() -> Option<EnvConfig> {
    dotenv::dotenv().expect("Failed to read .env file");
    return match envy::from_env::<EnvConfig>() {
        Ok(config) => {
            println!("Successful get config {config:?}");
            Some(config)
        },
        Err(e) => {
            println!("{}", e);
            None
        },
    }
}