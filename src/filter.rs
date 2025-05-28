use std::fs::DirEntry;

#[derive(Default)]
pub struct TreeFilter {
    pub show_hidden_files: bool,
}

impl TreeFilter {
    pub(crate) fn include(&self, entry: &DirEntry) -> bool {
        if !self.show_hidden_files {
            if let Some(char) = entry.file_name().to_string_lossy().chars().next() {
                if char == '.' {
                    return false;
                }
            }
        }

        true
    }
}
