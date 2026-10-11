#[cfg(feature = "bstr")]
use bstr::BStr;
use rsphp_sys::{zend_string, zend_string_init, zend_string_release_ex};
use std::ffi::c_char;
use std::ops::Deref;

pub struct StackZendString {
    /// The wrapped string.
    inner: *mut zend_string,
}

/// Rust handling of Rust-tracked [`rsphp_sys::zend_string`] pointer.
///
/// Can be dereferenced into the underlying `zend_string`.
///
/// Calls [`rsphp_sys::zend_string_release_ex`] automatically on drop.
impl StackZendString {
    /// Create a new [`rsphp_sys::zend_string`] for the given `&str`.
    #[must_use]
    pub fn new(string: &str) -> Self {
        StackZendString {
            // SAFETY: length matches the number of bytes given
            inner: unsafe {
                zend_string_init(string.as_ptr().cast::<c_char>(), string.len(), false)
            },
        }
    }
    /// Create a new [`rsphp_sys::zend_string`] for the given `BStr` or `BString`.
    #[must_use]
    #[cfg(feature = "bstr")]
    pub fn from_bstring<B>(bstr: B) -> Self
    where
        B: AsRef<BStr>,
    {
        let bstr_ref: &BStr = bstr.as_ref();
        StackZendString {
            // SAFETY: length matches the number of bytes given
            inner: unsafe {
                zend_string_init(bstr_ref.as_ptr().cast::<c_char>(), bstr_ref.len(), false)
            },
        }
    }

    /// Access a mutable pointer to the contained instance, for passing to FFI.
    pub fn mut_ptr(&mut self) -> *mut zend_string {
        self.inner
    }
}

impl Drop for StackZendString {
    fn drop(&mut self) {
        // SAFETY: inner is a valid zend_string pointer
        unsafe { zend_string_release_ex(self.inner, false) };
    }
}

impl Deref for StackZendString {
    type Target = *mut zend_string;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
