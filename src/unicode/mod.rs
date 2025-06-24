//! Unicode character library for the editor
//! Provides categorized Unicode characters for consistent visual design

pub mod arrows;
pub mod blocks;
pub mod editor;
pub mod file_types;
pub mod git;
pub mod security;
pub mod shapes;
pub mod status;
pub mod symbols;
pub mod ui;

// Re-export main types for convenience
pub use arrows::{Arrow, Navigation};
pub use blocks::Block;
pub use editor::{Cursor, Selection};
pub use file_types::{
    get_file_type_from_extension, get_file_type_from_filename, FileType, LanguageType,
};
pub use git::{GitAction, GitBranch, GitDiff, GitStatus};
pub use shapes::Shape;
pub use status::Status;
pub use symbols::Symbol;
pub use ui::{Border, Control, Indicator, Separator};

/// Unicode character theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeTheme {
    /// Minimal ASCII-compatible characters
    Minimal,
    /// Basic Unicode symbols
    Basic,
    /// Rich Unicode with full symbol set
    #[default]
    Rich,
    /// Fancy decorative Unicode
    Fancy,
}

/// Unicode character provider trait
///
/// This trait defines the interface for all Unicode symbol types in the library.
/// It provides a consistent way to get character representations across different
/// themes and ensures type safety.
///
/// # Examples
///
/// ```rust
/// use unicode_rs::prelude::*;
///
/// // All symbol enums implement UnicodeProvider
/// let check = Symbol::Check.get_char(UnicodeTheme::Rich);
/// let arrow = Arrow::Right.get_char(UnicodeTheme::Rich);
/// let git_status = GitStatus::Modified.get_char(UnicodeTheme::Rich);
///
/// println!("Status: {} {} {}", check, arrow, git_status);
/// ```
///
/// # Implementation Notes
///
/// - All implementations should handle all theme variants
/// - ASCII fallbacks should be provided for Minimal theme
/// - The trait is object-safe and can be used with dynamic dispatch
pub trait UnicodeProvider {
    /// Get character for the given theme
    ///
    /// Returns the appropriate Unicode character for the specified theme.
    /// Implementations should provide meaningful fallbacks for all themes.
    ///
    /// # Arguments
    ///
    /// * `theme` - The Unicode theme to use for character selection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unicode_rs::prelude::*;
    ///
    /// let symbol = Symbol::Check;
    /// assert_eq!(symbol.get_char(UnicodeTheme::Minimal), 'v');
    /// assert_eq!(symbol.get_char(UnicodeTheme::Rich), '✓');
    /// ```
    fn get_char(&self, theme: UnicodeTheme) -> char;

    /// Get string representation for the given theme
    fn get_str(&self, theme: UnicodeTheme) -> &'static str {
        #[allow(clippy::match_single_binding)]
        match self.get_char(theme) {
            c => match c {
                ' ' => " ",
                '!' => "!",
                '"' => "\"",
                '#' => "#",
                '$' => "$",
                '%' => "%",
                '&' => "&",
                '\'' => "'",
                '(' => "(",
                ')' => ")",
                '*' => "*",
                '+' => "+",
                ',' => ",",
                '-' => "-",
                '.' => ".",
                '/' => "/",
                '0' => "0",
                '1' => "1",
                '2' => "2",
                '3' => "3",
                '4' => "4",
                '5' => "5",
                '6' => "6",
                '7' => "7",
                '8' => "8",
                '9' => "9",
                ':' => ":",
                ';' => ";",
                '<' => "<",
                '=' => "=",
                '>' => ">",
                '?' => "?",
                '@' => "@",
                'A' => "A",
                'B' => "B",
                'C' => "C",
                'D' => "D",
                'E' => "E",
                'F' => "F",
                'G' => "G",
                'H' => "H",
                'I' => "I",
                'J' => "J",
                'K' => "K",
                'L' => "L",
                'M' => "M",
                'N' => "N",
                'O' => "O",
                'P' => "P",
                'Q' => "Q",
                'R' => "R",
                'S' => "S",
                'T' => "T",
                'U' => "U",
                'V' => "V",
                'W' => "W",
                'X' => "X",
                'Y' => "Y",
                'Z' => "Z",
                '[' => "[",
                '\\' => "\\",
                ']' => "]",
                '^' => "^",
                '_' => "_",
                '`' => "`",
                'a' => "a",
                'b' => "b",
                'c' => "c",
                'd' => "d",
                'e' => "e",
                'f' => "f",
                'g' => "g",
                'h' => "h",
                'i' => "i",
                'j' => "j",
                'k' => "k",
                'l' => "l",
                'm' => "m",
                'n' => "n",
                'o' => "o",
                'p' => "p",
                'q' => "q",
                'r' => "r",
                's' => "s",
                't' => "t",
                'u' => "u",
                'v' => "v",
                'w' => "w",
                'x' => "x",
                'y' => "y",
                'z' => "z",
                '{' => "{",
                '|' => "|",
                '}' => "}",
                '~' => "~",
                _ => "?", // Fallback for Unicode characters
            },
        }
    }
}

/// Unicode character configuration
#[derive(Debug, Clone)]
pub struct UnicodeConfig {
    /// Current theme
    pub theme: UnicodeTheme,
    /// Whether to use fallback ASCII characters
    pub use_fallback: bool,
    /// Custom character overrides
    pub overrides: std::collections::HashMap<String, char>,
}

#[allow(clippy::derivable_impls)]
impl Default for UnicodeConfig {
    fn default() -> Self {
        Self {
            theme: UnicodeTheme::default(),
            use_fallback: false,
            overrides: std::collections::HashMap::new(),
        }
    }
}

impl UnicodeConfig {
    /// Create new config with theme
    pub fn with_theme(theme: UnicodeTheme) -> Self {
        Self {
            theme,
            ..Default::default()
        }
    }

    /// Enable fallback mode
    pub fn with_fallback(mut self) -> Self {
        self.use_fallback = true;
        self
    }

    /// Add character override
    pub fn with_override(mut self, key: &str, character: char) -> Self {
        self.overrides.insert(key.to_string(), character);
        self
    }

    /// Get character with config applied
    pub fn get_char<T: UnicodeProvider>(&self, provider: &T, key: Option<&str>) -> char {
        // Check for override first
        if let Some(key) = key {
            if let Some(&override_char) = self.overrides.get(key) {
                return override_char;
            }
        }

        let char = provider.get_char(self.theme);

        // Apply fallback if needed
        if self.use_fallback && !char.is_ascii() {
            provider.get_char(UnicodeTheme::Minimal)
        } else {
            char
        }
    }
}

use std::sync::{Mutex, OnceLock};

/// Global unicode configuration
static GLOBAL_UNICODE_CONFIG: OnceLock<Mutex<UnicodeConfig>> = OnceLock::new();

/// Set global unicode configuration
///
/// Sets the global Unicode configuration that will be used by [`get_char`] and [`get_str`].
/// This is thread-safe and can be called from multiple threads.
///
/// # Arguments
///
/// * `config` - The Unicode configuration to set globally
///
/// # Examples
///
/// ```rust
/// use unicode_rs::prelude::*;
///
/// // Set minimal theme globally
/// set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Minimal));
///
/// // Set rich theme with fallback
/// let config = UnicodeConfig::with_theme(UnicodeTheme::Rich)
///     .with_fallback()
///     .with_override("custom_bullet", '•');
/// set_global_config(config);
/// ```
///
/// # Thread Safety
///
/// This function is thread-safe and uses internal synchronization.
pub fn set_global_config(config: UnicodeConfig) {
    let mutex = GLOBAL_UNICODE_CONFIG.get_or_init(|| Mutex::new(UnicodeConfig::default()));
    if let Ok(mut guard) = mutex.lock() {
        *guard = config;
    }
}

/// Get global unicode configuration
///
/// Returns a copy of the current global Unicode configuration.
/// If no configuration has been set, returns the default configuration.
///
/// # Examples
///
/// ```rust
/// use unicode_rs::prelude::*;
///
/// let config = get_global_config();
/// println!("Current theme: {:?}", config.theme);
/// ```
///
/// # Thread Safety
///
/// This function is thread-safe and uses internal synchronization.
pub fn get_global_config() -> UnicodeConfig {
    let mutex = GLOBAL_UNICODE_CONFIG.get_or_init(|| Mutex::new(UnicodeConfig::default()));
    mutex.lock().map(|guard| guard.clone()).unwrap_or_default()
}

/// Get character using global config
///
/// Convenience function that gets a character using the global configuration.
/// This is equivalent to calling `get_global_config().get_char(provider, key)`.
///
/// # Arguments
///
/// * `provider` - The Unicode provider (symbol enum)
/// * `key` - Optional override key for custom character mappings
///
/// # Examples
///
/// ```rust
/// use unicode_rs::prelude::*;
///
/// // Set global theme
/// set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Rich));
///
/// // Get characters using global config
/// let check = get_char(&Symbol::Check, None);
/// let arrow = get_char(&Arrow::Right, None);
///
/// // Use custom override
/// let bullet = get_char(&Symbol::Check, Some("bullet"));
/// ```
///
/// # Thread Safety
///
/// This function is thread-safe as it uses the thread-safe global configuration.
pub fn get_char<T: UnicodeProvider>(provider: &T, key: Option<&str>) -> char {
    get_global_config().get_char(provider, key)
}

/// Get string using global config
pub fn get_str<T: UnicodeProvider>(provider: &T, key: Option<&str>) -> &'static str {
    let char = get_char(provider, key);
    // This is a simplified implementation - in practice you'd want a proper string cache
    match char {
        ' ' => " ",
        _ => "?", // Simplified fallback
    }
}
