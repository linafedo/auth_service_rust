use anyhow::Context;
use tracing::{Subscriber};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing_appender::rolling;
use serde::{Deserialize, Serialize};
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
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
) -> Result<((impl Subscriber + Send + Sync), WorkerGuard), anyhow::Error> {
    let level_str = serde_json::to_string(&level).context(
        "Convert level filter for logging failed"
    )?;
    let level_str = level_str.as_str().trim_matches('\"');

    let env_filter = EnvFilter::new(level_str);

    let log_file = rolling::hourly("./logs", "log");
    let (non_blocking, guard) = tracing_appender::non_blocking(log_file);
    let registry = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            name.into(),
            non_blocking)
        );
    Ok((registry, guard))
}

pub fn init_logger(logger: impl Subscriber + Send + Sync) -> Result<(), anyhow::Error> {
    LogTracer::init().context("Init for log tracing failed")?;
    set_global_default(logger).context("Set log subscriber failed")?;
    Ok(())
}