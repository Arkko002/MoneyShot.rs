use money_shot::get_config;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use super::models::{MarketTickerForInstrument, AllTradesPage};

const BASE_URL: &str = "https://api.exchange.bitpanda.com/public/v1";

pub async fn get_btc_to_eur() -> Result<MarketTickerForInstrument, ()> {
    let mut url = BASE_URL.to_owned();
    url.push_str("/public/v1/market-ticker/BTC_EUR");

    let resp: MarketTickerForInstrument = reqwest::get(url)
        .await.unwrap().json().await.unwrap();

    Ok(resp)
}

pub async fn get_all_trades(cursor: Option<&str>) -> AllTradesPage {
    let mut url = BASE_URL.to_owned();
    url.push_str("/account/trades");

    let config: Value = get_config().await.unwrap();

    let client = reqwest::Client::new();

    let token = format!("Bearer {}", config["bitpanda"]["apiKey"].as_str().unwrap());

    let resp: AllTradesPage = client.get(url).header(AUTHORIZATION, token).send()
        .await.unwrap().json().await.unwrap();

    resp
}
