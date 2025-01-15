use reqwest::Client;
use serde_json::{ json, Value, Map };
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature;
use std::error::Error;
use fern::colors::{ Color, ColoredLevelConfig };
use log::LevelFilter;
use solana_sdk::pubkey::Pubkey;
// use solana_client::websocket::{SolanaWebSocket, WebSocketConfig};
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{
  RpcTransactionLogsFilter,
  RpcTransactionLogsConfig,
  RpcAccountInfoConfig,
};
use solana_sdk::commitment_config::{ CommitmentConfig, CommitmentLevel };

use crate::common::constants::PROJECT_NAME;

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
  Ok(response)
}

// Function to format our console logs
pub fn setup_logger() -> Result<(), fern::InitError> {
  let colors = ColoredLevelConfig {
    trace: Color::Cyan,
    debug: Color::Magenta,
    info: Color::Green,
    warn: Color::Red,
    error: Color::BrightRed,
    ..ColoredLevelConfig::new()
  };

  fern::Dispatch
    ::new()
    .format(move |out, message, record| {
      out.finish(
        format_args!(
          "{}[{}] {}",
          chrono::Local::now().format("[%H:%M:%S]"),
          colors.color(record.level()),
          message
        )
      )
    })
    .chain(std::io::stdout())
    .level(log::LevelFilter::Error)
    .level_for(PROJECT_NAME, LevelFilter::Info)
    .apply()?;

  Ok(())
}

pub async fn get_transaction_details(signature: &str) -> Result<Value, Box<dyn Error>> {
  let url = String::from("https://api.mainnet-beta.solana.com");
  let client = Client::new();
  let payload =
    json!({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getTransaction",
    "params": [
      signature,
      {
        "encode": "json",
        "maxSupportedTransactionVersion": 0
      }
    ]
  });

  let response = client.post(url).json(&payload).send().await?.json::<Value>().await?;
  Ok(response)
}

pub fn parse_transaction(transaction: Option<&Value>) -> Map<String, Value> {
  let mut transaction_info = Map::new();

  if let Some(transaction) = transaction {
    if let Some(transaction_map) = transaction.get("transaction").and_then(Value::as_object) {
      let signatures = transaction_map
        .get("signatures")
        .cloned()
        .unwrap_or_else(|| Value::Array(vec![]));
      transaction_info.insert("signatures".to_string(), signatures);

      let message = transaction_map
        .get("message")
        .and_then(Value::as_object)
        .unwrap_or(&Map::new())
        .clone();
      let instructions = message
        .get("instructions")
        .cloned()
        .unwrap_or_else(|| Value::Array(vec![]));
      transaction_info.insert("instructions".to_string(), instructions);

      if let Some(meta) = transaction.get("meta").and_then(Value::as_object) {
        let post_balances = meta
          .get("postBalances")
          .cloned()
          .unwrap_or_else(|| Value::Array(vec![]));
        transaction_info.insert("postBalances".to_string(), post_balances);

        let pre_balances = meta
          .get("preBalances")
          .cloned()
          .unwrap_or_else(|| Value::Array(vec![]));
        transaction_info.insert("preBalances".to_string(), pre_balances);

        let status = meta
          .get("status")
          .cloned()
          .unwrap_or_else(|| Value::Object(Map::new()));
        transaction_info.insert("status".to_string(), status);
      }
    }
  }

  transaction_info
}

pub async fn logs_subscribe(
  ws_url: &str,
  account_address: &str,
  commitment: &str
) -> Result<(), Box<dyn std::error::Error>> {
  // ) -> () {
    const SOLANA_RPC_URL_WS: &str = "wss://api.mainnet-beta.solana.com/";
  let filter = RpcTransactionLogsFilter::Mentions(
    vec![String::from("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C")]
  );
  let config = RpcTransactionLogsConfig {
    commitment: Some(CommitmentConfig::processed()),
  };

  let subscription_result = PubsubClient::logs_subscribe(&SOLANA_RPC_URL_WS, filter, config);
  match subscription_result {
    Ok((tx_log, rx_log)) => {
      loop {
        match rx_log.recv() {
            Ok(response) => {
              println!("{:?}\n", response.value.logs);
            }
            Err(e) => {
              println!("Log Subscription Error: {:?}", e);
            }
        }
      }
    }
    Err(e) => {
      println!("Failed to subscribe to log: {:?}", e);
    }
  }

  Ok(())
}
