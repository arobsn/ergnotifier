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

pub async fn get_transactions_by_address(
    address: &str,
    limit: usize,
) -> Result<Vec<Transaction>, reqwest::Error> {
    let url = format!(
        "http://192.168.68.102:9053/blockchain/transaction/byAddress?address={}&limit={}",
        address, limit
    );
    let response = reqwest::get(&url).await?.json::<Vec<Transaction>>().await?;

    Ok(response)
}
