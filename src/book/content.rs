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
    pub chapters: Vec<String>,
    // Character offsets of each chapter start
    pub chapter_offsets: Vec<usize>,
}

impl BookContent {
    pub fn new(full_text: String) -> Self {
        Self {
            full_text: full_text.clone(),
            chapters: vec![full_text],
            chapter_offsets: vec![0],
        }
    }

    pub fn new_with_chapters(full_text: String, chapters: Vec<String>) -> Self {
        let mut chapter_offsets = Vec::with_capacity(chapters.len());
        let mut current_offset = 0;

        for chapter in &chapters {
            chapter_offsets.push(current_offset);
            current_offset += chapter.len();
        }

        Self {
            full_text,
            chapters,
            chapter_offsets,
        }
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

    // Get number of chapters
    pub fn chapter_count(&self) -> usize {
        self.chapters.len()
    }

    // Get chapter text by index
    pub fn get_chapter(&self, index: usize) -> Option<&str> {
        self.chapters.get(index).map(|s| s.as_str())
    }

    // Find chapter index containing the given character offset
    pub fn chapter_index_for_offset(&self, offset: usize) -> usize {
        // Find the last chapter offset that is <= the given offset
        for i in (0..self.chapter_offsets.len()).rev() {
            if self.chapter_offsets[i] <= offset {
                return i;
            }
        }
        0 // Fallback to first chapter
    }

    // Get start and end character offsets for a chapter
    pub fn chapter_boundaries(&self, chapter_index: usize) -> (usize, usize) {
        let start = self.chapter_offsets.get(chapter_index).copied().unwrap_or(0);
        let end = self.chapter_offsets.get(chapter_index + 1).copied().unwrap_or(self.full_text.len());
        (start, end)
    }

    // Calculate pages for specific chapter based on window height
    pub fn pages_for_chapter(&self, chapter_index: usize, lines_per_page: usize) -> Vec<String> {
        if let Some(chapter) = self.chapters.get(chapter_index) {
            let lines: Vec<&str> = chapter.split('\n').collect();
            lines.chunks(lines_per_page)
                .map(|chunk| chunk.join("\n"))
                .collect()
        } else {
            Vec::new()
        }
    }

    // Calculate total pages for specific chapter
    pub fn page_count_for_chapter(&self, chapter_index: usize, lines_per_page: usize) -> usize {
        if let Some(chapter) = self.chapters.get(chapter_index) {
            let lines: Vec<&str> = chapter.split('\n').collect();
            let (quotient, remainder) = (lines.len() / lines_per_page, lines.len() % lines_per_page);
            quotient + if remainder > 0 { 1 } else { 0 }
        } else {
            0
        }
    }
}

impl fmt::Display for BookContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} characters, {} chapters", self.full_text.len(), self.chapters.len())
    }
}

impl Book {
    pub fn new(metadata: BookMetadata, content: BookContent) -> Self {
        Self { metadata, content }
    }
}
