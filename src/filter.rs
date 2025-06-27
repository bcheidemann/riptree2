use std::{ffi::OsStr, path::Path};

use crate::{entry::Entry, ignore::IgnoreDir, options::TreeOptions};

pub struct FilteredEntry {
    pub filter_state: FilterState,
    pub entry: Entry,
}

impl AsRef<Entry> for FilteredEntry {
    fn as_ref(&self) -> &Entry {
        &self.entry
    }
}

#[derive(Default)]
pub struct FilterState {
    skip_include_matchers: bool,
}

pub struct TreeFilter<'filter> {
    state: FilterState,
    ignore_dir: Option<IgnoreDir<'filter>>,
}

impl<'filter> TreeFilter<'filter> {
    pub(crate) fn new(dir: &Path, options: &TreeOptions) -> anyhow::Result<Self> {
        Ok(Self {
            state: FilterState::default(),
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
        state: FilterState,
    ) -> anyhow::Result<Self> {
        if let Some(ignore_dir) = &self.ignore_dir {
            Ok(Self {
                state,
                ignore_dir: Some(ignore_dir.enter_dir(dir.path())?),
            })
        } else {
            Ok(Self {
                state,
                ignore_dir: None,
            })
        }
    }

    #[inline]
    fn file_name_included_by_pattern(&self, file_name: &OsStr, options: &TreeOptions) -> bool {
        if let Some(file_include_globset) = options.file_include_globset.as_ref() {
            if !file_include_globset.is_match(file_name) {
                return false;
            }
        }

        true
    }

    #[inline]
    fn file_name_excluded_by_pattern(&self, file_name: &OsStr, options: &TreeOptions) -> bool {
        if let Some(file_exclude_globset) = options.file_exclude_globset.as_ref() {
            if file_exclude_globset.is_match(file_name) {
                return true;
            }
        }

        false
    }

    pub(crate) fn filter(&self, entry: Entry, options: &TreeOptions) -> Option<FilteredEntry> {
        if !options.show_hidden_files && entry.is_hidden() {
            return None;
        }

        let mut filter_state = FilterState::default();

        if entry.file_type().is_file() {
            if options.list_directories_only {
                return None;
            }

            if !self.state.skip_include_matchers
                && !self.file_name_included_by_pattern(entry.file_name(), options)
            {
                return None;
            }

            if self.file_name_excluded_by_pattern(entry.file_name(), options) {
                return None;
            }
        } else if entry.file_type().is_dir() {
            if options.compat {
                if !self.state.skip_include_matchers
                    && options.match_dirs
                    && self.file_name_included_by_pattern(entry.file_name(), options)
                {
                    filter_state.skip_include_matchers = true;
                }
            } else if !self.file_name_included_by_pattern(entry.file_name(), options) {
                return None;
            }

            if self.file_name_excluded_by_pattern(entry.file_name(), options) {
                return None;
            }
        }

        if !self
            .ignore_dir
            .as_ref()
            .map(|ignore_dir| ignore_dir.include(entry.path(), entry.file_type().is_dir()))
            .unwrap_or(true)
        {
            return None;
        }

        Some(FilteredEntry {
            filter_state,
            entry,
        })
    }
}
