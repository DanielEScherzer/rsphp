//! Extension trait for [`rsphp_sys::zval`].
use crate::PHPValue;
use rsphp_sys::zval;

pub trait ZvalExt {
    fn set_value<T>(&mut self, value: T)
    where
        T: Into<PHPValue>;
}

impl ZvalExt for zval {
    fn set_value<T>(&mut self, value: T)
    where
        T: Into<PHPValue>,
    {
        let php_value: PHPValue = value.into();
        // SAFETY: we have a valid zval
        unsafe {
            php_value.apply_to(self);
        }
    }
}
