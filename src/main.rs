use std::{io::Write, sync::Arc};

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

    let output_to_file = args.output_to_file.clone();

    let roots = if args.roots.is_empty() {
        vec![".".to_string()]
    } else {
        args.roots.clone()
    };

    let no_report = args.no_report;

    let opts = Arc::new(TreeOptions::try_from(args).context("Failed to validate options")?);

    if let Some(file) = output_to_file {
        let mut writer = std::fs::File::create(file).unwrap();
        print(&mut writer, !no_report, &roots, opts)
    } else {
        let mut writer = std::io::stdout();
        print(&mut writer, !no_report, &roots, opts)
    }
}

#[inline]
fn print(
    writer: &mut impl Write,
    print_report: bool,
    roots: &Vec<String>,
    opts: Arc<TreeOptions>,
) -> anyhow::Result<()> {
    if print_report {
        let mut stats = DefaultTreeStats::new(opts.clone());
        print_tree(writer, roots, opts.clone(), &mut stats)?;
        writeln!(writer)?;
        stats.write(writer)?;
    } else {
        print_tree(writer, roots, opts.clone(), &mut NoopTreeStats)?;
    }

    Ok(())
}

#[inline]
fn print_tree(
    writer: &mut impl Write,
    roots: &Vec<String>,
    opts: Arc<TreeOptions>,
    stats: &mut impl TreeStats,
) -> anyhow::Result<()> {
    for root in roots {
        let tree = Tree::new(root.clone().into(), opts.clone())?;
        writeln!(writer, "{root}")?;
        tree.write(writer, stats)?;
    }

    Ok(())
}
