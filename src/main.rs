use auth_service::configuration::Config;
use auth_service::telemetry;
use auth_service::bootstrap::Application;
use tokio;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

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

