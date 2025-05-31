use std::{fs::File, io::BufReader, path::Path, process::Command};

use assert_cmd::cargo::CommandCargoExt;
use fixtures::fixtures;
use serde::Deserialize;
use test_utils::{TestWorkingDir, update_dir_template_archive};

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
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
