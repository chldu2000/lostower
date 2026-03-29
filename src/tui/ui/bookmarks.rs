use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::AppState;
use crate::app::settings::Bookmark;

pub struct BookmarksView {
    pub list_state: ListState,
}

impl BookmarksView {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn next(&mut self, state: &AppState) {
        let bookmarks = &state.settings.bookmarks;
        if !bookmarks.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= bookmarks.len() - 1 {
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

    pub fn previous(&mut self, state: &AppState) {
        let bookmarks = &state.settings.bookmarks;
        if !bookmarks.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        bookmarks.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn get_selected_bookmark(&self, state: &AppState) -> Option<Bookmark> {
        self.list_state
            .selected()
            .and_then(|i| state.settings.bookmarks.get(i).cloned())
    }

    pub fn delete_selected(&mut self, state: &mut AppState) {
        if let Some(i) = self.list_state.selected() {
            state.settings.bookmarks.remove(i);
            let _ = state.settings.save();
            if state.settings.bookmarks.is_empty() {
                self.list_state.select(None);
            } else if i >= state.settings.bookmarks.len() {
                self.list_state
                    .select(Some(state.settings.bookmarks.len() - 1));
            } else {
                self.list_state.select(Some(i));
            }
        }
    }

    pub fn render(frame: &mut Frame, state: &AppState, bookmarks: &mut BookmarksView) {
        let area = frame.area();

        let fg = state
            .settings
            .theme
            .parse_color(&state.settings.theme.foreground_color);
        let bg = state
            .settings
            .theme
            .parse_color(&state.settings.theme.background_color);
        let style = Style::default().fg(fg).bg(bg);

        let block = Block::default()
            .title(format!(
                "Bookmarks - {} saved",
                state.settings.bookmarks.len()
            ))
            .borders(Borders::ALL)
            .style(style);

        let items: Vec<ListItem> = state
            .settings
            .bookmarks
            .iter()
            .map(|bookmark| {
                let title = format!(
                    "{} - Ch:{} Off:{}",
                    bookmark.title,
                    bookmark.chapter + 1,
                    bookmark.scroll_offset
                );
                ListItem::new(title).style(style)
            })
            .collect();

        let list = List::new(items).block(block).highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut bookmarks.list_state);
    }
}
