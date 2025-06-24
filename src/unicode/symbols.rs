//! Symbol Unicode characters
//!
//! General symbols and special characters for common use cases.
//!
//! # Examples
//!
//! ```rust
//! use unicode_rs::prelude::*;
//!
//! // Get a checkmark in different themes
//! let minimal_check = Symbol::Check.get_char(UnicodeTheme::Minimal); // 'v'
//! let rich_check = Symbol::Check.get_char(UnicodeTheme::Rich);       // '✓'
//! let fancy_check = Symbol::Check.get_char(UnicodeTheme::Fancy);     // '✅'
//!
//! // Use with global configuration
//! set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Rich));
//! let check = get_char(&Symbol::Check, None); // '✓'
//! ```
//!
//! # Character Reference
//!
//! | Symbol | Minimal | Basic | Rich | Fancy |
//! |--------|---------|-------|------|-------|
//! | Check  | `v`     | `✓`   | `✓`  | `✅`  |
//! | X      | `X`     | `✗`   | `✖`  | `❌`  |
//! | !      | `!`     | `!`   | `❗`  | `❗`  |
//! | ?      | `?`     | `?`   | `❓`  | `❓`  |

use super::{UnicodeProvider, UnicodeTheme};

/// General symbols for common UI elements and indicators
///
/// This enum provides access to commonly used symbols with support for different
/// Unicode themes. Each symbol can be rendered in four different styles depending
/// on the terminal capabilities and user preferences.
///
/// # Examples
///
/// ```rust
/// use unicode_rs::prelude::*;
///
/// // Basic usage
/// let check = Symbol::Check.get_char(UnicodeTheme::Rich);
/// println!("Task completed {}", check); // Task completed ✓
///
/// // Compare themes
/// for theme in [UnicodeTheme::Minimal, UnicodeTheme::Rich] {
///     let symbol = Symbol::X.get_char(theme);
///     println!("Error symbol in {:?} theme: {}", theme, symbol);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// Checkmark symbol - indicates success, completion, or affirmative state
    ///
    /// # Theme variants
    /// - Minimal: `v`
    /// - Basic: `✓`
    /// - Rich: `✓`
    /// - Fancy: `✅`
    Check,
    /// X mark symbol - indicates failure, error, or negative state
    ///
    /// # Theme variants
    /// - Minimal: `X`
    /// - Basic: `✗`
    /// - Rich: `✖`
    /// - Fancy: `❌`
    X,
    /// Exclamation symbol - indicates warning or important information
    ///
    /// # Theme variants
    /// - Minimal: `!`
    /// - Basic: `!`
    /// - Rich: `❗`
    /// - Fancy: `❗`
    Exclamation,
    /// Question mark symbol - indicates uncertainty or help
    ///
    /// # Theme variants
    /// - Minimal: `?`
    /// - Basic: `?`
    /// - Rich: `❓`
    /// - Fancy: `❓`
    Question,
    /// At symbol
    At,
    /// Hash/pound
    Hash,
    /// Dollar sign
    Dollar,
    /// Percent
    Percent,
    /// Ampersand
    Ampersand,
    /// Copyright
    Copyright,
    /// Trademark
    Trademark,
    /// Registered
    Registered,
}

impl UnicodeProvider for Symbol {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Symbol::Check, UnicodeTheme::Minimal) => 'v',
            (Symbol::Check, UnicodeTheme::Basic) => '✓',
            (Symbol::Check, UnicodeTheme::Rich) => '✓',
            (Symbol::Check, UnicodeTheme::Fancy) => '✅',

            (Symbol::X, UnicodeTheme::Minimal) => 'X',
            (Symbol::X, UnicodeTheme::Basic) => '✗',
            (Symbol::X, UnicodeTheme::Rich) => '✖',
            (Symbol::X, UnicodeTheme::Fancy) => '❌',

            (Symbol::Exclamation, UnicodeTheme::Minimal) => '!',
            (Symbol::Exclamation, UnicodeTheme::Basic) => '!',
            (Symbol::Exclamation, UnicodeTheme::Rich) => '❗',
            (Symbol::Exclamation, UnicodeTheme::Fancy) => '❗',

            (Symbol::Question, UnicodeTheme::Minimal) => '?',
            (Symbol::Question, UnicodeTheme::Basic) => '?',
            (Symbol::Question, UnicodeTheme::Rich) => '❓',
            (Symbol::Question, UnicodeTheme::Fancy) => '❓',

            (Symbol::At, _) => '@',
            (Symbol::Hash, _) => '#',
            (Symbol::Dollar, _) => '$',
            (Symbol::Percent, _) => '%',
            (Symbol::Ampersand, _) => '&',

            (Symbol::Copyright, UnicodeTheme::Minimal) => 'C',
            (Symbol::Copyright, UnicodeTheme::Basic) => '©',
            (Symbol::Copyright, UnicodeTheme::Rich) => '©',
            (Symbol::Copyright, UnicodeTheme::Fancy) => '©',

            (Symbol::Trademark, UnicodeTheme::Minimal) => 'T',
            (Symbol::Trademark, UnicodeTheme::Basic) => '™',
            (Symbol::Trademark, UnicodeTheme::Rich) => '™',
            (Symbol::Trademark, UnicodeTheme::Fancy) => '™',

            (Symbol::Registered, UnicodeTheme::Minimal) => 'R',
            (Symbol::Registered, UnicodeTheme::Basic) => '®',
            (Symbol::Registered, UnicodeTheme::Rich) => '®',
            (Symbol::Registered, UnicodeTheme::Fancy) => '®',
        }
    }
}

/// Convenience constants for symbols
pub mod chars {
    use super::*;

    pub const CHECK: Symbol = Symbol::Check;
    pub const X: Symbol = Symbol::X;
    pub const EXCLAMATION: Symbol = Symbol::Exclamation;
    pub const QUESTION: Symbol = Symbol::Question;
    pub const AT: Symbol = Symbol::At;
    pub const HASH: Symbol = Symbol::Hash;
    pub const DOLLAR: Symbol = Symbol::Dollar;
    pub const PERCENT: Symbol = Symbol::Percent;
    pub const AMPERSAND: Symbol = Symbol::Ampersand;
    pub const COPYRIGHT: Symbol = Symbol::Copyright;
    pub const TRADEMARK: Symbol = Symbol::Trademark;
    pub const REGISTERED: Symbol = Symbol::Registered;
}
