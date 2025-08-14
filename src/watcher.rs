use std::{env, thread::sleep, time::Duration};

use once_cell::sync::Lazy;
use tracing::{error, info};

use crate::{
    node::{self, ErgoTransaction},
    notifier::{self, Notification},
    state,
};

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

        let mut notifications = vec![];
        for tx in untracked_txs {
            if tx.num_confirmations < *ERGO_CONF_NUM {
                continue; // Skip unconfirmed transactions
            }
            let incoming_value = calc_incoming_value(&tx);
            if incoming_value == 0 {
                continue; // Skip transactions with no incoming value
            }

            notifications.push(Notification {
                tx_id: tx.id,
                coin: "ERG",
                wallet: &ERGO_ADDRESS,
                amount: incoming_value,
            });
        }

        if !notifications.is_empty() {
            info!(count = notifications.len(), "Untracked transactions found");

            for notification in notifications {
                let notified = notifier::dispatch(&notification).await;
                if notified {
                    state.last_tx_id = notification.tx_id;
                    info!(last_tx_id = state.last_tx_id, "Updated last_tx_id");
                    let _ = state::save(&state);
                }
            }
        }
    }
}

fn calc_incoming_value(tx: &ErgoTransaction) -> u64 {
    tx.outputs
        .iter()
        .filter(|o| o.address == *ERGO_ADDRESS)
        .map(|o| o.value)
        .sum()
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
