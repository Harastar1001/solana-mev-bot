use reqwest::Client;
use serde_json::{ json, Value };
use solana_client::rpc_client::RpcClient;
use std::error::Error;

pub fn fetch_blockchain_data() -> () {
  let url = String::from("https://api.mainnet-beta.solana.com");
  let client = RpcClient::new(url);
  let epoch = client.get_epoch_info().unwrap();

  println!("{:?}", epoch);
}

pub async fn get_confirmed_signatures_for_address(limit: usize) -> Result<Value, Box<dyn Error>> {
  let url = String::from("https://api.mainnet-beta.solana.com");
  let client = Client::new();
  let payload =
    json!({
      "jsonrpc": "2.0",
      "id": 1,
      "method": "getSignaturesForAddress",
      "params": [
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
        {
          "commitment": "confirmed",
          "limit": limit
        }
      ]
  });

  let response = client.post(url).json(&payload).send().await?.json::<Value>().await?;
  print!("{:?}", response);
  Ok(response)
}
