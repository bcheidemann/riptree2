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

#[derive(Default, Clone)]
pub struct FilterState {
    /// This counter is used to reproduce some very unintuitive behaviour in the
    /// reference implementation. It is not used unless the --compat option is
    /// set.
    ///
    /// In compat mode, if --matchdirs is passed, then include matchers are
    /// ignored when the parent directory was matched by an include matcher.
    /// However, this is not applied recursively, so if an ancestor directory
    /// was already matched, then no more directories can be matched.
    ///
    /// There are three possible states:
    ///
    /// 0: No ancestor directory was matched
    ///     - If the entry is a file, and it matches the include patterns, then
    ///       it will be displayed
    ///     - If the entry is a directory, it will be displayed
    ///     - If the entry is a directory and matches the include pattern, the
    ///       value is incremented by 1 for it's children
    ///
    /// 1: The parent directory was matched
    ///     - If the entry is a file, it will be displayed (include matchers are
    ///       not checked)
    ///     - If the entry is a directory, it will be displayed
    ///     - If the entry is a directory, and it matches the include pattern,
    ///       the value is incremented by 1 for all children
    ///
    /// 2: An ancestor directory was matched
    ///     - If the entry is a file, and it matches the include patterns, then
    ///       it will be displayed
    ///     - If the entry is a directory, it will be displayed
    matched_dir_depth: u8,
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

        let mut filter_state = self.state.clone();

        if entry.file_type().is_file() {
            if options.list_directories_only {
                return None;
            }

            if !(self.state.matched_dir_depth == 1)
                && !self.file_name_included_by_pattern(entry.file_name(), options)
            {
                return None;
            }

            if self.file_name_excluded_by_pattern(entry.file_name(), options) {
                return None;
            }
        } else if entry.file_type().is_dir() {
            if options.compat {
                let mut matched_dir = false;

                if (self.state.matched_dir_depth == 0)
                    && options.match_dirs
                    && self.file_name_included_by_pattern(entry.file_name(), options)
                {
                    matched_dir = true;
                }

                if filter_state.matched_dir_depth == 1 || matched_dir {
                    filter_state.matched_dir_depth += 1;
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
