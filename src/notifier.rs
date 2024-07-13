use crate::bybit_api::CryptoPairsList;
use reqwest::Client;
use std::collections::HashMap;

pub async fn alert_on_price_changes(
    webhook_url: &str,
    initial_data: &mut CryptoPairsList,
    current_data: &CryptoPairsList,
    target_percent_change: f32,
) {
    for (key, initial_pair_data) in &mut initial_data.items {
        let current_pair_data = &current_data.items[key];
        let percent_change = calculate_percentage(initial_pair_data.price, current_pair_data.price);

        if percent_change.abs() >= target_percent_change {
            let seconds_passed =
                calculate_delta_seconds(initial_pair_data.timestamp, current_pair_data.timestamp);

            println!(
                "{} PERCENT CHANGE {:.2}% Price1: {:?} Price2: {:?} Time: {}s",
                key,
                percent_change,
                initial_pair_data.price,
                current_pair_data.price,
                seconds_passed
            );

            notify_webhook(
                webhook_url,
                key,
                percent_change,
                seconds_passed,
                initial_pair_data.price,
                current_pair_data.price,
            )
            .await;

            initial_pair_data.alert_triggered = true;
        }
    }
}

async fn notify_webhook(
    webhook_url: &str,
    ticker: &str,
    percent_change: f32,
    delta_seconds: u64,
    initial_price: f32,
    last_price: f32,
) {
    let client = Client::new();
    let mut body: HashMap<&str, String> = HashMap::new();

    body.insert("ticker", ticker.to_string());
    body.insert("percent_change", percent_change.to_string());
    body.insert("delta_seconds", delta_seconds.to_string());
    body.insert("initial_price", initial_price.to_string());
    body.insert("last_price", last_price.to_string());

    let response = client.post(webhook_url).json(&body).send().await.unwrap();

    if response.status().is_success() {
        println!("Webhook notified. Response status: {}", response.status())
    } else {
        panic!(
            "Failed to send data to webhook. HTTP Status: {}",
            response.status()
        );
    }
}

fn calculate_delta_seconds(start_timestamp: u64, end_timestamp: u64) -> u64 {
    (end_timestamp - start_timestamp) / 1000
}

fn calculate_percentage(old_price: f32, new_price: f32) -> f32 {
    ((new_price - old_price) / old_price) * 100.0
}
