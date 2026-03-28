use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Reader {
    pub scroll_offset: usize,
    pub last_known_height: u16,
    pub current_chapter: usize,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            last_known_height: 52, // Default with borders
            current_chapter: 0,
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

    pub fn next_chapter(&mut self, state: &AppState) {
        if let Some(book) = &state.current_book {
            if self.current_chapter < book.content.chapter_count() - 1 {
                self.current_chapter += 1;
                self.scroll_offset = 0; // Reset scroll for new chapter
            }
        }
    }

    pub fn previous_chapter(&mut self) {
        if self.current_chapter > 0 {
            self.current_chapter -= 1;
            self.scroll_offset = 0; // Reset scroll for new chapter
        }
    }

    pub fn render(frame: &mut Frame, state: &AppState, reader: &mut Reader, area: ratatui::layout::Rect) {
        reader.last_known_height = area.height;

        let block = Block::default()
            .title("Reader")
            .borders(Borders::ALL);

        match &state.current_book {
            Some(book) => {
                let content = if book.content.chapters.len() > 1 {
                    // If we have multiple chapters, display current chapter
                    book.content.get_chapter(reader.current_chapter)
                } else {
                    // If only one chapter, display full text
                    Some(&book.content.full_text[..])
                };

                if let Some(text) = content {
                    let paragraph = Paragraph::new(text)
                        .block(block)
                        .wrap(Wrap { trim: true })
                        .scroll((reader.scroll_offset as u16, 0)); // Scroll vertically

                    frame.render_widget(paragraph, area);
                } else {
                    let paragraph = Paragraph::new("Chapter not found")
                        .block(block)
                        .wrap(Wrap { trim: true });
                    frame.render_widget(paragraph, area);
                }
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
