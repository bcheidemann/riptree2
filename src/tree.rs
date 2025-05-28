use std::{
    cmp::Ordering,
    fs::DirEntry,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;

use crate::{filter::TreeFilter, options::TreeOptions};

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
    filter: TreeFilter,
    options: Arc<TreeOptions>,
    depth: usize,
    prefix: String,
    root: PathBuf,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            filter: TreeFilter::default(),
            options: Arc::new(TreeOptions::default()),
            depth: 0,
            prefix: "".to_string(),
            root: ".".into(),
        }
    }
}

impl Tree {
    pub fn new(options: TreeOptions, root: PathBuf) -> Self {
        Self {
            filter: TreeFilter::default(),
            options: options.into(),
            depth: 0,
            prefix: "".to_string(),
            root,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn enter_dir(&self, dir: &DirEntry, is_last: bool) -> Self {
        let new_prefix = if is_last { "    " } else { "│   " };
        Tree {
            filter: self.filter.enter_dir(dir, &self.options),
            options: self.options.clone(),
            depth: self.depth + 1,
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
                    .map(|entry| self.filter.include(&entry, &self.options))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        // Don't ask... for some reason tree counts the root dir, but only if it
        // is not empty.
        if self.depth > 0 || entries.len() > 0 {
            stats.dirs += 1;
        }

        entries.sort_by(|a, b| match (a, b) {
            (&Ok(ref a), &Ok(ref b)) => (self.options.sorter)(a, b),
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

    pub fn write_root(&self, w: &mut impl Write) -> anyhow::Result<()> {
        writeln!(w, "{}", self.root.to_string_lossy())?;
        Ok(())
    }

    pub fn print(&self, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        self.write(&mut writer, stats)
    }

    pub fn print_root(&self) -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        self.write_root(&mut writer)
    }
}
