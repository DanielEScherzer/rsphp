//! Extension trait for [`rsphp_sys::zend_constant`].
use rsphp_sys::zend_constant;

/// Extension trait for [`rsphp_sys::zend_constant`].
pub trait ZendConstantExt {
    /// Get the flags of the constant.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_CONSTANT_FLAGS!()`].
    fn get_flags(&self) -> u32;

    /// Get the module number of the constant.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_CONSTANT_MODULE_NUMBER!()`].
    fn get_module_number(&self) -> u32;
}

impl ZendConstantExt for &zend_constant {
    fn get_flags(&self) -> u32 {
        // SAFETY: the u2 union is always a u32 and safe to read
        // ZEND_CONSTANT_FLAGS and Z_CONSTANT_FLAGS
        (unsafe { self.value.u2.constant_flags }) & 0xFF
    }
    fn get_module_number(&self) -> u32 {
        // SAFETY: the u2 union is always a u32 and safe to read
        // ZEND_CONSTANT_MODULE_NUMBER and Z_CONSTANT_FLAGS
        (unsafe { self.value.u2.constant_flags }) >> 8
    }
}

impl ZendConstantExt for &mut zend_constant {
    fn get_flags(&self) -> u32 {
        // SAFETY: the u2 union is always a u32 and safe to read
        // ZEND_CONSTANT_FLAGS and Z_CONSTANT_FLAGS
        (unsafe { self.value.u2.constant_flags }) & 0xFF
    }
    fn get_module_number(&self) -> u32 {
        // SAFETY: the u2 union is always a u32 and safe to read
        // ZEND_CONSTANT_MODULE_NUMBER and Z_CONSTANT_FLAGS
        (unsafe { self.value.u2.constant_flags }) >> 8
    }
}
