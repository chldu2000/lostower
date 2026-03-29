use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    symbols::scrollbar,
    widgets::{Scrollbar as RatatuiScrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::app::AppState;
use crate::tui::ui::reader::Reader;

pub struct Scrollbar;

impl Scrollbar {
    pub fn render(frame: &mut Frame, state: &AppState, reader: &Reader, area: Rect) {
        // Only show scrollbar if we have a book with content
        let Some(book) = &state.current_book else {
            return;
        };

        // Get current content
        let current_content = if book.content.chapters.len() > 1 {
            book.content.get_chapter(reader.current_chapter)
        } else {
            Some(&book.content.full_text[..])
        };

        let Some(text) = current_content else {
            return;
        };

        let total_lines = text.split('\n').count();
        let available_height = Reader::calculate_lines_per_page(reader.last_known_height);

        // Only show scrollbar if content exceeds available height
        if total_lines <= available_height {
            return;
        }

        // Calculate scrollbar state
        let content_length = total_lines.saturating_sub(available_height);
        let mut scrollbar_state =
            ScrollbarState::new(content_length).position(reader.scroll_offset);

        let track_color = state
            .settings
            .theme
            .parse_color(&state.settings.theme.scrollbar_track);
        let thumb_color = state
            .settings
            .theme
            .parse_color(&state.settings.theme.scrollbar_thumb);
        let scrollbar = RatatuiScrollbar::new(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .end_symbol(None)
            .track_style(Style::default().fg(track_color))
            .thumb_style(Style::default().fg(thumb_color));

        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}
