//! File browser example
//!
//! This example demonstrates how to use unicode-rs to create a file browser
//! with file type indicators, similar to what you might see in a terminal file manager.

use unicode_rs::prelude::*;

#[derive(Debug)]
struct FileEntry {
    name: String,
    is_directory: bool,
    extension: Option<String>,
}

impl FileEntry {
    fn new(name: &str, is_directory: bool) -> Self {
        let extension = if is_directory {
            None
        } else {
            std::path::Path::new(name)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string())
        };

        Self {
            name: name.to_string(),
            is_directory,
            extension,
        }
    }
}

fn main() {
    println!("File Browser Example");
    println!("===================\n");

    // Sample file entries
    let files = vec![
        FileEntry::new("src", true),
        FileEntry::new("target", true),
        FileEntry::new("examples", true),
        FileEntry::new("main.rs", false),
        FileEntry::new("lib.rs", false),
        FileEntry::new("Cargo.toml", false),
        FileEntry::new("README.md", false),
        FileEntry::new("package.json", false),
        FileEntry::new("index.html", false),
        FileEntry::new("style.css", false),
        FileEntry::new("script.js", false),
        FileEntry::new("image.png", false),
        FileEntry::new("document.pdf", false),
        FileEntry::new("archive.zip", false),
        FileEntry::new("config.yaml", false),
        FileEntry::new("data.json", false),
        FileEntry::new("test.py", false),
        FileEntry::new("app.go", false),
        FileEntry::new("component.tsx", false),
        FileEntry::new("unknown_file", false),
    ];

    // Display with different themes
    let themes = vec![
        ("Minimal (ASCII)", UnicodeTheme::Minimal),
        ("Rich Unicode", UnicodeTheme::Rich),
    ];

    for (theme_name, theme) in themes {
        println!("{} theme:", theme_name);
        println!("{}:", "─".repeat(theme_name.len() + 7));

        for file in &files {
            let icon = if file.is_directory {
                FileType::Directory.get_char(theme)
            } else if let Some(ext) = &file.extension {
                let ft = get_file_type_from_extension(ext);
                ft.get_char(theme)
            } else {
                let ft = get_file_type_from_filename(&file.name);
                ft.get_char(theme)
            };

            let file_type_name = if file.is_directory {
                "Directory".to_string()
            } else if let Some(ext) = &file.extension {
                let ft = get_file_type_from_extension(ext);
                format!("{:?}", ft)
            } else {
                let ft = get_file_type_from_filename(&file.name);
                format!("{:?}", ft)
            };

            println!("  {} {} ({})", icon, file.name, file_type_name);
        }
        println!();
    }

    // Demonstrate language-specific file types
    println!("Language-specific file types:");
    println!("============================");

    let theme = UnicodeTheme::Rich;
    let languages = vec![
        ("Rust", LanguageType::Rust),
        ("JavaScript/TypeScript", LanguageType::JavaScript),
        ("Python", LanguageType::Python),
        ("Go", LanguageType::Go),
        ("HTML", LanguageType::Html),
        ("CSS", LanguageType::Css),
        ("JSON", LanguageType::Json),
        ("YAML", LanguageType::Yaml),
        ("Markdown", LanguageType::Markdown),
        ("Shell", LanguageType::Shell),
    ];

    for (name, lang_type) in languages {
        println!("  {} {}", lang_type.get_char(theme), name);
    }
    println!();

    // Create a more realistic file browser display
    println!("Realistic File Browser Display:");
    println!("==============================");

    set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Rich));

    println!("📁 /home/user/project");
    println!("├── {} src/", get_char(&FileType::Directory, None));
    println!("│   ├── {} main.rs", get_char(&LanguageType::Rust, None));
    println!("│   ├── {} lib.rs", get_char(&LanguageType::Rust, None));
    println!("│   └── {} mod.rs", get_char(&LanguageType::Rust, None));
    println!("├── {} examples/", get_char(&FileType::Directory, None));
    println!("│   └── {} basic.rs", get_char(&LanguageType::Rust, None));
    println!("├── {} target/", get_char(&FileType::Directory, None));
    println!("├── {} Cargo.toml", get_char(&FileType::Config, None));
    println!("├── {} README.md", get_char(&LanguageType::Markdown, None));
    println!("├── {} package.json", get_char(&LanguageType::Json, None));
    println!("└── {} .gitignore", get_char(&FileType::Config, None));

    println!(
        "\nFile count: {} directories, {} files",
        files.iter().filter(|f| f.is_directory).count(),
        files.iter().filter(|f| !f.is_directory).count()
    );
}
