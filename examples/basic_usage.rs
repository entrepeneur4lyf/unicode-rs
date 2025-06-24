//! Basic usage example for unicode-rs
//!
//! This example demonstrates the basic functionality of the unicode-rs crate,
//! showing how to use different symbols with various themes.

use unicode_rs::prelude::*;

fn main() {
    println!("Unicode-rs Basic Usage Example");
    println!("==============================\n");

    // Demonstrate different themes for the same symbol
    println!("Symbol themes:");
    println!(
        "  Minimal: {}",
        Symbol::Check.get_char(UnicodeTheme::Minimal)
    );
    println!("  Basic:   {}", Symbol::Check.get_char(UnicodeTheme::Basic));
    println!("  Rich:    {}", Symbol::Check.get_char(UnicodeTheme::Rich));
    println!("  Fancy:   {}", Symbol::Check.get_char(UnicodeTheme::Fancy));
    println!();

    // Show various symbol categories
    println!("Symbol categories:");

    // Symbols
    println!("  Symbols:");
    println!("    Check: {}", Symbol::Check.get_char(UnicodeTheme::Rich));
    println!("    X:     {}", Symbol::X.get_char(UnicodeTheme::Rich));
    println!(
        "    !:     {}",
        Symbol::Exclamation.get_char(UnicodeTheme::Rich)
    );
    println!(
        "    ?:     {}",
        Symbol::Question.get_char(UnicodeTheme::Rich)
    );
    println!();

    // Arrows
    println!("  Arrows:");
    println!("    Up:    {}", Arrow::Up.get_char(UnicodeTheme::Rich));
    println!("    Down:  {}", Arrow::Down.get_char(UnicodeTheme::Rich));
    println!("    Left:  {}", Arrow::Left.get_char(UnicodeTheme::Rich));
    println!("    Right: {}", Arrow::Right.get_char(UnicodeTheme::Rich));
    println!();

    // Git symbols
    println!("  Git Status:");
    println!(
        "    Modified:  {}",
        GitStatus::Modified.get_char(UnicodeTheme::Rich)
    );
    println!(
        "    Added:     {}",
        GitStatus::Added.get_char(UnicodeTheme::Rich)
    );
    println!(
        "    Deleted:   {}",
        GitStatus::Deleted.get_char(UnicodeTheme::Rich)
    );
    println!(
        "    Untracked: {}",
        GitStatus::Untracked.get_char(UnicodeTheme::Rich)
    );
    println!();

    // Demonstrate global configuration
    println!("Global configuration example:");

    // Set global theme to minimal
    set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Minimal));
    println!("  With Minimal theme:");
    println!("    Check: {}", get_char(&Symbol::Check, None));
    println!("    Arrow: {}", get_char(&Arrow::Right, None));

    // Set global theme to rich with fallback
    set_global_config(
        UnicodeConfig::with_theme(UnicodeTheme::Rich)
            .with_fallback()
            .with_override("custom_check", '√'),
    );
    println!("  With Rich theme + fallback:");
    println!("    Check: {}", get_char(&Symbol::Check, None));
    println!(
        "    Custom: {}",
        get_char(&Symbol::Check, Some("custom_check"))
    );

    println!("\nExample complete!");
}
