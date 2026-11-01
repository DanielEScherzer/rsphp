//! Extension trait for [`rsphp_sys::zend_class_constant`].
use rsphp_sys::zend_class_constant;

/// Extension trait for [`rsphp_sys::zend_class_constant`].
pub trait ZendClassConstantExt {
    /// Get the flags of the constant.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_CLASS_CONST_FLAGS!()`].
    fn get_flags(&self) -> u32;
}

impl ZendClassConstantExt for &zend_class_constant {
    fn get_flags(&self) -> u32 {
        // SAFETY: the u2 union is always a u32 and safe to read
        // ZEND_CLASS_CONST_FLAGS and Z_CONSTANT_FLAGS
        unsafe { self.value.u2.constant_flags }
    }
}
