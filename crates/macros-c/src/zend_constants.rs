//! Macros corresponding to `zend_constants.h` in php/php-src.

use crate::c_version;

/// Access the flags of a global [`rsphp_sys::zend_constant`] pointer.
#[doc = c_version!(
    "#define ZEND_CONSTANT_FLAGS(c)",
    "(Z_CONSTANT_FLAGS((c)->value) & 0xff)"
)]
#[macro_export]
macro_rules! ZEND_CONSTANT_FLAGS {
    ($c:expr) => {
        $crate::Z_CONSTANT_FLAGS!((*$c).value) & 0xFF
    };
}

/// Access the module number of a global [`rsphp_sys::zend_constant`] pointer.
#[doc = c_version!(
    "#define ZEND_CONSTANT_MODULE_NUMBER(c)",
    "(Z_CONSTANT_FLAGS((c)->value) >> 8)"
)]
#[macro_export]
macro_rules! ZEND_CONSTANT_MODULE_NUMBER {
    ($c:expr) => {
        $crate::Z_CONSTANT_FLAGS!((*$c).value) >> 8
    };
}
