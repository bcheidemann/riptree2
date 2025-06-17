use std::{
    fs::File,
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

use assert_cmd::assert::OutputAssertExt;
use criterion::{Criterion, criterion_group, criterion_main};
use serde::Deserialize;
use test_utils::{TestWorkingDir, snapshot::assert_snapshot};

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
}

fn run_cli_benchmark(c: &mut Criterion, test_name: &str) {
    let test_dir = format!("tests/fixtures/bench/{test_name}");
    let test_dir = Path::new(&test_dir);

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .assert()
        .code(0);

    let release_bin = std::env::current_dir().unwrap().join("target/release/rt");

    let test_description: TestDescription = {
        let path = test_dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    eprintln!("[TEST DESCRIPTION]\n{}\n\n", test_description.description);

    let test_working_dir = TestWorkingDir::new(test_dir);

    let sut_output = Command::new(&release_bin)
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

    c.bench_function(&test_name, |b| {
        b.iter(|| {
            Command::new(&release_bin)
                .current_dir(&test_working_dir)
                .args(&test_description.args)
                .stdout(Stdio::null())
                .status()
                .unwrap();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    run_cli_benchmark(c, "cli_all_file_types");
    run_cli_benchmark(c, "cli_nested_dirs");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
