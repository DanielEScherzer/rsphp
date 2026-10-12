//! Test that the expected output for each case actually compiles.

use std::fs;
use std::path::Path;
use tempdir::TempDir;

fn verify_directory(data_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("TESTS_UPDATE_EXPECTED").is_ok() {
        return Err("Expectations are being updated, not testing them".into());
    }

    let tmp_dir = TempDir::new("generated")?;

    for entry_result in fs::read_dir(data_dir)? {
        let entry = entry_result?;
        let path = entry.path();
        if path.extension().ok_or("File missing extension")?.to_str() != Some("expected") {
            // This is the expected result
            continue;
        }
        let test_case_name = path
            .file_name()
            .ok_or("Missing file name")?
            .to_str()
            .ok_or("File name cannot be converted to string")?
            .strip_suffix(".expected")
            .ok_or("File missing .expected suffix")?
            .to_owned();
        let arginfo_name = test_case_name.clone() + "_arginfo.rs";

        let content = fs::read_to_string(path)?;

        // There isn't a way to check that library code compiles, need to make
        // things into valid binary targets, i.e. with main functions
        let content_to_compile =
            format!("#![allow(non_snake_case)]\n\n{content}\n\nfn main() {{}}");
        let generated_content_file = tmp_dir.path().join(arginfo_name);
        fs::write(&generated_content_file, content_to_compile)?;
    }
    let test_cases = trybuild::TestCases::new();
    test_cases.pass(format!("{}/*", tmp_dir.path().display()));
    Ok(())
}

#[test]
fn compile_cases() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cases");
    verify_directory(&data_dir).expect("Should succeed");
}

#[test]
fn compile_verification() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("verification-cases");
    verify_directory(&data_dir).expect("Should succeed");
}
