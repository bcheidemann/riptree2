use riptree::tree::{Tree, TreeStats};

fn main() -> anyhow::Result<()> {
    let mut stats = TreeStats::default();
    let tree = Tree::default();

    println!("{}", tree.root().to_string_lossy());
    tree.print(&mut stats)?;
    println!("");

    let files = stats.files();
    let dirs = match (stats.dirs(), files) {
        (0, 0) => 0,
        (dirs, _) => dirs + 1,
    };

    println!("{dirs} directories, {files} files");

    Ok(())
}
