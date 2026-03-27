use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Reader;

impl Reader {
    pub fn render(frame: &mut Frame, _state: &AppState) {
        let area = frame.area();

        let block = Block::default()
            .title("Reader")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new("Reader view coming soon...")
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
