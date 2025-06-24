//! UI element Unicode characters
//! Characters for buttons, borders, separators, and interface elements

use super::{UnicodeProvider, UnicodeTheme};

/// UI border characters
#[derive(Debug, Clone, Copy)]
pub enum Border {
    /// Horizontal line
    Horizontal,
    /// Vertical line
    Vertical,
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Cross/intersection
    Cross,
    /// T-junction up
    TeeUp,
    /// T-junction down
    TeeDown,
    /// T-junction left
    TeeLeft,
    /// T-junction right
    TeeRight,
}

impl UnicodeProvider for Border {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Border::Horizontal, UnicodeTheme::Minimal) => '-',
            (Border::Horizontal, UnicodeTheme::Basic) => '-',
            (Border::Horizontal, UnicodeTheme::Rich) => '─',
            (Border::Horizontal, UnicodeTheme::Fancy) => '━',

            (Border::Vertical, UnicodeTheme::Minimal) => '|',
            (Border::Vertical, UnicodeTheme::Basic) => '|',
            (Border::Vertical, UnicodeTheme::Rich) => '│',
            (Border::Vertical, UnicodeTheme::Fancy) => '┃',

            (Border::TopLeft, UnicodeTheme::Minimal) => '+',
            (Border::TopLeft, UnicodeTheme::Basic) => '+',
            (Border::TopLeft, UnicodeTheme::Rich) => '┌',
            (Border::TopLeft, UnicodeTheme::Fancy) => '┏',

            (Border::TopRight, UnicodeTheme::Minimal) => '+',
            (Border::TopRight, UnicodeTheme::Basic) => '+',
            (Border::TopRight, UnicodeTheme::Rich) => '┐',
            (Border::TopRight, UnicodeTheme::Fancy) => '┓',

            (Border::BottomLeft, UnicodeTheme::Minimal) => '+',
            (Border::BottomLeft, UnicodeTheme::Basic) => '+',
            (Border::BottomLeft, UnicodeTheme::Rich) => '└',
            (Border::BottomLeft, UnicodeTheme::Fancy) => '┗',

            (Border::BottomRight, UnicodeTheme::Minimal) => '+',
            (Border::BottomRight, UnicodeTheme::Basic) => '+',
            (Border::BottomRight, UnicodeTheme::Rich) => '┘',
            (Border::BottomRight, UnicodeTheme::Fancy) => '┛',

            (Border::Cross, UnicodeTheme::Minimal) => '+',
            (Border::Cross, UnicodeTheme::Basic) => '+',
            (Border::Cross, UnicodeTheme::Rich) => '┼',
            (Border::Cross, UnicodeTheme::Fancy) => '╋',

            (Border::TeeUp, UnicodeTheme::Minimal) => '+',
            (Border::TeeUp, UnicodeTheme::Basic) => '+',
            (Border::TeeUp, UnicodeTheme::Rich) => '┴',
            (Border::TeeUp, UnicodeTheme::Fancy) => '┻',

            (Border::TeeDown, UnicodeTheme::Minimal) => '+',
            (Border::TeeDown, UnicodeTheme::Basic) => '+',
            (Border::TeeDown, UnicodeTheme::Rich) => '┬',
            (Border::TeeDown, UnicodeTheme::Fancy) => '┳',

            (Border::TeeLeft, UnicodeTheme::Minimal) => '+',
            (Border::TeeLeft, UnicodeTheme::Basic) => '+',
            (Border::TeeLeft, UnicodeTheme::Rich) => '┤',
            (Border::TeeLeft, UnicodeTheme::Fancy) => '┫',

            (Border::TeeRight, UnicodeTheme::Minimal) => '+',
            (Border::TeeRight, UnicodeTheme::Basic) => '+',
            (Border::TeeRight, UnicodeTheme::Rich) => '├',
            (Border::TeeRight, UnicodeTheme::Fancy) => '┣',
        }
    }
}

/// UI button and control characters
#[derive(Debug, Clone, Copy)]
pub enum Control {
    /// Checkbox unchecked
    CheckboxUnchecked,
    /// Checkbox checked
    CheckboxChecked,
    /// Radio button unselected
    RadioUnselected,
    /// Radio button selected
    RadioSelected,
    /// Button
    Button,
    /// Menu item
    MenuItem,
    /// Dropdown arrow
    DropdownArrow,
    /// Expand/collapse indicator (collapsed)
    ExpandCollapsed,
    /// Expand/collapse indicator (expanded)
    ExpandExpanded,
    /// Loading/spinner
    Loading,
    /// Close button
    Close,
    /// Minimize button
    Minimize,
    /// Maximize button
    Maximize,
}

impl UnicodeProvider for Control {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Control::CheckboxUnchecked, UnicodeTheme::Minimal) => '[',
            (Control::CheckboxUnchecked, UnicodeTheme::Basic) => '☐',
            (Control::CheckboxUnchecked, UnicodeTheme::Rich) => '☐',
            (Control::CheckboxUnchecked, UnicodeTheme::Fancy) => '🔲',

            (Control::CheckboxChecked, UnicodeTheme::Minimal) => 'X',
            (Control::CheckboxChecked, UnicodeTheme::Basic) => '☑',
            (Control::CheckboxChecked, UnicodeTheme::Rich) => '☑',
            (Control::CheckboxChecked, UnicodeTheme::Fancy) => '✅',

            (Control::RadioUnselected, UnicodeTheme::Minimal) => '(',
            (Control::RadioUnselected, UnicodeTheme::Basic) => '○',
            (Control::RadioUnselected, UnicodeTheme::Rich) => '○',
            (Control::RadioUnselected, UnicodeTheme::Fancy) => '⚪',

            (Control::RadioSelected, UnicodeTheme::Minimal) => '*',
            (Control::RadioSelected, UnicodeTheme::Basic) => '●',
            (Control::RadioSelected, UnicodeTheme::Rich) => '●',
            (Control::RadioSelected, UnicodeTheme::Fancy) => '🔘',

            (Control::Button, UnicodeTheme::Minimal) => '[',
            (Control::Button, UnicodeTheme::Basic) => '▢',
            (Control::Button, UnicodeTheme::Rich) => '▢',
            (Control::Button, UnicodeTheme::Fancy) => '🔳',

            (Control::MenuItem, UnicodeTheme::Minimal) => '-',
            (Control::MenuItem, UnicodeTheme::Basic) => '•',
            (Control::MenuItem, UnicodeTheme::Rich) => '▸',
            (Control::MenuItem, UnicodeTheme::Fancy) => '🔸',

            (Control::DropdownArrow, UnicodeTheme::Minimal) => 'v',
            (Control::DropdownArrow, UnicodeTheme::Basic) => '▼',
            (Control::DropdownArrow, UnicodeTheme::Rich) => '▼',
            (Control::DropdownArrow, UnicodeTheme::Fancy) => '🔽',

            (Control::ExpandCollapsed, UnicodeTheme::Minimal) => '>',
            (Control::ExpandCollapsed, UnicodeTheme::Basic) => '▶',
            (Control::ExpandCollapsed, UnicodeTheme::Rich) => '▶',
            (Control::ExpandCollapsed, UnicodeTheme::Fancy) => '▶',

            (Control::ExpandExpanded, UnicodeTheme::Minimal) => 'v',
            (Control::ExpandExpanded, UnicodeTheme::Basic) => '▼',
            (Control::ExpandExpanded, UnicodeTheme::Rich) => '▼',
            (Control::ExpandExpanded, UnicodeTheme::Fancy) => '🔽',

            (Control::Loading, UnicodeTheme::Minimal) => '|',
            (Control::Loading, UnicodeTheme::Basic) => '◐',
            (Control::Loading, UnicodeTheme::Rich) => '◐',
            (Control::Loading, UnicodeTheme::Fancy) => '🔄',

            (Control::Close, UnicodeTheme::Minimal) => 'X',
            (Control::Close, UnicodeTheme::Basic) => '✕',
            (Control::Close, UnicodeTheme::Rich) => '✕',
            (Control::Close, UnicodeTheme::Fancy) => '❌',

            (Control::Minimize, UnicodeTheme::Minimal) => '_',
            (Control::Minimize, UnicodeTheme::Basic) => '−',
            (Control::Minimize, UnicodeTheme::Rich) => '−',
            (Control::Minimize, UnicodeTheme::Fancy) => '➖',

            (Control::Maximize, UnicodeTheme::Minimal) => '^',
            (Control::Maximize, UnicodeTheme::Basic) => '□',
            (Control::Maximize, UnicodeTheme::Rich) => '□',
            (Control::Maximize, UnicodeTheme::Fancy) => '⬜',
        }
    }
}

/// UI separator characters
#[derive(Debug, Clone, Copy)]
pub enum Separator {
    /// Thin separator
    Thin,
    /// Thick separator
    Thick,
    /// Dotted separator
    Dotted,
    /// Dashed separator
    Dashed,
    /// Double separator
    Double,
    /// Wavy separator
    Wavy,
}

impl UnicodeProvider for Separator {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Separator::Thin, UnicodeTheme::Minimal) => '-',
            (Separator::Thin, UnicodeTheme::Basic) => '─',
            (Separator::Thin, UnicodeTheme::Rich) => '─',
            (Separator::Thin, UnicodeTheme::Fancy) => '─',

            (Separator::Thick, UnicodeTheme::Minimal) => '=',
            (Separator::Thick, UnicodeTheme::Basic) => '━',
            (Separator::Thick, UnicodeTheme::Rich) => '━',
            (Separator::Thick, UnicodeTheme::Fancy) => '━',

            (Separator::Dotted, UnicodeTheme::Minimal) => '.',
            (Separator::Dotted, UnicodeTheme::Basic) => '┄',
            (Separator::Dotted, UnicodeTheme::Rich) => '┄',
            (Separator::Dotted, UnicodeTheme::Fancy) => '┈',

            (Separator::Dashed, UnicodeTheme::Minimal) => '-',
            (Separator::Dashed, UnicodeTheme::Basic) => '┅',
            (Separator::Dashed, UnicodeTheme::Rich) => '┅',
            (Separator::Dashed, UnicodeTheme::Fancy) => '┉',

            (Separator::Double, UnicodeTheme::Minimal) => '=',
            (Separator::Double, UnicodeTheme::Basic) => '═',
            (Separator::Double, UnicodeTheme::Rich) => '═',
            (Separator::Double, UnicodeTheme::Fancy) => '═',

            (Separator::Wavy, UnicodeTheme::Minimal) => '~',
            (Separator::Wavy, UnicodeTheme::Basic) => '〜',
            (Separator::Wavy, UnicodeTheme::Rich) => '〜',
            (Separator::Wavy, UnicodeTheme::Fancy) => '〰',
        }
    }
}

/// UI indicator characters
#[derive(Debug, Clone, Copy)]
pub enum Indicator {
    /// Success/OK
    Success,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Info
    Info,
    /// Question
    Question,
    /// Attention
    Attention,
    /// Progress
    Progress,
    /// Complete
    Complete,
    /// Pending
    Pending,
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl UnicodeProvider for Indicator {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (Indicator::Success, UnicodeTheme::Minimal) => '+',
            (Indicator::Success, UnicodeTheme::Basic) => '✓',
            (Indicator::Success, UnicodeTheme::Rich) => '✓',
            (Indicator::Success, UnicodeTheme::Fancy) => '✅',

            (Indicator::Warning, UnicodeTheme::Minimal) => '!',
            (Indicator::Warning, UnicodeTheme::Basic) => '⚠',
            (Indicator::Warning, UnicodeTheme::Rich) => '⚠',
            (Indicator::Warning, UnicodeTheme::Fancy) => '⚠',

            (Indicator::Error, UnicodeTheme::Minimal) => 'X',
            (Indicator::Error, UnicodeTheme::Basic) => '✗',
            (Indicator::Error, UnicodeTheme::Rich) => '✗',
            (Indicator::Error, UnicodeTheme::Fancy) => '❌',

            (Indicator::Info, UnicodeTheme::Minimal) => 'i',
            (Indicator::Info, UnicodeTheme::Basic) => 'ℹ',
            (Indicator::Info, UnicodeTheme::Rich) => 'ℹ',
            (Indicator::Info, UnicodeTheme::Fancy) => 'ℹ',

            (Indicator::Question, UnicodeTheme::Minimal) => '?',
            (Indicator::Question, UnicodeTheme::Basic) => '?',
            (Indicator::Question, UnicodeTheme::Rich) => '❓',
            (Indicator::Question, UnicodeTheme::Fancy) => '❓',

            (Indicator::Attention, UnicodeTheme::Minimal) => '*',
            (Indicator::Attention, UnicodeTheme::Basic) => '●',
            (Indicator::Attention, UnicodeTheme::Rich) => '●',
            (Indicator::Attention, UnicodeTheme::Fancy) => '🔴',

            (Indicator::Progress, UnicodeTheme::Minimal) => '.',
            (Indicator::Progress, UnicodeTheme::Basic) => '◐',
            (Indicator::Progress, UnicodeTheme::Rich) => '◐',
            (Indicator::Progress, UnicodeTheme::Fancy) => '🔄',

            (Indicator::Complete, UnicodeTheme::Minimal) => '*',
            (Indicator::Complete, UnicodeTheme::Basic) => '●',
            (Indicator::Complete, UnicodeTheme::Rich) => '●',
            (Indicator::Complete, UnicodeTheme::Fancy) => '🟢',

            (Indicator::Pending, UnicodeTheme::Minimal) => 'o',
            (Indicator::Pending, UnicodeTheme::Basic) => '○',
            (Indicator::Pending, UnicodeTheme::Rich) => '○',
            (Indicator::Pending, UnicodeTheme::Fancy) => '⚪',

            (Indicator::Active, UnicodeTheme::Minimal) => '*',
            (Indicator::Active, UnicodeTheme::Basic) => '●',
            (Indicator::Active, UnicodeTheme::Rich) => '●',
            (Indicator::Active, UnicodeTheme::Fancy) => '🟢',

            (Indicator::Inactive, UnicodeTheme::Minimal) => 'o',
            (Indicator::Inactive, UnicodeTheme::Basic) => '○',
            (Indicator::Inactive, UnicodeTheme::Rich) => '○',
            (Indicator::Inactive, UnicodeTheme::Fancy) => '⚪',
        }
    }
}

/// Convenience constants for UI elements
pub mod chars {
    use super::*;

    // Borders
    pub const HORIZONTAL: Border = Border::Horizontal;
    pub const VERTICAL: Border = Border::Vertical;
    pub const TOP_LEFT: Border = Border::TopLeft;
    pub const TOP_RIGHT: Border = Border::TopRight;
    pub const BOTTOM_LEFT: Border = Border::BottomLeft;
    pub const BOTTOM_RIGHT: Border = Border::BottomRight;
    pub const CROSS: Border = Border::Cross;

    // Controls
    pub const CHECKBOX_UNCHECKED: Control = Control::CheckboxUnchecked;
    pub const CHECKBOX_CHECKED: Control = Control::CheckboxChecked;
    pub const RADIO_UNSELECTED: Control = Control::RadioUnselected;
    pub const RADIO_SELECTED: Control = Control::RadioSelected;
    pub const BUTTON: Control = Control::Button;
    pub const MENU_ITEM: Control = Control::MenuItem;
    pub const DROPDOWN_ARROW: Control = Control::DropdownArrow;
    pub const EXPAND_COLLAPSED: Control = Control::ExpandCollapsed;
    pub const EXPAND_EXPANDED: Control = Control::ExpandExpanded;
    pub const LOADING: Control = Control::Loading;
    pub const CLOSE: Control = Control::Close;
    pub const MINIMIZE: Control = Control::Minimize;
    pub const MAXIMIZE: Control = Control::Maximize;

    // Separators
    pub const SEPARATOR_THIN: Separator = Separator::Thin;
    pub const SEPARATOR_THICK: Separator = Separator::Thick;
    pub const SEPARATOR_DOTTED: Separator = Separator::Dotted;
    pub const SEPARATOR_DASHED: Separator = Separator::Dashed;
    pub const SEPARATOR_DOUBLE: Separator = Separator::Double;
    pub const SEPARATOR_WAVY: Separator = Separator::Wavy;

    // Indicators
    pub const SUCCESS: Indicator = Indicator::Success;
    pub const WARNING: Indicator = Indicator::Warning;
    pub const ERROR: Indicator = Indicator::Error;
    pub const INFO: Indicator = Indicator::Info;
    pub const QUESTION: Indicator = Indicator::Question;
    pub const ATTENTION: Indicator = Indicator::Attention;
    pub const PROGRESS: Indicator = Indicator::Progress;
    pub const COMPLETE: Indicator = Indicator::Complete;
    pub const PENDING: Indicator = Indicator::Pending;
    pub const ACTIVE: Indicator = Indicator::Active;
    pub const INACTIVE: Indicator = Indicator::Inactive;
}
