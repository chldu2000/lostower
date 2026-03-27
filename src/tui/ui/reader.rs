use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Reader {
    pub current_line: usize,      // Track current line instead of page
    pub lines_per_page: usize,   // Store last known lines per page
}

impl Reader {
    pub fn new() -> Self {
        Self {
            current_line: 0,
            lines_per_page: 50, // Default until we know terminal size
        }
    }

    // Calculate available lines in the terminal area (subtract borders)
    fn calculate_lines_per_page(height: u16) -> usize {
        // Subtract 2 for top and bottom borders
        let available = height.saturating_sub(2);
        if available == 0 {
            1 // At least 1 line to avoid division by zero
        } else {
            available as usize
        }
    }

    pub fn scroll_down(&mut self, book: &crate::book::content::Book) {
        let total_lines: usize = book.content.full_text.split('\n').count();
        let max_line = total_lines.saturating_sub(1);

        // Scroll down by 1 page
        self.current_line = std::cmp::min(self.current_line + self.lines_per_page, max_line);
    }

    pub fn scroll_up(&mut self) {
        // Scroll up by 1 page
        self.current_line = self.current_line.saturating_sub(self.lines_per_page);
    }

    pub fn render(frame: &mut Frame, state: &AppState, reader: &mut Reader) {
        let area = frame.area();
        reader.lines_per_page = Self::calculate_lines_per_page(area.height);

        let block = Block::default()
            .title("Reader")
            .borders(Borders::ALL);

        match &state.current_book {
            Some(book) => {
                let all_lines: Vec<&str> = book.content.full_text.split('\n').collect();
                let start_line = reader.current_line;
                let end_line = std::cmp::min(start_line + reader.lines_per_page, all_lines.len());

                let visible_lines = if start_line < all_lines.len() {
                    &all_lines[start_line..end_line]
                } else {
                    &[]
                };

                let page_content = visible_lines.join("\n");

                let paragraph = Paragraph::new(page_content)
                    .block(block)
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, area);
            }
            None => {
                let paragraph = Paragraph::new("No book loaded")
                    .block(block)
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, area);
            }
        }
    }
}
