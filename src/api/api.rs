use std::str::FromStr;

use async_trait::async_trait;
use chrono::DateTime;

use super::{
    bitpanda::{
        btc::get_all_trades,
        models::{AllTradesPage, TradeWrapper},
    },
    models::{ExchangeDetails, TradeFromApi, TradeOperation},
};

#[async_trait]
pub trait ExchangeApi {
    async fn get_exchange_details() -> ExchangeDetails;
}

#[async_trait]
pub trait TradesApi {
    async fn get_all_trades(self) -> Vec<TradeFromApi>;
    async fn get_btc_exchange(self) -> Vec<ExchangeDetails>;
}

pub struct Api;

#[async_trait]
impl TradesApi for Api {
    async fn get_all_trades(self) -> Vec<TradeFromApi> {
        let trades: AllTradesPage = get_all_trades(None).await;

        let trades_mapped: Vec<TradeFromApi> = trades
            .trade_history
            .into_iter()
            .map(|t| map_trade_to_api_trade(t).unwrap())
            .collect();

        trades_mapped
    }

    async fn get_btc_exchange(self) -> Vec<ExchangeDetails> {
        // let bitpanda_exchange = get_bt
        unimplemented!()
    }
}

fn map_trade_to_api_trade(trade: TradeWrapper) -> Result<TradeFromApi, ()> {
    // TODO Error handling
    let trade = TradeFromApi {
        trade_id: trade.trade.trade_id,
        amount: trade.trade.amount,
        side: if trade.trade.side == "BUY" {
            TradeOperation::BuyLong
        } else {
            TradeOperation::SellLong
        },
        instrument_code: super::models::InstrumentCode::BTC,
        price: trade.trade.price.parse::<f64>().unwrap(),
        time: DateTime::from_str(&trade.trade.time).unwrap(),

        fee_amount: trade.fee.fee_amount,
        fee_currency: trade.fee.fee_currency,
        fee_type: trade.fee.fee_type,
    };

    Ok(trade)
}
