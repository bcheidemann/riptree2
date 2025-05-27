use std::{cmp::Ordering, fs::DirEntry, io::Write, path::PathBuf};

use anyhow::Context as _;

#[derive(Default)]
struct TreeStats {
    dirs: usize,
    files: usize,
}

struct Tree {
    sorter: fn(&DirEntry, &DirEntry) -> Ordering,
    prefix: String,
    root: PathBuf,
}

impl Tree {
    fn enter_dir(&self, dir: &DirEntry, is_last: bool) -> Self {
        let new_prefix = if is_last { "    " } else { "│   " };
        Tree {
            sorter: self.sorter,
            prefix: format!("{}{}", self.prefix, new_prefix),
            root: dir.path(),
        }
    }

    #[inline]
    fn write_entry(
        &self,
        w: &mut impl Write,
        entry: &DirEntry,
        is_last: bool,
        stats: &mut TreeStats,
    ) -> anyhow::Result<()> {
        let result = if is_last {
            writeln!(
                w,
                "{}└── {}",
                self.prefix,
                entry.file_name().to_string_lossy(),
            )
        } else {
            writeln!(
                w,
                "{}├── {}",
                self.prefix,
                entry.file_name().to_string_lossy(),
            )
        };
        result.context("Failed to write entry")?;

        if entry.file_type().unwrap().is_dir() {
            stats.dirs += 1;
            self.enter_dir(entry, is_last).write(w, stats)?;
        } else {
            stats.files += 1;
        }

        Ok(())
    }

    fn write(&self, w: &mut impl Write, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut entries = std::fs::read_dir(&self.root)
            .context("Failed to read directory")?
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| match (a, b) {
            (&Ok(ref a), &Ok(ref b)) => default_sorter(a, b),
            (&Err(_), &Err(_)) => Ordering::Equal,
            (&Ok(_), &Err(_)) => Ordering::Greater,
            (&Err(_), &Ok(_)) => Ordering::Less,
        });

        if let Some((last_entry, leading_entries)) = entries.split_last() {
            for entry in leading_entries.iter() {
                let entry = entry.as_ref().unwrap();
                self.write_entry(w, entry, false, stats)?;
            }
            self.write_entry(w, &last_entry.as_ref().unwrap(), true, stats)?;
        }

        Ok(())
    }

    fn print(&self, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        self.write(&mut writer, stats)
    }
}

fn main() -> anyhow::Result<()> {
    let root = ".";

    let mut stats = TreeStats::default();
    let tree = Tree {
        sorter: default_sorter,
        prefix: "".to_string(),
        root: root.into(),
    };

    println!("{root}");
    tree.print(&mut stats)?;
    println!("");
    println!("{} directories, {} files", stats.dirs + 1, stats.files);

    Ok(())
}

fn default_sorter(a: &DirEntry, b: &DirEntry) -> Ordering {
    return a.file_name().cmp(&b.file_name());

    // Folders last
    // match (
    //     a.file_type().unwrap().is_dir(),
    //     b.file_type().unwrap().is_dir(),
    // ) {
    //     (true, false) => Ordering::Greater,
    //     (false, true) => Ordering::Less,
    //     _ => a.file_name().cmp(&b.file_name()),
    // }
}
