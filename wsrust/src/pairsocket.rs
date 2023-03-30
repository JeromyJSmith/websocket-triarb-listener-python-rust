use std::collections::HashMap;
use tungstenite::{connect, Message};
use url::Url;
use serde_json::Value;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

// Check if arbitrage opportunity
fn check_arbitrage_opportunity(prices: &HashMap<String, (f64, f64)>) -> bool {
  let btc_usdt = prices.get("btcusdt").unwrap();
  let eth_btc = prices.get("ethbtc").unwrap();
  let eth_usdt = prices.get("ethusdt").unwrap();

  // Calculate forward and reverse arbitrage
  let opportunity_1 = (1.0 / btc_usdt.0) * (1.0 / eth_btc.0) * eth_usdt.1;
  let opportunity_2 = btc_usdt.1 * eth_btc.1 * (1.0 / eth_usdt.0);

  // Return arb signal
  opportunity_1 > 1.0001 || opportunity_2 > 1.0001
}

// Listen for prices from Exchange
pub fn price_listener() {

  // Listen for arb opportunities to Binance websocket for a selected pair
  let binance_url = format!("{}/stream?streams=ethbtc@bookTicker/btcusdt@bookTicker/ethusdt@bookTicker", BINANCE_WS_API);
  let (mut socket, _) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

  let mut prices: HashMap<String, (f64, f64)> = HashMap::new();

  loop {
    let msg = socket.read_message().expect("Error reading message");
    let msg = match msg {
        Message::Text(s) => s,
        _ => panic!("Error getting text"),
    };

    let parsed_data: Value = serde_json::from_str(&msg).expect("Unable to parse message");
    let trading_pair = parsed_data["data"]["s"].as_str().unwrap().to_lowercase();
    let best_ask_price = parsed_data["data"]["a"].as_str().unwrap().parse::<f64>().unwrap();
    let best_bid_price = parsed_data["data"]["b"].as_str().unwrap().parse::<f64>().unwrap();

    prices.insert(trading_pair.clone(), (best_ask_price, best_bid_price));

    if prices.len() == 3 {
        let has_opportunity = check_arbitrage_opportunity(&prices);
        if has_opportunity {
            println!("Arbitrage opportunity detected: {:?}", prices);
        }
    }
  }
}