use tokio;
use std::net::TcpListener;
use crate::bootstrap::Application;
use crate::Config;

pub async fn spawn_app() -> String {
    let mut config = Config::load().expect("Can't load config");
    config.app_config.port = 0;
    let app = Application::build(config).await.expect("Failed to build application.");
    let port = app.port();
    let _ = tokio::spawn(app.run());
    format!("http://localhost:{}", port)
}


