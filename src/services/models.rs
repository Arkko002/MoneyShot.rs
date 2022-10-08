use crate::api::models::TradeOperation;

#[derive(Debug)]
pub struct TradesReport {
    pub operation: TradeOperation,
    pub price: String,
    pub volume: f64,
    pub price_per_unit: f64,
    pub instrument_code: String,
}

