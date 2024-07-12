
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CryptoPair {
    pub price: f32,
    pub timestamp: u64,
    pub alert_triggered: bool,
}

pub struct CryptoPairsList {
    pub items: HashMap<String, CryptoPair>,
}

pub async fn get_price_data(bybit_get_market_tickers_url: &str) -> Result<CryptoPairsList, Box<dyn std::error::Error>> {
    let response = reqwest::get(bybit_get_market_tickers_url).await?;

    if !response.status().is_success() {
        panic!("Failed to fetch data. HTTP Status: {}", response.status());
    }

    let body = response.text().await?;
    let response_body: Value = serde_json::from_str(&body)?;

    let timestamp = response_body["time"].as_u64().ok_or("Failed to parse timestamp")?;
    let pairs_list = response_body["result"]["list"].as_array().ok_or("Failed to parse pairs list")?;
    let mut crypto_pairs = HashMap::new();

    for pair in pairs_list {
        let pair_name = pair.get("symbol")
            .and_then(|s| s.as_str())
            .ok_or("Failed to get symbol")?
            .to_string();

        let last_price = pair.get("lastPrice")
            .and_then(|lp| lp.as_str())
            .and_then(|lp_str| lp_str.parse::<f32>().ok())
            .ok_or("Failed to convert lastPrice to f32")?;

        crypto_pairs.insert(pair_name, 
            CryptoPair{
                price: last_price,
                timestamp,
                alert_triggered: false,
            }
        );
    }

    Ok(CryptoPairsList {
        items: crypto_pairs
    })
}