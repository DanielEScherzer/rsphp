//! Extension trait for [`rsphp_sys::zend_function`].
use rsphp_sys::{_zend_function__common, zend_function};

pub trait ZendFunctionExt {
    fn get_type(&self) -> u8;
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
