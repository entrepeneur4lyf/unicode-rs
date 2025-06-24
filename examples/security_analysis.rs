//! Unicode security analysis example
//! 
//! This example demonstrates how to use the unicode-rs security utilities
//! to detect and analyze potentially dangerous Unicode characters that could
//! be used in security attacks.

use unicode_rs::security::*;

fn main() {
    println!("Unicode Security Analysis Example");
    println!("=================================\n");

    // Example 1: Safe text
    println!("1. Analyzing safe text:");
    let safe_text = "Hello World! This is normal text.";
    analyze_and_report(safe_text);

    // Example 2: Text with invisible characters
    println!("\n2. Analyzing text with invisible characters:");
    let invisible_text = "Hello\u{200B}World\u{200C}Test"; // Zero-width space and non-joiner
    analyze_and_report(invisible_text);

    // Example 3: Bidirectional override attack
    println!("\n3. Analyzing bidirectional override attack:");
    let bidi_attack = "filename\u{202E}gpj.exe"; // Right-to-left override
    analyze_and_report(bidi_attack);

    // Example 4: Homograph attack (Cyrillic characters that look like Latin)
    println!("\n4. Analyzing potential homograph attack:");
    let homograph = "раураӏ.com"; // Cyrillic characters that look like "paypal.com"
    analyze_and_report(homograph);

    // Example 5: Mixed script attack
    println!("\n5. Analyzing mixed script text:");
    let mixed_script = "Secure Bank αccount Login"; // Greek alpha instead of 'a'
    analyze_and_report(mixed_script);

    // Example 6: Complex attack with multiple vectors
    println!("\n6. Analyzing complex multi-vector attack:");
    let complex_attack = "bank\u{200B}login\u{202E}moc.evil"; // Invisible char + bidi override
    analyze_and_report(complex_attack);

    // Example 7: Demonstrate sanitization
    println!("\n7. Text sanitization example:");
    let dangerous = "Hello\u{200B}World\u{202E}Dangerous\u{200C}Text";
    println!("Original: {:?}", dangerous);
    let sanitized = sanitize_text(dangerous);
    println!("Sanitized: {:?}", sanitized);
    println!("Safe to use: {}", analyze_text(&sanitized).risk_level == RiskLevel::Low);

    // Example 8: Character-by-character analysis
    println!("\n8. Character-by-character analysis:");
    let test_chars = "a\u{200B}b\u{202E}c";
    for (i, ch) in test_chars.char_indices() {
        println!("  Position {}: '{}' (U+{:04X})", i, ch, ch as u32);
        if is_invisible_char(ch) {
            println!("    ⚠️  Invisible character: {}", get_char_description(ch));
        }
        if is_bidi_char(ch) {
            println!("    ⚠️  Bidirectional character: {}", get_char_description(ch));
        }
        if is_confusable_char(ch) {
            println!("    ⚠️  Potentially confusable character");
        }
    }

    // Example 9: Script detection
    println!("\n9. Script detection example:");
    let multi_script = "Hello мир 世界 שלום";
    let analysis = analyze_text(multi_script);
    println!("Text: {}", multi_script);
    println!("Detected scripts:");
    for script in &analysis.scripts {
        println!("  - {:?}", script);
    }

    // Example 10: Security recommendations
    println!("\n10. Security recommendations:");
    println!("✅ Always validate user input for invisible characters");
    println!("✅ Check for bidirectional override attacks in filenames");
    println!("✅ Be aware of homograph attacks in domain names");
    println!("✅ Consider normalizing Unicode text before processing");
    println!("✅ Use allowlists for acceptable character ranges when possible");
}

fn analyze_and_report(text: &str) {
    println!("Text: {:?}", text);
    
    let analysis = analyze_text(text);
    
    // Quick summary
    print!("Risk Level: ");
    match analysis.risk_level {
        RiskLevel::Low => println!("🟢 LOW"),
        RiskLevel::Medium => println!("🟡 MEDIUM"),
        RiskLevel::High => println!("🟠 HIGH"),
        RiskLevel::Critical => println!("🔴 CRITICAL"),
    }

    // Detailed findings
    if analysis.has_invisible_chars {
        println!("⚠️  {} invisible character(s) detected", analysis.invisible_chars.len());
    }
    if analysis.has_bidi_overrides {
        println!("⚠️  {} bidirectional override(s) detected", analysis.bidi_chars.len());
    }
    if analysis.has_mixed_scripts {
        println!("⚠️  Mixed scripts detected ({} different scripts)", analysis.scripts.len());
    }
    if analysis.has_confusables {
        println!("⚠️  Confusable characters detected");
    }

    if analysis.risk_level == RiskLevel::Low {
        println!("✅ No security concerns detected");
    }

    // Show detailed report for high-risk items
    if analysis.risk_level >= RiskLevel::High {
        println!("\nDetailed Security Report:");
        println!("{}", generate_security_report(text));
    }
}
