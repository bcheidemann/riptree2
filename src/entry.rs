use std::{
    ffi::OsString,
    fs::{DirEntry, FileType},
    os::unix::ffi::OsStrExt as _,
    path::{Path, PathBuf},
};

use anyhow::Context as _;

pub struct Entry {
    file_name: OsString,
    path: PathBuf,
    ty: FileType,
}

impl Entry {
    pub(crate) fn new(entry: DirEntry) -> anyhow::Result<Self> {
        let file_name = entry.file_name();
        let path = entry.path();
        let ty = entry
            .file_type()
            .with_context(|| format!("Failed to get file type of {}", path.to_string_lossy()))?;

        Ok(Self {
            file_name,
            path,
            ty,
        })
    }

    pub fn file_name(&self) -> &OsString {
        &self.file_name
    }

    pub fn file_type(&self) -> FileType {
        self.ty
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_hidden(&self) -> bool {
        // SAFETY: File names are always at least one byte long
        unsafe { self.file_name.as_bytes().first().unwrap_unchecked() == &b'.' }
    }

    pub fn into_path(self) -> PathBuf {
        self.path
    }
}
