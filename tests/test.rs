use std::{fs::File, io::BufReader, path::Path, process::Command};

use assert_cmd::{assert::OutputAssertExt as _, cargo::CommandCargoExt};
use fixtures::fixtures;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestDescription {
    cwd: String,
    args: Vec<String>,
}

#[fixtures(["tests/fixtures/conformance/*"])]
fn test(dir: &Path) {
    let test_description: TestDescription = {
        let path = dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };
    let working_dir = dir.join(test_description.cwd);

    let reference_binary = std::env::var_os("TREE_REFERENCE_BIN").unwrap_or("tree".into());
    let reference_output = Command::new(reference_binary)
        .current_dir(&working_dir)
        .args(&test_description.args)
        .output()
        .unwrap();
    let reference_code = reference_output.status.code().unwrap();
    let reference_stdout = String::from_utf8(reference_output.stdout).unwrap();
    let reference_stderr = String::from_utf8(reference_output.stderr).unwrap();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(&working_dir)
        .args(&test_description.args)
        .assert()
        .code(reference_code)
        .stdout(reference_stdout)
        .stderr(reference_stderr);
}
