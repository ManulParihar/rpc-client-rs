use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Vec<Value>,
    id: u64,
}

#[derive(Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<T>
}

impl JsonRpcRequest {
    pub fn new(jsonrpc: String, method: String, params: Vec<Value>, id: u64) -> Self {
        Self {
            jsonrpc, method, params, id
        }
    }
}
