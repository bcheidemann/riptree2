use std::path::Path;

use crate::{entry::Entry, ignore::IgnoreDir, options::TreeOptions};

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
        dir: &Entry,
        _options: &TreeOptions,
    ) -> anyhow::Result<Self> {
        if let Some(ignore_dir) = &self.ignore_dir {
            Ok(Self {
                ignore_dir: Some(ignore_dir.enter_dir(dir.path())?),
            })
        } else {
            Ok(Self { ignore_dir: None })
        }
    }

    pub(crate) fn include(&self, entry: &Entry, options: &TreeOptions) -> bool {
        if !options.show_hidden_files && entry.is_hidden() {
            return false;
        }

        if entry.file_type().is_file() {
            if options.list_directories_only {
                return false;
            }

            if let Some(file_include_globset) = options.file_include_globset.as_ref() {
                if !file_include_globset.is_match(entry.file_name()) {
                    return false;
                }
            }

            if let Some(file_exclude_globset) = options.file_exclude_globset.as_ref() {
                if file_exclude_globset.is_match(entry.file_name()) {
                    return false;
                }
            }
        }

        if !self
            .ignore_dir
            .as_ref()
            .map(|ignore_dir| ignore_dir.include(entry.path(), entry.file_type().is_dir()))
            .unwrap_or(true)
        {
            return false;
        }

        true
    }
}
