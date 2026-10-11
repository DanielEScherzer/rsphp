//! Macros corresponding to `zend_string.h` in php/php-src.

use crate::c_version;

/// Access the underlying value of a [`rsphp_sys::zend_string`].
#[doc = c_version!("#define ZSTR_VAL(zstr)  (zstr)->val")]
#[macro_export]
macro_rules! ZSTR_VAL {
    ($zstr:expr) => {
        (*$zstr).val.as_ptr()
    };
}

/// Access the underlying length of a [`rsphp_sys::zend_string`].
#[doc = c_version!("#define ZSTR_LEN(zstr)  (zstr)->len")]
#[macro_export]
macro_rules! ZSTR_LEN {
    ($zstr:expr) => {
        (*$zstr).len
    };
}

/// Check if a [`rsphp_sys::zend_string`] is interned.
#[doc = c_version!("#define ZSTR_IS_INTERNED(s)                    (GC_FLAGS(s) & IS_STR_INTERNED)")]
#[macro_export]
macro_rules! ZSTR_IS_INTERNED {
    ($s:expr) => {
        ($crate::GC_FLAGS!($s) & rsphp_sys::IS_STR_INTERNED) != 0
    };
}

/// Check if two [`rsphp_sys::zend_string`]s are equal in a case-insensitive
/// manner.
#[doc = c_version!(
    "#define zend_string_equals_ci(s1, s2)",
    "(ZSTR_LEN(s1) == ZSTR_LEN(s2) && !zend_binary_strcasecmp(ZSTR_VAL(s1), ZSTR_LEN(s1), ZSTR_VAL(s2), ZSTR_LEN(s2)))"
)]
#[macro_export]
macro_rules! zend_string_equals_ci {
    ($s1:expr, $s2:expr) => {
        $crate::ZSTR_LEN!($s1) == $crate::ZSTR_LEN!($s2)
            && rsphp_sys::zend_binary_strcasecmp(
                $crate::ZSTR_VAL!($s1),
                $crate::ZSTR_LEN!($s1),
                $crate::ZSTR_VAL!($s2),
                $crate::ZSTR_LEN!($s2),
            ) == 0
    };
}

/// Get a [`rsphp_sys::zend_string`] from a known string id.
#[doc = c_version!("#define ZSTR_KNOWN(idx) zend_known_strings[idx]")]
#[macro_export]
macro_rules! ZSTR_KNOWN {
    ($idx:expr) => {{
        let known_strings_start: *mut *mut rsphp_sys::zend_string =
            unsafe { rsphp_sys::zend_known_strings };
        let result: *mut rsphp_sys::zend_string =
            unsafe { *(known_strings_start.add($idx as usize)) };
        result
    }};
}
