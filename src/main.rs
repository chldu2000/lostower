use std::io;
use std::time::Duration;

use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;

mod app;
mod tui;
mod book;
mod utils;

use app::{AppState, View};
use tui::event::{Event, EventHandler};
use tui::ui::{library::Library, reader::Reader, help::Help};
use tui::components::status_bar::StatusBar;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut state = AppState::new();
    let mut library = Library::new();
    let mut reader = Reader::new();

    // Setup event handler
    let tick_rate = Duration::from_millis(250);
    let events = EventHandler::new(tick_rate);

    // Main loop
    while !state.should_quit {
        // Draw UI
        terminal.draw(|frame| {
            match state.current_view {
                View::Library => Library::render(frame, &state, &mut library),
                View::Reader => {
                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(95),
                            Constraint::Percentage(5),
                        ])
                        .split(frame.area());

                    Reader::render(frame, &state, &mut reader, layout[0]);
                    StatusBar::render(frame, &state, &reader, layout[1]);
                },
                View::Help => Help::render(frame, &state),
            }
        })?;

        // Handle events
        match events.next()? {
            Event::Key(key_event) => handle_key_event(
                &mut state,
                key_event,
                &mut library,
                &mut reader
            ),
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

fn handle_key_event(
    state: &mut AppState,
    key_event: crossterm::event::KeyEvent,
    library: &mut Library,
    reader: &mut Reader
) {
    match state.current_view {
        View::Library => {
            match key_event.code {
                KeyCode::Char('q') => state.quit(),
                KeyCode::Char('h') => state.switch_view(View::Help),
                KeyCode::Down | KeyCode::Char('j') => library.next(),
                KeyCode::Up | KeyCode::Char('k') => library.previous(),
                KeyCode::Enter => {
                    if let Some(book_path) = library.get_selected_book() {
                        if let Ok(content) = utils::path::read_file_bytes(book_path) {
                            let extension = utils::path::file_extension(book_path);
                            if let Some(parser_type) = book::parser::ParserType::from_extension(&extension) {
                                let parser = if parser_type == book::parser::ParserType::Txt {
                                    book::parser::BookParserFactory::create_txt_parser_with_charset(state.current_charset)
                                } else {
                                    book::parser::BookParserFactory::create_parser(parser_type)
                                };

                                if let Ok(mut book) = parser.parse(&content) {
                                    // Only set title from filename for TXT files (which don't have metadata)
                                    if parser_type == book::parser::ParserType::Txt {
                                        let title = utils::path::file_name_without_extension(book_path);
                                        book.metadata.title = title;
                                    }
                                    state.load_book(book);
                                    reader.scroll_offset = 0; // Reset scroll offset when loading book
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        View::Reader => {
            match key_event.code {
                KeyCode::Char('q') => state.quit(),
                KeyCode::Char('h') => state.switch_view(View::Help),
                KeyCode::Char('l') => state.switch_view(View::Library),
                KeyCode::Down | KeyCode::Char('j') | KeyCode::PageDown => {
                    reader.scroll_down();
                }
                KeyCode::Up | KeyCode::Char('k') | KeyCode::PageUp => {
                    reader.scroll_up();
                }
                KeyCode::Char('n') => {
                    reader.next_chapter(state);
                }
                KeyCode::Char('p') => {
                    reader.previous_chapter();
                }
                KeyCode::Char('c') => {
                    state.cycle_charset();
                    if let Some(book_path) = library.get_selected_book() {
                        if let Ok(content) = utils::path::read_file_bytes(book_path) {
                            let extension = utils::path::file_extension(book_path);
                            if let Some(parser_type) = book::parser::ParserType::from_extension(&extension) {
                                if parser_type == book::parser::ParserType::Txt {
                                    let parser = book::parser::BookParserFactory::create_txt_parser_with_charset(state.current_charset);
                                    if let Ok(mut book) = parser.parse(&content) {
                                        let title = utils::path::file_name_without_extension(book_path);
                                        book.metadata.title = title;
                                        state.load_book(book);
                                        reader.scroll_offset = 0;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        View::Help => {
            match key_event.code {
                KeyCode::Char('q') => state.quit(),
                KeyCode::Char('l') => state.switch_view(View::Library),
                KeyCode::Char('r') => state.switch_view(View::Reader),
                KeyCode::Esc => state.switch_view(View::Library),
                _ => {}
            }
        }
    }
}
