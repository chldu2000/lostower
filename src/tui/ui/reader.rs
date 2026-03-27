use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Reader {
    pub scroll_offset: usize,
    pub last_known_height: u16,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            last_known_height: 52, // Default with borders
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

    pub fn scroll_down(&mut self) {
        let lines_per_page = Self::calculate_lines_per_page(self.last_known_height);
        self.scroll_offset += lines_per_page;
    }

    pub fn scroll_up(&mut self) {
        let lines_per_page = Self::calculate_lines_per_page(self.last_known_height);
        if lines_per_page > self.scroll_offset {
            self.scroll_offset = 0;
        } else {
            self.scroll_offset -= lines_per_page;
        }
    }

    pub fn render(frame: &mut Frame, state: &AppState, reader: &mut Reader) {
        let area = frame.area();
        reader.last_known_height = area.height;

        let block = Block::default()
            .title("Reader")
            .borders(Borders::ALL);

        match &state.current_book {
            Some(book) => {
                let paragraph = Paragraph::new(book.content.full_text.as_str())
                    .block(block)
                    .wrap(Wrap { trim: true })
                    .scroll((reader.scroll_offset as u16, 0)); // Scroll vertically

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
