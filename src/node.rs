use std::{boxed::Box, env, error::Error};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub token_id: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErgoBox {
    pub value: u64,
    pub ergo_tree: String,
    pub assets: Vec<TokenAmount>,
    pub creation_height: u32,
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Debug, Deserialize)]
pub struct NodeAPIResponse<T> {
    pub items: T,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErgoTransaction {
    pub id: String,
    pub inputs: Vec<ErgoBox>,
    pub outputs: Vec<ErgoBox>,
    pub num_confirmations: u32,
    pub inclusion_height: Option<u32>,
}

pub static NODE_URL: Lazy<String> =
    Lazy::new(|| env::var("NODE_URL").expect("NODE_URL must be set"));

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");
    client
});

#[tracing::instrument]
pub async fn get_transactions_by_address(
    address: &str,
) -> Result<NodeAPIResponse<Vec<ErgoTransaction>>, reqwest::Error> {
    let url = build_url(
        &*NODE_URL,
        &format!("blockchain/transaction/byAddress?address={}", address),
    );
    let response: NodeAPIResponse<Vec<ErgoTransaction>> = HTTP_CLIENT
        .post(url)
        .body(address.to_string())
        .send()
        .await?
        .json()
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
pub async fn get_indexed_height() -> Result<IndexedHeightResponse, Box<dyn Error>> {
    let url = build_url(&*NODE_URL, "blockchain/indexedHeight");
    let resp = HTTP_CLIENT.get(&url).send().await?.json().await?;
    Ok(resp)
}

#[tracing::instrument]
pub async fn check_node_index_status() -> Result<(), Box<dyn Error>> {
    info!("Checking if node is fully indexed...");
    let index_status = get_indexed_height().await?;

    if index_status.indexed_height != index_status.full_height {
        error!("Node is not fully indexed.");
        return Err(format!(
            "Node is not fully indexed. Indexed height: {}, Full height: {}",
            index_status.indexed_height, index_status.full_height
        )
        .into());
    }

    Ok(())
}

fn build_url(base: &str, endpoint: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        endpoint.trim_start_matches('/')
    )
}
