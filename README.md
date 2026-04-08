# AI-Powered File Explorer

A high-performance, cross-platform file explorer built with Tauri, Vue.js, and Rust that combines powerful traditional file management with optional AI-powered features.

## Build Status

Successfully builds and runs on Linux
- Frontend: Vue.js build working (Vite production build)
- Backend: Rust compilation successful with all dependencies
- Packages: Debian (.deb) and RPM (.rpm) packages generated
- Tests: 146 core tests passing (services and stores)
- Binary size: ~15.5MB optimized release build

## Features

### Core File Management
- **High-Performance File Operations**: Built with Rust for blazing-fast file operations
- **Advanced Search**: Full-text search with regex and semantic search capabilities
- **Tabbed Interface**: Multiple tabs with navigation history and pinning support
- **Split View**: Side-by-side split panes for efficient file comparison and management
- **Tagging System**: Organize files with custom tags and hierarchical organization
- **Multiple View Modes**: List, grid, and tree views with virtual scrolling
- **Cloud Storage Integration**: Support for Google Drive, Dropbox, OneDrive, and more

### AI-Powered Features (Optional)
- **Smart File Organization**: AI automatically suggests file organization
- **Content Analysis**: Extract text from images, transcribe audio, analyze documents
- **Intelligent Tagging**: Auto-generate relevant tags based on file content
- **Duplicate Detection**: Find similar files using AI-powered comparison
- **Semantic Search**: Search files by meaning, not just keywords

### User Experience
- **Modern Interface**: Clean, responsive UI built with Vue.js 3
- **Customizable Themes**: Light, dark, and custom themes
- **Dashboard Analytics**: Visualize your file system with interactive charts
- **Keyboard Shortcuts**: Full keyboard navigation support
- **Plugin System**: Extensible architecture for custom functionality

## Technology Stack

- **Frontend**: Vue.js 3 with Composition API, TypeScript, Pinia
- **Backend**: Rust with Tauri framework
- **Search**: Tantivy full-text search engine
- **AI**: Local models with optional cloud AI integration
- **Database**: SQLite for metadata and indexing
