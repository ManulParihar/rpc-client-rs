use dotenv::dotenv;
use std::env;
use rpc_client::{RpcClient, rpc_client::RpcError};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = String::from(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set"));
    let client = RpcClient::new(url);

    if let Err(e) = rpc(client).await {
        println!("Error: {}", e);
    };
}

pub async fn rpc(client: RpcClient) -> Result<(), RpcError>{
    let block_number = client.get_block_number().await?;
    println!("block_number: {}", block_number);

    let chain_id = client.get_chain_id().await?;
    println!("chain_id: {}", chain_id);

    let balance = client.get_balance("0x0000000000000000000000000000000000000000").await?;
    println!("balance: {}", balance);

    let block = client.get_block_by_number(block_number).await?;
    println!("block by number {}: {}", block_number, block);

    Ok(())
}