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
    stats.print()?;

    Ok(())
}
