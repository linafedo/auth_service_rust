use crate::route::auth::login::{authentication, registration};
use crate::configuration::{Config};

use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use actix_web::middleware::Logger;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub struct Application {
    server: Server,
    port: u16
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let pg_pool = PgPool::connect(
            &config.database.connection_string().expose_secret()
        )
            .await?;
        let connection = web::Data::new(pg_pool);

        let listener = TcpListener::bind(
            (config.application.host, config.application.port)
        )?;
        let port = listener.local_addr().unwrap().port();

        let server = HttpServer::new(move||
            {
                App::new()
                    .wrap(TracingLogger::default())
                    .route("/registration", web::post().to(registration))
                    .route("/authentication", web::get().to(authentication))
                    .app_data(connection.clone())
            })
            .listen(listener)?
            .run();
        Ok(Self {server, port})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Running application");
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

