use std::path::Path;

use rsphp_stub_parser::generate_arginfo;

#[expect(
    clippy::expect_used,
    reason = "Failure here is a compile-time issue rather than a runtime panic"
)]
fn main() {
    println!("cargo::rustc-check-cfg=cfg(php_zend_zts)");
    println!("cargo::rustc-check-cfg=cfg(php_is_86)");

    if rsphp_sys::USING_ZTS == 1 {
        println!("cargo:rustc-cfg=php_zend_zts");
    }
    if rsphp_sys::PHP_MAJOR_VERSION == 8 && rsphp_sys::PHP_MINOR_VERSION == 6 {
        println!("cargo:rustc-cfg=php_is_86");
    }

    println!("cargo::rerun-if-changed=src/rustdemo.stub.php");
    println!("cargo::rerun-if-changed=src/opaquedata.stub.php");
    println!("cargo::rerun-if-changed=src/ini_demo.stub.php");

    generate_arginfo(
        &Path::new("src/rustdemo.stub.php").to_path_buf(),
        None,
        rsphp_stub_parser::VerificationMode::DebugAssert,
    )
    .expect("Stub processing should succeed");

    generate_arginfo(
        &Path::new("src/ini_demo.stub.php").to_path_buf(),
        None,
        rsphp_stub_parser::VerificationMode::DebugAssert,
    )
    .expect("Stub processing should succeed");

    generate_arginfo(
        &Path::new("src/opaquedata.stub.php").to_path_buf(),
        None,
        rsphp_stub_parser::VerificationMode::DebugAssert,
    )
    .expect("Stub processing should succeed");
}
