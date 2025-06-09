pub mod snapshot;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::assert::OutputAssertExt;
use uuid::Uuid;

static DIRECTORY_TEMPLATE_DIR: &str = "directory.template";
static DIRECTORY_SETUP_SCRIPT: &str = "setup.sh";

pub struct TestWorkingDir(PathBuf);

impl TestWorkingDir {
    pub fn new(test_dir: &Path) -> Self {
        let directory_template_path = test_dir.join(DIRECTORY_TEMPLATE_DIR);
        let absolute_setup_script_path = test_dir
            .join(DIRECTORY_SETUP_SCRIPT)
            .canonicalize()
            .unwrap()
            .into_os_string();
        if directory_template_path.exists() {
            std::fs::remove_dir_all(&directory_template_path).unwrap();
        }
        std::fs::create_dir_all(&directory_template_path).unwrap();
        Command::new(&absolute_setup_script_path)
            .current_dir(&directory_template_path)
            .assert()
            .success();

        // Copy the directory template to a temporary directory
        let temp_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());
        let errors = copy_dir::copy_dir(&directory_template_path, &temp_dir).unwrap();
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
