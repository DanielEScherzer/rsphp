//! Macros corresponding to `zend_globals_macros.h` in php/php-src.
//!
//! For ZTS purposes, `ZEND_ENABLE_STATIC_TSRMLS_CACHE` is assumed to be false,
//! meaning that `ZEND_TSRMG_FAST` is an alias for `TSRMG_FAST`.

#[cfg(any(doc, not(php_zend_zts)))]
use crate::c_version;

/// Access a compiler global.
#[doc = c_version!(
    NTS => "#define CG(v) (compiler_globals.v)",
    ZTS => "#define CG(v) ZEND_TSRMG_FAST(compiler_globals_offset, zend_compiler_globals *, v)",
)]
#[macro_export]
#[cfg(any(doc, not(php_zend_zts)))]
macro_rules! CG {
    ($field:ident) => {
        unsafe { rsphp_sys::compiler_globals.$field }
    };
}

/// Access a compiler global.
#[macro_export]
#[cfg(all(not(doc), php_zend_zts))]
macro_rules! CG {
    ($field:ident) => {
        unsafe {
            $crate::TSRMG_FAST!(
                rsphp_sys::compiler_globals_offset,
                rsphp_sys::zend_compiler_globals,
                $field
            )
        }
    };
}

/// Access an executer global.
#[doc = c_version!(
    NTS => "#define EG(v) (executor_globals.v)",
    ZTS => "#define EG(v) ZEND_TSRMG_FAST(executor_globals_offset, zend_executor_globals *, v)",
)]
#[macro_export]
#[cfg(any(doc, not(php_zend_zts)))]
macro_rules! EG {
    ($field:ident) => {
        unsafe { rsphp_sys::executor_globals.$field }
    };
}

/// Access an executer global.
#[macro_export]
#[cfg(all(not(doc), php_zend_zts))]
macro_rules! EG {
    ($field:ident) => {
        unsafe {
            $crate::TSRMG_FAST!(
                rsphp_sys::executor_globals_offset,
                rsphp_sys::zend_executor_globals,
                $field
            )
        }
    };
}
