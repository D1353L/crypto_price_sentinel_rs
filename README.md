# crypto_price_sentinel_rs 
This is a lightweight Rust project that monitors cryptocurrency price changes across all trading pairs over a specified timeframe (in seconds) and reports the percentage change to data consumers via webhook.

# 📈 Project Overview
This tool continuously tracks crypto pairs and computes the price difference between two points in time. It sends structured JSON data to a configurable webhook endpoint using the following format:

json
```
{
  "ticker": "BTC-USDT",
  "percent_change": 10,
  "delta_seconds": "60",
  "initial_price": "55000",
  "last_price": "60500"
}
```
# ⚙️ Features
* Monitoring all crypto pairs
* Configurable timeframe (in seconds)
* Triggering a webhook of the data consumer
* Clear JSON output with key metrics:
  - ticker: Trading pair symbol
  - percent_change: Price percentage difference
  - delta_seconds: Timeframe in seconds
  - initial_price: Price at the beginning
  - last_price: Price at the end

# 📄 Output Example
json
```
{
  "ticker": "ETH-USDT",
  "percent_change": -2.5,
  "delta_seconds": "120",
  "initial_price": "3000",
  "last_price": "2925"
}
```
# 🤝 Contributing
Pull requests are welcome!
For major changes, please open an issue first to discuss what you would like to change.

# 📝 License
This project is licensed under the MIT License. See the LICENSE file for details.
