use std::sync::Arc;

use anyhow::Context as _;
use clap::Parser as _;
use riptree2::{
    args::TreeArgs,
    options::TreeOptions,
    tree::{Tree, TreeStats},
};

fn main() -> anyhow::Result<()> {
    let args = TreeArgs::parse();

    let roots = if args.roots.is_empty() {
        vec![".".to_string()]
    } else {
        args.roots.clone()
    };

    let opts = Arc::new(TreeOptions::try_from(args).context("Failed to validate options")?);

    let mut stats = TreeStats::new(opts.clone());

    for root in &roots {
        let tree = Tree::new(root.clone().into(), opts.clone())?;
        println!("{root}");
        tree.print(&mut stats)?;
    }

    println!();
    stats.print()?;

    Ok(())
}
