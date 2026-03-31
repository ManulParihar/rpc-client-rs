use std::{fmt::Display, num::ParseIntError, str::FromStr};

use eth_types::U256;
use serde_json::{Value, json};

use crate::{
    JsonRpcRequest,
    json_rpc::JsonRpcResponse,
    types::block::Block,
};

pub struct RpcClient {
    url: String,
}

#[derive(Debug)]
pub enum RpcError {
    Http(reqwest::Error),
    Parse(ParseIntError),
}

impl RpcClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_block_number(&self) -> Result<u64, RpcError> {
        let res = self.request::<String>("eth_blockNumber", vec![]).await?;
        let num = Self::hex_to_u64(&res)?;
        Ok(num)
    }

    pub async fn get_chain_id(&self) -> Result<u64, RpcError> {
        let id = self.request::<String>("eth_chainId", vec![]).await?;
        let id = Self::hex_to_u64(&id)?;
        Ok(id)
    }

    pub async fn get_balance(&self, address: &str) -> Result<U256, RpcError> {
        let params = vec![json!(address), json!("latest")];
        let balance = self.request::<String>("eth_getBalance", params).await?;
        let balance = U256::from_str(&balance).map_err(|e| RpcError::Parse(e))?;
        Ok(balance)
    }

    pub async fn get_block_by_number(&self, block: u64) -> Result<Option<Block>, RpcError> {
        let hex_block = format!("{:#x}", block);
        let params = vec![json!(hex_block), json!(true)];
        let block = self.request::<Option<Block>>("eth_getBlockByNumber", params).await?;
        Ok(block)
    }

    async fn request<T>(&self, method: &str, params: Vec<Value>) -> Result<T, reqwest::Error> 
    where T: serde::de::DeserializeOwned
    {
        let client = reqwest::Client::new();

        let body = JsonRpcRequest::new(
            "2.0".to_string(),
            method.to_string(),
            params,
            1
        );

        let response = client
            .post(&self.url)
            .json(&body)
            .send()
            .await?;


        let value: JsonRpcResponse<T>= response
            .json()
            .await?;
        
        Ok(value.result.expect("RPC did not return any result"))
    }

    fn hex_to_u64(hex: &str) -> Result<u64, ParseIntError> {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        u64::from_str_radix(hex, 16)
    }
}

impl From<reqwest::Error> for RpcError {
    fn from(value: reqwest::Error) -> Self {
        RpcError::Http(value)
    }
}

impl From<ParseIntError> for RpcError {
    fn from(value: ParseIntError) -> Self {
        RpcError::Parse(value)
    }
}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RpcError::Http(e) => write!(f, "RPC HTTP Error: {}", e),
            RpcError::Parse(e) => write!(f, "RPC Parse Error: {}", e),
        }
    }
}
