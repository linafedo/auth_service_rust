mod configuration;
mod tests;
mod bootstrap;
mod route;

use crate::configuration::Config as app_config;
use tokio;
use bootstrap::Application;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let config = app_config::load()?;
    let app = Application::build(config).await?;
    app.run().await?;
    Ok(())
}

