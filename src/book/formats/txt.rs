use crate::book::{
    content::{Book, BookContent, BookMetadata},
    parser::BookParser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    Utf8,
    Gb2312,
    Gbk,
    Gb18030,
}

impl Charset {
    pub fn name(&self) -> &'static str {
        match self {
            Charset::Utf8 => "UTF-8",
            Charset::Gb2312 => "GB2312",
            Charset::Gbk => "GBK",
            Charset::Gb18030 => "GB18030",
        }
    }

    pub fn encoding_label(&self) -> &'static encoding_rs::Encoding {
        match self {
            Charset::Utf8 => encoding_rs::UTF_8,
            // GB2312 is treated as a subset of GBK in encoding_rs
            Charset::Gb2312 => encoding_rs::GBK,
            Charset::Gbk => encoding_rs::GBK,
            Charset::Gb18030 => encoding_rs::GB18030,
        }
    }
}

#[derive(Debug)]
pub struct TxtParser {
    charset: Charset,
}

impl TxtParser {
    pub fn new() -> Self {
        Self {
            charset: Charset::Utf8,
        }
    }

    pub fn with_charset(charset: Charset) -> Self {
        Self { charset }
    }

    pub fn charset(&self) -> Charset {
        self.charset
    }

    pub fn set_charset(&mut self, charset: Charset) {
        self.charset = charset;
    }

    pub fn detect_charset(_content: &[u8]) -> Charset {
        // For now, default to UTF-8 - we'll add detection later
        Charset::Utf8
    }
}

impl Default for TxtParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BookParser for TxtParser {
    fn parse(&self, content: &[u8]) -> Result<Book, Box<dyn std::error::Error>> {
        let text = self.decode_content(content);

        let metadata = BookMetadata {
            title: "Unknown Title".to_string(),
            author: "Unknown Author".to_string(),
            format: "txt".to_string(),
        };

        let pages = self.split_into_pages(&text);
        let content = BookContent::new(pages);

        Ok(Book::new(metadata, content))
    }
}

impl TxtParser {
    fn decode_content(&self, content: &[u8]) -> String {
        let (cow, _, had_errors) = self.charset.encoding_label().decode(content);
        if had_errors {
            eprintln!("Warning: Some characters could not be decoded with {}", self.charset.name());
        }
        cow.to_string()
    }

    fn split_into_pages(&self, text: &str) -> Vec<String> {
        // Simple page splitting: split by newlines or every 50 lines
        let lines_per_page = 50;
        let lines: Vec<&str> = text.split('\n').collect();

        lines.chunks(lines_per_page)
            .map(|chunk| chunk.join("\n"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_parser_basic_utf8() {
        let parser = TxtParser::new();
        let test_content = b"Line 1\nLine 2\nLine 3";
        let book = parser.parse(test_content).expect("Failed to parse TXT content");

        assert_eq!(book.metadata.format, "txt");
        assert!(book.content.page_count() > 0);
    }

    #[test]
    fn test_txt_parser_split_pages() {
        let parser = TxtParser::new();
        let mut test_content = Vec::new();
        for i in 1..=100 {
            test_content.push(format!("Line {}", i));
        }
        let joined_content = test_content.join("\n");
        let test_content_bytes = joined_content.as_bytes();

        let book = parser.parse(test_content_bytes).expect("Failed to parse TXT content");
        assert_eq!(book.content.page_count(), 2); // 100 lines / 50 lines per page
    }

    #[test]
    fn test_txt_parser_empty_content() {
        let parser = TxtParser::new();
        let test_content = b"";
        let book = parser.parse(test_content).expect("Failed to parse TXT content");

        assert_eq!(book.content.page_count(), 1); // Should have at least one page
        assert!(book.content.get_page(0).unwrap().is_empty());
    }

    #[test]
    fn test_charset_names() {
        assert_eq!(Charset::Utf8.name(), "UTF-8");
        assert_eq!(Charset::Gb2312.name(), "GB2312");
        assert_eq!(Charset::Gbk.name(), "GBK");
        assert_eq!(Charset::Gb18030.name(), "GB18030");
    }

    #[test]
    fn test_txt_parser_with_charset() {
        let parser = TxtParser::with_charset(Charset::Gbk);
        assert_eq!(parser.charset(), Charset::Gbk);

        let test_content = b"Test content";
        let book = parser.parse(test_content).expect("Failed to parse with GBK charset");
        assert_eq!(book.metadata.format, "txt");
    }
}
