//! Block Unicode characters
//! Block elements for progress bars, charts, and visual elements

use super::{UnicodeProvider, UnicodeTheme};

/// Block characters for progress and visual elements
#[derive(Debug, Clone, Copy)]
pub enum Block {
    /// Full block
    Full,
    /// Three quarters block
    ThreeQuarters,
    /// Half block
    Half,
    /// Quarter block
    Quarter,
    /// Eighth block
    Eighth,
    /// Upper half block
    UpperHalf,
    /// Lower half block
    LowerHalf,
    /// Left half block
    LeftHalf,
    /// Right half block
    RightHalf,
    /// Light shade
    LightShade,
    /// Medium shade
    MediumShade,
    /// Dark shade
    DarkShade,
}

impl UnicodeProvider for Block {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Block::Full, UnicodeTheme::Minimal) => '#',
            (Block::Full, UnicodeTheme::Basic) => '█',
            (Block::Full, UnicodeTheme::Rich) => '█',
            (Block::Full, UnicodeTheme::Fancy) => '█',

            (Block::ThreeQuarters, UnicodeTheme::Minimal) => '#',
            (Block::ThreeQuarters, UnicodeTheme::Basic) => '▉',
            (Block::ThreeQuarters, UnicodeTheme::Rich) => '▉',
            (Block::ThreeQuarters, UnicodeTheme::Fancy) => '▉',

            (Block::Half, UnicodeTheme::Minimal) => '=',
            (Block::Half, UnicodeTheme::Basic) => '▌',
            (Block::Half, UnicodeTheme::Rich) => '▌',
            (Block::Half, UnicodeTheme::Fancy) => '▌',

            (Block::Quarter, UnicodeTheme::Minimal) => '|',
            (Block::Quarter, UnicodeTheme::Basic) => '▎',
            (Block::Quarter, UnicodeTheme::Rich) => '▎',
            (Block::Quarter, UnicodeTheme::Fancy) => '▎',

            (Block::Eighth, UnicodeTheme::Minimal) => '|',
            (Block::Eighth, UnicodeTheme::Basic) => '▏',
            (Block::Eighth, UnicodeTheme::Rich) => '▏',
            (Block::Eighth, UnicodeTheme::Fancy) => '▏',

            (Block::UpperHalf, UnicodeTheme::Minimal) => '^',
            (Block::UpperHalf, UnicodeTheme::Basic) => '▀',
            (Block::UpperHalf, UnicodeTheme::Rich) => '▀',
            (Block::UpperHalf, UnicodeTheme::Fancy) => '▀',

            (Block::LowerHalf, UnicodeTheme::Minimal) => '_',
            (Block::LowerHalf, UnicodeTheme::Basic) => '▄',
            (Block::LowerHalf, UnicodeTheme::Rich) => '▄',
            (Block::LowerHalf, UnicodeTheme::Fancy) => '▄',

            (Block::LeftHalf, UnicodeTheme::Minimal) => '|',
            (Block::LeftHalf, UnicodeTheme::Basic) => '▌',
            (Block::LeftHalf, UnicodeTheme::Rich) => '▌',
            (Block::LeftHalf, UnicodeTheme::Fancy) => '▌',

            (Block::RightHalf, UnicodeTheme::Minimal) => '|',
            (Block::RightHalf, UnicodeTheme::Basic) => '▐',
            (Block::RightHalf, UnicodeTheme::Rich) => '▐',
            (Block::RightHalf, UnicodeTheme::Fancy) => '▐',

            (Block::LightShade, UnicodeTheme::Minimal) => '.',
            (Block::LightShade, UnicodeTheme::Basic) => '░',
            (Block::LightShade, UnicodeTheme::Rich) => '░',
            (Block::LightShade, UnicodeTheme::Fancy) => '░',

            (Block::MediumShade, UnicodeTheme::Minimal) => ':',
            (Block::MediumShade, UnicodeTheme::Basic) => '▒',
            (Block::MediumShade, UnicodeTheme::Rich) => '▒',
            (Block::MediumShade, UnicodeTheme::Fancy) => '▒',

            (Block::DarkShade, UnicodeTheme::Minimal) => '#',
            (Block::DarkShade, UnicodeTheme::Basic) => '▓',
            (Block::DarkShade, UnicodeTheme::Rich) => '▓',
            (Block::DarkShade, UnicodeTheme::Fancy) => '▓',
        }
    }
}

/// Convenience constants for blocks
pub mod chars {
    use super::*;

    pub const FULL: Block = Block::Full;
    pub const THREE_QUARTERS: Block = Block::ThreeQuarters;
    pub const HALF: Block = Block::Half;
    pub const QUARTER: Block = Block::Quarter;
    pub const EIGHTH: Block = Block::Eighth;
    pub const UPPER_HALF: Block = Block::UpperHalf;
    pub const LOWER_HALF: Block = Block::LowerHalf;
    pub const LEFT_HALF: Block = Block::LeftHalf;
    pub const RIGHT_HALF: Block = Block::RightHalf;
    pub const LIGHT_SHADE: Block = Block::LightShade;
    pub const MEDIUM_SHADE: Block = Block::MediumShade;
    pub const DARK_SHADE: Block = Block::DarkShade;
}
