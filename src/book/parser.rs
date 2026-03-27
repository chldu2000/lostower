use crate::book::{content::Book, formats::txt::Charset};

pub trait BookParser {
    fn parse(&self, content: &[u8]) -> Result<Book, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserType {
    Txt,
    Epub,
    Mobi,
}

impl ParserType {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "txt" => Some(Self::Txt),
            "epub" => Some(Self::Epub),
            "mobi" => Some(Self::Mobi),
            _ => None,
        }
    }
}

pub struct BookParserFactory;

impl BookParserFactory {
    pub fn create_parser(parser_type: ParserType) -> Box<dyn BookParser> {
        match parser_type {
            ParserType::Txt => Box::new(crate::book::formats::txt::TxtParser::new()),
            ParserType::Epub => Box::new(crate::book::formats::epub::EpubParser::new()),
            ParserType::Mobi => unimplemented!("MOBI parser not implemented yet"),
        }
    }

    pub fn create_txt_parser_with_charset(charset: Charset) -> Box<dyn BookParser> {
        Box::new(crate::book::formats::txt::TxtParser::with_charset(charset))
    }
}
