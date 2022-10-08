use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Hash)]
pub struct ExchangeDetails {
    pub from: InstrumentCode,
    pub to: InstrumentCode,
    pub exchange_source: ExchangeSource,
    pub exchange_rate: String,
    pub datetime: String,
    pub time_zone: String,
    pub bid_price: String,
    pub ask_price: String,
}

impl PartialEq for ExchangeDetails {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
            && self.exchange_source == other.exchange_source
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Eq for ExchangeDetails {}

#[derive(Debug, Serialize, Clone, PartialEq, Hash)]
pub enum ExchangeSource {
    Alpha,
    Bitpanda,
    Cryptowatch,
}

#[derive(Debug, Serialize, Clone, PartialEq, Hash)]
pub enum InstrumentCode {
    EUR,
    USD,
    BTC,
    ETH,
    XMR,
}

impl Display for InstrumentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InstrumentCode::EUR => write!(f, "EUR"),
            InstrumentCode::BTC => write!(f, "BTC"),
            InstrumentCode::USD => write!(f, "USD"),
            InstrumentCode::ETH => write!(f, "ETH"),
            InstrumentCode::XMR => write!(f, "XMR"),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum TradeOperation {
    BuyLong,
    SellLong,
}

#[derive(Debug, Serialize)]
pub struct TradeFromApi {
    pub trade_id: String,
    pub amount: String,
    pub side: TradeOperation,
    pub instrument_code: InstrumentCode,
    pub price: f64,
    pub time: DateTime<Utc>,

    pub fee_amount: String,
    pub fee_currency: String,
    pub fee_type: String,
}
