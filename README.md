# unicode-rs

A comprehensive Unicode character library for Rust applications, particularly useful for terminal applications, editors, and CLI tools that need consistent Unicode symbol support across different environments and themes.

[![Crates.io](https://img.shields.io/crates/v/unicode-rs.svg)](https://crates.io/crates/unicode-rs)
[![Downloads](https://img.shields.io/crates/d/unicode-rs.svg)](https://crates.io/crates/unicode-rs)
[![GitHub Stars](https://img.shields.io/github/stars/entrepeneur4lyf/unicode-rs)](https://github.com/entrepeneur4lyf/unicode-rs/stargazers)
[![Documentation](https://docs.rs/unicode-rs/badge.svg)](https://docs.rs/unicode-rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

## Features

- **Multiple themes**: Support for Minimal (ASCII), Basic, Rich, and Fancy Unicode themes
- **Categorized symbols**: Organized into logical groups (arrows, blocks, shapes, git, etc.)
- **Fallback support**: Graceful degradation to ASCII when Unicode isn't supported
- **Global configuration**: Set theme and overrides globally for your application
- **Type-safe**: All symbols are strongly typed enums
- **Security utilities**: Detect dangerous Unicode characters and potential attacks
- **Zero dependencies**: Pure Rust implementation with no external dependencies

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
unicode-rs = "0.1.0"
```

## Usage

### Basic Usage

```rust
use unicode_rs::prelude::*;

// Use symbols with different themes
let check_minimal = Symbol::Check.get_char(UnicodeTheme::Minimal); // 'v'
let check_rich = Symbol::Check.get_char(UnicodeTheme::Rich);       // '✓'
let check_fancy = Symbol::Check.get_char(UnicodeTheme::Fancy);     // '✅'

// Use arrows
let right_arrow = Arrow::Right.get_char(UnicodeTheme::Rich);       // '→'
let up_arrow = Arrow::Up.get_char(UnicodeTheme::Rich);             // '↑'

// Git symbols
let modified = GitStatus::Modified.get_char(UnicodeTheme::Rich);   // '●'
let added = GitStatus::Added.get_char(UnicodeTheme::Rich);         // '+'
```

### Global Configuration

```rust
use unicode_rs::prelude::*;

// Set global theme
set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Minimal));

// Now all symbols will use the minimal theme by default
let check = get_char(&Symbol::Check, None); // 'v'
let arrow = get_char(&Arrow::Right, None);  // '>'
```

### Custom Overrides

```rust
use unicode_rs::prelude::*;

// Create config with custom overrides
let config = UnicodeConfig::with_theme(UnicodeTheme::Rich)
    .with_override("custom_check", '√')
    .with_fallback(); // Enable ASCII fallback for unsupported terminals

set_global_config(config);
```

### Unicode Security

```rust
use unicode_rs::security::*;

// Analyze text for security issues
let analysis = analyze_text("Hello\u{200B}World"); // Contains zero-width space
if analysis.risk_level >= RiskLevel::High {
    println!("⚠️ Security risk detected!");
    println!("{}", generate_security_report("Hello\u{200B}World"));
}

// Sanitize dangerous text
let safe_text = sanitize_text("Hello\u{200B}World\u{202E}");
assert_eq!(safe_text, "HelloWorld");
```

## Available Symbol Categories

### Symbols
- Check marks, X marks, exclamation, question marks
- Copyright, trademark, registered symbols

### Arrows
- Directional arrows (up, down, left, right)
- Navigation symbols (home, end, page up/down)

### Blocks
- Box drawing characters
- Block elements for progress bars and layouts

### Shapes
- Geometric shapes (circles, squares, triangles)
- Filled and outlined variants

### Git
- Status indicators (modified, added, deleted, etc.)
- Diff symbols (plus, minus, delta)
- Branch and action symbols

### File Types
- Language-specific file type indicators
- Generic file and folder symbols

### UI Elements
- Borders and separators
- Control symbols
- Status indicators

### Editor
- Cursor and selection indicators
- Text editing symbols

## Themes

The library supports four different themes:

1. **Minimal**: ASCII-only characters for maximum compatibility
2. **Basic**: Basic Unicode symbols
3. **Rich**: Full Unicode symbol set (default)
4. **Fancy**: Decorative Unicode with emoji-style characters

## Examples

Check out the `examples/` directory for more comprehensive usage examples:

- `basic_usage.rs` - Simple symbol usage
- `git_status.rs` - Git status display
- `file_browser.rs` - File browser with type indicators
- `security_analysis.rs` - Unicode security vulnerability detection

## License

This project is licensed under MIT

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
