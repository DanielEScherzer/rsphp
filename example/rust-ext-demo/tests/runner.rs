//! Integrate run-tests.php into `Cargo test`.
#![expect(clippy::expect_used, reason = "Tests should panic on failure")]

use std::path::{Path, PathBuf};
use std::process::Command;

fn get_run_tests() -> PathBuf {
    // Normal development: up a few levels
    let relative = Path::new("../../../../run-tests.php");
    if relative.exists() {
        return relative.to_path_buf();
    }
    let path_from_env = std::env::var("RUN_TESTS_PATH").expect("RUN_TESTS_PATH should be provided");
    Path::new(&path_from_env).to_path_buf()
}

fn get_so_file() -> PathBuf {
    let out_dir = std::env::var("OUT_DIR").expect("Out dir should be set");
    let out_path = Path::new(&out_dir);
    // Go up to the main output directory
    let result_dir = out_path
        .parent()
        .expect("going up from /out")
        .parent()
        .expect("going up from package build directory")
        .parent()
        .expect("going up from /build");
    let package_name = env!("CARGO_PKG_NAME").replace('-', "_");
    result_dir.join(format!("lib{package_name}.so"))
}

#[test]
fn php_tests() {
    let so_file = get_so_file();
    assert!(so_file.exists(), "Extension .so file not found");
    let run_tests = get_run_tests();
    let status = Command::new("php")
        .arg(&run_tests)
        .arg("-n")
        .arg("-d")
        .arg(format!("extension={}", so_file.display()))
        .arg("-q")
        .arg(".")
        .status()
        .expect("Tests should run");
    assert!(status.success(), "Tests should pass");

    let status = Command::new("php")
        .arg(&run_tests)
        .arg("-n")
        .arg("-d")
        .arg(format!("extension={}", so_file.display()))
        .arg(".")
        .arg("-m")
        .status()
        .expect("Tests should run");
    assert!(status.success(), "No memory leaks expected");
}
