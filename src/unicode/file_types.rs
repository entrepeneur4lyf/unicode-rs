//! File type Unicode characters
//! Icons and indicators for different file types and extensions

use super::{UnicodeProvider, UnicodeTheme};

/// File type indicators
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    /// Regular file
    File,
    /// Directory/folder
    Directory,
    /// Executable file
    Executable,
    /// Symbolic link
    SymLink,
    /// Hidden file
    Hidden,
    /// Configuration file
    Config,
    /// Documentation file
    Documentation,
    /// Image file
    Image,
    /// Video file
    Video,
    /// Audio file
    Audio,
    /// Archive file
    Archive,
    /// Database file
    Database,
    /// Log file
    Log,
    /// Temporary file
    Temporary,
    /// Backup file
    Backup,
}

impl UnicodeProvider for FileType {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (FileType::File, UnicodeTheme::Minimal) => '-',
            (FileType::File, UnicodeTheme::Basic) => '•',
            (FileType::File, UnicodeTheme::Rich) => '📄',
            (FileType::File, UnicodeTheme::Fancy) => '📋',

            (FileType::Directory, UnicodeTheme::Minimal) => 'D',
            (FileType::Directory, UnicodeTheme::Basic) => '/',
            (FileType::Directory, UnicodeTheme::Rich) => '📁',
            (FileType::Directory, UnicodeTheme::Fancy) => '🗂',

            (FileType::Executable, UnicodeTheme::Minimal) => 'X',
            (FileType::Executable, UnicodeTheme::Basic) => '*',
            (FileType::Executable, UnicodeTheme::Rich) => '⚡',
            (FileType::Executable, UnicodeTheme::Fancy) => '🔧',

            (FileType::SymLink, UnicodeTheme::Minimal) => 'L',
            (FileType::SymLink, UnicodeTheme::Basic) => '@',
            (FileType::SymLink, UnicodeTheme::Rich) => '🔗',
            (FileType::SymLink, UnicodeTheme::Fancy) => '⛓',

            (FileType::Hidden, UnicodeTheme::Minimal) => '.',
            (FileType::Hidden, UnicodeTheme::Basic) => '.',
            (FileType::Hidden, UnicodeTheme::Rich) => '👁',
            (FileType::Hidden, UnicodeTheme::Fancy) => '🕵',

            (FileType::Config, UnicodeTheme::Minimal) => 'C',
            (FileType::Config, UnicodeTheme::Basic) => '#',
            (FileType::Config, UnicodeTheme::Rich) => '⚙',
            (FileType::Config, UnicodeTheme::Fancy) => '🔧',

            (FileType::Documentation, UnicodeTheme::Minimal) => 'D',
            (FileType::Documentation, UnicodeTheme::Basic) => '?',
            (FileType::Documentation, UnicodeTheme::Rich) => '📖',
            (FileType::Documentation, UnicodeTheme::Fancy) => '📚',

            (FileType::Image, UnicodeTheme::Minimal) => 'I',
            (FileType::Image, UnicodeTheme::Basic) => '%',
            (FileType::Image, UnicodeTheme::Rich) => '🖼',
            (FileType::Image, UnicodeTheme::Fancy) => '🎨',

            (FileType::Video, UnicodeTheme::Minimal) => 'V',
            (FileType::Video, UnicodeTheme::Basic) => '&',
            (FileType::Video, UnicodeTheme::Rich) => '🎬',
            (FileType::Video, UnicodeTheme::Fancy) => '📹',

            (FileType::Audio, UnicodeTheme::Minimal) => 'A',
            (FileType::Audio, UnicodeTheme::Basic) => '~',
            (FileType::Audio, UnicodeTheme::Rich) => '🎵',
            (FileType::Audio, UnicodeTheme::Fancy) => '🎶',

            (FileType::Archive, UnicodeTheme::Minimal) => 'Z',
            (FileType::Archive, UnicodeTheme::Basic) => '=',
            (FileType::Archive, UnicodeTheme::Rich) => '📦',
            (FileType::Archive, UnicodeTheme::Fancy) => '🗜',

            (FileType::Database, UnicodeTheme::Minimal) => 'B',
            (FileType::Database, UnicodeTheme::Basic) => '#',
            (FileType::Database, UnicodeTheme::Rich) => '🗄',
            (FileType::Database, UnicodeTheme::Fancy) => '💾',

            (FileType::Log, UnicodeTheme::Minimal) => 'L',
            (FileType::Log, UnicodeTheme::Basic) => '|',
            (FileType::Log, UnicodeTheme::Rich) => '📜',
            (FileType::Log, UnicodeTheme::Fancy) => '📋',

            (FileType::Temporary, UnicodeTheme::Minimal) => 'T',
            (FileType::Temporary, UnicodeTheme::Basic) => '~',
            (FileType::Temporary, UnicodeTheme::Rich) => '⏳',
            (FileType::Temporary, UnicodeTheme::Fancy) => '🗑',

            (FileType::Backup, UnicodeTheme::Minimal) => 'B',
            (FileType::Backup, UnicodeTheme::Basic) => '+',
            (FileType::Backup, UnicodeTheme::Rich) => '💾',
            (FileType::Backup, UnicodeTheme::Fancy) => '🔄',
        }
    }
}

/// Programming language file types
#[derive(Debug, Clone, Copy)]
pub enum LanguageType {
    /// Rust files
    Rust,
    /// JavaScript/TypeScript files
    JavaScript,
    /// Python files
    Python,
    /// C/C++ files
    C,
    /// Java files
    Java,
    /// Go files
    Go,
    /// HTML files
    Html,
    /// CSS files
    Css,
    /// JSON files
    Json,
    /// XML files
    Xml,
    /// YAML files
    Yaml,
    /// TOML files
    Toml,
    /// Markdown files
    Markdown,
    /// Shell script files
    Shell,
    /// SQL files
    Sql,
    /// Docker files
    Docker,
    /// Git files
    Git,
    /// Unknown/generic code file
    Code,
}

impl UnicodeProvider for LanguageType {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (LanguageType::Rust, UnicodeTheme::Minimal) => 'R',
            (LanguageType::Rust, UnicodeTheme::Basic) => 'R',
            (LanguageType::Rust, UnicodeTheme::Rich) => '🦀',
            (LanguageType::Rust, UnicodeTheme::Fancy) => '⚙',

            (LanguageType::JavaScript, UnicodeTheme::Minimal) => 'J',
            (LanguageType::JavaScript, UnicodeTheme::Basic) => 'J',
            (LanguageType::JavaScript, UnicodeTheme::Rich) => '⚡',
            (LanguageType::JavaScript, UnicodeTheme::Fancy) => '📜',

            (LanguageType::Python, UnicodeTheme::Minimal) => 'P',
            (LanguageType::Python, UnicodeTheme::Basic) => 'P',
            (LanguageType::Python, UnicodeTheme::Rich) => '🐍',
            (LanguageType::Python, UnicodeTheme::Fancy) => '🐍',

            (LanguageType::C, UnicodeTheme::Minimal) => 'C',
            (LanguageType::C, UnicodeTheme::Basic) => 'C',
            (LanguageType::C, UnicodeTheme::Rich) => '⚡',
            (LanguageType::C, UnicodeTheme::Fancy) => '🔧',

            (LanguageType::Java, UnicodeTheme::Minimal) => 'J',
            (LanguageType::Java, UnicodeTheme::Basic) => 'J',
            (LanguageType::Java, UnicodeTheme::Rich) => '☕',
            (LanguageType::Java, UnicodeTheme::Fancy) => '☕',

            (LanguageType::Go, UnicodeTheme::Minimal) => 'G',
            (LanguageType::Go, UnicodeTheme::Basic) => 'G',
            (LanguageType::Go, UnicodeTheme::Rich) => '🐹',
            (LanguageType::Go, UnicodeTheme::Fancy) => '🚀',

            (LanguageType::Html, UnicodeTheme::Minimal) => 'H',
            (LanguageType::Html, UnicodeTheme::Basic) => '<',
            (LanguageType::Html, UnicodeTheme::Rich) => '🌐',
            (LanguageType::Html, UnicodeTheme::Fancy) => '📄',

            (LanguageType::Css, UnicodeTheme::Minimal) => 'S',
            (LanguageType::Css, UnicodeTheme::Basic) => '#',
            (LanguageType::Css, UnicodeTheme::Rich) => '🎨',
            (LanguageType::Css, UnicodeTheme::Fancy) => '✨',

            (LanguageType::Json, UnicodeTheme::Minimal) => '{',
            (LanguageType::Json, UnicodeTheme::Basic) => '{',
            (LanguageType::Json, UnicodeTheme::Rich) => '📋',
            (LanguageType::Json, UnicodeTheme::Fancy) => '🗂',

            (LanguageType::Xml, UnicodeTheme::Minimal) => '<',
            (LanguageType::Xml, UnicodeTheme::Basic) => '<',
            (LanguageType::Xml, UnicodeTheme::Rich) => '📄',
            (LanguageType::Xml, UnicodeTheme::Fancy) => '🗃',

            (LanguageType::Yaml, UnicodeTheme::Minimal) => 'Y',
            (LanguageType::Yaml, UnicodeTheme::Basic) => ':',
            (LanguageType::Yaml, UnicodeTheme::Rich) => '📝',
            (LanguageType::Yaml, UnicodeTheme::Fancy) => '⚙',

            (LanguageType::Toml, UnicodeTheme::Minimal) => 'T',
            (LanguageType::Toml, UnicodeTheme::Basic) => '=',
            (LanguageType::Toml, UnicodeTheme::Rich) => '⚙',
            (LanguageType::Toml, UnicodeTheme::Fancy) => '🔧',

            (LanguageType::Markdown, UnicodeTheme::Minimal) => 'M',
            (LanguageType::Markdown, UnicodeTheme::Basic) => '#',
            (LanguageType::Markdown, UnicodeTheme::Rich) => '📝',
            (LanguageType::Markdown, UnicodeTheme::Fancy) => '📖',

            (LanguageType::Shell, UnicodeTheme::Minimal) => '$',
            (LanguageType::Shell, UnicodeTheme::Basic) => '$',
            (LanguageType::Shell, UnicodeTheme::Rich) => '🐚',
            (LanguageType::Shell, UnicodeTheme::Fancy) => '⚡',

            (LanguageType::Sql, UnicodeTheme::Minimal) => 'Q',
            (LanguageType::Sql, UnicodeTheme::Basic) => 'Q',
            (LanguageType::Sql, UnicodeTheme::Rich) => '🗄',
            (LanguageType::Sql, UnicodeTheme::Fancy) => '💾',

            (LanguageType::Docker, UnicodeTheme::Minimal) => 'D',
            (LanguageType::Docker, UnicodeTheme::Basic) => '□',
            (LanguageType::Docker, UnicodeTheme::Rich) => '🐳',
            (LanguageType::Docker, UnicodeTheme::Fancy) => '📦',

            (LanguageType::Git, UnicodeTheme::Minimal) => 'G',
            (LanguageType::Git, UnicodeTheme::Basic) => '*',
            (LanguageType::Git, UnicodeTheme::Rich) => '🌿',
            (LanguageType::Git, UnicodeTheme::Fancy) => '🔀',

            (LanguageType::Code, UnicodeTheme::Minimal) => 'C',
            (LanguageType::Code, UnicodeTheme::Basic) => '<',
            (LanguageType::Code, UnicodeTheme::Rich) => '💻',
            (LanguageType::Code, UnicodeTheme::Fancy) => '⌨',
        }
    }
}

/// File extension to type mapping
pub fn get_file_type_from_extension(extension: &str) -> LanguageType {
    match extension.to_lowercase().as_str() {
        "rs" => LanguageType::Rust,
        "js" | "jsx" | "ts" | "tsx" | "mjs" => LanguageType::JavaScript,
        "py" | "pyw" | "pyc" | "pyo" | "pyd" => LanguageType::Python,
        "c" | "h" | "cpp" | "cxx" | "cc" | "hpp" | "hxx" => LanguageType::C,
        "java" | "class" | "jar" => LanguageType::Java,
        "go" | "mod" | "sum" => LanguageType::Go,
        "html" | "htm" | "xhtml" => LanguageType::Html,
        "css" | "scss" | "sass" | "less" => LanguageType::Css,
        "json" | "jsonc" => LanguageType::Json,
        "xml" | "xsd" | "xsl" | "xslt" => LanguageType::Xml,
        "yml" | "yaml" => LanguageType::Yaml,
        "toml" => LanguageType::Toml,
        "md" | "markdown" | "mdown" | "mkd" | "mkdn" => LanguageType::Markdown,
        "sh" | "bash" | "zsh" | "fish" | "csh" | "tcsh" => LanguageType::Shell,
        "sql" | "mysql" | "pgsql" | "sqlite" => LanguageType::Sql,
        "dockerfile" | "containerfile" => LanguageType::Docker,
        "gitignore" | "gitattributes" | "gitmodules" => LanguageType::Git,
        _ => LanguageType::Code,
    }
}

/// Get file type from filename
pub fn get_file_type_from_filename(filename: &str) -> FileType {
    if filename.starts_with('.') {
        FileType::Hidden
    } else if filename.ends_with(".tmp") || filename.ends_with(".temp") {
        FileType::Temporary
    } else if filename.ends_with(".bak") || filename.ends_with(".backup") {
        FileType::Backup
    } else if filename.ends_with(".log") {
        FileType::Log
    } else if matches!(
        filename.to_lowercase().as_str(),
        "config" | "configuration" | "settings" | "preferences"
    ) {
        FileType::Config
    } else if matches!(
        filename.to_lowercase().as_str(),
        "readme" | "readme.md" | "readme.txt" | "doc" | "docs"
    ) {
        FileType::Documentation
    } else {
        FileType::File
    }
}

/// Convenience constants for file types
pub mod chars {
    use super::*;

    // File types
    pub const FILE: FileType = FileType::File;
    pub const DIRECTORY: FileType = FileType::Directory;
    pub const EXECUTABLE: FileType = FileType::Executable;
    pub const SYMLINK: FileType = FileType::SymLink;
    pub const HIDDEN: FileType = FileType::Hidden;
    pub const CONFIG: FileType = FileType::Config;
    pub const DOCUMENTATION: FileType = FileType::Documentation;
    pub const IMAGE: FileType = FileType::Image;
    pub const VIDEO: FileType = FileType::Video;
    pub const AUDIO: FileType = FileType::Audio;
    pub const ARCHIVE: FileType = FileType::Archive;
    pub const DATABASE: FileType = FileType::Database;
    pub const LOG: FileType = FileType::Log;
    pub const TEMPORARY: FileType = FileType::Temporary;
    pub const BACKUP: FileType = FileType::Backup;

    // Language types
    pub const RUST: LanguageType = LanguageType::Rust;
    pub const JAVASCRIPT: LanguageType = LanguageType::JavaScript;
    pub const PYTHON: LanguageType = LanguageType::Python;
    pub const C: LanguageType = LanguageType::C;
    pub const JAVA: LanguageType = LanguageType::Java;
    pub const GO: LanguageType = LanguageType::Go;
    pub const HTML: LanguageType = LanguageType::Html;
    pub const CSS: LanguageType = LanguageType::Css;
    pub const JSON: LanguageType = LanguageType::Json;
    pub const XML: LanguageType = LanguageType::Xml;
    pub const YAML: LanguageType = LanguageType::Yaml;
    pub const TOML: LanguageType = LanguageType::Toml;
    pub const MARKDOWN: LanguageType = LanguageType::Markdown;
    pub const SHELL: LanguageType = LanguageType::Shell;
    pub const SQL: LanguageType = LanguageType::Sql;
    pub const DOCKER: LanguageType = LanguageType::Docker;
    pub const GIT: LanguageType = LanguageType::Git;
    pub const CODE: LanguageType = LanguageType::Code;
}
