use std::{io::Write, sync::Arc};

use crate::options::TreeOptions;

pub trait TreeStats {
    fn count_dir(&mut self);
    fn count_file(&mut self);
}

pub struct DefaultTreeStats {
    options: Arc<TreeOptions>,
    dirs: usize,
    files: usize,
}

impl TreeStats for DefaultTreeStats {
    #[inline(always)]
    fn count_dir(&mut self) {
        self.dirs += 1;
    }

    #[inline(always)]
    fn count_file(&mut self) {
        self.files += 1;
    }
}

impl DefaultTreeStats {
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

pub struct NoopTreeStats;

impl TreeStats for NoopTreeStats {
    fn count_dir(&mut self) {}

    fn count_file(&mut self) {}
}
