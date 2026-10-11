//! This crate holds tools for parsing a PHP stub file and generating the
//! argument information for the classes, interfaces, enums, functions, and
//! constants defined in it. It is inspired by the `gen_stub.php` script used
//! for PHP extensions written in C.

use std::fs;
use std::path::PathBuf;

/// Module for `ErrorCode`.
mod errors;
pub use crate::errors::ErrorCode;

/// Module for `ArgInfoBuilder`.
mod builder;
use crate::builder::ArgInfoBuilder;

/// Module for `ArgInfoTypeRepr`.
mod types;

/// Module for `VerificationMode`.
mod verification;
pub use crate::verification::VerificationMode;

/// Process a stub file and save the resulting arginfo file.
///
/// # Errors
///
/// Returns [`crate::ErrorCode`] on error.
pub fn generate_arginfo(
    stub_file: &PathBuf,
    output: Option<&PathBuf>,
    verification: VerificationMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = process_stub(stub_file, verification)?;
    let Some(file_name) = stub_file.file_name() else {
        return Err(ErrorCode::NonFilePath(stub_file.clone()).into());
    };
    let Some(as_str) = file_name.to_str() else {
        return Err(ErrorCode::CannotConvertToString(stub_file.clone()).into());
    };
    let Some(without_suffix) = as_str.strip_suffix(".stub.php") else {
        return Err(ErrorCode::NonStubFile(stub_file.clone()).into());
    };
    let result_file = match output {
        Some(output_file) => output_file,
        None => &stub_file.with_file_name(without_suffix.to_owned() + "_arginfo.rs"),
    };

    match fs::write(result_file, result) {
        Ok(()) => (),
        Err(error) => return Err(ErrorCode::ArginfoWriteFailed(result_file.clone(), error).into()),
    }
    Ok(())
}

/// Process a stub file and return the result.
///
/// # Errors
///
/// Returns [`crate::ErrorCode`] on error.
pub fn process_stub(
    stub_file: &PathBuf,
    verification: VerificationMode,
) -> Result<String, Box<dyn std::error::Error>> {
    if !stub_file.is_file() {
        return Err(ErrorCode::NonFilePath(stub_file.clone()).into());
    }
    // match rather than ? so that the compiler doesn't think stub_file moved
    let contents: String = match fs::read_to_string(stub_file) {
        Ok(file_content) => file_content,
        Err(error) => return Err(ErrorCode::StubReadFailed(stub_file.clone(), error).into()),
    };
    let parsed = php_rs_parser::parse(&contents);
    if !parsed.errors.is_empty() {
        return Err(ErrorCode::StubParseErrors(stub_file.clone(), parsed.errors).into());
    }
    let program = parsed.program;
    let mut builder = ArgInfoBuilder::new(&contents, verification);
    builder.statements(&program.stmts)?;
    Ok(builder.assemble())
}
