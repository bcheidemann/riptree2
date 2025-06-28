use std::{
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

use assert_cmd::assert::OutputAssertExt;
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main,
    measurement::Measurement,
};
use serde::Deserialize;
use test_utils::{TestWorkingDir, snapshot::assert_snapshot};

#[derive(Deserialize)]
struct TestDescription {
    description: String,
    args: Vec<String>,
}

fn run_cli_benchmark<M: Measurement>(
    c: &mut BenchmarkGroup<M>,
    test_name: &str,
    binary: &OsStr,
    args: &Vec<&str>,
    benchmark_id: BenchmarkId,
    snapshot_name: &str,
) {
    let test_dir = format!("tests/fixtures/bench/{test_name}");
    let test_dir = Path::new(&test_dir);

    let test_description: TestDescription = {
        let path = test_dir.join("test.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let test_working_dir = TestWorkingDir::new(test_dir);

    if std::env::var_os("SKIP_BENCH_ASSERTIONS").is_none() {
        eprintln!("[TEST DESCRIPTION]\n{}\n\n", test_description.description);

        let sut_output = Command::new(binary)
            .current_dir(&test_working_dir)
            .args(args)
            .args(&test_description.args)
            .output()
            .unwrap();
        let sut_code = sut_output.status.code().unwrap().to_string();
        let sut_stdout = String::from_utf8(sut_output.stdout).unwrap();
        let sut_stderr = String::from_utf8(sut_output.stderr).unwrap();

        let code_snapshot_path = test_dir.join(format!("code.{snapshot_name}.snap"));
        let stdout_snapshot_path = test_dir.join(format!("stdout.{snapshot_name}.snap"));
        let stderr_snapshot_path = test_dir.join(format!("stderr.{snapshot_name}.snap"));

        assert_snapshot(code_snapshot_path, sut_code);
        assert_snapshot(stdout_snapshot_path, sut_stdout);
        assert_snapshot(stderr_snapshot_path, sut_stderr);
    }

    c.bench_function(benchmark_id, |b| {
        b.iter(|| {
            Command::new(binary)
                .current_dir(&test_working_dir)
                .args(args)
                .args(&test_description.args)
                .stdout(Stdio::null())
                .status()
                .unwrap();
        })
    });
}

fn criterion_benchmark_riptree2<M: Measurement>(c: &mut BenchmarkGroup<M>, test_name: &str) {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .assert()
        .code(0);

    let release_bin = std::env::current_dir()
        .unwrap()
        .join("target/release/rt")
        .into_os_string();
    run_cli_benchmark(
        c,
        test_name,
        &release_bin,
        &vec![],
        BenchmarkId::from_parameter("riptree2"),
        "riptree2",
    );
}

fn criterion_benchmark_riptree2_compat<M: Measurement>(c: &mut BenchmarkGroup<M>, test_name: &str) {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .assert()
        .code(0);

    let release_bin = std::env::current_dir()
        .unwrap()
        .join("target/release/rt")
        .into_os_string();
    run_cli_benchmark(
        c,
        test_name,
        &release_bin,
        &vec!["--compat"],
        BenchmarkId::from_parameter("riptree2/compat"),
        "riptree2_compat",
    );
}

fn criterion_benchmark_reference<M: Measurement>(c: &mut BenchmarkGroup<M>, test_name: &str) {
    let release_bin = std::env::var_os("TREE_CMD").unwrap_or("tree".into());
    run_cli_benchmark(
        c,
        test_name,
        &release_bin,
        &vec![],
        BenchmarkId::from_parameter("reference"),
        "reference",
    );
}

fn bench_cli_all_file_types(c: &mut Criterion) {
    let test_name = "cli_all_file_types";
    let mut group = c.benchmark_group(test_name);
    criterion_benchmark_riptree2(&mut group, test_name);
    criterion_benchmark_riptree2_compat(&mut group, test_name);
    criterion_benchmark_reference(&mut group, test_name);
}

fn bench_cli_nested_dirs(c: &mut Criterion) {
    let test_name = "cli_nested_dirs";
    let mut group = c.benchmark_group(test_name);
    criterion_benchmark_riptree2(&mut group, test_name);
    criterion_benchmark_riptree2_compat(&mut group, test_name);
    criterion_benchmark_reference(&mut group, test_name);
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_cli_all_file_types, bench_cli_nested_dirs
}
criterion_main!(benches);
