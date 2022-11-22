use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;

pub fn create_logger<Sink>(
    name: String,
    env_filter: String,
    sink: Sink
) -> Result<impl Subscriber + Send + Sync, anyhow::Error>
    where Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        sink
    );

    Ok(Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
    )
}

pub fn init_logger(logger: impl Subscriber + Send + Sync) -> Result<(), anyhow::Error> {
    LogTracer::init()?;
    set_global_default(logger)?;
    Ok(())
}