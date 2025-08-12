use std::{env, thread::sleep, time::Duration};

use once_cell::sync::Lazy;
use tracing::{error, info};

use crate::{node, notifier, state};

pub static ERGO_ADDRESS: Lazy<String> =
    Lazy::new(|| env::var("ERGO_ADDRESS").expect("ERGO_ADDRESS must be set"));

pub static ERGO_CONF_NUM: Lazy<u32> = Lazy::new(|| {
    env::var("ERGO_CONF_NUM")
        .unwrap_or_else(|_| "10".into())
        .parse()
        .unwrap()
});

#[tracing::instrument]
pub async fn start() -> () {
    let mut state = state::load();
    let mut last_height = 0;
    loop {
        sleep(Duration::from_secs(5));
        let height = get_last_indexed_height().await;
        if height == 0 || height == last_height {
            continue;
        }

        info!(height = height, "New block found");
        last_height = height;

        let untracked_txs =
            node::get_untracked_transactions_by_address(&ERGO_ADDRESS, &state.last_tx_id).await;

        let untracked_txs = untracked_txs
            .iter()
            .filter(|tx| tx.num_confirmations > *ERGO_CONF_NUM)
            .map(|tx| tx.id.as_str())
            .collect::<Vec<_>>();

        if !untracked_txs.is_empty() {
            info!(count = untracked_txs.len(), "Untracked transactions found");

            let notified = notifier::dispatch(&untracked_txs).await;
            if notified {
                state.last_tx_id = untracked_txs.first().unwrap().to_string();
                let _ = state::save(&state);
            }
        }
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
