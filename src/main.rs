use clap::Parser as _;
use riptree2::{
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

    for root in &roots {
        let tree = Tree::new(root.clone().into(), TreeOptions::from(&args))?;
        println!("{root}");
        tree.print(&mut stats)?;
    }

    println!();

    match (stats.dirs(), stats.files()) {
        (1, 1) => println!("1 directory, 1 file"),
        (dirs, 1) => println!("{dirs} directories, 1 file"),
        (1, files) => println!("1 directory, {files} files"),
        (dirs, files) => println!("{dirs} directories, {files} files"),
    };

    Ok(())
}
