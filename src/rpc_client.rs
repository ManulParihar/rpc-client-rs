use serde_json::Value;

pub struct RpcClient {
    url: String,
}

impl RpcClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_block_number(&self) -> Result<String, reqwest::Error> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });
        
        let client = reqwest::Client::new();
        let response = client
            .post(&self.url)
            .json(&body)
            .send()
            .await?;

        let value: Value = response.json().await?;

        let result = value["result"]
            .as_str()
            .unwrap();
        
        Ok(result.to_string())
    }
}
