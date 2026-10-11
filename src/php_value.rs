use rsphp_macros_c::{
    ZVAL_DOUBLE, ZVAL_EMPTY_STRING, ZVAL_FALSE, ZVAL_LONG, ZVAL_NULL, ZVAL_OBJ, ZVAL_STR,
    ZVAL_STR_COPY, ZVAL_STRINGL, ZVAL_TRUE, ZVAL_UNDEF,
};
use rsphp_sys::{zend_long, zend_string, zval};
use std::ffi::CString;

use crate::PHPObject;

/// Sources of strings for PHP values.
enum PHPStringSource {
    /// Empty string, special cased because it doesn't require a new allocation.
    EmptyString,
    /// String constructor, based on a rust string to convert.
    RustString(String),
    /// String constructor, based on an existing `zend_string` that should not
    /// have its reference count increased.
    ZendStringWithoutCopy(*mut zend_string),
    /// String constructor, based on an existing `zend_string` that should have
    /// its reference count increased.
    ZendStringToCopy(*mut zend_string),
    /// String constructor, based on a `CString` to convert.
    CString(CString),
    #[cfg(feature = "bstr")]
    /// String constructor, based on a binary string.
    BString(bstr::BString),
}

/// Helper to force all string constructions to go through the API of this crate
/// rather than being constructed manually.
pub struct PHPStringWrapper {
    /// The actual string.
    inner: PHPStringSource,
}

/// A value to apply to a PHP `zval`.
pub enum PHPValue {
    /// None - do not change the value.
    _None,
    /// Undef - mark the value as undefined.
    Undef,
    /// Null - mark the value as null.
    Null,
    /// False - mark the value as false.
    False,
    /// True - mark the value as true.
    True,
    /// Long - mark the value as the given integer.
    Long(zend_long),
    /// Double - mark the value as the given floating point value.
    Double(f64),
    /// String - mark the value as holding the given string.
    String(PHPStringWrapper),
    /// Object - mark the value as holding the given object.
    Object(PHPObject),
}

impl PHPValue {
    /// Write the current value to the target zval, overwriting any existing
    /// value.
    ///
    /// # Safety
    ///
    /// The provided zval pointer must be safe to write to.
    pub unsafe fn apply_to(self, zval_p: *mut zval) {
        // For some reason the following unsafe blocks are not flagged as
        // unneeded, but *also* removing them doesn't result in errors about
        // them being needed, not sure why.
        match self {
            PHPValue::_None => (),
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::Undef => unsafe {
                ZVAL_UNDEF!(zval_p);
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::Null => unsafe {
                ZVAL_NULL!(zval_p);
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::False => unsafe {
                ZVAL_FALSE!(zval_p);
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::True => unsafe {
                ZVAL_TRUE!(zval_p);
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::Long(long) => unsafe {
                ZVAL_LONG!(zval_p, long);
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::Double(double) => unsafe {
                ZVAL_DOUBLE!(zval_p, double);
            },
            PHPValue::String(string_wrapper) => match string_wrapper.inner {
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::EmptyString => unsafe { ZVAL_EMPTY_STRING!(zval_p) },
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::RustString(string) => unsafe {
                    ZVAL_STRINGL!(zval_p, string.as_ptr().cast::<i8>(), string.len());
                },
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::ZendStringToCopy(str_ptr) => unsafe {
                    ZVAL_STR_COPY!(zval_p, str_ptr);
                },
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::ZendStringWithoutCopy(str_ptr) => unsafe {
                    ZVAL_STR!(zval_p, str_ptr);
                },
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::CString(string) => unsafe {
                    ZVAL_STRINGL!(zval_p, string.as_ptr().cast::<i8>(), string.count_bytes());
                },
                #[cfg(feature = "bstr")]
                // SAFETY: pointer must be valid, upheld by caller
                PHPStringSource::BString(string) => unsafe {
                    ZVAL_STRINGL!(zval_p, string.as_ptr().cast::<i8>(), string.len())
                },
            },
            // SAFETY: pointer must be valid, upheld by caller
            PHPValue::Object(object) => unsafe {
                ZVAL_OBJ!(zval_p, object.ptr());
            },
        }
    }

    /// Create a `PHPValue` indicating that a string should be copied.
    ///
    /// # Safety
    ///
    /// The string pointer must be valid to write to.
    ///
    /// # Panics
    ///
    /// Panics if the provided pointer is null.
    pub unsafe fn copy_string(string: *mut zend_string) -> Self {
        assert!(!string.is_null(), "String must not be null");
        PHPValue::String(PHPStringWrapper {
            inner: PHPStringSource::ZendStringToCopy(string),
        })
    }

    /// Create a `PHPValue` indicating that a string should be used but without
    /// adding another reference count.
    ///
    /// # Safety
    ///
    /// The string pointer must be valid to write to, and must become owned
    /// by the `PHPValue`.
    ///
    /// # Panics
    ///
    /// Panics if the provided pointer is null.
    pub unsafe fn no_copy_string(string: *mut zend_string) -> Self {
        assert!(!string.is_null(), "String must not be null");
        PHPValue::String(PHPStringWrapper {
            inner: PHPStringSource::ZendStringWithoutCopy(string),
        })
    }
}

impl From<bool> for PHPValue {
    fn from(value: bool) -> Self {
        if value {
            PHPValue::True
        } else {
            PHPValue::False
        }
    }
}

impl From<()> for PHPValue {
    fn from(_v: ()) -> Self {
        PHPValue::_None
    }
}

impl From<&str> for PHPValue {
    fn from(string: &str) -> Self {
        if string.is_empty() {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::EmptyString,
            })
        } else {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::RustString(string.to_owned()),
            })
        }
    }
}

impl From<String> for PHPValue {
    fn from(string: String) -> Self {
        if string.is_empty() {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::EmptyString,
            })
        } else {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::RustString(string),
            })
        }
    }
}

impl From<CString> for PHPValue {
    fn from(string: CString) -> Self {
        if string.is_empty() {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::EmptyString,
            })
        } else {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::CString(string),
            })
        }
    }
}

impl From<PHPObject> for PHPValue {
    fn from(object: PHPObject) -> Self {
        PHPValue::Object(object)
    }
}

#[cfg(feature = "bstr")]
impl From<bstr::BString> for PHPValue {
    fn from(s: bstr::BString) -> Self {
        if s.is_empty() {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::EmptyString,
            })
        } else {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::BString(s),
            })
        }
    }
}

#[cfg(feature = "bstr")]
impl From<&bstr::BStr> for PHPValue {
    fn from(s: &bstr::BStr) -> Self {
        if s.is_empty() {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::EmptyString,
            })
        } else {
            PHPValue::String(PHPStringWrapper {
                inner: PHPStringSource::BString(s.into()),
            })
        }
    }
}

/// Generate a `From` implementation for the integer type that just uses
/// `i64::from()`.
macro_rules! from_int_as {
    ( $( $type:ty ),+ ) => { $(
        impl From<$type> for PHPValue {
            fn from(value: $type) -> Self {
                PHPValue::Long(i64::from(value))
            }
        }
    )+ }
}

from_int_as!(u8, u16, u32, i8, i16, i32);
