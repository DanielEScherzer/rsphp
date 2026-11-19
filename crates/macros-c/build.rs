//! Build script for [`rsphp_macros_rs`] that automatically sets
//! - `php_zend_zts` if built with [`rsphp_sys::USING_ZST`] being `1`.
//! - `php_zend_debug` if built with [`rsphp_sys::ZEND_DEBUG`] being `1`.
//! - `php_is_86` if built for PHP 8.6.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(php_zend_debug)");
    println!("cargo::rustc-check-cfg=cfg(php_zend_zts)");
    println!("cargo::rustc-check-cfg=cfg(php_is_86)");

    if rsphp_sys::ZEND_DEBUG == 1 {
        println!("cargo:rustc-cfg=php_zend_debug");
    }
    if rsphp_sys::USING_ZTS == 1 {
        println!("cargo:rustc-cfg=php_zend_zts");
    }
    if rsphp_sys::PHP_MAJOR_VERSION == 8 && rsphp_sys::PHP_MINOR_VERSION == 6 {
        println!("cargo:rustc-cfg=php_is_86");
    }
}
