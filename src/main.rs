use crypto_price_sentinel::config_loader;
use crypto_price_sentinel::listener;

#[tokio::main]
async fn main() {
    let config = config_loader::Config::from_file("config.json").expect("Failed to load config");

    if let Err(e) = listener::listen_for_price_changes(
        &config.bybit_get_market_tickers_url,
        &config.notifications_webhook_url,
        config.timeframe_in_seconds,
        config.polling_interval_millis,
        config.target_percent_change
    ).await {
        eprintln!("Error processing data: {}", e);
    }
}
