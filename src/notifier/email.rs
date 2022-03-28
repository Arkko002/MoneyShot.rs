use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::api::models::RealtimeCurrencyExchangeRate;

use super::notifier::Notifier;

pub struct EmailNotifier {
    config: serde_json::Value,
    mailer: SmtpTransport,
}

impl EmailNotifier {
    fn new(config: serde_json::Value) -> Self {
        // TODO Creds
        let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        return Self { config, mailer };
    }
}

impl Notifier for EmailNotifier {
    fn notify(self, details: RealtimeCurrencyExchangeRate) -> Option<()> {
        let email_destination = self.config.get("notifier.email").unwrap();
        let destination = format!("Me <{}>", email_destination);
        let body = serde_json::to_string_pretty(&details).unwrap();

        let email = Message::builder()
            .from("MoneyShot <moneyshot@nodomain.com>".parse().unwrap())
            .reply_to(destination.parse().unwrap())
            // TODO separate high, low notifications
            .subject(format!(
                "New high/low: High = {} | Low = {}",
                details.bid_price, details.ask_price
            ))
            .body(body)
            .unwrap();

        match self.mailer.send(&email) {
            Ok(_) => Some(()),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
