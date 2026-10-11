//! Extension trait for [`rsphp_sys::zend_string`].
use rsphp_macros_c::{ZSTR_LEN, ZSTR_VAL};
use rsphp_sys::zend_string;

pub trait ZendStringExt {
    fn as_slice(&self) -> &[u8];
    #[cfg(feature = "bstr")]
    fn as_bstr(&self) -> &bstr::BStr {
        bstr::BStr::new(self.as_slice())
    }
}

impl ZendStringExt for &zend_string {
    fn as_slice(&self) -> &[u8] {
        let char_ptr = ZSTR_VAL!((*self));
        let str_len = ZSTR_LEN!((*self));
        // SAFETY: conversion from zend_string based on its storage semantics
        unsafe { std::slice::from_raw_parts(char_ptr.cast::<u8>(), str_len) }
    }
}

impl ZendStringExt for &mut zend_string {
    fn as_slice(&self) -> &[u8] {
        // SAFETY: conversion from zend_string based on its storage semantics
        unsafe { std::slice::from_raw_parts(ZSTR_VAL!((*self)).cast::<u8>(), ZSTR_LEN!((*self))) }
    }
}
