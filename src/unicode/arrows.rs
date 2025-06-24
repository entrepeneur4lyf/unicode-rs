//! Arrow Unicode characters
//! Directional arrows for navigation, flow, and indicators

use super::{UnicodeProvider, UnicodeTheme};

/// Arrow directions
#[derive(Debug, Clone, Copy)]
pub enum Arrow {
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up-left diagonal
    UpLeft,
    /// Up-right diagonal
    UpRight,
    /// Down-left diagonal
    DownLeft,
    /// Down-right diagonal
    DownRight,
    /// Double up arrow
    DoubleUp,
    /// Double down arrow
    DoubleDown,
    /// Double left arrow
    DoubleLeft,
    /// Double right arrow
    DoubleRight,
    /// Curved left arrow
    CurvedLeft,
    /// Curved right arrow
    CurvedRight,
    /// Return/enter arrow
    Return,
    /// Refresh/reload arrow
    Refresh,
}

impl UnicodeProvider for Arrow {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Arrow::Up, UnicodeTheme::Minimal) => '^',
            (Arrow::Up, UnicodeTheme::Basic) => '↑',
            (Arrow::Up, UnicodeTheme::Rich) => '↑',
            (Arrow::Up, UnicodeTheme::Fancy) => '⬆',

            (Arrow::Down, UnicodeTheme::Minimal) => 'v',
            (Arrow::Down, UnicodeTheme::Basic) => '↓',
            (Arrow::Down, UnicodeTheme::Rich) => '↓',
            (Arrow::Down, UnicodeTheme::Fancy) => '⬇',

            (Arrow::Left, UnicodeTheme::Minimal) => '<',
            (Arrow::Left, UnicodeTheme::Basic) => '←',
            (Arrow::Left, UnicodeTheme::Rich) => '←',
            (Arrow::Left, UnicodeTheme::Fancy) => '⬅',

            (Arrow::Right, UnicodeTheme::Minimal) => '>',
            (Arrow::Right, UnicodeTheme::Basic) => '→',
            (Arrow::Right, UnicodeTheme::Rich) => '→',
            (Arrow::Right, UnicodeTheme::Fancy) => '➡',

            (Arrow::UpLeft, UnicodeTheme::Minimal) => '\\',
            (Arrow::UpLeft, UnicodeTheme::Basic) => '↖',
            (Arrow::UpLeft, UnicodeTheme::Rich) => '↖',
            (Arrow::UpLeft, UnicodeTheme::Fancy) => '↖',

            (Arrow::UpRight, UnicodeTheme::Minimal) => '/',
            (Arrow::UpRight, UnicodeTheme::Basic) => '↗',
            (Arrow::UpRight, UnicodeTheme::Rich) => '↗',
            (Arrow::UpRight, UnicodeTheme::Fancy) => '↗',

            (Arrow::DownLeft, UnicodeTheme::Minimal) => '/',
            (Arrow::DownLeft, UnicodeTheme::Basic) => '↙',
            (Arrow::DownLeft, UnicodeTheme::Rich) => '↙',
            (Arrow::DownLeft, UnicodeTheme::Fancy) => '↙',

            (Arrow::DownRight, UnicodeTheme::Minimal) => '\\',
            (Arrow::DownRight, UnicodeTheme::Basic) => '↘',
            (Arrow::DownRight, UnicodeTheme::Rich) => '↘',
            (Arrow::DownRight, UnicodeTheme::Fancy) => '↘',

            (Arrow::DoubleUp, UnicodeTheme::Minimal) => '^',
            (Arrow::DoubleUp, UnicodeTheme::Basic) => '⇑',
            (Arrow::DoubleUp, UnicodeTheme::Rich) => '⇑',
            (Arrow::DoubleUp, UnicodeTheme::Fancy) => '⏫',

            (Arrow::DoubleDown, UnicodeTheme::Minimal) => 'v',
            (Arrow::DoubleDown, UnicodeTheme::Basic) => '⇓',
            (Arrow::DoubleDown, UnicodeTheme::Rich) => '⇓',
            (Arrow::DoubleDown, UnicodeTheme::Fancy) => '⏬',

            (Arrow::DoubleLeft, UnicodeTheme::Minimal) => '<',
            (Arrow::DoubleLeft, UnicodeTheme::Basic) => '⇐',
            (Arrow::DoubleLeft, UnicodeTheme::Rich) => '⇐',
            (Arrow::DoubleLeft, UnicodeTheme::Fancy) => '⏪',

            (Arrow::DoubleRight, UnicodeTheme::Minimal) => '>',
            (Arrow::DoubleRight, UnicodeTheme::Basic) => '⇒',
            (Arrow::DoubleRight, UnicodeTheme::Rich) => '⇒',
            (Arrow::DoubleRight, UnicodeTheme::Fancy) => '⏩',

            (Arrow::CurvedLeft, UnicodeTheme::Minimal) => '<',
            (Arrow::CurvedLeft, UnicodeTheme::Basic) => '↰',
            (Arrow::CurvedLeft, UnicodeTheme::Rich) => '↰',
            (Arrow::CurvedLeft, UnicodeTheme::Fancy) => '↰',

            (Arrow::CurvedRight, UnicodeTheme::Minimal) => '>',
            (Arrow::CurvedRight, UnicodeTheme::Basic) => '↱',
            (Arrow::CurvedRight, UnicodeTheme::Rich) => '↱',
            (Arrow::CurvedRight, UnicodeTheme::Fancy) => '↱',

            (Arrow::Return, UnicodeTheme::Minimal) => '\\',
            (Arrow::Return, UnicodeTheme::Basic) => '↵',
            (Arrow::Return, UnicodeTheme::Rich) => '↵',
            (Arrow::Return, UnicodeTheme::Fancy) => '⏎',

            (Arrow::Refresh, UnicodeTheme::Minimal) => 'R',
            (Arrow::Refresh, UnicodeTheme::Basic) => '↻',
            (Arrow::Refresh, UnicodeTheme::Rich) => '↻',
            (Arrow::Refresh, UnicodeTheme::Fancy) => '🔄',
        }
    }
}

/// Navigation arrows
#[derive(Debug, Clone, Copy)]
pub enum Navigation {
    /// First/beginning
    First,
    /// Previous
    Previous,
    /// Next
    Next,
    /// Last/end
    Last,
    /// Home
    Home,
    /// End
    End,
    /// Page up
    PageUp,
    /// Page down
    PageDown,
    /// Back
    Back,
    /// Forward
    Forward,
}

impl UnicodeProvider for Navigation {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Navigation::First, UnicodeTheme::Minimal) => '<',
            (Navigation::First, UnicodeTheme::Basic) => '⇤',
            (Navigation::First, UnicodeTheme::Rich) => '⇤',
            (Navigation::First, UnicodeTheme::Fancy) => '⏮',

            (Navigation::Previous, UnicodeTheme::Minimal) => '<',
            (Navigation::Previous, UnicodeTheme::Basic) => '◀',
            (Navigation::Previous, UnicodeTheme::Rich) => '◀',
            (Navigation::Previous, UnicodeTheme::Fancy) => '⏪',

            (Navigation::Next, UnicodeTheme::Minimal) => '>',
            (Navigation::Next, UnicodeTheme::Basic) => '▶',
            (Navigation::Next, UnicodeTheme::Rich) => '▶',
            (Navigation::Next, UnicodeTheme::Fancy) => '⏩',

            (Navigation::Last, UnicodeTheme::Minimal) => '>',
            (Navigation::Last, UnicodeTheme::Basic) => '⇥',
            (Navigation::Last, UnicodeTheme::Rich) => '⇥',
            (Navigation::Last, UnicodeTheme::Fancy) => '⏭',

            (Navigation::Home, UnicodeTheme::Minimal) => 'H',
            (Navigation::Home, UnicodeTheme::Basic) => '⌂',
            (Navigation::Home, UnicodeTheme::Rich) => '⌂',
            (Navigation::Home, UnicodeTheme::Fancy) => '🏠',

            (Navigation::End, UnicodeTheme::Minimal) => 'E',
            (Navigation::End, UnicodeTheme::Basic) => '⌐',
            (Navigation::End, UnicodeTheme::Rich) => '⌐',
            (Navigation::End, UnicodeTheme::Fancy) => '🔚',

            (Navigation::PageUp, UnicodeTheme::Minimal) => '^',
            (Navigation::PageUp, UnicodeTheme::Basic) => '⇞',
            (Navigation::PageUp, UnicodeTheme::Rich) => '⇞',
            (Navigation::PageUp, UnicodeTheme::Fancy) => '📄',

            (Navigation::PageDown, UnicodeTheme::Minimal) => 'v',
            (Navigation::PageDown, UnicodeTheme::Basic) => '⇟',
            (Navigation::PageDown, UnicodeTheme::Rich) => '⇟',
            (Navigation::PageDown, UnicodeTheme::Fancy) => '📄',

            (Navigation::Back, UnicodeTheme::Minimal) => '<',
            (Navigation::Back, UnicodeTheme::Basic) => '⬅',
            (Navigation::Back, UnicodeTheme::Rich) => '⬅',
            (Navigation::Back, UnicodeTheme::Fancy) => '🔙',

            (Navigation::Forward, UnicodeTheme::Minimal) => '>',
            (Navigation::Forward, UnicodeTheme::Basic) => '➡',
            (Navigation::Forward, UnicodeTheme::Rich) => '➡',
            (Navigation::Forward, UnicodeTheme::Fancy) => '🔜',
        }
    }
}

/// Convenience constants for arrows
pub mod chars {
    use super::*;

    // Basic arrows
    pub const UP: Arrow = Arrow::Up;
    pub const DOWN: Arrow = Arrow::Down;
    pub const LEFT: Arrow = Arrow::Left;
    pub const RIGHT: Arrow = Arrow::Right;
    pub const UP_LEFT: Arrow = Arrow::UpLeft;
    pub const UP_RIGHT: Arrow = Arrow::UpRight;
    pub const DOWN_LEFT: Arrow = Arrow::DownLeft;
    pub const DOWN_RIGHT: Arrow = Arrow::DownRight;

    // Double arrows
    pub const DOUBLE_UP: Arrow = Arrow::DoubleUp;
    pub const DOUBLE_DOWN: Arrow = Arrow::DoubleDown;
    pub const DOUBLE_LEFT: Arrow = Arrow::DoubleLeft;
    pub const DOUBLE_RIGHT: Arrow = Arrow::DoubleRight;

    // Special arrows
    pub const CURVED_LEFT: Arrow = Arrow::CurvedLeft;
    pub const CURVED_RIGHT: Arrow = Arrow::CurvedRight;
    pub const RETURN: Arrow = Arrow::Return;
    pub const REFRESH: Arrow = Arrow::Refresh;

    // Navigation
    pub const FIRST: Navigation = Navigation::First;
    pub const PREVIOUS: Navigation = Navigation::Previous;
    pub const NEXT: Navigation = Navigation::Next;
    pub const LAST: Navigation = Navigation::Last;
    pub const HOME: Navigation = Navigation::Home;
    pub const END: Navigation = Navigation::End;
    pub const PAGE_UP: Navigation = Navigation::PageUp;
    pub const PAGE_DOWN: Navigation = Navigation::PageDown;
    pub const BACK: Navigation = Navigation::Back;
    pub const FORWARD: Navigation = Navigation::Forward;
}
