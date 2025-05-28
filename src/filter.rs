use std::fs::DirEntry;

use crate::options::TreeOptions;

#[derive(Clone, Default)]
pub struct TreeFilter;

impl TreeFilter {
    pub(crate) fn enter_dir(&self, _dir: &DirEntry, _options: &TreeOptions) -> Self {
        self.clone()
    }

    pub(crate) fn include(&self, entry: &DirEntry, options: &TreeOptions) -> bool {
        if !options.show_hidden_files {
            if let Some(char) = entry.file_name().to_string_lossy().chars().next() {
                if char == '.' {
                    return false;
                }
            }
        }

        true
    }
}
