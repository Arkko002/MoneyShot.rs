use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename(deserialize = "Realtime Currency Exchange Rate"))]
    pub exchangeDetails: RealtimeCurrencyExchangeRate
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(deserialize = "Realtime Currency Exchange Rate"))]
pub struct RealtimeCurrencyExchangeRate {
    #[serde(rename(deserialize = "1. From_Currency Code"))]
    pub from_currency: String,

    #[serde(rename(deserialize = "3. To_Currency Code"))]
    pub to_currency: String,

    #[serde(rename(deserialize = "5. Exchange Rate"))]
    pub exchange_rate: String,

    #[serde(rename(deserialize = "6. Last Refreshed"))]
    pub last_refreshed: String,

    #[serde(rename(deserialize = "7. Time Zone"))]
    pub time_zone: String,

    #[serde(rename(deserialize = "8. Bid Price"))]
    pub bid_price: String,

    #[serde(rename(deserialize = "9. Ask Price"))]
    pub ask_price: String,
}
