use std::time::Duration;

use crate::api::models::ExchangeDetails;

pub mod handler;
// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize, // Launch to initialize the application
    UpdatePair(ExchangeDetails),
    Sleep(Duration), // Just take a little break
}
