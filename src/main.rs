use clap::Parser as _;
use riptree::{
    args::TreeArgs,
    options::TreeOptions,
    tree::{Tree, TreeStats},
};

fn main() -> anyhow::Result<()> {
    let args = TreeArgs::parse();

    let mut stats = TreeStats::default();

    let roots = if args.roots.is_empty() {
        vec![".".to_string()]
    } else {
        args.roots.clone()
    };

    for root in roots {
        let tree = Tree::new(TreeOptions::from(&args), root.clone().into());
        println!("{root}");
        tree.print(&mut stats)?;
    }

    println!("");

    match (stats.dirs(), stats.files()) {
        (0, 0) => println!("0 directories, 0 files"),
        (0, 1) => println!("1 directory, 1 file"),
        (dirs, 1) => println!("{} directories, 1 file", dirs + 1),
        (0, files) => println!("1 directory, {files} files"),
        (dirs, files) => println!("{} directories, {files} files", dirs + 1),
    };

    Ok(())
}
