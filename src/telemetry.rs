use std::env;

use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn default_subscriber() -> impl Subscriber + Send + Sync {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));
    let stdout_log = tracing_subscriber::fmt::layer();
    Registry::default().with(env_filter).with(stdout_log)
}

pub fn init(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
