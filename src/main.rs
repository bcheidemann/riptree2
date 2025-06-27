use std::sync::Arc;

use anyhow::Context as _;
use clap::Parser as _;
use riptree2::{
    args::TreeArgs,
    options::TreeOptions,
    stats::{DefaultTreeStats, NoopTreeStats, TreeStats},
    tree::Tree,
};

fn main() -> anyhow::Result<()> {
    let args = TreeArgs::parse();

    let roots = if args.roots.is_empty() {
        vec![".".to_string()]
    } else {
        args.roots.clone()
    };

    let no_report = args.no_report;

    let opts = Arc::new(TreeOptions::try_from(args).context("Failed to validate options")?);

    if no_report {
        print_tree(&roots, opts.clone(), &mut NoopTreeStats)?;
    } else {
        let mut stats = DefaultTreeStats::new(opts.clone());
        print_tree(&roots, opts.clone(), &mut stats)?;
        println!();
        stats.print()?;
    }

    Ok(())
}

#[inline]
fn print_tree(
    roots: &Vec<String>,
    opts: Arc<TreeOptions>,
    stats: &mut impl TreeStats,
) -> anyhow::Result<()> {
    for root in roots {
        let tree = Tree::new(root.clone().into(), opts.clone())?;
        println!("{root}");
        tree.print(stats)?;
    }

    Ok(())
}
