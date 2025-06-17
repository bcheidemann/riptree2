use std::{fs::File, io::BufReader, path::Path, process::Command};

use assert_cmd::cargo::CommandCargoExt;
use fixtures::fixtures;
use serde::Deserialize;
use test_utils::TestWorkingDir;

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
    current_directory: Option<String>,
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

    let test_working_dir = TestWorkingDir::new(test_dir);
    let command_current_directory = match test_description.current_directory {
        Some(current_directory) => test_working_dir.as_ref().join(current_directory),
        None => test_working_dir.as_ref().to_path_buf(),
    };

    let reference_binary = std::env::var_os("TREE_REFERENCE_BIN").unwrap_or("tree".into());
    let reference_output = Command::new(reference_binary)
        .current_dir(&command_current_directory)
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
        .current_dir(&command_current_directory)
        .arg("--compat")
        .args(&test_description.args)
        .output()
        .unwrap();
    let sut_code = sut_output.status.code().unwrap();
    let sut_stdout = String::from_utf8(sut_output.stdout).unwrap();
    let sut_stderr = String::from_utf8(sut_output.stderr).unwrap();

    let code_sut_snapshot_path = test_dir.join("code.sut.snap");
    let stdout_sut_snapshot_path = test_dir.join("stdout.sut.snap");
    let stderr_sut_snapshot_path = test_dir.join("stderr.sut.snap");

    std::fs::write(code_sut_snapshot_path, sut_code.to_string()).unwrap();
    std::fs::write(stdout_sut_snapshot_path, &sut_stdout).unwrap();
    std::fs::write(stderr_sut_snapshot_path, &sut_stderr).unwrap();

    pretty_assertions::assert_eq!(reference_code, sut_code);
    pretty_assertions::assert_eq!(reference_stdout, sut_stdout);
    pretty_assertions::assert_eq!(reference_stderr, sut_stderr);
}

#[fixtures(["tests/fixtures/conformance/*.skip"])]
#[ignore]
fn skipped_test(_: &Path) {}
