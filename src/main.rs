mod common;

use common::utils::{fetch_blockchain_data, get_confirmed_signatures_for_address, get_transaction_details, parse_transaction};

#[tokio::main]
async fn main() {
    let signature = String::from("2xFiis5uErY2uzGaMs11wiPfkWBHEaCKpmuJpXhTonxjU4oT2BewpZ9WVXfSswVEyNsj2vif9Zo5YtYt2GSK1aZd");
    fetch_blockchain_data();
    // match get_confirmed_signatures_for_address(10).await {
    //     Ok(response) => println!("Response: {:?}", response),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
    match get_transaction_details(&signature).await {
        Ok(response) => println!("transation details: {:?}", parse_transaction(Some(&response))),
        Err(e) => eprintln!("Error: {}", e),
    }
}
