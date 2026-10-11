//! Test that we have the expected output for various stub files.
use std::fs;
use std::path::Path;

use tempdir::TempDir;

use rsphp_stub_parser::{VerificationMode, generate_arginfo};

fn check_file_content_matches(
    expected: &Path,
    actual: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual_content = fs::read_to_string(actual)?;
    if std::env::var("TESTS_UPDATE_EXPECTED").is_ok() {
        fs::write(expected, &actual_content)?;
    }

    let expected_content = fs::read_to_string(expected)?;

    if expected_content != actual_content {
        for diff in diff::lines(&expected_content, &actual_content) {
            #[expect(clippy::print_stderr, reason = "Okay in test")]
            match diff {
                diff::Result::Left(left) => eprintln!("-{left}"),
                diff::Result::Both(left, _) => eprintln!(" {left}"),
                diff::Result::Right(right) => eprintln!("+{right}"),
            }
        }
        return Err(format!("Results do not match for {}", expected.display()).into());
    }
    Ok(())
}

fn verify_directory(
    data_dir: &Path,
    mode: VerificationMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new("example")?;

    for entry_result in fs::read_dir(data_dir)? {
        let entry = entry_result?;
        let stub_file = entry.path();
        if stub_file
            .extension()
            .ok_or("File missing extension")?
            .to_str()
            != Some("php")
        {
            // This is the expected result
            continue;
        }
        let test_case_name = stub_file
            .file_name()
            .ok_or("Missing file name")?
            .to_str()
            .ok_or("File name cannot be converted to string")?
            .strip_suffix(".stub.php")
            .ok_or("File missing .stub.php suffix")?
            .to_owned();
        let arginfo_name = test_case_name.clone() + "_arginfo.rs";
        let actual_content_file = tmp_dir.path().join(arginfo_name);
        let expected_file = stub_file.with_file_name(test_case_name + ".expected");
        generate_arginfo(&stub_file, Some(actual_content_file.clone()).as_ref(), mode)?;
        check_file_content_matches(&expected_file, &actual_content_file)?;
    }
    Ok(())
}

#[test]
fn verify_generated_output() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cases");
    verify_directory(&data_dir, VerificationMode::Never).expect("Should succeed");
}

#[test]
fn method_verification() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("verification-cases");
    verify_directory(&data_dir, VerificationMode::DebugAssert).expect("Should succeed");
}
