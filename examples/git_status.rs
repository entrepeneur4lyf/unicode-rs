//! Git status display example
//!
//! This example shows how to use unicode-rs to create a git status display
//! similar to what you might see in a terminal git client.

use unicode_rs::prelude::*;

#[derive(Debug)]
struct FileStatus {
    path: String,
    status: GitStatus,
}

fn main() {
    println!("Git Status Display Example");
    println!("==========================\n");

    // Sample file statuses
    let files = vec![
        FileStatus {
            path: "src/main.rs".to_string(),
            status: GitStatus::Modified,
        },
        FileStatus {
            path: "README.md".to_string(),
            status: GitStatus::Added,
        },
        FileStatus {
            path: "old_file.txt".to_string(),
            status: GitStatus::Deleted,
        },
        FileStatus {
            path: "new_feature.rs".to_string(),
            status: GitStatus::Untracked,
        },
        FileStatus {
            path: "Cargo.toml".to_string(),
            status: GitStatus::Modified,
        },
    ];

    // Display with different themes
    let themes = vec![
        ("Minimal (ASCII)", UnicodeTheme::Minimal),
        ("Basic Unicode", UnicodeTheme::Basic),
        ("Rich Unicode", UnicodeTheme::Rich),
        ("Fancy Unicode", UnicodeTheme::Fancy),
    ];

    for (theme_name, theme) in themes {
        println!("{} theme:", theme_name);
        println!("{}:", "─".repeat(theme_name.len() + 7));

        for file in &files {
            let status_char = file.status.get_char(theme);
            let status_name = format!("{:?}", file.status);
            println!("  {} {} ({})", status_char, file.path, status_name);
        }
        println!();
    }

    // Demonstrate branch and diff symbols
    println!("Branch and Diff Symbols:");
    println!("========================");

    let theme = UnicodeTheme::Rich;

    println!("Branch symbols:");
    println!("  Current: {}", GitBranch::Current.get_char(theme));
    println!("  Remote:  {}", GitBranch::Remote.get_char(theme));
    println!("  Local:   {}", GitBranch::Local.get_char(theme));
    println!();

    println!("Diff symbols:");
    println!("  Added:   {}", GitDiff::Added.get_char(theme));
    println!("  Removed: {}", GitDiff::Removed.get_char(theme));
    println!("  Modified: {}", GitDiff::Modified.get_char(theme));
    println!();

    // Create a more realistic git status display
    println!("Realistic Git Status Display:");
    println!("============================");

    set_global_config(UnicodeConfig::with_theme(UnicodeTheme::Rich));

    println!("On branch {} main", get_char(&GitBranch::Current, None));
    println!("Your branch is up to date with 'origin/main'.\n");

    println!("Changes to be committed:");
    println!("  (use \"git reset HEAD <file>...\" to unstage)\n");
    for file in files
        .iter()
        .filter(|f| matches!(f.status, GitStatus::Added))
    {
        println!(
            "        {} {}",
            get_char(&GitStatus::Added, None),
            file.path
        );
    }

    println!("\nChanges not staged for commit:");
    println!("  (use \"git add <file>...\" to update what will be committed)");
    println!("  (use \"git checkout -- <file>...\" to discard changes in working directory)\n");
    for file in files
        .iter()
        .filter(|f| matches!(f.status, GitStatus::Modified | GitStatus::Deleted))
    {
        println!("        {} {}", get_char(&file.status, None), file.path);
    }

    println!("\nUntracked files:");
    println!("  (use \"git add <file>...\" to include in what will be committed)\n");
    for file in files
        .iter()
        .filter(|f| matches!(f.status, GitStatus::Untracked))
    {
        println!("        {} {}", get_char(&file.status, None), file.path);
    }
}
