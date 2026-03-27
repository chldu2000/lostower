use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Help;

impl Help {
    pub fn render(frame: &mut Frame, _state: &AppState) {
        let area = frame.area();

        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL);

        let help_text = vec![
            "Key Bindings:",
            "  q - Quit",
            "  h - Show this help",
            "  l - Library view",
            "  r - Reader view",
            "",
            "lostower - Terminal E-book Reader",
        ];

        let paragraph = Paragraph::new(help_text.join("\n"))
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
