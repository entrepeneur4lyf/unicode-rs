//! Editor-specific Unicode characters
//! Characters for editor functionality like cursors, selections, etc.

use super::{UnicodeProvider, UnicodeTheme};

/// Editor cursor types
#[derive(Debug, Clone, Copy)]
pub enum Cursor {
    /// Text cursor
    Text,
    /// Block cursor
    Block,
    /// Underline cursor
    Underline,
    /// Vertical bar cursor
    VerticalBar,
}

impl UnicodeProvider for Cursor {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Cursor::Text, UnicodeTheme::Minimal) => '|',
            (Cursor::Text, UnicodeTheme::Basic) => '│',
            (Cursor::Text, UnicodeTheme::Rich) => '│',
            (Cursor::Text, UnicodeTheme::Fancy) => '┃',

            (Cursor::Block, UnicodeTheme::Minimal) => '#',
            (Cursor::Block, UnicodeTheme::Basic) => '█',
            (Cursor::Block, UnicodeTheme::Rich) => '█',
            (Cursor::Block, UnicodeTheme::Fancy) => '█',

            (Cursor::Underline, UnicodeTheme::Minimal) => '_',
            (Cursor::Underline, UnicodeTheme::Basic) => '▁',
            (Cursor::Underline, UnicodeTheme::Rich) => '▁',
            (Cursor::Underline, UnicodeTheme::Fancy) => '▁',

            (Cursor::VerticalBar, UnicodeTheme::Minimal) => '|',
            (Cursor::VerticalBar, UnicodeTheme::Basic) => '▎',
            (Cursor::VerticalBar, UnicodeTheme::Rich) => '▎',
            (Cursor::VerticalBar, UnicodeTheme::Fancy) => '▎',
        }
    }
}

/// Editor selection indicators
#[derive(Debug, Clone, Copy)]
pub enum Selection {
    /// Primary selection
    Primary,
    /// Secondary selection
    Secondary,
    /// Selection start
    Start,
    /// Selection end
    End,
}

impl UnicodeProvider for Selection {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Selection::Primary, UnicodeTheme::Minimal) => '*',
            (Selection::Primary, UnicodeTheme::Basic) => '●',
            (Selection::Primary, UnicodeTheme::Rich) => '●',
            (Selection::Primary, UnicodeTheme::Fancy) => '🔴',

            (Selection::Secondary, UnicodeTheme::Minimal) => 'o',
            (Selection::Secondary, UnicodeTheme::Basic) => '○',
            (Selection::Secondary, UnicodeTheme::Rich) => '○',
            (Selection::Secondary, UnicodeTheme::Fancy) => '⚪',

            (Selection::Start, UnicodeTheme::Minimal) => '[',
            (Selection::Start, UnicodeTheme::Basic) => '⟨',
            (Selection::Start, UnicodeTheme::Rich) => '⟨',
            (Selection::Start, UnicodeTheme::Fancy) => '⟨',

            (Selection::End, UnicodeTheme::Minimal) => ']',
            (Selection::End, UnicodeTheme::Basic) => '⟩',
            (Selection::End, UnicodeTheme::Rich) => '⟩',
            (Selection::End, UnicodeTheme::Fancy) => '⟩',
        }
    }
}

/// Convenience constants for editor elements
pub mod chars {
    use super::*;

    // Cursors
    pub const TEXT_CURSOR: Cursor = Cursor::Text;
    pub const BLOCK_CURSOR: Cursor = Cursor::Block;
    pub const UNDERLINE_CURSOR: Cursor = Cursor::Underline;
    pub const VERTICAL_BAR_CURSOR: Cursor = Cursor::VerticalBar;

    // Selections
    pub const PRIMARY_SELECTION: Selection = Selection::Primary;
    pub const SECONDARY_SELECTION: Selection = Selection::Secondary;
    pub const SELECTION_START: Selection = Selection::Start;
    pub const SELECTION_END: Selection = Selection::End;
}
