use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String
}
