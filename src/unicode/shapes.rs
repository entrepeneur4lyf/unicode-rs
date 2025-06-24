//! Shape Unicode characters
//! Geometric shapes for icons and visual elements

use super::{UnicodeProvider, UnicodeTheme};

/// Basic geometric shapes
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Circle
    Circle,
    /// Square
    Square,
    /// Triangle
    Triangle,
    /// Diamond
    Diamond,
    /// Star
    Star,
    /// Heart
    Heart,
    /// Plus
    Plus,
    /// Cross
    Cross,
    /// Dot
    Dot,
    /// Bullet
    Bullet,
}

impl UnicodeProvider for Shape {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Shape::Circle, UnicodeTheme::Minimal) => 'o',
            (Shape::Circle, UnicodeTheme::Basic) => '○',
            (Shape::Circle, UnicodeTheme::Rich) => '●',
            (Shape::Circle, UnicodeTheme::Fancy) => '🔴',

            (Shape::Square, UnicodeTheme::Minimal) => '#',
            (Shape::Square, UnicodeTheme::Basic) => '□',
            (Shape::Square, UnicodeTheme::Rich) => '■',
            (Shape::Square, UnicodeTheme::Fancy) => '🟦',

            (Shape::Triangle, UnicodeTheme::Minimal) => '^',
            (Shape::Triangle, UnicodeTheme::Basic) => '△',
            (Shape::Triangle, UnicodeTheme::Rich) => '▲',
            (Shape::Triangle, UnicodeTheme::Fancy) => '🔺',

            (Shape::Diamond, UnicodeTheme::Minimal) => '<',
            (Shape::Diamond, UnicodeTheme::Basic) => '◇',
            (Shape::Diamond, UnicodeTheme::Rich) => '◆',
            (Shape::Diamond, UnicodeTheme::Fancy) => '💎',

            (Shape::Star, UnicodeTheme::Minimal) => '*',
            (Shape::Star, UnicodeTheme::Basic) => '☆',
            (Shape::Star, UnicodeTheme::Rich) => '★',
            (Shape::Star, UnicodeTheme::Fancy) => '⭐',

            (Shape::Heart, UnicodeTheme::Minimal) => '<',
            (Shape::Heart, UnicodeTheme::Basic) => '♡',
            (Shape::Heart, UnicodeTheme::Rich) => '♥',
            (Shape::Heart, UnicodeTheme::Fancy) => '❤',

            (Shape::Plus, UnicodeTheme::Minimal) => '+',
            (Shape::Plus, UnicodeTheme::Basic) => '+',
            (Shape::Plus, UnicodeTheme::Rich) => '✚',
            (Shape::Plus, UnicodeTheme::Fancy) => '➕',

            (Shape::Cross, UnicodeTheme::Minimal) => 'x',
            (Shape::Cross, UnicodeTheme::Basic) => '✕',
            (Shape::Cross, UnicodeTheme::Rich) => '✖',
            (Shape::Cross, UnicodeTheme::Fancy) => '❌',

            (Shape::Dot, UnicodeTheme::Minimal) => '.',
            (Shape::Dot, UnicodeTheme::Basic) => '•',
            (Shape::Dot, UnicodeTheme::Rich) => '●',
            (Shape::Dot, UnicodeTheme::Fancy) => '🔴',

            (Shape::Bullet, UnicodeTheme::Minimal) => '*',
            (Shape::Bullet, UnicodeTheme::Basic) => '•',
            (Shape::Bullet, UnicodeTheme::Rich) => '●',
            (Shape::Bullet, UnicodeTheme::Fancy) => '🔸',
        }
    }
}

/// Convenience constants for shapes
pub mod chars {
    use super::*;

    pub const CIRCLE: Shape = Shape::Circle;
    pub const SQUARE: Shape = Shape::Square;
    pub const TRIANGLE: Shape = Shape::Triangle;
    pub const DIAMOND: Shape = Shape::Diamond;
    pub const STAR: Shape = Shape::Star;
    pub const HEART: Shape = Shape::Heart;
    pub const PLUS: Shape = Shape::Plus;
    pub const CROSS: Shape = Shape::Cross;
    pub const DOT: Shape = Shape::Dot;
    pub const BULLET: Shape = Shape::Bullet;
}
