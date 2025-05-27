use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};
use uuid::Uuid;

use assert_cmd::{assert::OutputAssertExt as _, cargo::CommandCargoExt};
use fixtures::fixtures;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestDescription {
    args: Vec<String>,
    create_dirs: Option<Vec<PathBuf>>,
}

#[fixtures(["tests/fixtures/conformance/*", "!*.skip"])]
fn test(dir: &Path) {
    let test_description: TestDescription = {
        let path = dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };
    let dir_template = dir.join("dir_template");
    let temp_working_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());

    if dir_template.exists() {
        let errors = copy_dir::copy_dir(&dir_template, &temp_working_dir).unwrap();
        if !errors.is_empty() {
            let errors = errors
                .iter()
                .enumerate()
                .map(|(idx, error)| format!("[{idx}] {error}"))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("Errors:\n\n{errors}");
        }
    }

    std::fs::create_dir_all(&temp_working_dir).unwrap();

    for dir in test_description.create_dirs.unwrap_or(vec![]) {
        let dir = temp_working_dir.join(dir);
        std::fs::create_dir_all(dir).unwrap();
    }

    let reference_binary = std::env::var_os("TREE_REFERENCE_BIN").unwrap_or("tree".into());
    let reference_output = Command::new(reference_binary)
        .current_dir(&temp_working_dir)
        .args(&test_description.args)
        .output()
        .unwrap();
    let reference_code = reference_output.status.code().unwrap();
    let reference_stdout = String::from_utf8(reference_output.stdout).unwrap();
    let reference_stderr = String::from_utf8(reference_output.stderr).unwrap();

    let code_snapshot_path = dir.join("code.snap");
    let stdout_snapshot_path = dir.join("stdout.snap");
    let stderr_snapshot_path = dir.join("stderr.snap");

    std::fs::write(code_snapshot_path, format!("{reference_code}")).unwrap();
    std::fs::write(stdout_snapshot_path, format!("{reference_stdout}")).unwrap();
    std::fs::write(stderr_snapshot_path, format!("{reference_stderr}")).unwrap();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(&temp_working_dir)
        .args(&test_description.args)
        .assert()
        .code(reference_code)
        .stdout(reference_stdout)
        .stderr(reference_stderr);
}

#[fixtures(["tests/fixtures/conformance/*.skip"])]
#[ignore]
fn skipped_test(_: &Path) {}
