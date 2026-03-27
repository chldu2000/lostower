#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Library,
    Reader,
    Help,
}

pub struct AppState {
    pub current_view: View,
    pub should_quit: bool,
    // pub current_book: Option<Book>, // Will be implemented in Phase 2
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::Library,
            should_quit: false,
        }
    }

    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
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
