//! Build script for [`rsphp_macros_rs`] that automatically sets `php_zend_zts`
//! if built with [`rsphp_sys::USING_ZST`] being `1`, and `php_zend_debug` if
//! built with [`rsphp_sys::ZEND_DEBUG`] being `1`.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(php_zend_debug)");
    println!("cargo::rustc-check-cfg=cfg(php_zend_zts)");

    if rsphp_sys::ZEND_DEBUG == 1 {
        println!("cargo:rustc-cfg=php_zend_debug");
    }
    if rsphp_sys::USING_ZTS == 1 {
        println!("cargo:rustc-cfg=php_zend_zts");
    }
}
