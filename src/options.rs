use std::cmp::Ordering;

use crate::{args::TreeArgs, entry::Entry, sorter::default_sorter};

pub struct TreeOptions {
    pub show_hidden_files: bool,
    pub list_directories_only: bool,
    pub respect_gitignore: bool,
    pub sorter: fn(&Entry, &Entry) -> Ordering,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            list_directories_only: false,
            respect_gitignore: true,
            sorter: default_sorter,
        }
    }
}

impl From<&TreeArgs> for TreeOptions {
    fn from(args: &TreeArgs) -> Self {
        Self {
            show_hidden_files: args.show_hidden_files,
            list_directories_only: args.list_directories_only,
            respect_gitignore: if args.compat {
                args.gitignore
            } else {
                !args.no_gitignore
            },
            sorter: default_sorter,
        }
    }
}
