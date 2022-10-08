use money_shot::get_config;
use serde_json::Value;

// TODO Add bitpanda API support, track amount of btc bought based on price

const BASE_URL: &str = "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency=BTC&to_currency=EUR";

use super::models::{RealtimeCurrencyExchangeRate, ApiResponse};

pub async fn get_btc_to_eur() -> Result<RealtimeCurrencyExchangeRate, ()> {
    let config: Value = get_config().await.unwrap();

    let mut url = format!("{}&apikey=", BASE_URL);
    if let Some(api_key) = config["apiKey"].as_str() {
        url.push_str(api_key);
    }

    let resp: ApiResponse = reqwest::get(url)
        .await.unwrap().json().await.unwrap();

    Ok(resp.exchangeDetails)
}
