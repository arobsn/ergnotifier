use dotenvy::dotenv;
use ergnotifier::telemetry;
use tracing::{debug, info, warn};

fn main() {
    // Initialize the tracing subscriber
    telemetry::init(telemetry::default_subscriber());

    // Load environment variables from .env file
    match dotenv() {
        Ok(p) => info!(path = ?p, ".env file loaded successfully"),
        Err(e) if e.not_found() => debug!("No .env file found, skipping."),
        Err(e) => warn!("Could not load .env file: {:?}", e),
    }

    println!("Hello, world!");
}
