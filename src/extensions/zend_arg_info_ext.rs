//! Extension trait for [`rsphp_sys::zend_arg_info`].
use crate::extensions::ZendTypeExt as _;
use rsphp_sys::{
    _ZEND_IS_PROMOTED_BIT, _ZEND_IS_TENTATIVE_BIT, _ZEND_IS_VARIADIC_BIT, _ZEND_SEND_MODE_SHIFT,
    zend_arg_info,
};

pub trait ZendArgInfoExt {
    fn send_mode(&self) -> u8;
    fn is_variadic(&self) -> bool;
    fn is_promoted(&self) -> bool;
    fn is_tentative(&self) -> bool;
}

impl ZendArgInfoExt for &zend_arg_info {
    fn send_mode(&self) -> u8 {
        ((self.type_.full_mask() >> _ZEND_SEND_MODE_SHIFT) & 3)
            .try_into()
            .expect("(_ & 3) is valid u8")
    }

    fn is_variadic(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_VARIADIC_BIT != 0
    }

    fn is_promoted(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_PROMOTED_BIT != 0
    }

    fn is_tentative(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_TENTATIVE_BIT != 0
    }
}

impl ZendArgInfoExt for &mut zend_arg_info {
    fn send_mode(&self) -> u8 {
        ((self.type_.full_mask() >> _ZEND_SEND_MODE_SHIFT) & 3)
            .try_into()
            .expect("(_ & 3) is valid u8")
    }

    fn is_variadic(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_VARIADIC_BIT != 0
    }

    fn is_promoted(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_PROMOTED_BIT != 0
    }

    fn is_tentative(&self) -> bool {
        self.type_.full_mask() & _ZEND_IS_TENTATIVE_BIT != 0
    }
}
