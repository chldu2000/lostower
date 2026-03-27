use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Library;

impl Library {
    pub fn render(frame: &mut Frame, _state: &AppState) {
        let area = frame.area();

        let block = Block::default()
            .title("Library")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new("Library view coming soon...")
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
