use std::{
    cmp::Ordering,
    fs::read_link,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;

use crate::{entry::Entry, filter::TreeFilter, options::TreeOptions};

pub struct TreeStats {
    options: Arc<TreeOptions>,
    dirs: usize,
    files: usize,
}

impl TreeStats {
    pub fn new(options: Arc<TreeOptions>) -> Self {
        Self {
            options,
            dirs: 0,
            files: 0,
        }
    }

    #[inline(always)]
    pub fn dirs(&self) -> usize {
        self.dirs
    }

    #[inline(always)]
    pub fn files(&self) -> usize {
        self.files
    }

    pub fn write(&self, w: &mut impl Write) -> anyhow::Result<()> {
        if self.options.list_directories_only {
            match self.dirs() {
                1 => writeln!(w, "1 directory, 1 file"),
                dirs => writeln!(w, "{dirs} directories"),
            }?;

            return Ok(());
        }

        match (self.dirs(), self.files()) {
            (1, 1) => writeln!(w, "1 directory, 1 file"),
            (dirs, 1) => writeln!(w, "{dirs} directories, 1 file"),
            (1, files) => writeln!(w, "1 directory, {files} files"),
            (dirs, files) => writeln!(w, "{dirs} directories, {files} files"),
        }?;

        Ok(())
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let mut writer = std::io::stdout();
        self.write(&mut writer)
    }
}

pub struct Tree<'tree> {
    filter: TreeFilter<'tree>,
    options: Arc<TreeOptions>,
    depth: usize,
    prefix: String,
    path_prefix: Option<PathBuf>,
    root: PathBuf,
}

impl<'tree> Tree<'tree> {
    pub fn new(root: PathBuf, options: Arc<TreeOptions>) -> anyhow::Result<Self> {
        let path_prefix = if options.print_full_path_prefix {
            Some(root.clone())
        } else {
            None
        };

        Ok(Self {
            filter: TreeFilter::new(&root, &options)?,
            options,
            depth: 0,
            prefix: "".to_string(),
            path_prefix,
            root,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn enter_dir(&'tree self, dir: Entry, is_last: bool) -> anyhow::Result<Self> {
        let new_prefix = if is_last { "    " } else { "│   " };
        Ok(Tree {
            filter: self
                .filter
                .enter_dir(&dir, &self.options)
                .with_context(|| format!("Failed to enter {}", dir.path().to_string_lossy()))?,
            options: self.options.clone(),
            depth: self.depth + 1,
            prefix: format!("{}{}", self.prefix, new_prefix),
            path_prefix: self
                .path_prefix
                .as_ref()
                .map(|path_prefix| path_prefix.join(dir.file_name())),
            root: dir.into_path(),
        })
    }

    #[inline]
    fn write_entry(
        &self,
        w: &mut impl Write,
        entry: Entry,
        is_last: bool,
        stats: &mut TreeStats,
    ) -> anyhow::Result<()> {
        let file_name = match self.path_prefix.as_ref() {
            Some(path_prefix) => path_prefix
                .join(entry.file_name())
                .to_string_lossy()
                .to_string(),
            None => entry.file_name().to_string_lossy().to_string(),
        };
        let link_target = if entry.file_type().is_symlink() {
            let target = read_link(entry.path()).context("Failed to read link")?;
            format!(" -> {}", target.to_string_lossy())
        } else {
            "".to_string()
        };
        let result = if is_last {
            writeln!(w, "{}└── {file_name}{link_target}", self.prefix)
        } else {
            writeln!(w, "{}├── {file_name}{link_target}", self.prefix)
        };
        result.context("Failed to write entry")?;

        if entry.file_type().is_dir() {
            self.enter_dir(entry, is_last)?.write(w, stats)?;
        } else {
            stats.files += 1;
        }

        Ok(())
    }

    pub fn write(&self, w: &mut impl Write, stats: &mut TreeStats) -> anyhow::Result<()> {
        let mut entries = std::fs::read_dir(&self.root)
            .context("Failed to read directory")?
            .map(|entry| -> anyhow::Result<Entry> { Entry::new(entry?) })
            .filter(|entry| {
                entry
                    .as_ref()
                    .map(|entry| self.filter.include(entry, &self.options))
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        // Don't ask... for some reason tree counts the root dir, but only if it
        // is not empty.
        if self.depth > 0 || !entries.is_empty() {
            stats.dirs += 1;
        }

        entries.sort_by(|a, b| match (a, b) {
            (Ok(a), Ok(b)) => (self.options.sorter)(a, b),
            (Err(_), Err(_)) => Ordering::Equal,
            (Ok(_), Err(_)) => Ordering::Greater,
            (Err(_), Ok(_)) => Ordering::Less,
        });

        if let (Some(last_entry), leading_entries) = (entries.pop(), entries) {
            for entry in leading_entries.into_iter() {
                self.write_entry(w, entry?, false, stats)?;
            }
            self.write_entry(w, last_entry?, true, stats)?;
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
