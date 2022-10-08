use crate::api::models::ExchangeDetails;
use async_trait::async_trait;

use super::json::JsonStorage;

pub enum StorageType {
    JSON,
    SQL,
}

#[async_trait]
pub trait Storage {
    fn get_storage() -> Self;
    async fn get_last_value(self) -> Option<ExchangeDetails>;
    async fn insert_new_value(self, value: ExchangeDetails) -> Option<()>;
}

