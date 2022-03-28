use std::path::PathBuf;
use async_trait::async_trait;

use money_shot::append_path_to_root;
use tokio::fs;

use crate::api::models::RealtimeCurrencyExchangeRate;

use super::repository::Storage;

pub struct JsonStorage {
    path: PathBuf,
}

#[async_trait]
impl Storage for JsonStorage {
    fn get_storage() -> Self {
        let json_path = append_path_to_root("storage.json");
        return Self {
           path: json_path 
        }
    }

    async fn get_last_value(self) -> Option<crate::api::models::RealtimeCurrencyExchangeRate> {
        todo!()
    }

    async fn insert_new_value(self, value: RealtimeCurrencyExchangeRate) -> Option<()> {
        let value_serialized: String = serde_json::to_string_pretty(&value).unwrap();
        fs::write(self.path, value_serialized).await.unwrap();

        Some(())
    }
}

impl JsonStorage {

}
