//! Status Unicode characters
//! Characters for status indicators, progress, and state

use super::{UnicodeProvider, UnicodeTheme};

/// Status indicators
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// Online/connected
    Online,
    /// Offline/disconnected
    Offline,
    /// Busy/working
    Busy,
    /// Idle
    Idle,
    /// Error state
    Error,
    /// Warning state
    Warning,
    /// Success state
    Success,
    /// Unknown state
    Unknown,
}

impl UnicodeProvider for Status {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Status::Online, UnicodeTheme::Minimal) => '+',
            (Status::Online, UnicodeTheme::Basic) => '●',
            (Status::Online, UnicodeTheme::Rich) => '●',
            (Status::Online, UnicodeTheme::Fancy) => '🟢',

            (Status::Offline, UnicodeTheme::Minimal) => '-',
            (Status::Offline, UnicodeTheme::Basic) => '○',
            (Status::Offline, UnicodeTheme::Rich) => '○',
            (Status::Offline, UnicodeTheme::Fancy) => '⚪',

            (Status::Busy, UnicodeTheme::Minimal) => '*',
            (Status::Busy, UnicodeTheme::Basic) => '◐',
            (Status::Busy, UnicodeTheme::Rich) => '◐',
            (Status::Busy, UnicodeTheme::Fancy) => '🔄',

            (Status::Idle, UnicodeTheme::Minimal) => 'o',
            (Status::Idle, UnicodeTheme::Basic) => '◯',
            (Status::Idle, UnicodeTheme::Rich) => '◯',
            (Status::Idle, UnicodeTheme::Fancy) => '💤',

            (Status::Error, UnicodeTheme::Minimal) => 'X',
            (Status::Error, UnicodeTheme::Basic) => '✗',
            (Status::Error, UnicodeTheme::Rich) => '✗',
            (Status::Error, UnicodeTheme::Fancy) => '❌',

            (Status::Warning, UnicodeTheme::Minimal) => '!',
            (Status::Warning, UnicodeTheme::Basic) => '⚠',
            (Status::Warning, UnicodeTheme::Rich) => '⚠',
            (Status::Warning, UnicodeTheme::Fancy) => '⚠',

            (Status::Success, UnicodeTheme::Minimal) => '+',
            (Status::Success, UnicodeTheme::Basic) => '✓',
            (Status::Success, UnicodeTheme::Rich) => '✓',
            (Status::Success, UnicodeTheme::Fancy) => '✅',

            (Status::Unknown, UnicodeTheme::Minimal) => '?',
            (Status::Unknown, UnicodeTheme::Basic) => '?',
            (Status::Unknown, UnicodeTheme::Rich) => '❓',
            (Status::Unknown, UnicodeTheme::Fancy) => '❓',
        }
    }
}

/// Convenience constants for status
pub mod chars {
    use super::*;

    pub const ONLINE: Status = Status::Online;
    pub const OFFLINE: Status = Status::Offline;
    pub const BUSY: Status = Status::Busy;
    pub const IDLE: Status = Status::Idle;
    pub const ERROR: Status = Status::Error;
    pub const WARNING: Status = Status::Warning;
    pub const SUCCESS: Status = Status::Success;
    pub const UNKNOWN: Status = Status::Unknown;
}
