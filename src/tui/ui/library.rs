use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use std::path::PathBuf;

use crate::app::AppState;

pub struct Library {
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub book_files: Vec<PathBuf>,
}

impl Library {
    pub fn new() -> Self {
        let mut library = Self {
            list_state: ListState::default(),
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            book_files: Vec::new(),
        };
        library.load_books();
        library
    }

    pub fn load_books(&mut self) {
        self.book_files = crate::utils::path::list_books_in_directory(&self.current_dir)
            .unwrap_or_else(|_| Vec::new());
        if !self.book_files.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if !self.book_files.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.book_files.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        if !self.book_files.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.book_files.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn get_selected_book(&self) -> Option<&PathBuf> {
        self.list_state.selected().and_then(|i| self.book_files.get(i))
    }

    pub fn render(frame: &mut Frame, _state: &AppState, library: &mut Library) {
        let area = frame.area();

        let block = Block::default()
            .title(format!("Library - {}", library.current_dir.to_string_lossy()))
            .borders(Borders::ALL);

        let items: Vec<ListItem> = library.book_files
            .iter()
            .map(|path| {
                let name = crate::utils::path::file_name_without_extension(path);
                ListItem::new(name)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut library.list_state);
    }
}
