use crate::api::models::ExchangeDetails;

pub trait Notifier {
    fn notify(self, details: ExchangeDetails) -> Option<()>;
}

