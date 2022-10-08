use std::collections::HashMap;

use math::round;

use crate::api::{bitpanda::models::AllTradesPage, models::TradeOperation};

use super::models::TradesReport;

pub trait TradeReportTrait {
    fn generate_trades_reports(trades: AllTradesPage) -> Vec<TradesReport>;
    fn consolidate_reports(reports: Vec<TradesReport>) -> Vec<TradesReport>;
    fn calculate_profits(reports: Vec<TradesReport>) -> f64;
}

pub struct TradesService {}

impl TradeReportTrait for TradesService {
    fn generate_trades_reports(trades: AllTradesPage) -> Vec<TradesReport> {
        let mut reports: Vec<TradesReport> = Vec::new();

        for trade in trades.trade_history.iter() {
            let side = match trade.trade.side.as_str() {
                "BUY" => TradeOperation::BuyLong,
                "SELL" => TradeOperation::SellLong,
                _ => panic!(),
            };

            let rounded_price =
                round::half_away_from_zero(trade.trade.price.parse::<f64>().unwrap(), -2);

            let report = TradesReport {
                operation: side,
                price: rounded_price.to_string(),
                volume: trade.trade.amount.parse::<f64>().unwrap(),
                price_per_unit: 0.0,
                instrument_code: trade.trade.instrument_code.to_owned(),
            };

            reports.push(report);
        }

        reports
    }

    fn consolidate_reports(reports: Vec<TradesReport>) -> Vec<TradesReport> {
        let mut consolidated_buys: HashMap<String, f64> = HashMap::new();
        let buy_reports = reports
            .iter()
            .filter(|r| matches!(r.operation, TradeOperation::BuyLong));

        for report in buy_reports.into_iter() {
            if consolidated_buys.contains_key(&report.price) {
                *consolidated_buys.get_mut(&report.price).unwrap() += report.volume;
            } else {
                consolidated_buys.insert(report.price.clone(), report.volume);
            }
        }

        let mut consolidated_sells: HashMap<String, f64> = HashMap::new();
        let sell_reports = reports
            .iter()
            .filter(|r| matches!(r.operation, TradeOperation::SellLong));

        for report in sell_reports.into_iter() {
            if consolidated_sells.contains_key(&report.price) {
                *consolidated_sells.get_mut(&report.price).unwrap() += report.volume;
            } else {
                consolidated_sells.insert(report.price.clone(), report.volume);
            }
        }

        let mut consolidated_reports: Vec<TradesReport> = Vec::new();

        let mut buy_mapped: Vec<TradesReport> = consolidated_buys
            .iter()
            .map(|(p, v)| TradesReport {
                operation: TradeOperation::BuyLong,
                price: p.to_string(),
                volume: v.to_owned(),
                price_per_unit: 0.0,
                instrument_code: "Code".to_string(),
            })
            .collect();
        let mut sell_mapped: Vec<TradesReport> = consolidated_sells
            .iter()
            .map(|(p, v)| TradesReport {
                operation: TradeOperation::SellLong,
                price: p.to_string(),
                volume: v.to_owned(),
                price_per_unit: 0.0,
                instrument_code: "Code".to_string(),
            })
            .collect();

        consolidated_reports.append(&mut buy_mapped);
        consolidated_reports.append(&mut sell_mapped);

        consolidated_reports
    }

    fn calculate_profits(reports: Vec<TradesReport>) -> f64 {
        let mut profit = 0.0;

        for report in reports.iter() {
            let price_percentage = (report.volume / 1f64) * 100f64;
            let price_paid: f64 = (report.price.parse::<f64>().unwrap() / 100f64) * price_percentage;

            match report.operation {
                TradeOperation::BuyLong => profit += price_paid,
                TradeOperation::SellLong => profit -= price_paid,
            }
        }

        profit
    }
}
