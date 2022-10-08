use crate::tui::{
    app::App,
    io::{handler::IoAsyncHandler, IoEvent},
    start_ui,
};
use eyre::Result;
use log::LevelFilter;
use std::{io, sync::Arc};

mod api;
mod cli;
mod lib;
mod notifier;
mod repository;
mod services;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // \u2460 Create a channel for IoEvent
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    // \u2461 Create app
    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // \u2463 Handle I/O
    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    // \u2462 Start UI
    start_ui(&app_ui).await?;

    Ok(())
    // let btc_handle = tokio::spawn(Api.get_btc_exchange());
    // let btc_res = btc_handle.await.unwrap();
    //
    // let trades_handle = tokio::spawn(Api.get_all_trades());
    // let trades_res = trades_handle.await.unwrap();
    //
    // let reports = services::trades::TradesService::generate_trades_reports(trades_res);
    // let cons = TradesService::consolidate_reports(reports);
    //
    // let storage = JsonStorage::get_storage();
    //
    // pretty_print_reports(cons, true);
    // pretty_print_reports(reports, true);
    // let _ok = storage.insert_new_value(btc_res).await;
}
