use std::{thread::sleep, time::Duration};

use tracing::{error, info};

use crate::node;

#[tracing::instrument]
pub async fn start() -> () {
    let mut last_height = 0;
    loop {
        sleep(Duration::from_secs(5));
        let height = get_last_indexed_height().await;
        if last_height == height || height == 0 {
            continue;
        }

        info!(height = height, "New block found");
        last_height = height;
    }
}

async fn get_last_indexed_height() -> u64 {
    match node::get_indexed_height().await {
        Ok(x) => x.indexed_height,
        Err(e) => {
            error!("Failed to get indexed height: {:?}", e);
            0
        }
    }
}
