use std::{cmp::Ordering, sync::Arc};

use anyhow::Context as _;
use globset::{Glob, GlobSet};

use crate::{args::TreeArgs, entry::Entry, sorter::default_sorter};

pub struct TreeOptions {
    pub show_hidden_files: bool,
    pub list_directories_only: bool,
    pub print_full_path_prefix: bool,
    pub max_level: Option<usize>,
    pub file_include_globset: Option<Arc<GlobSet>>,
    pub respect_gitignore: bool,
    pub sorter: fn(&Entry, &Entry) -> Ordering,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            list_directories_only: false,
            print_full_path_prefix: false,
            max_level: None,
            file_include_globset: None,
            respect_gitignore: true,
            sorter: default_sorter,
        }
    }
}

impl TryFrom<TreeArgs> for TreeOptions {
    type Error = anyhow::Error;

    fn try_from(args: TreeArgs) -> anyhow::Result<TreeOptions> {
        Ok(Self {
            show_hidden_files: args.show_hidden_files,
            list_directories_only: args.list_directories_only,
            print_full_path_prefix: args.print_full_path_prefix,
            max_level: args.max_level,
            file_include_globset: build_globset(args.file_include_patterns)
                .context("Failed to build matcher for file include patterns (-P)")?,
            respect_gitignore: if args.compat {
                args.gitignore
            } else {
                !args.no_gitignore
            },
            sorter: default_sorter,
        })
    }
}

/// Builds a GlobSet matcher from a collection of globs. Returns `Ok(None)` if
/// the collection of globs is empty.
fn build_globset(globs: Vec<String>) -> anyhow::Result<Option<Arc<GlobSet>>> {
    if globs.is_empty() {
        return Ok(None);
    }

    // Split globs on | to match behaviour of reference implementation
    let globs = globs.into_iter().flat_map(|s| {
        s.split("|")
            .map(|s| s.to_string())
            .collect::<Box<[String]>>()
    });

    let mut file_include_globset_builder = GlobSet::builder();

    for glob in globs {
        // TODO: In compat mode we should port the glob parser from the
        //       reference implementation.
        // EXPLANATION: The glob parser used differes from the reference in a
        //              number of ways, including accepting the {} syntax, and
        //              how it handles invalid globs e.g. '*.[txt'
        let glob = Glob::new(&glob)?;
        file_include_globset_builder.add(glob);
    }

    Ok(Some(file_include_globset_builder.build()?.into()))
}
