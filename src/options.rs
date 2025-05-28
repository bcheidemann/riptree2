use std::{cmp::Ordering, fs::DirEntry};

use crate::{args::TreeArgs, filter::TreeFilter, sorter::default_sorter};

pub struct TreeOptions {
    pub filter: TreeFilter,
    pub sorter: fn(&DirEntry, &DirEntry) -> Ordering,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            filter: TreeFilter::default(),
            sorter: default_sorter,
        }
    }
}

impl From<&TreeArgs> for TreeOptions {
    fn from(args: &TreeArgs) -> Self {
        Self {
            filter: TreeFilter {
                show_hidden_files: args.show_hidden_files,
            },
            sorter: default_sorter,
        }
    }
}
