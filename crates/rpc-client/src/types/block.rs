use eth_types::H256;
use serde::Deserialize;

use crate::types::transaction::Transaction;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<String>,
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub transactions: Vec<Transaction>
}