use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::{app::AppState, tui::ui::reader::Reader};

pub struct StatusBar;

impl StatusBar {
    pub fn render(frame: &mut Frame, state: &AppState, reader: &Reader, area: ratatui::layout::Rect) {
        let status_text = if let Some(book) = &state.current_book {
            let chapter_info = if book.content.chapter_count() > 1 {
                format!("Chapter {}/{}", reader.current_chapter + 1, book.content.chapter_count())
            } else {
                String::from("Single Chapter")
            };
            format!(
                "{} - {} | {}",
                book.metadata.title,
                book.metadata.author,
                chapter_info
            )
        } else {
            String::from("No book loaded")
        };

        let paragraph = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
