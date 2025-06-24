# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-06-24

### Added
- Initial release of unicode-rs
- Support for multiple Unicode themes (Minimal, Basic, Rich, Fancy)
- Comprehensive symbol categories:
  - General symbols (checkmarks, exclamation, etc.)
  - Directional arrows and navigation symbols
  - Block drawing characters
  - Geometric shapes
  - Git status and diff symbols
  - File type indicators
  - UI elements (borders, separators, etc.)
  - Editor-specific symbols (cursor, selection)
  - Status indicators
- Global configuration system with theme and override support
- Fallback support for ASCII-only environments
- Type-safe enum-based symbol definitions
- Zero external dependencies
- Comprehensive documentation and examples
- MIT/Apache-2.0 dual licensing

### Features
- `UnicodeProvider` trait for consistent symbol access
- `UnicodeConfig` for global theme and override management
- Prelude module for convenient imports
- Example applications demonstrating usage
- Full test coverage

[Unreleased]: https://github.com/yourusername/unicode-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/unicode-rs/releases/tag/v0.1.0
