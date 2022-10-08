use serde::{Deserialize, Serialize};

pub static BASE_URL: &str = "https://api.cryptowat.ch";

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPrice {
    result: MarketPriceResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPriceResult {
    price: i64,
}
