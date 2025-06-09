pub mod snapshot;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::assert::OutputAssertExt;
use uuid::Uuid;

static DIRECTORY_SNAPSHOT_DIR: &str = "directory.snapshot";
static DIRECTORY_SETUP_SCRIPT: &str = "setup.sh";

pub struct TestWorkingDir(PathBuf);

impl TestWorkingDir {
    pub fn new(test_dir: &Path) -> Self {
        let directory_snapshot_path = test_dir.join(DIRECTORY_SNAPSHOT_DIR);
        let absolute_setup_script_path = test_dir
            .join(DIRECTORY_SETUP_SCRIPT)
            .canonicalize()
            .unwrap()
            .into_os_string();

        if directory_snapshot_path.exists() {
            std::fs::remove_dir_all(&directory_snapshot_path).unwrap();
        }
        std::fs::create_dir_all(&directory_snapshot_path).unwrap();
        Command::new(&absolute_setup_script_path)
            .current_dir(&directory_snapshot_path)
            .assert()
            .success();

        let temp_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());
        std::fs::create_dir_all(&temp_dir).unwrap();
        Command::new(&absolute_setup_script_path)
            .current_dir(&temp_dir)
            .assert()
            .success();

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
