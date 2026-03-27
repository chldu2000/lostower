use std::io;
use std::time::Duration;

use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod app;
mod tui;
mod book;
mod utils;

use app::{AppState, View};
use tui::event::{Event, EventHandler};
use tui::ui;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut state = AppState::new();

    // Setup event handler
    let tick_rate = Duration::from_millis(250);
    let events = EventHandler::new(tick_rate);

    // Main loop
    while !state.should_quit {
        // Draw UI
        terminal.draw(|frame| ui::render(frame, &state))?;

        // Handle events
        match events.next()? {
            Event::Key(key_event) => handle_key_event(&mut state, key_event),
            Event::Mouse(_mouse_event) => {}
            Event::Resize(_width, _height) => {}
            Event::Tick => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key_event(state: &mut AppState, key_event: crossterm::event::KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => state.quit(),
        KeyCode::Char('h') => state.switch_view(View::Help),
        KeyCode::Char('l') => state.switch_view(View::Library),
        KeyCode::Char('r') => state.switch_view(View::Reader),
        _ => {}
    }
}
