use std::{fs::DirEntry, path::Path};

use crate::{ignore::IgnoreDir, options::TreeOptions};

pub struct TreeFilter<'filter> {
    ignore_dir: Option<IgnoreDir<'filter>>,
}

impl<'filter> TreeFilter<'filter> {
    pub(crate) fn new(dir: &Path, options: &TreeOptions) -> anyhow::Result<Self> {
        Ok(Self {
            ignore_dir: if options.respect_gitignore {
                Some(IgnoreDir::new(dir)?)
            } else {
                None
            },
        })
    }

    pub(crate) fn enter_dir(
        &'filter self,
        dir: &DirEntry,
        _options: &TreeOptions,
    ) -> anyhow::Result<Self> {
        if let Some(ignore_dir) = &self.ignore_dir {
            Ok(Self {
                ignore_dir: Some(ignore_dir.enter_dir(&dir.path())?),
            })
        } else {
            Ok(Self { ignore_dir: None })
        }
    }

    pub(crate) fn include(&self, entry: &DirEntry, options: &TreeOptions) -> bool {
        if !options.show_hidden_files {
            if let Some(char) = entry.file_name().to_string_lossy().chars().next() {
                if char == '.' {
                    return false;
                }
            }
        }

        if !self
            .ignore_dir
            .as_ref()
            .map(|ignore_dir| {
                ignore_dir.include(&entry.path(), entry.file_type().unwrap().is_dir())
            })
            .unwrap_or(true)
        {
            return false;
        }

        true
    }
}
