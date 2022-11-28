mod configuration;
mod tests;
mod bootstrap;
mod route;
mod telemetry;
mod utils;

use crate::configuration::Config as app_config;

use tokio;
use bootstrap::Application;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let config = app_config::load()?;

    let logger = telemetry::create_logger(
        "auth_service".into(),
        "debug".into(),
        std::io::stdout
    )?;
    telemetry::init_logger(logger)?;

    let app = Application::build(config).await?;
    app.run().await?;
    Ok(())
}

