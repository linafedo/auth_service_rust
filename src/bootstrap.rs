use crate::configuration::{Config};
use crate::route::auth::authentication::authentication;
use crate::route::registration::registration::registration;
use crate::api_documentation::ServiceApiDoc;
use crate::error::error_to_json_handler;

use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::{HttpServer, App, web};
use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use tracing::instrument;
use std::fmt::{Debug, Formatter};
use anyhow::Context;

pub struct Application {
    server: Server,
    config: Config,
    pub bind_port: u16,
}

impl Debug for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server with host - {}, and port - {}",
               self.config.application.host,
               self.config.application.port
        )
    }
}

impl Application {
    #[instrument(
        name = "App build",
        skip(config),
        err
    )]
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let pg_pool = PgPool::connect(
            &config.database.connection_string().expose_secret()
        )
            .await?;
        let connection = web::Data::new(pg_pool);
        let auth_data = web::Data::new(config.authentication);

        let listener = TcpListener::bind(
            (config.application.host.clone(), config.application.port.clone())
        )
            .context("Bind for tcp listener failed")?;
        let bind_port = listener.local_addr().unwrap().port();

        let open_api = ServiceApiDoc::openapi();

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .wrap(error_to_json_handler())
                .service(
                    SwaggerUi::new(
                        "/swagger/{_:.*}")
                        .url(
                            "/api-doc/openapi.json",
                            open_api.clone()
                        )
                )
                .service(
                    web::scope("auth_service/v1")
                        .app_data(auth_data.clone())
                        .route(
                            "/registration",
                            web::post().to(registration),
                        )
                        .route(
                            "/authentication",
                            web::get().to(authentication),
                        )
                )
                .app_data(connection.clone())
        })
            .listen(listener)
            .context("Bind listener to connection failed")?
            .run();
        Ok(Self {server, config, bind_port})
    }

    #[instrument(
        name = "Run application",
        err
    )]
    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}