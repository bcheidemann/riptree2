use std::{cmp::Ordering, fs::DirEntry};

use crate::{args::TreeArgs, sorter::default_sorter};

pub struct TreeOptions {
    pub show_hidden_files: bool,
    pub sorter: fn(&DirEntry, &DirEntry) -> Ordering,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            sorter: default_sorter,
        }
    }
}

impl From<&TreeArgs> for TreeOptions {
    fn from(args: &TreeArgs) -> Self {
        Self {
            show_hidden_files: args.show_hidden_files,
            sorter: default_sorter,
        }
    }
}
