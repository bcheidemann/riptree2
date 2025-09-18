use std::{fs::File, io::BufReader, path::Path, process::Command};

use assert_cmd::cargo::CommandCargoExt;
use fixtures::fixtures;
use serde::Deserialize;
use test_utils::{TestWorkingDir, snapshot::assert_snapshot};

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
}

#[fixtures(["tests/fixtures/snapshot/*", "!*.skip"])]
#[test]
fn test(test_dir: &Path) {
    let test_description: TestDescription = {
        let path = test_dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    eprintln!("[TEST DESCRIPTION]\n{}\n\n", test_description.description);

    let test_working_dir = TestWorkingDir::new(test_dir);

    let sut_output = Command::cargo_bin("rt")
        .unwrap()
        .current_dir(&test_working_dir)
        .args(&test_description.args)
        .output()
        .unwrap();
    let sut_code = sut_output.status.code().unwrap().to_string();
    let sut_stdout = String::from_utf8(sut_output.stdout).unwrap();
    let sut_stderr = String::from_utf8(sut_output.stderr).unwrap();

    let code_snapshot_path = test_dir.join("code.snap");
    let stdout_snapshot_path = test_dir.join("stdout.snap");
    let stderr_snapshot_path = test_dir.join("stderr.snap");

    assert_snapshot(code_snapshot_path, sut_code);
    assert_snapshot(stdout_snapshot_path, sut_stdout);
    assert_snapshot(stderr_snapshot_path, sut_stderr);
}

// #[fixtures(["tests/fixtures/snapshot/*.skip"])]
// #[test]
// #[ignore]
// fn skipped_test(_: &Path) {}
