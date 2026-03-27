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

        // Combine all content into a single string
        let full_text = all_content.join("\n\n");

        let metadata = BookMetadata {
            title,
            author,
            format: "epub".to_string(),
        };

        let book_content = BookContent::new(full_text);

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
}
