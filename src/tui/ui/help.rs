use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Help;

impl Help {
    pub fn render(frame: &mut Frame, state: &AppState) {
        let area = frame.area();

        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL);

        let help_text = [
            "Key Bindings:",
            "  q - Quit",
            "  h - Show this help",
            "  l - Library view",
            "  r - Reader view",
            "  c - Cycle charset (UTF-8 → GB2312 → GBK → GB18030)",
            "  j/PageDown/Down - Next page",
            "  k/PageUp/Up - Previous page",
            "  n - Next chapter",
            "  p - Previous chapter",
            "",
            &format!("Current Charset: {}", state.current_charset.name()),
            "",
            "lostower - Terminal E-book Reader",
        ];

        let paragraph = Paragraph::new(help_text.join("\n"))
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
