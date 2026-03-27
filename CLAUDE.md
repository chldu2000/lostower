```
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
```

## Project Overview

lostower is a terminal-based e-book reader (TUI application) written in Rust using the Ratatui library (version 0.30.0).

## Current State

- **Basic TUI Framework**: Implemented using Ratatui and Crossterm
- **Views**: Library, Reader, and Help views are stubbed
- **Key Bindings**:
  - `q` - Quit
  - `h` - Show help
  - `l` - Library view
  - `r` - Reader view
- **Dependencies**: ratatui, crossterm, anyhow, thiserror

## Development Commands

### Build and Run

```bash
# Build the project
cargo build

# Run the project
cargo run

# Build for release
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run a single test file
cargo test --test <test_file_name>

# Run tests with verbose output
cargo test -- --verbose
```

### Code Quality

```bash
# Check code with clippy (Rust linter)
cargo clippy

# Format code with rustfmt
cargo fmt
```

## Architecture

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
│       ├── scrollbar.rs   # Scrollbar component (stub)
│       └── status_bar.rs  # Status bar component (stub)
├── book/
│   ├── mod.rs             # Book module exports
│   ├── parser.rs          # Book parser trait and factory (stub)
│   ├── formats/
│   │   ├── mod.rs         # Formats module exports
│   │   ├── txt.rs         # TXT format parser (stub)
│   │   ├── epub.rs        # EPUB format parser (stub)
│   │   └── mobi.rs        # MOBI format parser (stub)
│   └── content.rs         # Book content representation (stub)
└── utils/
    ├── mod.rs             # Utils module exports
    └── path.rs            # File path utilities (stub)
```

### Key Components

1. **App State Manager** - Manages application state (current book, reading progress, settings)
2. **TUI Engine** - Handles rendering and user input using Ratatui and Crossterm
3. **Book Parser** - Abstraction layer for parsing different e-book formats
4. **Format-Specific Parsers** - Implementations for epub, mobi, txt, etc.
5. **File I/O** - Handles loading books from the filesystem

## Implementation Plan

### High-Level Components

1. **App State Manager** - Manages application state (current book, reading progress, settings)
2. **TUI Engine** - Handles rendering and user input using Ratatui
3. **Book Parser** - Abstraction layer for parsing different e-book formats
4. **Format-Specific Parsers** - Implementations for epub, mobi, txt, etc.
5. **File I/O** - Handles loading books from the filesystem

### Implementation Phases

#### Phase 1: Basic TUI Framework (MVP) ✅

**Goal**: Establish a working Ratatui application with basic event handling and state management.

- [x] Updated `Cargo.toml` with necessary dependencies:
  - `crossterm` (for terminal event handling)
  - `anyhow` (for error handling)
  - `thiserror` (for custom errors)

- [x] Implemented core TUI structure:
  - [x] `src/tui/event.rs` - Event handling system (keyboard, resize)
  - [x] `src/tui/mod.rs` - TUI module exports
  - [x] `src/app/state.rs` - Basic app state (current view, input mode)

- [x] Created basic UI framework:
  - [x] `src/tui/ui/mod.rs` - UI rendering dispatcher
  - [x] `src/tui/ui/help.rs` - Simple help view
  - [ ] `src/tui/components/status_bar.rs` - Basic status bar (stub)

- [x] Updated `main.rs` to wire everything together:
  - [x] Initialize TUI
  - [x] Set up event loop
  - [x] Handle app exit (q key)

**Verification**: Run the app, see a basic terminal UI, press 'h' for help, 'q' to quit.

#### Phase 2: File I/O and TXT Parser (Next Step)

**Goal**: Add ability to load and display simple TXT files.

- [ ] Add dependencies:
  - (No new dependencies needed for TXT)

- [ ] Implement book content representation:
  - [ ] `src/book/content.rs` - Structs for book metadata and content

- [ ] Implement parser trait:
  - [ ] `src/book/parser.rs` - `BookParser` trait with factory

- [ ] Implement TXT parser:
  - [ ] `src/book/formats/txt.rs` - TXT format parser implementation

- [ ] Add file I/O utilities:
  - [ ] `src/utils/path.rs` - File path handling helpers

- [ ] Implement library view:
  - [ ] `src/tui/ui/library.rs` - File browser to select books

- [ ] Implement reader view:
  - [ ] `src/tui/ui/reader.rs` - Display book content with scrolling

- [ ] Update app state:
  - [ ] Track loaded book
  - [ ] Track reading position
  - [ ] Switch between views (library/reader/help)

**Verification**: Run the app, browse to a TXT file, open it, read and scroll through content.

#### Phase 3: EPUB Support

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

#### Phase 4: MOBI Support

**Goal**: Add support for reading MOBI files.

- [ ] Add dependencies:
  - `mobi` crate for MOBI parsing

- [ ] Implement MOBI parser:
  - [ ] `src/book/formats/mobi.rs` - MOBI format parser
  - [ ] Handle MOBI structure and text extraction

- [ ] Add book metadata display:
  - [ ] Show title, author, etc. in status bar or info panel

**Verification**: Open a MOBI file and read its content.

#### Phase 5: Polish and Features

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

## Key Files and Directories

- `src/main.rs`: Main application entry point
- `src/app/state.rs`: Application state management
- `src/tui/event.rs`: Event handling system
- `src/tui/ui/mod.rs`: UI rendering dispatcher
- `Cargo.toml`: Project dependencies and metadata
- `IMPLEMENTATION.md`: Detailed implementation plan (separate file)
