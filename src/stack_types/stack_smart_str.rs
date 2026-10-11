use rsphp_macros_c::{ZSTR_LEN, ZSTR_VAL};
use rsphp_sys::{
    smart_str, smart_str_0, smart_str_extract, smart_str_free, zend_string, zend_string_release,
};
use std::ops::Deref;

pub struct StackSmartStr {
    /// The wrapped smart string.
    inner: smart_str,
}

/// Rust handling of stack-allocated [`rsphp_sys::smart_str`] values.
///
/// Can be dereferenced into the underlying `smart_str`.
///
/// Calls [`rsphp_sys::smart_str_free`] automatically on drop.
///
/// # Panics
///
/// Panics when extracting a string that is not valid UTF8.
#[expect(clippy::new_without_default, reason = "Default is not appropriate")]
impl StackSmartStr {
    /// Create a new empty [`rsphp_sys::smart_str`].
    #[must_use]
    pub fn new() -> Self {
        StackSmartStr {
            // SAFETY: smart_str can be intialized with zero bytes
            inner: unsafe { std::mem::zeroed() },
        }
    }

    /// Ensure that the underlying smart string ends with a null byte.
    pub fn ensure_ending_nul(&mut self) {
        // SAFETY: inner is a valid smart string
        unsafe {
            smart_str_0(self.mut_ptr());
        }
    }

    /// Extract into a string, which must be valid UTF8.
    ///
    /// # Panics
    ///
    /// Panics if the held string is not valid UTF8.
    pub fn extract(&mut self) -> String {
        // SAFETY: inner is a valid smart string
        let string_ptr: *mut zend_string = unsafe { smart_str_extract(&raw mut self.inner) };
        // SAFETY: smart_str_extract() returns a valid zend_string
        let char_ptr = unsafe { ZSTR_VAL!(string_ptr) };
        // SAFETY: smart_str_extract() returns a valid zend_string
        let str_len = unsafe { ZSTR_LEN!(string_ptr) };
        // SAFETY: conversion from zend_string based on its storage semantics
        let str_slice: &[u8] =
            unsafe { std::slice::from_raw_parts(char_ptr.cast::<u8>(), str_len) };
        let str_raw = str::from_utf8(str_slice).expect("Should only be used for valid UTF8");
        let result = str_raw.to_owned();
        // Got to free the zend_string pointer though
        // SAFETY: smart_str_extract() returns a valid zend_string
        unsafe { zend_string_release(string_ptr) };
        result
    }

    /// Access a mutable pointer to the contained instance, for passing to FFI.
    pub fn mut_ptr(&mut self) -> *mut smart_str {
        std::ptr::from_mut(&mut self.inner)
    }
}

impl Drop for StackSmartStr {
    fn drop(&mut self) {
        // SAFETY: inner is a valid smart string
        unsafe { smart_str_free(std::ptr::from_mut(&mut self.inner)) };
    }
}

impl Deref for StackSmartStr {
    type Target = smart_str;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
