use std::error::Error;

use dotenvy::dotenv;
use ergnotifier::{node, telemetry};
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber
    telemetry::init(telemetry::default_subscriber());

    // Load environment variables from .env file
    match dotenv() {
        Ok(p) => info!(path = ?p, ".env file loaded successfully"),
        Err(e) if e.not_found() => debug!("No .env file found."),
        Err(e) => warn!("Error loading .env file: {:?}", e),
    }

    // Check if the node is fully indexed
    node::check_node_index().await?;

    Ok(())
}
