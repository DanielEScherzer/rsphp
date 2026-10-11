//! Build script for [`rsphp_macros_rs`] that automatically sets `php_zend_zts`
//! if built with [`rsphp_sys::USING_ZST`] being `1`.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(php_zend_zts)");

    if rsphp_sys::USING_ZTS == 1 {
        println!("cargo:rustc-cfg=php_zend_zts");
    }
}
