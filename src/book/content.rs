use std::fmt;

#[derive(Debug)]
pub struct Book {
    pub metadata: BookMetadata,
    pub content: BookContent,
}

#[derive(Debug)]
pub struct BookMetadata {
    pub title: String,
    pub author: String,
    pub format: String,
}

impl BookMetadata {
    pub fn new(title: String, author: String, format: String) -> Self {
        Self { title, author, format }
    }
}

#[derive(Debug)]
pub struct BookContent {
    pub full_text: String,
}

impl BookContent {
    pub fn new(full_text: String) -> Self {
        Self { full_text }
    }

    // Calculate pages based on window height (lines per page)
    pub fn pages_for_height(&self, lines_per_page: usize) -> Vec<String> {
        let lines: Vec<&str> = self.full_text.split('\n').collect();

        lines.chunks(lines_per_page)
            .map(|chunk| chunk.join("\n"))
            .collect()
    }

    // Get page at specific index for a given page height
    pub fn get_page_for_height(&self, index: usize, lines_per_page: usize) -> Option<String> {
        let pages = self.pages_for_height(lines_per_page);
        pages.get(index).map(|s| s.to_string())
    }

    // Calculate total pages for a given page height
    pub fn page_count_for_height(&self, lines_per_page: usize) -> usize {
        let lines: Vec<&str> = self.full_text.split('\n').collect();
        let (quotient, remainder) = (lines.len() / lines_per_page, lines.len() % lines_per_page);
        quotient + if remainder > 0 { 1 } else { 0 }
    }
}

impl fmt::Display for BookContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} characters", self.full_text.len())
    }
}

impl Book {
    pub fn new(metadata: BookMetadata, content: BookContent) -> Self {
        Self { metadata, content }
    }
}
