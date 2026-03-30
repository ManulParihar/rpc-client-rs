use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<String>,
    pub hash: Option<String>,
    pub parent_hash: String
}