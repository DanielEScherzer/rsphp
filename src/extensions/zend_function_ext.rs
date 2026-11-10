//! Extension trait for [`rsphp_sys::zend_function`].
use rsphp_sys::{_zend_function__common, zend_function};

/// Extension trait for [`rsphp_sys::zend_function`].
pub trait ZendFunctionExt {
    /// Get the type of the function.
    ///
    /// The result should be one of
    /// - [`rsphp_sys::ZEND_INTERNAL_FUNCTION`]
    /// - [`rsphp_sys::ZEND_USER_FUNCTION`]
    /// - [`rsphp_sys::ZEND_EVAL_CODE`]
    fn get_type(&self) -> u8;

    /// Safe access for the common information shared by internal and userland
    /// functions.
    fn get_common(&self) -> &_zend_function__common;
}

impl ZendFunctionExt for &zend_function {
    fn get_type(&self) -> u8 {
        // SAFETY: the point of the `type_` field is that it works for both internal
        // functions and user functions because they both have that field first
        unsafe { self.type_ }
    }
    fn get_common(&self) -> &_zend_function__common {
        // SAFETY: the point of the `common` field is that it works for both internal
        // functions and user functions by only holding the common layout at
        // the start
        unsafe { &self.common }
    }
}

impl ZendFunctionExt for &mut zend_function {
    fn get_type(&self) -> u8 {
        // SAFETY: the point of the `type_` field is that it works for both internal
        // functions and user functions because they both have that field first
        unsafe { self.type_ }
    }
    fn get_common(&self) -> &_zend_function__common {
        // SAFETY: the point of the `common` field is that it works for both internal
        // functions and user functions by only holding the common layout at
        // the start
        unsafe { &self.common }
    }
}
