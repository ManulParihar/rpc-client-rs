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
    let res = client.get_block_number().await.unwrap();
    println!("{res}");
}