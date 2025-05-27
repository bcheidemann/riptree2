use riptree::tree::{Tree, TreeStats};

fn main() -> anyhow::Result<()> {
    let mut stats = TreeStats::default();
    let tree = Tree::default();

    println!("{}", tree.root().to_string_lossy());
    tree.print(&mut stats)?;
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
