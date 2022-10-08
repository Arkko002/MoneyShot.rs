pub mod app;
pub mod inputs;
pub mod io;

use std::io::stdout;

use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use self::app::{ui, App, AppReturn};
use self::inputs::events::Events;
use self::inputs::InputEvent;
use self::io::IoEvent;

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    // Trigger state change from Init to Initialized
    {
        let mut app = app.lock().await;
        // Here we assume the the first load is a long task
        app.dispatch(IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // Handle inputs
        let result = match events.next().await {
            InputEvent::Input(key) => app.do_action(key).await,
            InputEvent::Tick => app.update_on_tick().await,
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
