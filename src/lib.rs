//! # Unicode-rs
//!
//! A comprehensive Unicode character library for Rust applications, particularly useful for
//! terminal applications, editors, and CLI tools that need consistent Unicode symbol support
//! across different environments and themes.
//!
//! ## Features
//!
//! - **Multiple themes**: Support for Minimal (ASCII), Basic, Rich, and Fancy Unicode themes
//! - **Categorized symbols**: Organized into logical groups (arrows, blocks, shapes, git, etc.)
//! - **Fallback support**: Graceful degradation to ASCII when Unicode isn't supported
//! - **Global configuration**: Set theme and overrides globally for your application
//! - **Type-safe**: All symbols are strongly typed enums
//!
//! ## Quick Start
//!
//! ```rust
//! use unicode_rs::prelude::*;
//!
//! // Use with default theme (Rich)
//! let check = Symbol::Check.get_char(UnicodeTheme::Rich); // ✓
//! let arrow = Arrow::Right.get_char(UnicodeTheme::Rich);  // →
//!
//! // Configure globally
//! set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Minimal));
//! let check_ascii = get_char(&Symbol::Check, None); // v
//! ```
//!
//! ## Theme Comparison
//!
//! | Symbol | Minimal | Basic | Rich | Fancy |
//! |--------|---------|-------|------|-------|
//! | Check  | `v`     | `✓`   | `✓`  | `✅`  |
//! | Arrow Right | `>` | `→`   | `→`  | `⮕`  |
//! | Git Modified | `M` | `●`   | `●`  | `◐`  |
//!
//! ## Advanced Usage
//!
//! ### Custom Configuration
//! ```rust
//! use unicode_rs::prelude::*;
//!
//! // Create a config with fallback and custom overrides
//! let config = UnicodeConfig::with_theme(UnicodeTheme::Rich)
//!     .with_fallback()  // Fall back to ASCII if Unicode fails
//!     .with_override("custom_bullet", '•');
//!
//! set_global_config(config);
//! ```
//!
//! ### Terminal Compatibility
//! ```rust
//! use unicode_rs::prelude::*;
//!
//! // Detect terminal capabilities and choose appropriate theme
//! let theme = if std::env::var("TERM").unwrap_or_default().contains("xterm") {
//!     UnicodeTheme::Rich
//! } else {
//!     UnicodeTheme::Minimal
//! };
//!
//! set_global_config(UnicodeConfig::with_theme(theme));
//! ```
//!
//! ## Modules
//!
//! - [`symbols`] - General symbols (checkmarks, exclamation, etc.)
//! - [`arrows`] - Directional arrows and navigation symbols
//! - [`blocks`] - Block drawing characters
//! - [`shapes`] - Geometric shapes
//! - [`git`] - Git status and diff symbols
//! - [`file_types`] - File type indicators
//! - [`ui`] - UI elements (borders, separators, etc.)
//! - [`editor`] - Editor-specific symbols (cursor, selection)
//! - [`status`] - Status indicators
//! - [`security`] - Unicode security utilities for detecting dangerous characters

pub mod unicode;

// Re-export the main types for convenience
pub use unicode::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::unicode::{
        get_char, get_file_type_from_extension, get_file_type_from_filename, get_global_config,
        get_str, set_global_config, Arrow, Block, Border, Control, Cursor, FileType, GitAction,
        GitBranch, GitDiff, GitStatus, Indicator, LanguageType, Navigation, Selection, Separator,
        Shape, Status, Symbol, UnicodeConfig, UnicodeProvider, UnicodeTheme,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_symbol_themes() {
        // Test different themes for the same symbol
        assert_eq!(Symbol::Check.get_char(UnicodeTheme::Minimal), 'v');
        assert_eq!(Symbol::Check.get_char(UnicodeTheme::Basic), '✓');
        assert_eq!(Symbol::Check.get_char(UnicodeTheme::Rich), '✓');
    }

    #[test]
    fn test_global_config() {
        // Test global configuration
        set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Minimal));
        let config = get_global_config();
        assert_eq!(config.theme, UnicodeTheme::Minimal);

        let check_char = get_char(&Symbol::Check, None);
        assert_eq!(check_char, 'v');
    }

    #[test]
    fn test_config_with_fallback() {
        let config = UnicodeConfig::with_theme(UnicodeTheme::Rich).with_fallback();
        assert!(config.use_fallback);
    }

    #[test]
    fn test_config_with_override() {
        let config = UnicodeConfig::default().with_override("custom_check", '√');
        assert_eq!(config.overrides.get("custom_check"), Some(&'√'));
    }
}
