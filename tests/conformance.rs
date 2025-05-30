use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};
use uuid::Uuid;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use fixtures::fixtures;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
}

static DIRECTORY_TEMPLATE_DIR: &str = "directory.template";
static DIRECTORY_TEMPLATE_ZIP: &str = "directory.template.zip";

fn update_dir_template_archive(test_dir: &Path) {
    if test_dir.join(DIRECTORY_TEMPLATE_DIR).exists() {
        Command::new("zip")
            .args(vec!["-r", DIRECTORY_TEMPLATE_ZIP, DIRECTORY_TEMPLATE_DIR])
            .current_dir(&test_dir)
            .assert()
            .success();
    }
}

struct TestWorkingDir(PathBuf);

impl TestWorkingDir {
    fn new(test_dir: &Path) -> Self {
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

#[fixtures(["tests/fixtures/conformance/*", "!*.skip"])]
fn test(test_dir: &Path) {
    let test_description: TestDescription = {
        let path = test_dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    eprintln!("[TEST DESCRIPTION]\n{}\n\n", test_description.description);

    update_dir_template_archive(test_dir);
    let test_working_dir = TestWorkingDir::new(test_dir);

    let reference_binary = std::env::var_os("TREE_REFERENCE_BIN").unwrap_or("tree".into());
    let reference_output = Command::new(reference_binary)
        .current_dir(&test_working_dir)
        .args(&test_description.args)
        .output()
        .unwrap();
    let reference_code = reference_output.status.code().unwrap();
    let reference_stdout = String::from_utf8(reference_output.stdout).unwrap();
    let reference_stderr = String::from_utf8(reference_output.stderr).unwrap();

    let code_reference_snapshot_path = test_dir.join("code.reference.snap");
    let stdout_reference_snapshot_path = test_dir.join("stdout.reference.snap");
    let stderr_reference_snapshot_path = test_dir.join("stderr.reference.snap");

    std::fs::write(code_reference_snapshot_path, format!("{reference_code}")).unwrap();
    std::fs::write(
        stdout_reference_snapshot_path,
        format!("{reference_stdout}"),
    )
    .unwrap();
    std::fs::write(
        stderr_reference_snapshot_path,
        format!("{reference_stderr}"),
    )
    .unwrap();

    let sut_output = Command::cargo_bin("rt")
        .unwrap()
        .current_dir(&test_working_dir)
        .args(&test_description.args)
        .output()
        .unwrap();
    let sut_code = sut_output.status.code().unwrap();
    let sut_stdout = String::from_utf8(sut_output.stdout).unwrap();
    let sut_stderr = String::from_utf8(sut_output.stderr).unwrap();

    let code_sut_snapshot_path = test_dir.join("code.sut.snap");
    let stdout_sut_snapshot_path = test_dir.join("stdout.sut.snap");
    let stderr_sut_snapshot_path = test_dir.join("stderr.sut.snap");

    std::fs::write(code_sut_snapshot_path, format!("{sut_code}")).unwrap();
    std::fs::write(stdout_sut_snapshot_path, format!("{sut_stdout}")).unwrap();
    std::fs::write(stderr_sut_snapshot_path, format!("{sut_stderr}")).unwrap();

    pretty_assertions::assert_eq!(reference_code, sut_code);
    pretty_assertions::assert_eq!(reference_stdout, sut_stdout);
    pretty_assertions::assert_eq!(reference_stderr, sut_stderr);
}

#[fixtures(["tests/fixtures/conformance/*.skip"])]
#[ignore]
fn skipped_test(_: &Path) {}
