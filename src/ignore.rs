use std::path::{Path, PathBuf};

use anyhow::Context;
use ignore::gitignore::{Gitignore, GitignoreBuilder};

pub struct IgnoreDir<'ignore> {
    parent: Option<&'ignore IgnoreDir<'ignore>>,
    gitignores: Vec<Gitignore>,
}

impl<'ignore> IgnoreDir<'ignore> {
    pub(crate) fn new(dir: &Path) -> anyhow::Result<Self> {
        let global_gitignore = match GitignoreBuilder::new("").build_global() {
            (_, Some(err)) => panic!("{err}"),
            (gitignore, None) => gitignore,
        };
        let mut gitignores = vec![global_gitignore];

        let canonicalized_root = dir.canonicalize().with_context(|| {
            format!(
                "Failed to canonicalize directory ({})",
                dir.to_string_lossy(),
            )
        })?;
        let mut path_components = canonicalized_root.components();

        let mut current_dir = if let Some(root) = path_components.next() {
            PathBuf::from(root.as_os_str())
        } else {
            return Ok(Self {
                parent: None,
                gitignores,
            });
        };

        for path_component in path_components {
            current_dir = current_dir.join(path_component.as_os_str());
            let mut builder = GitignoreBuilder::new(&current_dir);
            let gitignore_path = current_dir.join(".gitignore");
            if gitignore_path.exists() {
                if let Some(err) = builder.add(&gitignore_path) {
                    return Err(err).context(format!(
                        "Failed to load {}",
                        gitignore_path.to_string_lossy()
                    ));
                }
            }
            gitignores.push(
                builder.build().with_context(|| {
                    format!("Failed to load {}", gitignore_path.to_string_lossy())
                })?,
            );
        }

        // Reverse so the highest priority .gitignore is first
        gitignores.reverse();

        Ok(Self {
            parent: None,
            gitignores,
        })
    }

    pub(crate) fn enter_dir(&'ignore self, dir: &Path) -> anyhow::Result<Self> {
        let gitignore_path = dir.join(".gitignore");
        let gitignores =
            if gitignore_path.exists() {
                let mut builder = GitignoreBuilder::new(dir);
                if let Some(err) = builder.add(&gitignore_path) {
                    return Err(err).context(format!(
                        "Failed to load {}",
                        gitignore_path.to_string_lossy()
                    ));
                }
                vec![builder.build().with_context(|| {
                    format!("Failed to load {}", gitignore_path.to_string_lossy())
                })?]
            } else {
                vec![]
            };

        Ok(Self {
            parent: Some(self),
            gitignores,
        })
    }

    pub(crate) fn include(&self, path: &Path, is_dir: bool) -> bool {
        if is_dir && path.file_name().map(|name| name == ".git").unwrap_or(false) {
            return false;
        }

        for gitignore in &self.gitignores {
            let is_match = gitignore.matched(path, is_dir);

            if is_match.is_whitelist() {
                return true;
            }

            if is_match.is_ignore() {
                return false;
            }
        }

        if let Some(parent) = self.parent {
            parent.include(path, is_dir)
        } else {
            true
        }
    }
}
