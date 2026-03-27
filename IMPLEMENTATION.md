# Implementation Plan for lostower

## Overview

This document outlines the implementation plan for lostower, a terminal-based e-book reader written in Rust using Ratatui.

## Architecture

### High-Level Components

1. **App State Manager** - Manages application state (current book, reading progress, settings)
2. **TUI Engine** - Handles rendering and user input using Ratatui
3. **Book Parser** - Abstraction layer for parsing different e-book formats
4. **Format-Specific Parsers** - Implementations for epub, mobi, txt, etc.
5. **File I/O** - Handles loading books from the filesystem

### Project Structure

```
src/
├── main.rs                 # Application entry point
├── app/
│   ├── mod.rs             # App module exports
│   ├── state.rs           # Application state management
│   └── settings.rs        # User settings and configuration
├── tui/
│   ├── mod.rs             # TUI module exports
│   ├── event.rs           # Event handling (keyboard, mouse, resize)
│   ├── ui/
│   │   ├── mod.rs         # UI module exports
│   │   ├── reader.rs      # Book reader view
│   │   ├── library.rs     # Library view
│   │   └── help.rs        # Help view
│   └── components/
│       ├── mod.rs         # Components module exports
│       ├── scrollbar.rs   # Scrollbar component
│       └── status_bar.rs  # Status bar component
├── book/
│   ├── mod.rs             # Book module exports
│   ├── parser.rs          # Book parser trait and factory
│   ├── formats/
│   │   ├── mod.rs         # Formats module exports
│   │   ├── txt.rs         # TXT format parser
│   │   ├── epub.rs        # EPUB format parser
│   │   └── mobi.rs        # MOBI format parser
│   └── content.rs         # Book content representation
└── utils/
    ├── mod.rs             # Utils module exports
    └── path.rs            # File path utilities
```

## Implementation Phases

### Phase 1: Basic TUI Framework (MVP)

**Goal**: Establish a working Ratatui application with basic event handling and state management.

- [x] Update `Cargo.toml` with necessary dependencies:
  - `crossterm` (for terminal event handling)
  - `anyhow` (for error handling)
  - `thiserror` (for custom errors)

- [x] Implement core TUI structure:
  - [x] `src/tui/event.rs` - Event handling system (keyboard, resize)
  - [x] `src/tui/mod.rs` - TUI module exports
  - [x] `src/app/state.rs` - Basic app state (current view, input mode)

- [x] Create basic UI framework:
  - [x] `src/tui/ui/mod.rs` - UI rendering dispatcher
  - [x] `src/tui/ui/help.rs` - Simple help view
  - [ ] `src/tui/components/status_bar.rs` - Basic status bar

- [x] Update `main.rs` to wire everything together:
  - [x] Initialize TUI
  - [x] Set up event loop
  - [x] Handle app exit (q key)

**Verification**: Run the app, see a basic terminal UI, press 'h' for help, 'q' to quit.

### Phase 2: File I/O and TXT Parser ✅

**Goal**: Add ability to load and display simple TXT files with charset support.

- [x] Add dependencies:
  - [x] `encoding_rs` (0.8.35) for charset decoding (UTF-8, GB2312, GBK, GB18030)

- [x] Implement book content representation:
  - [x] `src/book/content.rs` - Structs for book metadata and content

- [x] Implement parser trait:
  - [x] `src/book/parser.rs` - `BookParser` trait with factory
  - [x] `BookParserFactory::create_parser()` for general parsers
  - [x] `BookParserFactory::create_txt_parser_with_charset()` for charset-specific TXT parsing

- [x] Implement TXT parser:
  - [x] `src/book/formats/txt.rs` - TXT format parser implementation
  - [x] Charset enum (UTF-8, GB2312, GBK, GB18030)
  - [x] Encoding/decoding support using `encoding_rs`
  - [x] Page splitting (50 lines per page)
  - [x] Charset cycling support
  - [x] Unit tests for all functionality

- [x] Add file I/O utilities:
  - [x] `src/utils/path.rs` - File path handling helpers
  - [x] `read_file_bytes()` for reading raw file contents
  - [x] `list_books_in_directory()` for discovering books
  - [x] `file_name_without_extension()` and `file_extension()

- [x] Implement library view:
  - [x] `src/tui/ui/library.rs` - File browser to select books
  - [x] Navigation with j/k or arrow keys
  - [x] Enter key to open a selected book

- [x] Implement reader view:
  - [x] `src/tui/ui/reader.rs` - Display book content with scrolling
  - [x] Page navigation with j/k/PageUp/PageDown
  - [x] 'c' key to cycle through charsets

- [x] Update app state:
  - [x] Track loaded book (`current_book: Option<Book>`)
  - [x] Track current charset (`current_charset: Charset`)
  - [x] `cycle_charset()` method for charset switching
  - [x] `load_book()` method to load a book and switch views

**Additional Features Implemented**:

- Charset decoding support for multiple encodings (UTF-8, GB2312, GBK, GB18030)
- Charset cycling in reader view
- Enhanced help view showing current charset

**Verification**: Run the app, browse to a TXT file, open it, read and scroll through content. Press 'c' to cycle through charsets!

### Phase 3: EPUB Support

**Goal**: Add support for reading EPUB files.

- [ ] Add dependencies:
  - `epub` crate for EPUB parsing
  - `html2text` or similar for HTML to plain text conversion

- [ ] Implement EPUB parser:
  - [ ] `src/book/formats/epub.rs` - EPUB format parser
  - [ ] Handle EPUB structure (container.xml, OPF, chapters)
  - [ ] Convert HTML content to readable text

- [ ] Enhance reader view:
  - [ ] Support for chapters/navigation
  - [ ] Better text formatting

- [ ] Update app state:
  - [ ] Track current chapter
  - [ ] Chapter navigation

**Verification**: Open an EPUB file, navigate through chapters, read content.

### Phase 4: MOBI Support

**Goal**: Add support for reading MOBI files.

- [ ] Add dependencies:
  - `mobi` crate for MOBI parsing

- [ ] Implement MOBI parser:
  - [ ] `src/book/formats/mobi.rs` - MOBI format parser
  - [ ] Handle MOBI structure and text extraction

- [ ] Add book metadata display:
  - [ ] Show title, author, etc. in status bar or info panel

**Verification**: Open a MOBI file and read its content.

### Phase 5: Polish and Features

**Goal**: Add user-friendly features and polish.

- [ ] Implement scrollbar component:
  - [ ] `src/tui/components/scrollbar.rs` - Visual scroll indicator

- [ ] Add user settings:
  - [ ] `src/app/settings.rs` - Persistent settings (theme, scroll speed, etc.)

- [ ] Add search functionality:
  - [ ] Search within book content
  - [ ] Search results navigation

- [ ] Add bookmarks:
  - [ ] Save and load bookmarks
  - [ ] Quick jump to bookmarks

- [ ] Improve UI/UX:
  - [ ] Better key bindings
  - [ ] Mouse support (optional)
  - [ ] Theme customization

## Key Design Decisions

1. **Event-Driven Architecture**: Use crossterm event handling with a central event loop.
2. **State Management**: Single app state struct that's mutated by event handlers.
3. **Parser Trait**: `BookParser` trait allows easy addition of new formats.
4. **Modular UI**: Separate views for library, reader, and help for clear separation of concerns.

## Next Steps

Start with Phase 1: Basic TUI Framework.
