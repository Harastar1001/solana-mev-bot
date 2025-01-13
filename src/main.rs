mod common;

use common::utils::{fetch_blockchain_data, get_confirmed_signatures_for_address};

#[tokio::main]
async fn main() {
    fetch_blockchain_data();
    match get_confirmed_signatures_for_address(10).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
