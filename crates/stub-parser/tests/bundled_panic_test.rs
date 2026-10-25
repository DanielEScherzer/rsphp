//! Test that for all of the bundled stub files arginfo generation doesn't fail.
use walkdir::WalkDir;

use rsphp_stub_parser::{VerificationMode, process_stub};

fn verify_walker(walker: WalkDir) -> Result<(), Box<dyn std::error::Error>> {
    for entry_result in walker {
        let entry = entry_result?;
        if entry.file_type().is_dir() {
            continue;
        }
        let name = entry.file_name().to_str().expect("Valid file name");
        if !name.ends_with(".stub.php") {
            continue;
        }
        macro_rules! skip {
            ($( $file:literal, )+) => {
                $(
                    if name.ends_with(concat!($file, ".stub.php")) {
                        continue;
                    }
                )+
            }
        }
        // zend_builtin_functions.stub.php declares things like clone(),
        // exit(), and die(), which are not supported by the parser
        // zend_constants.stub.php declares true/false/null which are also
        // not supported
        // php_date uses global constants to set the class constant
        // php_sql.stub.php uses iterable
        // zend_test splits the doc comment into two for one of the constants
        // listformatter.stub.php uses #if conditions to specify the definition
        // of IntlListFromatter::TYPE_AND and ::WIDTH_WIDE,
        skip!(
            "zend_builtin_functions",
            "zend_constants",
            "php_date",
            "php_spl",
            "test",
            "listformatter",
        );
        let path = entry.into_path();
        // We don't care about the actual result, just that .expect() does not
        // panic
        let _result = process_stub(&path, VerificationMode::Never)?;
    }
    Ok(())
}

#[test]
fn verify_extensions() {
    // Limit the walker so that we don't run over the stub parser tests again
    // some extensions (e.g. intl) have subdirectories that also have stubs
    // limit to `ext/{name}/{file}` and `ext/{name}/{dir}/{file}`
    verify_walker(WalkDir::new("../../../").max_depth(3)).expect("Should succeed");
}

#[test]
fn verify_zend() {
    verify_walker(WalkDir::new("../../../../Zend").max_depth(1)).expect("Should succeed");
}

#[test]
fn verify_sapis() {
    verify_walker(WalkDir::new("../../../../sapi")).expect("Should succeed");
}
