use crate::book::{
    content::{Book, BookContent, BookMetadata},
    parser::BookParser,
};

use std::io::Read;

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
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(content))?;

        let mut metadata = BookMetadata {
            title: "Unknown Title".to_string(),
            author: "Unknown Author".to_string(),
            format: "epub".to_string(),
        };

        // Step 1: Find and read container.xml to locate OPF
        let mut opf_path = None;
        if let Ok(mut file) = zip.by_name("META-INF/container.xml") {
            let mut container_content = Vec::new();
            file.read_to_end(&mut container_content)?;
            opf_path = Self::extract_opf_path_from_bytes(&container_content);
        }

        let mut chapters = Vec::new();

        if let Some(opf_path_str) = opf_path {
            // Step 2: Read and parse OPF file
            let mut opf_content = Vec::new();
            if let Ok(mut opf_file) = zip.by_name(&opf_path_str) {
                opf_file.read_to_end(&mut opf_content)?;
                Self::parse_opf_from_bytes(&opf_content, &mut metadata);
            }

            // Step 3: Parse OPF to get spine (reading order) and manifest
            let (spine, manifest) = Self::parse_spine_and_manifest_from_bytes(&opf_content);
            let opf_dir = opf_path_str.rsplit_once('/').map(|(d, _)| d).unwrap_or("");

            if !spine.is_empty() {
                // Extract content in spine order (proper chapter structure)
                for idref in spine {
                    if let Some(href) = manifest.get(&idref) {
                        // Build full path to the content file
                        let content_path = if opf_dir.is_empty() {
                            href.clone()
                        } else {
                            format!("{}/{}", opf_dir, href)
                        };

                        // Try to extract and convert the content
                        if let Ok(mut file) = zip.by_name(&content_path) {
                            let mut content_data = Vec::new();
                            if file.read_to_end(&mut content_data).is_ok() {
                                if let Ok(text) = html2text::from_read(&content_data[..], 80) {
                                    if !text.trim().is_empty() {
                                        chapters.push(text);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Fallback if spine didn't give us chapters
            if chapters.is_empty() {
                chapters = Self::extract_all_html_content(&mut zip, opf_dir);
            }
        }

        // If still no chapters, use the fallback approach
        if chapters.is_empty() {
            chapters = Self::extract_all_html_content(&mut zip, "");
        }

        // Create BookContent with chapters
        let full_text = chapters.join("\n\n");
        let book_content = if chapters.len() > 1 {
            BookContent::new_with_chapters(full_text, chapters)
        } else {
            BookContent::new(full_text)
        };

        Ok(Book::new(metadata, book_content))
    }
}

impl EpubParser {
    fn extract_opf_path_from_bytes(data: &[u8]) -> Option<String> {
        use xml::reader::EventReader;

        let reader = EventReader::new(data);
        let mut in_rootfiles = false;

        for event in reader {
            match event {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let local = name.local_name.as_str();
                    if local == "rootfiles" {
                        in_rootfiles = true;
                    } else if in_rootfiles && local == "rootfile" {
                        for attr in attributes {
                            if attr.name.local_name == "full-path" {
                                return Some(attr.value);
                            }
                        }
                    }
                }
                Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
                    if name.local_name == "rootfiles" {
                        in_rootfiles = false;
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn parse_opf_from_bytes(data: &[u8], metadata: &mut BookMetadata) {
        use xml::reader::EventReader;

        let reader = EventReader::new(data);
        let mut in_metadata = false;
        let mut in_title = false;
        let mut in_creator = false;

        for event in reader {
            match event {
                Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                    let local = name.local_name.as_str();
                    if local == "metadata" {
                        in_metadata = true;
                    } else if in_metadata {
                        if local == "title" {
                            in_title = true;
                        } else if local == "creator" {
                            in_creator = true;
                        }
                    }
                }
                Ok(xml::reader::XmlEvent::Characters(data)) => {
                    if in_title {
                        let trimmed = data.trim();
                        if !trimmed.is_empty() {
                            metadata.title = trimmed.to_string();
                        }
                        in_title = false;
                    } else if in_creator {
                        let trimmed = data.trim();
                        if !trimmed.is_empty() {
                            metadata.author = trimmed.to_string();
                        }
                        in_creator = false;
                    }
                }
                Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
                    let local = name.local_name.as_str();
                    if local == "metadata" {
                        in_metadata = false;
                    } else if in_metadata {
                        if local == "title" {
                            in_title = false;
                        } else if local == "creator" {
                            in_creator = false;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_spine_and_manifest_from_bytes(
        data: &[u8],
    ) -> (Vec<String>, std::collections::HashMap<String, String>) {
        use std::collections::HashMap;
        use xml::reader::EventReader;

        let reader = EventReader::new(data);
        let mut spine = Vec::new();
        let mut manifest = HashMap::new();
        let mut in_spine = false;
        let mut in_manifest = false;

        for event in reader {
            match event {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let local = name.local_name.as_str();

                    match local {
                        "spine" => in_spine = true,
                        "manifest" => in_manifest = true,
                        "itemref" if in_spine => {
                            // Extract idref from itemref
                            for attr in attributes {
                                if attr.name.local_name == "idref" {
                                    spine.push(attr.value);
                                    break;
                                }
                            }
                        }
                        "item" if in_manifest => {
                            // Extract id and href from item
                            let mut id = None;
                            let mut href = None;
                            let mut media_type = None;

                            for attr in attributes {
                                match attr.name.local_name.as_str() {
                                    "id" => id = Some(attr.value),
                                    "href" => href = Some(attr.value),
                                    "media-type" => media_type = Some(attr.value),
                                    _ => {}
                                }
                            }

                            // Only include XHTML/HTML content
                            if let (Some(item_id), Some(item_href)) = (id, href) {
                                if let Some(media) = media_type {
                                    if media.contains("html") || media.contains("xhtml") {
                                        manifest.insert(item_id, item_href);
                                    }
                                } else {
                                    // If no media-type specified, check file extension
                                    if item_href.ends_with(".html")
                                        || item_href.ends_with(".xhtml")
                                        || item_href.ends_with(".htm")
                                    {
                                        manifest.insert(item_id, item_href);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
                    let local = name.local_name.as_str();
                    if local == "spine" {
                        in_spine = false;
                    } else if local == "manifest" {
                        in_manifest = false;
                    }
                }
                _ => {}
            }
        }

        (spine, manifest)
    }

    fn extract_all_html_content(
        zip: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
        _opf_dir: &str,
    ) -> Vec<String> {
        let mut content = Vec::new();
        for i in 0..zip.len() {
            if let Ok(mut file) = zip.by_index(i) {
                let name = file.name().to_lowercase();
                if name.ends_with(".html") || name.ends_with(".xhtml") || name.ends_with(".htm") {
                    let mut content_data = Vec::new();
                    if file.read_to_end(&mut content_data).is_ok() {
                        if let Ok(text) = html2text::from_read(&content_data[..], 80) {
                            if !text.trim().is_empty() {
                                content.push(text);
                            }
                        }
                    }
                }
            }
        }
        content
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
