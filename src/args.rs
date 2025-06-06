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
    /// Enable compatibility mode. Makes riptree behave the same as tree.
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

    // ================================= Roots =================================
    #[arg()]
    pub roots: Vec<String>,
}
