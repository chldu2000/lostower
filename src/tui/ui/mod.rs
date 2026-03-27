pub mod reader;
pub mod library;
pub mod help;

use ratatui::Frame;

use crate::app::AppState;
use reader::Reader;
use library::Library;
use help::Help;

pub fn render(frame: &mut Frame, state: &AppState) {
    match state.current_view {
        crate::app::View::Library => Library::render(frame, state),
        crate::app::View::Reader => Reader::render(frame, state),
        crate::app::View::Help => Help::render(frame, state),
    }
}
