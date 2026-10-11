//! Macros corresponding to `zend_closures.h` in php/php-src.

use crate::c_version;

/// Get the closure object of an op array.
#[doc = c_version!(
    "#define ZEND_CLOSURE_OBJECT(op_array)",
    "((zend_object*)((char*)(op_array) - sizeof(zend_object)))"
)]
#[macro_export]
macro_rules! ZEND_CLOSURE_OBJECT {
    ($op_array:expr) => {
        $op_array
            .byte_sub(std::mem::size_of::<rsphp_sys::zend_object>())
            .cast::<rsphp_sys::zend_object>()
    };
}
