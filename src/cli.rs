use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "uncomment",
    version,
    about = "Remove comments from code files using tree-sitter parsing",
    long_about = "A fast, accurate CLI tool that removes comments from source code files using tree-sitter AST parsing. Automatically preserves important comments like linting directives, documentation, and metadata."
)]
pub struct Cli {
    /// Files or directories to process (supports glob patterns)
    #[arg(help = "Files, directories, or glob patterns to process")]
    pub paths: Vec<String>,

    /// Remove TODO comments (normally preserved)
    #[arg(short = 'r', long, help = "Remove TODO comments (normally preserved)")]
    pub remove_todo: bool,

    /// Remove FIXME comments (normally preserved)
    #[arg(short = 'f', long, help = "Remove FIXME comments (normally preserved)")]
    pub remove_fixme: bool,

    /// Remove documentation comments (normally preserved)
    #[arg(
        short = 'd',
        long,
        help = "Remove documentation comments and docstrings"
    )]
    pub remove_doc: bool,

    /// Additional patterns to preserve (beyond defaults)
    #[arg(
        short = 'i',
        long = "ignore",
        help = "Additional patterns to preserve (can be used multiple times)"
    )]
    pub ignore_patterns: Vec<String>,

    /// Disable automatic preservation of linting directives
    #[arg(
        long = "no-default-ignores",
        help = "Disable built-in preservation patterns (ESLint, Clippy, etc.)"
    )]
    pub no_default_ignores: bool,

    /// Show what would be changed without modifying files
    #[arg(short = 'n', long, help = "Show changes without modifying files")]
    pub dry_run: bool,

    /// Show detailed processing information
    #[arg(short = 'v', long, help = "Show detailed processing information")]
    pub verbose: bool,

    /// Ignore .gitignore rules when finding files
    #[arg(long = "no-gitignore", help = "Process files ignored by .gitignore")]
    pub no_gitignore: bool,

    /// Process files in nested git repositories
    #[arg(
        long = "traverse-git-repos",
        help = "Traverse into other git repositories (useful for monorepos)"
    )]
    pub traverse_git_repos: bool,

    /// Number of parallel threads (0 = number of CPU cores)
    #[arg(
        short = 'j',
        long = "threads",
        help = "Number of parallel threads (0 = auto-detect)",
        default_value = "1"
    )]
    pub threads: usize,
}

impl Cli {
    pub fn processing_options(&self) -> crate::processor::ProcessingOptions {
        crate::processor::ProcessingOptions {
            remove_todo: self.remove_todo,
            remove_fixme: self.remove_fixme,
            remove_doc: self.remove_doc,
            custom_preserve_patterns: self.ignore_patterns.clone(),
            use_default_ignores: !self.no_default_ignores,
            dry_run: self.dry_run,
            respect_gitignore: !self.no_gitignore,
            traverse_git_repos: self.traverse_git_repos,
        }
    }
}
