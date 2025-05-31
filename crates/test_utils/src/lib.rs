use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::assert::OutputAssertExt as _;
use uuid::Uuid;

static DIRECTORY_TEMPLATE_DIR: &str = "directory.template";
static DIRECTORY_TEMPLATE_ZIP: &str = "directory.template.zip";

pub fn update_dir_template_archive(test_dir: &Path) {
    if test_dir.join(DIRECTORY_TEMPLATE_DIR).exists() {
        Command::new("zip")
            .args(vec!["-r", DIRECTORY_TEMPLATE_ZIP, DIRECTORY_TEMPLATE_DIR])
            .current_dir(&test_dir)
            .assert()
            .success();
    }
}

pub struct TestWorkingDir(PathBuf);

impl TestWorkingDir {
    pub fn new(test_dir: &Path) -> Self {
        let absolute_template_path = test_dir
            .join(DIRECTORY_TEMPLATE_ZIP)
            .canonicalize()
            .unwrap()
            .into_os_string();

        // Expand the directory template if necessary
        let template_dir = test_dir.join(DIRECTORY_TEMPLATE_DIR);
        if !template_dir.exists() {
            Command::new("unzip")
                .args(vec![&absolute_template_path])
                .current_dir(&test_dir)
                .assert()
                .success();
        }

        // Copy the directory template to a temporary directory
        let temp_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());
        let errors = copy_dir::copy_dir(&template_dir, &temp_dir).unwrap();
        if !errors.is_empty() {
            let errors = errors
                .iter()
                .enumerate()
                .map(|(idx, error)| format!("[{idx}] {error}"))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("Errors:\n\n{errors}");
        }

        Self(temp_dir)
    }
}

impl AsRef<Path> for TestWorkingDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestWorkingDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
