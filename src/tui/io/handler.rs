use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use log::{error, info};

use crate::{api::models::ExchangeDetails, tui::app::App};

use super::IoEvent;

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
            IoEvent::UpdatePair(details) => self.do_update_exchange_details(details).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<()> {
        info!("Initialize the application");
        let mut app = self.app.lock().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized(); // we could update the app state
        info!("Application initialized");

        Ok(())
    }

    async fn do_upsert_exchange_details(&mut self, details: ExchangeDetails) -> Result<()> {
        info!("Updating pair {}/{}", details.from, details.to);
        let app = self.app.lock().await;
        app.state().upsert_exchange_details(details);

        Ok(())
    }

    /// Just take a little break
    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        info!("Go sleeping for {:?}...", duration);
        tokio::time::sleep(duration).await;
        info!("Wake up !");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.slept();

        Ok(())
    }
}
