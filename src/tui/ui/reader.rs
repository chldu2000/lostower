use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub struct Reader {
    pub current_page: usize,
}

impl Reader {
    pub fn new() -> Self {
        Self { current_page: 0 }
    }

    pub fn next_page(&mut self, book: &crate::book::content::Book) {
        if self.current_page < book.content.page_count() - 1 {
            self.current_page += 1;
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn render(frame: &mut Frame, state: &AppState, reader: &mut Reader) {
        let area = frame.area();

        let block = Block::default()
            .title("Reader")
            .borders(Borders::ALL);

        match &state.current_book {
            Some(book) => {
                if let Some(page) = book.content.get_page(reader.current_page) {
                    let paragraph = Paragraph::new(page)
                        .block(block)
                        .wrap(Wrap { trim: true });
                    frame.render_widget(paragraph, area);
                } else {
                    let paragraph = Paragraph::new("End of book")
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
