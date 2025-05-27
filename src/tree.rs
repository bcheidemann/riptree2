use std::{
    cmp::Ordering,
    fs::DirEntry,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;

use crate::{filter::TreeFilter, sorter::default_sorter};

#[derive(Default)]
pub struct TreeStats {
    dirs: usize,
    files: usize,
}

impl TreeStats {
    #[inline(always)]
    pub fn dirs(&self) -> usize {
        self.dirs
    }

    #[inline(always)]
    pub fn files(&self) -> usize {
        self.files
    }
}

pub struct Tree {
    filter: Arc<TreeFilter>,
    sorter: fn(&DirEntry, &DirEntry) -> Ordering,
    prefix: String,
    root: PathBuf,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            filter: Arc::new(TreeFilter::default()),
            sorter: default_sorter,
            prefix: "".to_string(),
            root: ".".into(),
        }
    }
}

impl Tree {
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn enter_dir(&self, dir: &DirEntry, is_last: bool) -> Self {
        let new_prefix = if is_last { "    " } else { "│   " };
        Tree {
            filter: self.filter.clone(),
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

    pub fn write(&self, w: &mut impl Write, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut entries = std::fs::read_dir(&self.root)
            .context("Failed to read directory")?
            .filter(|entry| {
                entry
                    .as_ref()
                    .map(|entry| self.filter.include(&entry))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| match (a, b) {
            (&Ok(ref a), &Ok(ref b)) => (self.sorter)(a, b),
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

    pub fn print(&self, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        self.write(&mut writer, stats)
    }
}
