use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::AppState;

pub struct Help;

impl Help {
    pub fn render(frame: &mut Frame, state: &AppState) {
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
            .title("Help")
            .borders(Borders::ALL)
            .style(style);

        let help_text = [
            "Key Bindings:",
            "  q - Quit",
            "  h - Show this help",
            "  l - Library view",
            "  m - Bookmarks view",
            "  b - Add bookmark",
            "  / - Start search",
            "  n - Next search match",
            "  p - Previous search match",
            "  c - Cycle charset (UTF-8 → GB2312 → GBK → GB18030)",
            "  j/PageDown/Down - Next page",
            "  k/PageUp/Up - Previous page",
            "  n - Next chapter",
            "  p - Previous chapter",
            "  d - Delete bookmark (in bookmarks view)",
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
