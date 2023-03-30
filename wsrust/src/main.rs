mod pairsocket;
mod pairmaker;

use pairsocket::price_listener;
use pairmaker::find_tradeable_pairs;

#[tokio::main]
async fn main() {

    // // Find tradeable pairs and save to Json file
    // match find_tradeable_pairs().await {
    //     Ok(exchange_info) => println!("{:#?}", exchange_info),
    //     Err(error) => eprintln!("Error: {}", error),
    // }

    // Listen for arbitrage opportunities
    price_listener();
}
