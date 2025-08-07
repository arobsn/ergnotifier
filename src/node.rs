use std::{boxed::Box, env, error::Error};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct TokenAmount {
    pub token_id: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct ErgoBox {
    pub value: u64,
    pub ergo_tree: String,
    pub tokens: Vec<TokenAmount>,
    pub creation_height: u64,
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Debug, Deserialize)]
pub struct ErgoTransaction {
    pub id: String,
    pub inputs: Vec<ErgoBox>,
    pub outputs: Vec<ErgoBox>,
    pub height: u64,
}

pub static NODE_URL: Lazy<String> =
    Lazy::new(|| env::var("NODE_URL").expect("NODE_URL must be set"));

#[tracing::instrument]
pub async fn get_transactions_by_address(
    address: &str,
) -> Result<Vec<ErgoTransaction>, reqwest::Error> {
    let url = format!(
        "{}/blockchain/transaction/byAddress?address={}",
        *NODE_URL, address
    );
    let response = reqwest::get(&url)
        .await?
        .json::<Vec<ErgoTransaction>>()
        .await?;

    Ok(response)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedHeightResponse {
    pub indexed_height: u64,
    pub full_height: u64,
}

#[tracing::instrument]
pub async fn check_node_index() -> Result<(), Box<dyn Error>> {
    info!("Checking if node is fully indexed...");
    let url = format!("{}/blockchain/indexedHeight", *NODE_URL);
    let resp: IndexedHeightResponse = reqwest::get(&url).await?.json().await?;

    if resp.indexed_height != resp.full_height {
        error!("Node is not fully indexed.");
        return Err(format!(
            "Node is not fully indexed. Indexed height: {}, Full height: {}",
            resp.indexed_height, resp.full_height
        )
        .into());
    }

    Ok(())
}
