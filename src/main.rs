use dotenv::dotenv;
use std::env;
use rpc_client_rs::RpcClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = String::from(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set"));
    let client = RpcClient::new(url);

    rpc(client).await;
}

pub async fn rpc(client: RpcClient) {
    let block_number = client.get_block_number().await;
    match block_number {
        Ok(b) => println!("block_number: {}", b),
        Err(e) => println!("error: {}", e),
    }

    let chain_id = client.get_chain_id().await;
    match chain_id {
        Ok(c) => println!("chain_id: {}", c),
        Err(e) => println!("error: {}", e),
    }
}