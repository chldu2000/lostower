use std::fs;
use std::io;
use std::path::PathBuf;

pub fn read_file_bytes(path: &PathBuf) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn list_books_in_directory(directory: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut books = Vec::new();

    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_str().unwrap_or("").to_lowercase();
                    if ext == "txt" || ext == "epub" || ext == "mobi" {
                        books.push(path);
                    }
                }
            }
        }
    }

    Ok(books)
}

pub fn file_name_without_extension(path: &PathBuf) -> String {
    path.file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("Unknown"))
}

pub fn file_extension(path: &PathBuf) -> String {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_string().to_lowercase())
        .unwrap_or_else(|| String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_list_books_in_directory() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let dir_path = temp_dir.path().to_path_buf();

        File::create(dir_path.join("test.txt"))
            .and_then(|mut f| writeln!(f, "Test content"))
            .expect("Failed to create test file");
        File::create(dir_path.join("test.epub"))
            .and_then(|mut f| writeln!(f, "Test epub"))
            .expect("Failed to create epub file");
        File::create(dir_path.join("test.pdf"))
            .and_then(|mut f| writeln!(f, "Test pdf"))
            .expect("Failed to create pdf file");

        let books = list_books_in_directory(&dir_path).expect("Failed to list books");

        assert_eq!(books.len(), 2);
        assert!(
            books
                .iter()
                .any(|p| p.to_str().unwrap().ends_with("test.txt"))
        );
        assert!(
            books
                .iter()
                .any(|p| p.to_str().unwrap().ends_with("test.epub"))
        );
        assert!(
            !books
                .iter()
                .any(|p| p.to_str().unwrap().ends_with("test.pdf"))
        );
    }

    #[test]
    fn test_file_name_without_extension() {
        let path = PathBuf::from("test_book.txt");
        assert_eq!(file_name_without_extension(&path), "test_book");

        let path = PathBuf::from("document.epub");
        assert_eq!(file_name_without_extension(&path), "document");
    }

    #[test]
    fn test_file_extension() {
        let path = PathBuf::from("test.txt");
        assert_eq!(file_extension(&path), "txt");

        let path = PathBuf::from("book.EPUB");
        assert_eq!(file_extension(&path), "epub");
    }
}
