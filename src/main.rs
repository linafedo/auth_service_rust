mod config;
mod tests;
mod bootstrap;
mod route;

use tokio;
use bootstrap::Application;
use config::Config;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let config = Config::load()?;
    let app = Application::build(config).await?;
    app.run().await?;
    Ok(())
}

