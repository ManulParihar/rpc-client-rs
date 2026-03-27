use serde_json::Value;

use crate::{JsonRpcRequest, json_rpc::JsonRpcResponse};

pub struct RpcClient {
    url: String,
}

impl RpcClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_block_number(&self) -> Result<String, reqwest::Error> {
        self.request::<String>("eth_blockNumber", vec![]).await
        
    }

    pub async fn get_chain_id(&self) -> Result<String, reqwest::Error> {
        self.request::<String>("eth_chainId", vec![]).await
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
}
