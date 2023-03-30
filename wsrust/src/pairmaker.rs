use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Debug)]
pub struct TriangularArbitrageCombination {
  buy: String,
  sell: String,
  quote: String,
}

pub async fn find_tradeable_pairs() -> Result<(), Error> {
  let exchange_info_url = "https://api.binance.com/api/v3/exchangeInfo";
  let response = reqwest::get(exchange_info_url)
      .await?
      .json::<Value>()
      .await?;

  if let Some(symbols) = response.get("symbols").and_then(Value::as_array) {
      let mut combinations: Vec<TriangularArbitrageCombination> = Vec::new();

      for base_asset in symbols {
          for quote_asset in symbols {
              for sell_asset in symbols {
                  if base_asset["baseAsset"] == quote_asset["baseAsset"]
                      && base_asset["quoteAsset"] == sell_asset["baseAsset"]
                      && quote_asset["quoteAsset"] == sell_asset["quoteAsset"]
                  {
                      combinations.push(TriangularArbitrageCombination {
                          buy: base_asset["symbol"].as_str().unwrap().to_string(),
                          sell: quote_asset["symbol"].as_str().unwrap().to_string(),
                          quote: sell_asset["symbol"].as_str().unwrap().to_string(),
                      });
                  }
              }
          }
      }

      let json_data = json!(combinations);
      let file = File::create("triangular_arbitrage_combinations.json");
      match file {
          Ok(mut file) => {
              match file.write_all(json_data.to_string().as_bytes()) {
                  Ok(_) => println!("Successfully wrote to file."),
                  Err(e) => println!("Error writing to file: {:?}", e),
              }
          }
          Err(e) => println!("Error creating file: {:?}", e),
      }
  } else {
      eprintln!("Error: symbols field not found or is not an array.");
  }

  Ok(())
}
