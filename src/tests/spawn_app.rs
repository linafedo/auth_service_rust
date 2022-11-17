use crate::bootstrap::Application;
use crate::configuration::Config;
use crate::configuration::Config as app_config;

use tokio;
use std::net::TcpListener;
use uuid::Uuid;
use sqlx::{PgConnection, Connection, PgPool, Executor};

pub struct TestData {
    pub address: String,
    pub db_pool: PgPool,
    pub db_name: String
}

pub async fn spawn_app() -> TestData {
    let mut config = Config::load().expect("Can't load config");
    config.application.port = 0;
    config.database.database_name =  Uuid::new_v4().to_string();

    let pool = get_pg_pool(&config).await;
    let database_name = config.database.database_name.clone();

    let app = Application::build(config).await.expect("Failed to build application.");
    let port = app.port();
    let _ = tokio::spawn(app.run());

    let address = format!("http://localhost:{}", port);
    TestData{ address, db_pool: pool, db_name: database_name }
}

async fn get_pg_pool(config: &Config) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(
        &config.database.connection_string_without_db()
    )
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}


