use clap::Parser;

#[derive(Parser, Debug)]
pub struct TreeArgs {
    // ============================ Listing options ============================
    #[arg(short = 'a')]
    /// All files are listed.
    pub show_hidden_files: bool,

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

    // ================================= Roots =================================
    #[arg()]
    pub roots: Vec<String>,
}
