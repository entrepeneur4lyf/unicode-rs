//! Git-related Unicode characters
//! Characters for git status, diff, blame, and version control

use super::{UnicodeProvider, UnicodeTheme};

/// Git file status indicators
#[derive(Debug, Clone, Copy)]
pub enum GitStatus {
    /// File is modified
    Modified,
    /// File is added/new
    Added,
    /// File is deleted
    Deleted,
    /// File is renamed
    Renamed,
    /// File is copied
    Copied,
    /// File is untracked
    Untracked,
    /// File is staged
    Staged,
    /// File is ignored
    Ignored,
    /// File has conflicts
    Conflicted,
    /// File is unchanged
    Unchanged,
}

impl UnicodeProvider for GitStatus {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (GitStatus::Modified, UnicodeTheme::Minimal) => 'M',
            (GitStatus::Modified, UnicodeTheme::Basic) => '●',
            (GitStatus::Modified, UnicodeTheme::Rich) => '●',
            (GitStatus::Modified, UnicodeTheme::Fancy) => '◐',

            (GitStatus::Added, UnicodeTheme::Minimal) => '+',
            (GitStatus::Added, UnicodeTheme::Basic) => '+',
            (GitStatus::Added, UnicodeTheme::Rich) => '✚',
            (GitStatus::Added, UnicodeTheme::Fancy) => '⊕',

            (GitStatus::Deleted, UnicodeTheme::Minimal) => '-',
            (GitStatus::Deleted, UnicodeTheme::Basic) => '-',
            (GitStatus::Deleted, UnicodeTheme::Rich) => '✖',
            (GitStatus::Deleted, UnicodeTheme::Fancy) => '⊖',

            (GitStatus::Renamed, UnicodeTheme::Minimal) => 'R',
            (GitStatus::Renamed, UnicodeTheme::Basic) => '>',
            (GitStatus::Renamed, UnicodeTheme::Rich) => '➜',
            (GitStatus::Renamed, UnicodeTheme::Fancy) => '⤷',

            (GitStatus::Copied, UnicodeTheme::Minimal) => 'C',
            (GitStatus::Copied, UnicodeTheme::Basic) => '=',
            (GitStatus::Copied, UnicodeTheme::Rich) => '⧉',
            (GitStatus::Copied, UnicodeTheme::Fancy) => '⎘',

            (GitStatus::Untracked, UnicodeTheme::Minimal) => '?',
            (GitStatus::Untracked, UnicodeTheme::Basic) => '?',
            (GitStatus::Untracked, UnicodeTheme::Rich) => '?',
            (GitStatus::Untracked, UnicodeTheme::Fancy) => '❓',

            (GitStatus::Staged, UnicodeTheme::Minimal) => 'S',
            (GitStatus::Staged, UnicodeTheme::Basic) => '*',
            (GitStatus::Staged, UnicodeTheme::Rich) => '✓',
            (GitStatus::Staged, UnicodeTheme::Fancy) => '✅',

            (GitStatus::Ignored, UnicodeTheme::Minimal) => 'I',
            (GitStatus::Ignored, UnicodeTheme::Basic) => '.',
            (GitStatus::Ignored, UnicodeTheme::Rich) => '⊘',
            (GitStatus::Ignored, UnicodeTheme::Fancy) => '🚫',

            (GitStatus::Conflicted, UnicodeTheme::Minimal) => '!',
            (GitStatus::Conflicted, UnicodeTheme::Basic) => '!',
            (GitStatus::Conflicted, UnicodeTheme::Rich) => '⚠',
            (GitStatus::Conflicted, UnicodeTheme::Fancy) => '⚡',

            (GitStatus::Unchanged, _) => ' ',
        }
    }
}

/// Git diff line indicators
#[derive(Debug, Clone, Copy)]
pub enum GitDiff {
    /// Added line
    Added,
    /// Removed line
    Removed,
    /// Modified line
    Modified,
    /// Context line
    Context,
    /// No newline at end of file
    NoNewline,
}

impl UnicodeProvider for GitDiff {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (GitDiff::Added, UnicodeTheme::Minimal) => '+',
            (GitDiff::Added, UnicodeTheme::Basic) => '+',
            (GitDiff::Added, UnicodeTheme::Rich) => '▎',
            (GitDiff::Added, UnicodeTheme::Fancy) => '┃',

            (GitDiff::Removed, UnicodeTheme::Minimal) => '-',
            (GitDiff::Removed, UnicodeTheme::Basic) => '-',
            (GitDiff::Removed, UnicodeTheme::Rich) => '▁',
            (GitDiff::Removed, UnicodeTheme::Fancy) => '━',

            (GitDiff::Modified, UnicodeTheme::Minimal) => '~',
            (GitDiff::Modified, UnicodeTheme::Basic) => '~',
            (GitDiff::Modified, UnicodeTheme::Rich) => '▎',
            (GitDiff::Modified, UnicodeTheme::Fancy) => '┃',

            (GitDiff::Context, _) => ' ',

            (GitDiff::NoNewline, UnicodeTheme::Minimal) => '\\',
            (GitDiff::NoNewline, UnicodeTheme::Basic) => '\\',
            (GitDiff::NoNewline, UnicodeTheme::Rich) => '⏎',
            (GitDiff::NoNewline, UnicodeTheme::Fancy) => '↵',
        }
    }
}

/// Git branch indicators
#[derive(Debug, Clone, Copy)]
pub enum GitBranch {
    /// Current branch
    Current,
    /// Remote branch
    Remote,
    /// Local branch
    Local,
    /// Detached HEAD
    Detached,
    /// Branch ahead
    Ahead,
    /// Branch behind
    Behind,
    /// Branch diverged
    Diverged,
}

impl UnicodeProvider for GitBranch {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (GitBranch::Current, UnicodeTheme::Minimal) => '*',
            (GitBranch::Current, UnicodeTheme::Basic) => '*',
            (GitBranch::Current, UnicodeTheme::Rich) => '●',
            (GitBranch::Current, UnicodeTheme::Fancy) => '🌿',

            (GitBranch::Remote, UnicodeTheme::Minimal) => 'R',
            (GitBranch::Remote, UnicodeTheme::Basic) => '@',
            (GitBranch::Remote, UnicodeTheme::Rich) => '⭐',
            (GitBranch::Remote, UnicodeTheme::Fancy) => '☁',

            (GitBranch::Local, UnicodeTheme::Minimal) => 'L',
            (GitBranch::Local, UnicodeTheme::Basic) => '|',
            (GitBranch::Local, UnicodeTheme::Rich) => '⎇',
            (GitBranch::Local, UnicodeTheme::Fancy) => '🌱',

            (GitBranch::Detached, UnicodeTheme::Minimal) => 'D',
            (GitBranch::Detached, UnicodeTheme::Basic) => '?',
            (GitBranch::Detached, UnicodeTheme::Rich) => '⚠',
            (GitBranch::Detached, UnicodeTheme::Fancy) => '🔗',

            (GitBranch::Ahead, UnicodeTheme::Minimal) => '^',
            (GitBranch::Ahead, UnicodeTheme::Basic) => '^',
            (GitBranch::Ahead, UnicodeTheme::Rich) => '↑',
            (GitBranch::Ahead, UnicodeTheme::Fancy) => '⬆',

            (GitBranch::Behind, UnicodeTheme::Minimal) => 'v',
            (GitBranch::Behind, UnicodeTheme::Basic) => 'v',
            (GitBranch::Behind, UnicodeTheme::Rich) => '↓',
            (GitBranch::Behind, UnicodeTheme::Fancy) => '⬇',

            (GitBranch::Diverged, UnicodeTheme::Minimal) => '<',
            (GitBranch::Diverged, UnicodeTheme::Basic) => '<',
            (GitBranch::Diverged, UnicodeTheme::Rich) => '↕',
            (GitBranch::Diverged, UnicodeTheme::Fancy) => '🔀',
        }
    }
}

/// Git action indicators
#[derive(Debug, Clone, Copy)]
pub enum GitAction {
    /// Stage changes
    Stage,
    /// Unstage changes
    Unstage,
    /// Commit changes
    Commit,
    /// Push changes
    Push,
    /// Pull changes
    Pull,
    /// Merge
    Merge,
    /// Rebase
    Rebase,
    /// Cherry-pick
    CherryPick,
    /// Stash
    Stash,
    /// Tag
    Tag,
}

impl UnicodeProvider for GitAction {
    fn get_char(&self, theme: UnicodeTheme) -> char {
        match (self, theme) {
            (GitAction::Stage, UnicodeTheme::Minimal) => '+',
            (GitAction::Stage, UnicodeTheme::Basic) => '+',
            (GitAction::Stage, UnicodeTheme::Rich) => '⊕',
            (GitAction::Stage, UnicodeTheme::Fancy) => '📥',

            (GitAction::Unstage, UnicodeTheme::Minimal) => '-',
            (GitAction::Unstage, UnicodeTheme::Basic) => '-',
            (GitAction::Unstage, UnicodeTheme::Rich) => '⊖',
            (GitAction::Unstage, UnicodeTheme::Fancy) => '📤',

            (GitAction::Commit, UnicodeTheme::Minimal) => 'C',
            (GitAction::Commit, UnicodeTheme::Basic) => '*',
            (GitAction::Commit, UnicodeTheme::Rich) => '✓',
            (GitAction::Commit, UnicodeTheme::Fancy) => '💾',

            (GitAction::Push, UnicodeTheme::Minimal) => '^',
            (GitAction::Push, UnicodeTheme::Basic) => '^',
            (GitAction::Push, UnicodeTheme::Rich) => '↑',
            (GitAction::Push, UnicodeTheme::Fancy) => '🚀',

            (GitAction::Pull, UnicodeTheme::Minimal) => 'v',
            (GitAction::Pull, UnicodeTheme::Basic) => 'v',
            (GitAction::Pull, UnicodeTheme::Rich) => '↓',
            (GitAction::Pull, UnicodeTheme::Fancy) => '⬇',

            (GitAction::Merge, UnicodeTheme::Minimal) => 'M',
            (GitAction::Merge, UnicodeTheme::Basic) => '&',
            (GitAction::Merge, UnicodeTheme::Rich) => '⚡',
            (GitAction::Merge, UnicodeTheme::Fancy) => '🔀',

            (GitAction::Rebase, UnicodeTheme::Minimal) => 'R',
            (GitAction::Rebase, UnicodeTheme::Basic) => '~',
            (GitAction::Rebase, UnicodeTheme::Rich) => '⤴',
            (GitAction::Rebase, UnicodeTheme::Fancy) => '🔄',

            (GitAction::CherryPick, UnicodeTheme::Minimal) => 'P',
            (GitAction::CherryPick, UnicodeTheme::Basic) => 'o',
            (GitAction::CherryPick, UnicodeTheme::Rich) => '🍒',
            (GitAction::CherryPick, UnicodeTheme::Fancy) => '🍒',

            (GitAction::Stash, UnicodeTheme::Minimal) => 'S',
            (GitAction::Stash, UnicodeTheme::Basic) => '#',
            (GitAction::Stash, UnicodeTheme::Rich) => '📦',
            (GitAction::Stash, UnicodeTheme::Fancy) => '📦',

            (GitAction::Tag, UnicodeTheme::Minimal) => 'T',
            (GitAction::Tag, UnicodeTheme::Basic) => '@',
            (GitAction::Tag, UnicodeTheme::Rich) => '🏷',
            (GitAction::Tag, UnicodeTheme::Fancy) => '🏷',
        }
    }
}

/// Convenience constants for common git characters
pub mod chars {
    use super::*;

    /// Git status characters
    pub const MODIFIED: GitStatus = GitStatus::Modified;
    pub const ADDED: GitStatus = GitStatus::Added;
    pub const DELETED: GitStatus = GitStatus::Deleted;
    pub const RENAMED: GitStatus = GitStatus::Renamed;
    pub const COPIED: GitStatus = GitStatus::Copied;
    pub const UNTRACKED: GitStatus = GitStatus::Untracked;
    pub const STAGED: GitStatus = GitStatus::Staged;
    pub const IGNORED: GitStatus = GitStatus::Ignored;
    pub const CONFLICTED: GitStatus = GitStatus::Conflicted;
    pub const UNCHANGED: GitStatus = GitStatus::Unchanged;

    /// Git diff characters
    pub const DIFF_ADDED: GitDiff = GitDiff::Added;
    pub const DIFF_REMOVED: GitDiff = GitDiff::Removed;
    pub const DIFF_MODIFIED: GitDiff = GitDiff::Modified;
    pub const DIFF_CONTEXT: GitDiff = GitDiff::Context;
    pub const DIFF_NO_NEWLINE: GitDiff = GitDiff::NoNewline;

    /// Git branch characters
    pub const BRANCH_CURRENT: GitBranch = GitBranch::Current;
    pub const BRANCH_REMOTE: GitBranch = GitBranch::Remote;
    pub const BRANCH_LOCAL: GitBranch = GitBranch::Local;
    pub const BRANCH_DETACHED: GitBranch = GitBranch::Detached;
    pub const BRANCH_AHEAD: GitBranch = GitBranch::Ahead;
    pub const BRANCH_BEHIND: GitBranch = GitBranch::Behind;
    pub const BRANCH_DIVERGED: GitBranch = GitBranch::Diverged;

    /// Git action characters
    pub const ACTION_STAGE: GitAction = GitAction::Stage;
    pub const ACTION_UNSTAGE: GitAction = GitAction::Unstage;
    pub const ACTION_COMMIT: GitAction = GitAction::Commit;
    pub const ACTION_PUSH: GitAction = GitAction::Push;
    pub const ACTION_PULL: GitAction = GitAction::Pull;
    pub const ACTION_MERGE: GitAction = GitAction::Merge;
    pub const ACTION_REBASE: GitAction = GitAction::Rebase;
    pub const ACTION_CHERRY_PICK: GitAction = GitAction::CherryPick;
    pub const ACTION_STASH: GitAction = GitAction::Stash;
    pub const ACTION_TAG: GitAction = GitAction::Tag;
}
