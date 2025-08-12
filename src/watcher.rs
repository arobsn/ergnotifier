use std::{thread::sleep, time::Duration};

use tracing::{error, info};

use crate::node;

#[tracing::instrument]
pub async fn start() -> () {
    let mut last_height = 0;
    loop {
        let height = get_last_indexed_height().await;
        if height == 0 || height == last_height {
            continue;
        }

        info!(height = height, "New block found");
        last_height = height;

        sleep(Duration::from_secs(5));
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
