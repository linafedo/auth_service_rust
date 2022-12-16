use std::io::stdout;
use tracing::{Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry, Layer};
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use std::{fs::File, sync::Arc, fmt};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub fn create_logger(
    name: String,
    level: Level
) -> Result<impl Subscriber + Send + Sync, anyhow::Error> {
    let env_filter = EnvFilter::new(level.to_string());

    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout
    );

    let file = File::create("debug.log").map_err(|e| { anyhow::Error::from(e) })?;

    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .json();

    Ok(Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer
            .and_then(debug_log)
        )
    )
}

pub fn init_logger(logger: impl Subscriber + Send + Sync) -> Result<(), anyhow::Error> {
    LogTracer::init()?;
    set_global_default(logger)?;
    Ok(())
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}