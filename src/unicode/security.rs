//! Unicode security utilities
//!
//! This module provides utilities to detect and analyze potentially dangerous Unicode characters
//! that could be used in security attacks such as:
//! - Homograph attacks (visually similar characters)
//! - Invisible character injection
//! - Bidirectional text attacks
//! - Mixed script attacks
//!
//! # Examples
//!
//! ```rust
//! use unicode_rs::security::*;
//!
//! // Check for suspicious characters
//! let result = analyze_text("Hello\u{200B}World"); // Zero-width space
//! assert!(result.has_invisible_chars);
//!
//! // Detect homograph attacks
//! let suspicious = "раураӏ.com"; // Cyrillic characters that look like "paypal.com"
//! let analysis = analyze_text(suspicious);
//! assert!(analysis.has_mixed_scripts);
//! ```

use std::collections::HashSet;

/// Security analysis result for Unicode text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAnalysis {
    /// Contains invisible or zero-width characters
    pub has_invisible_chars: bool,
    /// Contains bidirectional override characters
    pub has_bidi_overrides: bool,
    /// Contains mixed scripts (potential homograph attack)
    pub has_mixed_scripts: bool,
    /// Contains confusable characters
    pub has_confusables: bool,
    /// List of detected invisible characters with their positions
    pub invisible_chars: Vec<(usize, char, &'static str)>,
    /// List of detected bidirectional characters with their positions
    pub bidi_chars: Vec<(usize, char, &'static str)>,
    /// Set of detected scripts
    pub scripts: HashSet<Script>,
    /// Overall risk level
    pub risk_level: RiskLevel,
}

/// Unicode script categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Script {
    Latin,
    Cyrillic,
    Greek,
    Arabic,
    Hebrew,
    Chinese,
    Japanese,
    Korean,
    Thai,
    Devanagari,
    Other(u32), // Unicode script code
}

/// Risk level assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// No security concerns detected
    Low,
    /// Minor concerns, likely benign
    Medium,
    /// Significant security concerns
    High,
    /// Critical security risk
    Critical,
}

/// Analyze text for Unicode security issues
///
/// This function performs a comprehensive analysis of the input text to detect
/// various Unicode-based security vulnerabilities.
///
/// # Arguments
///
/// * `text` - The text to analyze
///
/// # Examples
///
/// ```rust
/// use unicode_rs::security::*;
///
/// // Safe text
/// let safe = analyze_text("Hello World");
/// assert_eq!(safe.risk_level, RiskLevel::Low);
///
/// // Text with invisible characters
/// let suspicious = analyze_text("Hello\u{200B}World");
/// assert_eq!(suspicious.risk_level, RiskLevel::High);
/// ```
pub fn analyze_text(text: &str) -> SecurityAnalysis {
    let mut analysis = SecurityAnalysis {
        has_invisible_chars: false,
        has_bidi_overrides: false,
        has_mixed_scripts: false,
        has_confusables: false,
        invisible_chars: Vec::new(),
        bidi_chars: Vec::new(),
        scripts: HashSet::new(),
        risk_level: RiskLevel::Low,
    };

    for (pos, ch) in text.char_indices() {
        // Check for invisible characters
        if is_invisible_char(ch) {
            analysis.has_invisible_chars = true;
            analysis.invisible_chars.push((pos, ch, get_char_description(ch)));
        }

        // Check for bidirectional override characters
        if is_bidi_char(ch) {
            analysis.has_bidi_overrides = true;
            analysis.bidi_chars.push((pos, ch, get_char_description(ch)));
        }

        // Collect scripts
        let script = get_script(ch);
        analysis.scripts.insert(script);

        // Check for confusable characters
        if is_confusable_char(ch) {
            analysis.has_confusables = true;
        }
    }

    // Determine if mixed scripts (potential homograph attack)
    // Only consider it mixed scripts if we have non-Latin scripts mixed with Latin,
    // or multiple non-Latin scripts
    let non_latin_scripts: Vec<_> = analysis.scripts.iter()
        .filter(|s| !matches!(s, Script::Latin))
        .collect();

    analysis.has_mixed_scripts = non_latin_scripts.len() > 1
        || (non_latin_scripts.len() == 1 && analysis.scripts.contains(&Script::Latin));

    // Calculate risk level
    analysis.risk_level = calculate_risk_level(&analysis);

    analysis
}

/// Check if a character is invisible or zero-width
pub fn is_invisible_char(ch: char) -> bool {
    matches!(ch,
        '\u{00AD}' | // Soft hyphen
        '\u{034F}' | // Combining grapheme joiner
        '\u{061C}' | // Arabic letter mark
        '\u{115F}' | // Hangul choseong filler
        '\u{1160}' | // Hangul jungseong filler
        '\u{17B4}' | // Khmer vowel inherent AQ
        '\u{17B5}' | // Khmer vowel inherent AA
        '\u{180E}' | // Mongolian vowel separator
        '\u{200B}' | // Zero width space
        '\u{200C}' | // Zero width non-joiner
        '\u{200D}' | // Zero width joiner
        '\u{200E}' | // Left-to-right mark
        '\u{200F}' | // Right-to-left mark
        '\u{202A}' | // Left-to-right embedding
        '\u{202B}' | // Right-to-left embedding
        '\u{202C}' | // Pop directional formatting
        '\u{202D}' | // Left-to-right override
        '\u{202E}' | // Right-to-left override
        '\u{2060}' | // Word joiner
        '\u{2061}' | // Function application
        '\u{2062}' | // Invisible times
        '\u{2063}' | // Invisible separator
        '\u{2064}' | // Invisible plus
        '\u{206A}' | // Inhibit symmetric swapping
        '\u{206B}' | // Activate symmetric swapping
        '\u{206C}' | // Inhibit Arabic form shaping
        '\u{206D}' | // Activate Arabic form shaping
        '\u{206E}' | // National digit shapes
        '\u{206F}' | // Nominal digit shapes
        '\u{3164}' | // Hangul filler
        '\u{FEFF}' | // Zero width no-break space (BOM)
        '\u{FFA0}' | // Halfwidth Hangul filler
        '\u{1D159}' | // Musical symbol null notehead
        '\u{1D173}' | // Musical symbol begin beam
        '\u{1D174}' | // Musical symbol end beam
        '\u{1D175}' | // Musical symbol begin tie
        '\u{1D176}' | // Musical symbol end tie
        '\u{1D177}' | // Musical symbol begin slur
        '\u{1D178}' | // Musical symbol end slur
        '\u{1D179}' | // Musical symbol begin phrase
        '\u{1D17A}'   // Musical symbol end phrase
    )
}

/// Check if a character is a bidirectional override character
pub fn is_bidi_char(ch: char) -> bool {
    matches!(ch,
        '\u{061C}' | // Arabic letter mark
        '\u{200E}' | // Left-to-right mark
        '\u{200F}' | // Right-to-left mark
        '\u{202A}' | // Left-to-right embedding
        '\u{202B}' | // Right-to-left embedding
        '\u{202C}' | // Pop directional formatting
        '\u{202D}' | // Left-to-right override
        '\u{202E}' | // Right-to-left override
        '\u{2066}' | // Left-to-right isolate
        '\u{2067}' | // Right-to-left isolate
        '\u{2068}' | // First strong isolate
        '\u{2069}'   // Pop directional isolate
    )
}

/// Check if a character is commonly used in confusable attacks
pub fn is_confusable_char(ch: char) -> bool {
    // Common confusable characters (this is a simplified set)
    matches!(ch,
        // Cyrillic that look like Latin
        'а' | 'е' | 'о' | 'р' | 'с' | 'у' | 'х' | 'А' | 'В' | 'Е' | 'К' | 'М' | 'Н' | 'О' | 'Р' | 'С' | 'Т' | 'У' | 'Х' |
        // Greek that look like Latin  
        'α' | 'β' | 'γ' | 'δ' | 'ε' | 'ζ' | 'η' | 'θ' | 'ι' | 'κ' | 'λ' | 'μ' | 'ν' | 'ξ' | 'ο' | 'π' | 'ρ' | 'σ' | 'τ' | 'υ' | 'φ' | 'χ' | 'ψ' | 'ω' |
        // Mathematical symbols that look like Latin
        '𝐀' | '𝐁' | '𝐂' | '𝐃' | '𝐄' | '𝐅' | '𝐆' | '𝐇' | '𝐈' | '𝐉' | '𝐊' | '𝐋' | '𝐌' | '𝐍' | '𝐎' | '𝐏' | '𝐐' | '𝐑' | '𝐒' | '𝐓' | '𝐔' | '𝐕' | '𝐖' | '𝐗' | '𝐘' | '𝐙'
    )
}

/// Get the script category for a character
pub fn get_script(ch: char) -> Script {
    match ch {
        'A'..='Z' | 'a'..='z' => Script::Latin,
        'А'..='я' | 'Ё' | 'ё' => Script::Cyrillic,
        'Α'..='ω' => Script::Greek,
        '\u{0600}'..='\u{06FF}' => Script::Arabic,
        '\u{0590}'..='\u{05FF}' => Script::Hebrew,
        '\u{4E00}'..='\u{9FFF}' => Script::Chinese,
        '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' => Script::Japanese,
        '\u{AC00}'..='\u{D7AF}' => Script::Korean,
        '\u{0E00}'..='\u{0E7F}' => Script::Thai,
        '\u{0900}'..='\u{097F}' => Script::Devanagari,
        // Don't count common punctuation, digits, and whitespace as separate scripts
        '0'..='9' | ' ' | '\t' | '\n' | '\r' | '!' | '?' | '.' | ',' | ';' | ':' |
        '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '-' | '_' | '=' | '+' |
        '*' | '/' | '\\' | '|' | '@' | '#' | '$' | '%' | '^' | '&' | '~' | '`' => Script::Latin,
        _ => Script::Other(ch as u32),
    }
}

/// Get a human-readable description of a character
pub fn get_char_description(ch: char) -> &'static str {
    match ch {
        '\u{00AD}' => "Soft Hyphen",
        '\u{200B}' => "Zero Width Space",
        '\u{200C}' => "Zero Width Non-Joiner",
        '\u{200D}' => "Zero Width Joiner",
        '\u{200E}' => "Left-to-Right Mark",
        '\u{200F}' => "Right-to-Left Mark",
        '\u{202A}' => "Left-to-Right Embedding",
        '\u{202B}' => "Right-to-Left Embedding",
        '\u{202C}' => "Pop Directional Formatting",
        '\u{202D}' => "Left-to-Right Override",
        '\u{202E}' => "Right-to-Left Override",
        '\u{2060}' => "Word Joiner",
        '\u{FEFF}' => "Zero Width No-Break Space (BOM)",
        _ => "Unknown Special Character",
    }
}

/// Calculate the overall risk level based on analysis results
fn calculate_risk_level(analysis: &SecurityAnalysis) -> RiskLevel {
    let mut score = 0;

    if analysis.has_invisible_chars {
        score += 3;
    }
    if analysis.has_bidi_overrides {
        score += 4;
    }
    if analysis.has_mixed_scripts {
        score += 2;
    }
    if analysis.has_confusables {
        score += 2;
    }

    // Additional scoring based on quantity
    if analysis.invisible_chars.len() > 3 {
        score += 2;
    }
    if analysis.bidi_chars.len() > 1 {
        score += 2;
    }

    match score {
        0 => RiskLevel::Low,
        1..=3 => RiskLevel::Medium,
        4..=6 => RiskLevel::High,
        _ => RiskLevel::Critical,
    }
}

/// Sanitize text by removing dangerous Unicode characters
///
/// # Examples
///
/// ```rust
/// use unicode_rs::security::*;
///
/// let dangerous = "Hello\u{200B}World\u{202E}";
/// let safe = sanitize_text(dangerous);
/// assert_eq!(safe, "HelloWorld");
/// ```
pub fn sanitize_text(text: &str) -> String {
    text.chars()
        .filter(|&ch| !is_invisible_char(ch) && !is_bidi_char(ch))
        .collect()
}

/// Generate a security report for the given text
///
/// # Examples
///
/// ```rust
/// use unicode_rs::security::*;
///
/// let report = generate_security_report("Hello\u{200B}World");
/// println!("{}", report);
/// ```
pub fn generate_security_report(text: &str) -> String {
    let analysis = analyze_text(text);
    let mut report = String::new();

    report.push_str(&format!("Unicode Security Analysis\n"));
    report.push_str(&format!("========================\n\n"));
    report.push_str(&format!("Risk Level: {:?}\n\n", analysis.risk_level));

    if analysis.has_invisible_chars {
        report.push_str("⚠️  INVISIBLE CHARACTERS DETECTED:\n");
        for (pos, ch, desc) in &analysis.invisible_chars {
            report.push_str(&format!("  Position {}: U+{:04X} ({})\n", pos, *ch as u32, desc));
        }
        report.push('\n');
    }

    if analysis.has_bidi_overrides {
        report.push_str("⚠️  BIDIRECTIONAL OVERRIDE CHARACTERS DETECTED:\n");
        for (pos, ch, desc) in &analysis.bidi_chars {
            report.push_str(&format!("  Position {}: U+{:04X} ({})\n", pos, *ch as u32, desc));
        }
        report.push('\n');
    }

    if analysis.has_mixed_scripts {
        report.push_str("⚠️  MIXED SCRIPTS DETECTED (Potential Homograph Attack):\n");
        for script in &analysis.scripts {
            report.push_str(&format!("  {:?}\n", script));
        }
        report.push('\n');
    }

    if analysis.has_confusables {
        report.push_str("⚠️  CONFUSABLE CHARACTERS DETECTED\n\n");
    }

    if analysis.risk_level == RiskLevel::Low {
        report.push_str("✅ No security concerns detected.\n");
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_text() {
        let analysis = analyze_text("Hello World");
        assert_eq!(analysis.risk_level, RiskLevel::Low);
        assert!(!analysis.has_invisible_chars);
        assert!(!analysis.has_bidi_overrides);
        assert!(!analysis.has_confusables);
    }

    #[test]
    fn test_invisible_characters() {
        let text = "Hello\u{200B}World";
        let analysis = analyze_text(text);
        assert!(analysis.has_invisible_chars);
        assert_eq!(analysis.invisible_chars.len(), 1);
        assert_eq!(analysis.invisible_chars[0].1, '\u{200B}');
        assert!(analysis.risk_level >= RiskLevel::High);
    }

    #[test]
    fn test_bidi_override() {
        let text = "filename\u{202E}gpj.exe";
        let analysis = analyze_text(text);
        assert!(analysis.has_bidi_overrides);
        assert_eq!(analysis.bidi_chars.len(), 1);
        assert_eq!(analysis.bidi_chars[0].1, '\u{202E}');
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_mixed_scripts() {
        let text = "раураӏ.com"; // Cyrillic that looks like "paypal.com"
        let analysis = analyze_text(text);
        assert!(analysis.has_mixed_scripts);
        assert!(analysis.scripts.len() > 1);
        assert!(analysis.risk_level >= RiskLevel::High);
    }

    #[test]
    fn test_sanitization() {
        let dangerous = "Hello\u{200B}World\u{202E}Test";
        let sanitized = sanitize_text(dangerous);
        assert_eq!(sanitized, "HelloWorldTest");

        let analysis = analyze_text(&sanitized);
        assert_eq!(analysis.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_character_detection() {
        assert!(is_invisible_char('\u{200B}')); // Zero width space
        assert!(is_invisible_char('\u{FEFF}')); // BOM
        assert!(!is_invisible_char('a'));

        assert!(is_bidi_char('\u{202E}')); // Right-to-left override
        assert!(is_bidi_char('\u{200F}')); // Right-to-left mark
        assert!(!is_bidi_char('a'));

        assert!(is_confusable_char('а')); // Cyrillic 'a'
        assert!(is_confusable_char('α')); // Greek alpha
        assert!(!is_confusable_char('a')); // Latin 'a'
    }

    #[test]
    fn test_script_detection() {
        assert_eq!(get_script('a'), Script::Latin);
        assert_eq!(get_script('А'), Script::Cyrillic);
        assert_eq!(get_script('α'), Script::Greek);
        assert_eq!(get_script('世'), Script::Chinese);
    }

    #[test]
    fn test_risk_calculation() {
        // Low risk
        let safe = analyze_text("Hello World");
        assert_eq!(safe.risk_level, RiskLevel::Low);

        // High risk - invisible chars
        let invisible = analyze_text("Hello\u{200B}World");
        assert!(invisible.risk_level >= RiskLevel::High);

        // Critical risk - bidi override
        let bidi = analyze_text("test\u{202E}evil");
        assert_eq!(bidi.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_security_report() {
        let text = "Hello\u{200B}World";
        let report = generate_security_report(text);
        assert!(report.contains("INVISIBLE CHARACTERS DETECTED"));
        assert!(report.contains("U+200B"));
        assert!(report.contains("Zero Width Space"));
    }
}
