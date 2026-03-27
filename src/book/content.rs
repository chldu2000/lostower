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
    pub pages: Vec<String>,
}

impl BookContent {
    pub fn new(pages: Vec<String>) -> Self {
        Self { pages }
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_page(&self, index: usize) -> Option<&str> {
        self.pages.get(index).map(String::as_str)
    }
}

impl fmt::Display for BookContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} pages", self.page_count())
    }
}

impl Book {
    pub fn new(metadata: BookMetadata, content: BookContent) -> Self {
        Self { metadata, content }
    }
}
