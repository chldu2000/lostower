use std::io;
use std::time::Duration;

use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};

mod app;
mod book;
mod tui;
mod utils;

use app::{AppState, View};
use tui::components::status_bar::StatusBar;
use tui::event::{Event, EventHandler};
use tui::ui::{bookmarks::BookmarksView, help::Help, library::Library, reader::Reader};

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
    let mut bookmarks = BookmarksView::new();

    // Setup event handler
    let tick_rate = Duration::from_millis(250);
    let events = EventHandler::new(tick_rate);

    // Main loop
    while !state.should_quit {
        // Draw UI
        terminal.draw(|frame| match state.current_view {
            View::Library => Library::render(frame, &state, &mut library),
            View::Reader => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
                    .split(frame.area());

                Reader::render(frame, &state, &mut reader, layout[0]);
                StatusBar::render(frame, &state, &reader, layout[1]);
            }
            View::Help => Help::render(frame, &state),
            View::Bookmarks => BookmarksView::render(frame, &state, &mut bookmarks),
        })?;

        // Handle events
        match events.next()? {
            Event::Key(key_event) => handle_key_event(
                &mut state,
                key_event,
                &mut library,
                &mut reader,
                &mut bookmarks,
            ),
            Event::Mouse(_mouse_event) => {}
            Event::Resize(_width, _height) => {}
            Event::Tick => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key_event(
    state: &mut AppState,
    key_event: crossterm::event::KeyEvent,
    library: &mut Library,
    reader: &mut Reader,
    bookmarks: &mut BookmarksView,
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
                            if let Some(parser_type) =
                                book::parser::ParserType::from_extension(&extension)
                            {
                                let parser = if parser_type == book::parser::ParserType::Txt {
                                    book::parser::BookParserFactory::create_txt_parser_with_charset(
                                        state.current_charset,
                                    )
                                } else {
                                    book::parser::BookParserFactory::create_parser(parser_type)
                                };

                                if let Ok(mut book) = parser.parse(&content) {
                                    // Only set title from filename for TXT files (which don't have metadata)
                                    if parser_type == book::parser::ParserType::Txt {
                                        let title =
                                            utils::path::file_name_without_extension(book_path);
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
            if reader.search.active {
                // Handle search input
                match key_event.code {
                    KeyCode::Enter => {
                        // Confirm search and keep search active
                        if !reader.search.query.is_empty() {
                            reader.update_search(state);
                        }
                    }
                    KeyCode::Esc => {
                        reader.cancel_search();
                    }
                    KeyCode::Backspace => {
                        if !reader.search.query.is_empty() {
                            reader.search.query.pop();
                            reader.update_search(state);
                        }
                    }
                    KeyCode::Char('n') => {
                        reader.next_match(state);
                    }
                    KeyCode::Char('p') => {
                        reader.previous_match(state);
                    }
                    KeyCode::Char(c) => {
                        reader.search.query.push(c);
                        reader.update_search(state);
                    }
                    _ => {}
                }
            } else {
                // Normal reader controls
                match key_event.code {
                    KeyCode::Char('q') => state.quit(),
                    KeyCode::Char('h') => state.switch_view(View::Help),
                    KeyCode::Char('l') => state.switch_view(View::Library),
                    KeyCode::Char('m') => {
                        state.switch_view(View::Bookmarks);
                        // Initialize selection when opening
                        if !state.settings.bookmarks.is_empty()
                            && bookmarks.list_state.selected().is_none()
                        {
                            bookmarks.list_state.select(Some(0));
                        }
                    }
                    KeyCode::Char('/') => {
                        reader.start_search();
                    }
                    KeyCode::Char('b') => {
                        // Add current position as bookmark
                        if let Some(book) = &state.current_book {
                            if let Some(selected_path) = library.get_selected_book() {
                                let bookmark = crate::app::settings::Bookmark {
                                    book_path: selected_path.to_string_lossy().to_string(),
                                    chapter: reader.current_chapter,
                                    scroll_offset: reader.scroll_offset,
                                    title: book.metadata.title.clone(),
                                };
                                // Check if bookmark already exists for this position, don't add duplicate
                                let exists = state.settings.bookmarks.iter().any(|b| {
                                    b.book_path == bookmark.book_path
                                        && b.chapter == bookmark.chapter
                                        && b.scroll_offset == bookmark.scroll_offset
                                });
                                if !exists {
                                    state.settings.bookmarks.push(bookmark);
                                    let _ = state.settings.save();
                                }
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::PageDown => {
                        reader.scroll_down(&state);
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::PageUp => {
                        reader.scroll_up(&state);
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
                                if let Some(parser_type) =
                                    book::parser::ParserType::from_extension(&extension)
                                {
                                    if parser_type == book::parser::ParserType::Txt {
                                        let parser = book::parser::BookParserFactory::create_txt_parser_with_charset(state.current_charset);
                                        if let Ok(mut book) = parser.parse(&content) {
                                            let title =
                                                utils::path::file_name_without_extension(book_path);
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
        }
        View::Help => match key_event.code {
            KeyCode::Char('q') => state.quit(),
            KeyCode::Char('l') => state.switch_view(View::Library),
            KeyCode::Char('r') => state.switch_view(View::Reader),
            KeyCode::Esc => state.switch_view(View::Library),
            _ => {}
        },
        View::Bookmarks => {
            match key_event.code {
                KeyCode::Char('q') => state.quit(),
                KeyCode::Char('h') => state.switch_view(View::Help),
                KeyCode::Char('l') => state.switch_view(View::Library),
                KeyCode::Char('r') => state.switch_view(View::Reader),
                KeyCode::Esc => state.switch_view(View::Reader),
                KeyCode::Down | KeyCode::Char('j') => bookmarks.next(state),
                KeyCode::Up | KeyCode::Char('k') => bookmarks.previous(state),
                KeyCode::Enter => {
                    // Jump to selected bookmark
                    if let Some(bookmark) = bookmarks.get_selected_bookmark(state) {
                        use std::path::PathBuf;
                        let path = PathBuf::from(&bookmark.book_path);
                        if let Ok(content) = utils::path::read_file_bytes(&path) {
                            let extension = utils::path::file_extension(&path);
                            if let Some(parser_type) =
                                book::parser::ParserType::from_extension(&extension)
                            {
                                let parser = if parser_type == book::parser::ParserType::Txt {
                                    book::parser::BookParserFactory::create_txt_parser_with_charset(
                                        state.current_charset,
                                    )
                                } else {
                                    book::parser::BookParserFactory::create_parser(parser_type)
                                };

                                if let Ok(mut book) = parser.parse(&content) {
                                    if parser_type == book::parser::ParserType::Txt {
                                        let title = utils::path::file_name_without_extension(&path);
                                        book.metadata.title = title;
                                    }
                                    state.load_book(book);
                                    reader.current_chapter = bookmark.chapter;
                                    reader.scroll_offset = bookmark.scroll_offset;
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                    // Delete selected bookmark
                    bookmarks.delete_selected(state);
                    if state.settings.bookmarks.is_empty() {
                        state.switch_view(View::Library);
                    }
                }
                _ => {}
            }
        }
    }
}
