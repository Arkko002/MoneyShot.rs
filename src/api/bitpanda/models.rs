use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketTickerForInstrument {
    instrument_code: String,
    state: String,
    is_frozen: bool,
    last_price: String,
    best_bid: String,
    best_ask: String,
    high: String,
    low: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTradesPage {
    pub trade_history: Vec<TradeWrapper>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeWrapper {
    pub trade: Trade,
    pub fee: Fee,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub trade_id: String,
    pub order_id: String,
    pub account_id: String,
    pub amount: String,
    pub side: String,
    pub instrument_code: String,
    pub price: String,
    pub time: String,
    pub price_tick_sequence: i64,
    pub sequence: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fee {
    pub fee_amount: String,
    pub fee_percentage: String,
    pub fee_group_id: String,
    pub running_trading_volume: String,
    pub fee_currency: String,
    pub fee_type: String,
}
