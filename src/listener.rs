use crate::bybit_api::*;
use crate::notifier::*;
use tokio::time::{sleep, Duration};

pub async fn listen_for_price_changes(
    bybit_get_market_tickers_url: &str,
    webhook_url: &str,
    timeframe_in_seconds: u64,
    polling_interval_in_millis: u64,
    target_percent_change: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut initial_data: CryptoPairsList = get_price_data(bybit_get_market_tickers_url).await?;

    println!(
        "LISTENING FOR CRYPTO PRICE CHANGES >= {}% in <= {}sec ...\n",
        target_percent_change, timeframe_in_seconds
    );

    loop {
        sleep(Duration::from_millis(polling_interval_in_millis)).await;

        let current_data = get_price_data(bybit_get_market_tickers_url).await?;

        alert_on_price_changes(
            webhook_url,
            &mut initial_data,
            &current_data,
            target_percent_change,
        )
        .await;
        refresh_price_data(&mut initial_data, &current_data, timeframe_in_seconds);
    }
}

fn refresh_price_data(
    initial_data: &mut CryptoPairsList,
    current_data: &CryptoPairsList,
    interval_seconds: u64,
) {
    initial_data
        .items
        .iter_mut()
        .for_each(|(key, initial_pair)| {
            if initial_pair.alert_triggered
                || is_time_passed(
                    initial_pair.timestamp,
                    current_data.items[key].timestamp,
                    interval_seconds,
                )
            {
                *initial_pair = current_data.items[key].clone();
            }
        });
}

fn is_time_passed(start_timestamp: u64, end_timestamp: u64, interval_seconds: u64) -> bool {
    end_timestamp - start_timestamp >= interval_seconds * 1000
}
