use once_cell::sync::Lazy;

pub mod node;
pub mod notifier;
pub mod state;
pub mod telemetry;
pub mod watcher;

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");
    client
});
