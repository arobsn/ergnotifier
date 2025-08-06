use std::env;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenAmount {
    pub token_id: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct Box {
    pub value: u64,
    pub ergo_tree: String,
    pub tokens: Vec<TokenAmount>,
    pub creation_height: u64,
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<Box>,
    pub outputs: Vec<Box>,
    pub height: u64,
}

pub static NODE_URL: Lazy<String> =
    Lazy::new(|| env::var("NODE_URL").expect("NODE_URL must be set"));

pub async fn get_transactions_by_address(
    address: &str,
) -> Result<Vec<Transaction>, reqwest::Error> {
    let url = format!(
        "{}/blockchain/transaction/byAddress?address={}",
        *NODE_URL, address
    );
    let response = reqwest::get(&url).await?.json::<Vec<Transaction>>().await?;

    Ok(response)
}
