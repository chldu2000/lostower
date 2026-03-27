use crate::book::{
    content::{Book, BookContent, BookMetadata},
    parser::BookParser,
};

#[derive(Debug)]
pub struct EpubParser;

impl EpubParser {
    pub fn new() -> Self {
        Self
    }

    fn split_into_pages(&self, text: &str) -> Vec<String> {
        // Use the same page splitting method as TXT parser: 50 lines per page
        let lines_per_page = 50;
        let lines: Vec<&str> = text.split('\n').collect();

        lines.chunks(lines_per_page)
            .map(|chunk| chunk.join("\n"))
            .collect()
    }
}

impl Default for EpubParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BookParser for EpubParser {
    fn parse(&self, content: &[u8]) -> Result<Book, Box<dyn std::error::Error>> {
        use std::io::Cursor;

        // First, create a zip archive from the content
        let mut zip = zip::ZipArchive::new(Cursor::new(content))?;

        // Extract content from all files
        let mut all_content = Vec::new();
        let title = "Unknown Title".to_string();
        let author = "Unknown Author".to_string();

        // Iterate through all files in the zip
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let name = file.name().to_lowercase();

            // Look for HTML files
            if name.ends_with(".html") || name.ends_with(".xhtml") || name.ends_with(".htm") {
                let mut content = Vec::new();
                std::io::Read::read_to_end(&mut file, &mut content)?;

                // Convert HTML to plain text
                if let Ok(plain_text) = html2text::from_read(content.as_slice(), 80) {
                    if !plain_text.trim().is_empty() {
                        all_content.push(plain_text);
                    }
                }
            }
        }

        // Combine all content into a single string and split into pages
        let full_text = all_content.join("\n\n");
        let pages = self.split_into_pages(&full_text);

        // If no pages were extracted, add at least one page
        let final_pages = if pages.is_empty() {
            vec!["No readable content found in EPUB.".to_string()]
        } else {
            pages
        };

        let metadata = BookMetadata {
            title,
            author,
            format: "epub".to_string(),
        };

        let book_content = BookContent::new(final_pages);

        Ok(Book::new(metadata, book_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_parser_basic() {
        let _parser = EpubParser::new();
        assert!(true);
    }

    #[test]
    fn test_epub_parser_page_splitting() {
        let parser = EpubParser::new();
        let mut long_text = Vec::new();
        for i in 1..=100 {
            long_text.push(format!("Line {}", i));
        }
        let test_content = long_text.join("\n");

        let pages = parser.split_into_pages(&test_content);

        // 100 lines should be split into 2 pages of 50 lines each
        assert_eq!(pages.len(), 2);
    }

    #[test]
    fn test_epub_parser_page_splitting_short() {
        let parser = EpubParser::new();
        let test_content = "Line 1\nLine 2\nLine 3";

        let pages = parser.split_into_pages(test_content);

        // Short content should be one page
        assert_eq!(pages.len(), 1);
    }

    #[test]
    fn test_epub_parser_page_splitting_exact() {
        let parser = EpubParser::new();
        let mut long_text = Vec::new();
        for i in 1..=50 {
            long_text.push(format!("Line {}", i));
        }
        let test_content = long_text.join("\n");

        let pages = parser.split_into_pages(&test_content);

        // Exactly 50 lines should be one page
        assert_eq!(pages.len(), 1);
    }
}
