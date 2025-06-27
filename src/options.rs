use std::cmp::Ordering;

use anyhow::Context as _;
use globset::{GlobBuilder, GlobSet};

use crate::{args::TreeArgs, entry::Entry, sorter::default_sorter};

pub struct TreeOptions {
    pub compat: bool,
    pub show_hidden_files: bool,
    pub list_directories_only: bool,
    pub print_full_path_prefix: bool,
    pub max_level: Option<usize>,
    pub file_include_globset: Option<GlobSet>,
    pub file_exclude_globset: Option<GlobSet>,
    pub respect_gitignore: bool,
    pub icons: bool,
    pub sorter: fn(&Entry, &Entry) -> Ordering,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            compat: false,
            show_hidden_files: false,
            list_directories_only: false,
            print_full_path_prefix: false,
            max_level: None,
            file_include_globset: None,
            file_exclude_globset: None,
            respect_gitignore: true,
            icons: true,
            sorter: default_sorter,
        }
    }
}

impl TryFrom<TreeArgs> for TreeOptions {
    type Error = anyhow::Error;

    fn try_from(args: TreeArgs) -> anyhow::Result<TreeOptions> {
        Ok(Self {
            compat: args.compat,
            show_hidden_files: args.show_hidden_files,
            list_directories_only: args.list_directories_only,
            print_full_path_prefix: args.print_full_path_prefix,
            max_level: args.max_level,
            file_include_globset: build_globset(args.file_include_patterns, args.ignore_case)
                .context("Failed to build matcher for file include patterns (-P)")?,
            file_exclude_globset: build_globset(args.file_exclude_patterns, args.ignore_case)
                .context("Failed to build matcher for file exclude patterns (-I)")?,
            respect_gitignore: if args.compat {
                args.gitignore
            } else {
                !args.no_gitignore
            },
            icons: if args.compat {
                args.icons
            } else {
                !args.no_icons
            },
            sorter: default_sorter,
        })
    }
}

/// Builds a GlobSet matcher from a collection of globs. Returns `Ok(None)` if
/// the collection of globs is empty.
fn build_globset(globs: Vec<String>, case_insensitive: bool) -> anyhow::Result<Option<GlobSet>> {
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
        let glob = GlobBuilder::new(&glob)
            .case_insensitive(case_insensitive)
            .build()
            .context("Failed to build glob")?;
        file_include_globset_builder.add(glob);
    }

    Ok(Some(file_include_globset_builder.build()?))
}
