//! Build script for [`rsphp_compat`] that sets `php_is_86` if building for PHP
//! 8.6.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(php_is_86)");

    if rsphp_sys::PHP_MAJOR_VERSION == 8 && rsphp_sys::PHP_MINOR_VERSION == 6 {
        println!("cargo:rustc-cfg=php_is_86");
    }
}
