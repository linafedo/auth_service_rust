use crate::route::auth::login::{authentication, registration};
use crate::configuration::{Config};
use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use std::net::TcpListener;

pub struct Application {
    server: Server,
    port: u16
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(
            (config.application.host, config.application.port)
        )?;
        let port = listener.local_addr().unwrap().port();
        let server = HttpServer::new(||
            {
                App::new()
                    .route("/registration", web::post().to(registration))
                    .route("/authentication", web::get().to(authentication))
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

