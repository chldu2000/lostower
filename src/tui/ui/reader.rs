use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::AppState;
use crate::tui::components::scrollbar::Scrollbar;

#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub active: bool,
    pub query: String,
    pub matches: Vec<(usize, usize)>, // (line index, start column)
    pub current_match_index: usize,
}

pub struct Reader {
    pub scroll_offset: usize,
    pub last_known_height: u16,
    pub current_chapter: usize,
    pub search: SearchState,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            last_known_height: 52, // Default with borders
            current_chapter: 0,
            search: SearchState::default(),
        }
    }

    // Calculate available lines in the terminal area (subtract borders)
    pub fn calculate_lines_per_page(height: u16) -> usize {
        // Subtract 2 for top and bottom borders
        let available = height.saturating_sub(2);
        if available == 0 {
            1 // At least 1 line to avoid division by zero
        } else {
            available as usize
        }
    }

    pub fn scroll_down(&mut self, state: &AppState) {
        let lines_per_page = Self::calculate_lines_per_page(self.last_known_height);

        if let Some(book) = &state.current_book {
            let current_content = if book.content.chapters.len() > 1 {
                book.content.get_chapter(self.current_chapter)
            } else {
                Some(&book.content.full_text[..])
            };

            if let Some(text) = current_content {
                // Calculate total lines in current content
                let total_lines = text.split('\n').count();

                // Calculate if we're at or near the end
                let available_height = Self::calculate_lines_per_page(self.last_known_height);

                // If scrolling further would go beyond the content
                if self.scroll_offset + available_height >= total_lines {
                    // If there's a next chapter, navigate to it
                    if self.current_chapter < book.content.chapter_count() - 1 {
                        self.current_chapter += 1;
                        self.scroll_offset = 0;
                    }
                } else {
                    // Otherwise, just scroll down
                    self.scroll_offset += available_height;
                }
            } else {
                // If no content, just scroll (shouldn't happen)
                self.scroll_offset += lines_per_page;
            }
        } else {
            self.scroll_offset += lines_per_page;
        }
    }

    pub fn scroll_up(&mut self, state: &AppState) {
        let lines_per_page = Self::calculate_lines_per_page(self.last_known_height);

        if self.scroll_offset > 0 {
            if lines_per_page > self.scroll_offset {
                self.scroll_offset = 0;
            } else {
                self.scroll_offset -= lines_per_page;
            }
        } else if self.current_chapter > 0 {
            // If at beginning of chapter and previous chapter exists
            self.current_chapter -= 1;

            // Scroll to end of previous chapter
            if let Some(book) = &state.current_book {
                let previous_content = book.content.get_chapter(self.current_chapter);
                if let Some(text) = previous_content {
                    let total_lines = text.split('\n').count();
                    let available_height = Self::calculate_lines_per_page(self.last_known_height);
                    self.scroll_offset = total_lines.saturating_sub(available_height);
                } else {
                    self.scroll_offset = 0;
                }
            }
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

    pub fn start_search(&mut self) {
        self.search.active = true;
        self.search.query.clear();
        self.search.matches.clear();
        self.search.current_match_index = 0;
    }

    pub fn cancel_search(&mut self) {
        self.search.active = false;
        self.search.query.clear();
        self.search.matches.clear();
        self.search.current_match_index = 0;
    }

    pub fn update_search(&mut self, state: &AppState) {
        self.search.matches.clear();
        if self.search.query.is_empty() {
            return;
        }

        if let Some(book) = &state.current_book {
            let content = if book.content.chapters.len() > 1 {
                book.content.get_chapter(self.current_chapter)
            } else {
                Some(&book.content.full_text[..])
            };

            if let Some(text) = content {
                let query_lower = self.search.query.to_lowercase();
                for (line_idx, line) in text.split('\n').enumerate() {
                    let line_lower = line.to_lowercase();
                    let mut start = 0;
                    while let Some(pos) = line_lower[start..].find(&query_lower) {
                        let absolute_pos = start + pos;
                        self.search.matches.push((line_idx, absolute_pos));
                        start = absolute_pos + query_lower.len();
                    }
                }
            }
        }

        if !self.search.matches.is_empty() {
            self.jump_to_current_match();
        }
    }

    pub fn jump_to_current_match(&mut self) {
        if let Some(&(line_idx, _)) = self.search.matches.get(self.search.current_match_index) {
            let available_height = Self::calculate_lines_per_page(self.last_known_height);
            // Calculate scroll offset so that the matched line is visible
            if line_idx >= self.scroll_offset && line_idx < self.scroll_offset + available_height {
                // Already visible, no need to scroll
            } else if line_idx < available_height / 2 {
                self.scroll_offset = 0;
            } else {
                // Center the match vertically
                self.scroll_offset = line_idx.saturating_sub(available_height / 2);
            }
        }
    }

    pub fn next_match(&mut self, _state: &AppState) {
        if self.search.matches.is_empty() {
            return;
        }
        self.search.current_match_index =
            (self.search.current_match_index + 1) % self.search.matches.len();
        self.jump_to_current_match();
    }

    pub fn previous_match(&mut self, _state: &AppState) {
        if self.search.matches.is_empty() {
            return;
        }
        if self.search.current_match_index == 0 {
            self.search.current_match_index = self.search.matches.len() - 1;
        } else {
            self.search.current_match_index -= 1;
        }
        self.jump_to_current_match();
    }

    pub fn render(
        frame: &mut Frame,
        state: &AppState,
        reader: &mut Reader,
        area: ratatui::layout::Rect,
    ) {
        reader.last_known_height = area.height;

        // Split area for content and scrollbar
        let [content_area, scrollbar_area] = Layout::horizontal([
            Constraint::Min(0),    // Content takes all available space except 1 for scrollbar
            Constraint::Length(1), // Scrollbar is 1 character wide
        ])
        .areas(area);

        let mut title = "Reader".to_string();
        if reader.search.active {
            title.push_str(&format!(
                " - Search: \"{}\" ({} matches)",
                reader.search.query,
                reader.search.matches.len()
            ));
        }

        let fg = state
            .settings
            .theme
            .parse_color(&state.settings.theme.foreground_color);
        let bg = state
            .settings
            .theme
            .parse_color(&state.settings.theme.background_color);

        let block = Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .style(Style::default().fg(fg).bg(bg));

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
                    let mut lines: Vec<Line> = Vec::new();
                    let query_len = reader.search.query.len();

                    let fg = state
                        .settings
                        .theme
                        .parse_color(&state.settings.theme.foreground_color);
                    let bg = state
                        .settings
                        .theme
                        .parse_color(&state.settings.theme.background_color);
                    let base_style = Style::default().fg(fg).bg(bg);

                    for (line_idx, line_str) in text.split('\n').enumerate() {
                        if !reader.search.active
                            || reader.search.matches.is_empty()
                            || query_len == 0
                        {
                            lines.push(Line::from(Span::styled(line_str, base_style)));
                            continue;
                        }

                        // Find matches on this line
                        let mut spans = Vec::new();
                        let mut current_pos = 0;

                        for &(match_line, match_col) in &reader.search.matches {
                            if match_line != line_idx {
                                continue;
                            }
                            if match_col >= current_pos {
                                // Add text before the match
                                if match_col > current_pos {
                                    spans.push(Span::styled(
                                        &line_str[current_pos..match_col],
                                        base_style,
                                    ));
                                }
                                // Add highlighted match
                                let end = match_col + query_len;
                                let is_current = reader.search.matches
                                    [reader.search.current_match_index]
                                    == (match_line, match_col);
                                let style = if is_current {
                                    Style::default().bg(Color::Yellow).fg(Color::Black)
                                } else {
                                    Style::default().bg(Color::LightYellow).fg(Color::Black)
                                };
                                spans.push(Span::styled(&line_str[match_col..end], style));
                                current_pos = end;
                            }
                        }

                        // Add remaining text after last match
                        if current_pos < line_str.len() {
                            spans.push(Span::styled(&line_str[current_pos..], base_style));
                        }

                        if spans.is_empty() {
                            spans.push(Span::styled(line_str, base_style));
                        }

                        lines.push(Line::from(spans));
                    }

                    let paragraph = Paragraph::new(lines)
                        .block(block)
                        .wrap(Wrap { trim: true })
                        .scroll((reader.scroll_offset as u16, 0)); // Scroll vertically

                    frame.render_widget(paragraph, content_area);
                    // Render scrollbar
                    Scrollbar::render(frame, state, reader, scrollbar_area);
                } else {
                    let paragraph = Paragraph::new("Chapter not found")
                        .block(block)
                        .wrap(Wrap { trim: true });
                    frame.render_widget(paragraph, content_area);
                }
            }
            None => {
                let paragraph = Paragraph::new("No book loaded")
                    .block(block)
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, content_area);
            }
        }
    }
}
