use crate::book::{Book, Charset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Library,
    Reader,
    Help,
}

pub struct AppState {
    pub current_view: View,
    pub should_quit: bool,
    pub current_book: Option<Book>,
    pub current_charset: Charset,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::Library,
            should_quit: false,
            current_book: None,
            current_charset: Charset::Utf8,
        }
    }

    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn load_book(&mut self, book: Book) {
        self.current_book = Some(book);
        self.switch_view(View::Reader);
    }

    pub fn cycle_charset(&mut self) {
        self.current_charset = match self.current_charset {
            Charset::Utf8 => Charset::Gb2312,
            Charset::Gb2312 => Charset::Gbk,
            Charset::Gbk => Charset::Gb18030,
            Charset::Gb18030 => Charset::Utf8,
        };
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
