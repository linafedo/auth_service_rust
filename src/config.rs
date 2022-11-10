use rocket::config::{Environment, Value};
use rocket::Config;
use std::collections::HashMap;
use serde_json::ser::State;
use crate::env_var;

pub fn from_env() -> Result<Config, String> {
    let Ok(environment) = Environment::active() else {
        return Err(String::from(ENV_NOT_FOUND))
    };

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let Some(config) = env_var::get_env_config() else {
        return Err(String::from(ENV_CONFIG_ERROR))
    };

    // todo зачем передавать database?
    database_config.insert(URL_KEY, Value::from(config.database_url));
    databases.insert(POSTGRES_POOL_KEY, Value::from(database_config));

    let config = Config::build(environment)
        .environment(environment)
        .port(config.port)
        .extra(DB_KEY, databases)
        .finalize();

    return match config {
        Ok(config) => Ok(config),
        Err(_) => Err(String::from(ENV_CONFIG_ERROR))
    }
}

// Errors
const ENV_NOT_FOUND: &str = "No environment found";
const ENV_CONFIG_ERROR: &str = "Failed to get env config";
const ENV_BUILD_CONFIG_ERROR: &str = "Build config error";

// Keys
const URL_KEY: &str = "url";
const POSTGRES_POOL_KEY: &str = "diesel_postgres_pool";
const DB_KEY: &str = "databases";
