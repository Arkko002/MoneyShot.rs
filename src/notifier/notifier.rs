use crate::api::models::RealtimeCurrencyExchangeRate;

pub trait Notifier {
    fn notify(self, details: RealtimeCurrencyExchangeRate) -> Option<()>;
}

