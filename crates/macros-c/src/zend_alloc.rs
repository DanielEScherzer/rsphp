//! Macros corresponding to `zend_alloc.h` in php/php-src.

use crate::c_version;

#[cfg(php_zend_debug)]
/// Allocate a pointer.
#[doc = c_version!("#define emalloc(size)                        _emalloc((size) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! emalloc {
    ($size:expr) => {
        rsphp_sys::_emalloc(
            $size,
            concat!(file!(), "\0").as_ptr().cast::<std::ffi::c_char>(),
            line!(),
            std::ptr::null_mut(),
            0,
        )
    };
}

#[cfg(not(php_zend_debug))]
/// Allocate a pointer.
#[doc = c_version!("#define emalloc(size)                        _emalloc((size) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! emalloc {
    ($size:expr) => {
        rsphp_sys::_emalloc($size)
    };
}

#[cfg(php_zend_debug)]
/// Reallocate a pointer.
#[doc = c_version!("#define erealloc(ptr, size)                    _erealloc((ptr), (size) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! erealloc {
    ($ptr:expr, $size:expr) => {
        rsphp_sys::_erealloc(
            $ptr,
            $size,
            concat!(file!(), "\0").as_ptr().cast::<std::ffi::c_char>(),
            line!(),
            std::ptr::null_mut(),
            0,
        )
    };
}

#[cfg(not(php_zend_debug))]
/// Reallocate a pointer.
#[doc = c_version!("#define erealloc(ptr, size)                    _erealloc((ptr), (size) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! erealloc {
    ($ptr:expr, $size:expr) => {
        rsphp_sys::_erealloc($ptr, $size)
    };
}

#[cfg(php_zend_debug)]
/// Free a pointer.
#[doc = c_version!("#define efree(ptr)                            _efree((ptr) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! efree {
    ($ptr:expr) => {
        rsphp_sys::_efree(
            $ptr,
            concat!(file!(), "\0").as_ptr().cast::<std::ffi::c_char>(),
            line!(),
            std::ptr::null_mut(),
            0,
        )
    };
}

#[cfg(not(php_zend_debug))]
/// Free a pointer.
#[doc = c_version!("#define efree(ptr)                            _efree((ptr) ZEND_FILE_LINE_CC ZEND_FILE_LINE_EMPTY_CC)")]
#[macro_export]
macro_rules! efree {
    ($ptr:expr) => {
        rsphp_sys::_efree($ptr)
    };
}
