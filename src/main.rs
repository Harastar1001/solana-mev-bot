mod common;

use common::utils::{
  fetch_blockchain_data,
  get_confirmed_signatures_for_address,
  get_transaction_details,
  parse_transaction,
  logs_subscribe
};

#[tokio::main]
async fn main() {
//   let signature = String::from(
//     "2xFiis5uErY2uzGaMs11wiPfkWBHEaCKpmuJpXhTonxjU4oT2BewpZ9WVXfSswVEyNsj2vif9Zo5YtYt2GSK1aZd"
//   );
//   fetch_blockchain_data();
  // match get_confirmed_signatures_for_address(10).await {
  //     Ok(response) => println!("Response: {:?}", response),
  //     Err(e) => eprintln!("Error: {}", e),
  // }
//   match get_transaction_details(&signature).await {
//     Ok(response) => println!("transation details: {:?}", parse_transaction(response.get("result"))),
//     Err(e) => eprintln!("Error: {}", e),
//   }
  let ws_url = "wss://api.mainnet-beta.solana.com/ws";
//   let ws_url = "https://api.mainnet-beta.solana.com";
  let account_address = "So11111111111111111111111111111111111111112";
  let commitment = "confirmed"; 
  logs_subscribe(ws_url, account_address, commitment).await;
}
