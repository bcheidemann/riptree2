use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TreeArgs {
    // ============================ Listing options ============================
    #[arg(short = 'a')]
    /// All files are listed.
    pub show_hidden_files: bool,
    #[arg(short = 'd')]
    /// List directories only.
    pub list_directories_only: bool,
    // TODO: -l
    #[arg(short = 'f')]
    /// Print the full path prefix for each file.
    pub print_full_path_prefix: bool,
    // TODO: -x
    #[arg(short = 'L')]
    /// Descend only level directories deep.
    pub max_level: Option<usize>,
    // TODO: -R
    #[arg(short = 'P')]
    /// List only those files that match the pattern given.
    pub file_include_patterns: Vec<String>,
    #[arg(short = 'I')]
    /// Do not list files that match the given pattern.
    pub file_exclude_patterns: Vec<String>,
    #[arg(long = "ignore-case")]
    /// Ignore case when pattern matching.
    pub ignore_case: bool,
    // TODO: --matchdirs

    // ============================= File options ==============================
    // TODO

    // ============================ Sorting options ============================
    // TODO

    // =========================== Graphics options ============================
    // TODO

    // ========================= XML/HTML/JSON options =========================
    // TODO

    // ============================= Input options =============================
    // TODO

    // ============================ Riptree options ============================
    #[arg(long)]
    /// Enable compatibility mode. Makes riptree2 behave the same as tree.
    pub compat: bool,
    #[arg(long, conflicts_with = "compat")]
    /// Filter rules from .gitignore files are not respected.
    ///
    /// Incompatible with the --compat option.
    pub no_gitignore: bool,
    #[arg(long, requires = "compat")]
    /// Filter rules from .gitignore files are respected (default).
    ///
    /// Requires the --compat option.
    pub gitignore: bool,
    #[arg(long, requires = "compat")]
    /// Show Nerd Fonts icons.
    ///
    /// Requires the --compat option.
    pub icons: bool,
    #[arg(long, conflicts_with = "compat")]
    /// Hide Nerd Fonts icons.
    ///
    /// Incompatible with the --compat option.
    pub no_icons: bool,

    // ================================= Roots =================================
    #[arg()]
    pub roots: Vec<String>,
}
