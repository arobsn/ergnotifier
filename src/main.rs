use std::error::Error;

use dotenvy::dotenv;
use ergnotifier::{node, telemetry, watcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv()?;

    // Initialize the tracing subscriber
    telemetry::init(telemetry::default_subscriber());

    // Check if the node is fully indexed
    node::check_node_index_status().await?;

    // Spawn the watcher
    watcher::start().await;

    Ok(())
}
