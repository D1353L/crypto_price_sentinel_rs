use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bybit_get_market_tickers_url: String,
    pub notifications_webhook_url: String,
    pub timeframe_in_seconds: u64,
    pub polling_interval_millis: u64,
    pub target_percent_change: f32,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
