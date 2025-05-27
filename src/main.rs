use riptree::tree::{Tree, TreeStats};

fn main() -> anyhow::Result<()> {
    let mut stats = TreeStats::default();
    let tree = Tree::default();

    println!("{}", tree.root().to_string_lossy());
    tree.print(&mut stats)?;
    println!("");
    println!("{} directories, {} files", stats.dirs() + 1, stats.files());

    Ok(())
}
