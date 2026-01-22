// use bitcoincore_rpc::{Client,Auth};
// use bitcoincore_rpc::RpcApi;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    //WONT WORK CUS bitcoincore_rpc expects a Bitcoin Core JSON-RPC node.
    // let rpc_url="";
    // let rpc= Client::new(rpc_url, Auth::None).expect("failed to create client");
    // match rpc.get_block_count(){
    //     Ok(count) => println!("the block count is : {count}"),
    //     Err(e) => println!("the error is {e}"),
    // }
let url = "https://bitcoin-mainnet.g.alchemy.com/v2/xZm_3oHYyW9C_DuXQN7mf";

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getblockcount",
        "params": []
    });

    let res = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    println!("{res:#}");

}
