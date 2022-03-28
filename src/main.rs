use repository::{json::JsonStorage, repository::Storage};

mod lib;
mod api;
mod repository;
mod notifier;

#[tokio::main]
async fn main() -> () {
    let btc_handle = tokio::spawn(api::btc::get_btc_to_eur());
    let btc_res = btc_handle.await.unwrap().unwrap();

    let storage = JsonStorage::get_storage();
    let _ok = storage.insert_new_value(btc_res).await;
}
