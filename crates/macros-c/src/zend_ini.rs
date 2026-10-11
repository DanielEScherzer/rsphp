//! Macros corresponding to `zend_ini.h` in php/php-src.

use crate::{c_differences, c_version};

/// End a list of INI entry definitions.
#[doc = c_version!("#define ZEND_INI_END()        { NULL, NULL, NULL, NULL, NULL, NULL, NULL, 0, 0, 0} };")]
#[doc = c_differences!("Unlike the C version, this macro does not end the \
    overall list of INI entries being defined; such a macro is unhygienic.")]
#[macro_export]
macro_rules! ZEND_INI_END {
    () => {
        rsphp_sys::zend_ini_entry_def {
            name: std::ptr::null(),
            on_modify: None,
            mh_arg1: std::ptr::null_mut(),
            mh_arg2: std::ptr::null_mut(),
            mh_arg3: std::ptr::null_mut(),
            value: std::ptr::null(),
            displayer: None,
            value_length: 0,
            name_length: 0,
            modifiable: 0,
        }
    };
}

/// Set up an INI entry definition (3 argument displayer variant).
#[doc = c_version!(
    "#define ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, arg1, arg2, arg3, displayer)",
    "{ name, on_modify, arg1, arg2, arg3, default_value, displayer, sizeof(default_value)-1, sizeof(name)-1, modifiable },"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY3_EX {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $arg1:expr,
        $arg2:expr,
        $arg3:expr,
        $displayer:expr $(,)?
    ) => {
        rsphp_sys::zend_ini_entry_def {
            name: $crate::c_str!($name).as_ptr(),
            on_modify: $on_modify,
            mh_arg1: $arg1,
            mh_arg2: $arg2,
            mh_arg3: $arg3,
            value: $crate::c_str!($default_value).as_ptr(),
            displayer: $displayer,
            value_length: $crate::c_str!($default_value).count_bytes() as u32,
            name_length: $crate::c_str!($name).count_bytes() as u16,
            modifiable: $modifiable,
        }
    };
}

/// Set up an INI entry definition (3 argument no displayer variant)
#[doc = c_version!(
    "#define ZEND_INI_ENTRY3(name, default_value, modifiable, on_modify, arg1, arg2, arg3)",
    "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, arg1, arg2, arg3, NULL)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY3 {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $arg1:expr,
        $arg2:expr,
        $arg3:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY3_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            $arg1,
            $arg2,
            $arg3,
            None,
        )
    };
}

/// Set up an INI entry definition (2 argument displayer variant).
#[doc = c_version!(
    "#define ZEND_INI_ENTRY2_EX(name, default_value, modifiable, on_modify, arg1, arg2, displayer)",
    "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, arg1, arg2, NULL, displayer)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY2_EX {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:ident,
        $arg1:expr,
        $arg2:expr,
        $displayer:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY3_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            $arg1,
            $arg2,
            std::ptr::null_mut(),
            $displayer,
        )
    };
}

/// Set up an INI entry definition (2 argument no displayer variant).
#[doc = c_version!(
    "#define ZEND_INI_ENTRY2(name, default_value, modifiable, on_modify, arg1, arg2)",
    "ZEND_INI_ENTRY2_EX(name, default_value, modifiable, on_modify, arg1, arg2, NULL)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY2 {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $arg1:expr,
        $arg2:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY2_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            $arg1,
            $arg2,
            None,
        )
    };
}

/// Set up an INI entry definition (1 argument displayer variant).
#[doc = c_version!(
    "#define ZEND_INI_ENTRY1_EX(name, default_value, modifiable, on_modify, arg1, displayer)",
    "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, arg1, NULL, NULL, displayer)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY1_EX {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $arg1:expr,
        $displayer:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY3_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            $arg1,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            $displayer,
        )
    };
}

/// Set up an INI entry definition (1 argument no displayer variant).
#[doc = c_version!(
    "#define ZEND_INI_ENTRY1(name, default_value, modifiable, on_modify, arg1)",
    "ZEND_INI_ENTRY1_EX(name, default_value, modifiable, on_modify, arg1, NULL)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY1 {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $arg1:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY1_EX!($name, $default_value, $modifiable, $on_modify, $arg1, None,)
    };
}

/// Set up an INI entry definition (no arguments displayer variant)
#[doc = c_version!(
    "#define ZEND_INI_ENTRY_EX(name, default_value, modifiable, on_modify, displayer)",
    "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, NULL, NULL, NULL, displayer)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY_EX {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $displayer:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY3_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            $displayer,
        )
    };
}

/// Set up an INI entry definition (no arguments no displayer variant)
#[doc = c_version!(
    "#define ZEND_INI_ENTRY(name, default_value, modifiable, on_modify)",
    "ZEND_INI_ENTRY_EX(name, default_value, modifiable, on_modify, NULL)"
)]
#[macro_export]
macro_rules! ZEND_INI_ENTRY {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY_EX!($name, $default_value, $modifiable, $on_modify, None,)
    };
}

/// Set up an INI entry definition for a boolean option.
#[doc = c_version!(
    NTS => [
        "#define STD_ZEND_INI_BOOLEAN(name, default_value, modifiable, on_modify, property_name, struct_type, struct_ptr)",
        "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, (void *) XtOffsetOf(struct_type, property_name), (void *) &struct_ptr, NULL, zend_ini_boolean_displayer_cb)",
    ],
    ZTS => [
        "#define STD_ZEND_INI_BOOLEAN(name, default_value, modifiable, on_modify, property_name, struct_type, struct_ptr)",
        "ZEND_INI_ENTRY3_EX(name, default_value, modifiable, on_modify, (void *) XtOffsetOf(struct_type, property_name), (void *) &struct_ptr##_id, NULL, zend_ini_boolean_displayer_cb)",
    ],
)]
#[macro_export]
macro_rules! STD_ZEND_INI_BOOLEAN {
    (
        $name:literal,
        $default_value:literal,
        $modifiable:expr,
        $on_modify:expr,
        $property_name:expr,
        $struct_type:ty,
        $struct_ptr:expr $(,)?
    ) => {
        $crate::ZEND_INI_ENTRY3_EX!(
            $name,
            $default_value,
            $modifiable,
            $on_modify,
            std::mem::offset_of!($struct_type, $property_name) as *mut _,
            unsafe { &raw mut $struct_ptr }.cast::<std::ffi::c_void>(),
            std::ptr::null_mut(),
            Some(rsphp_sys::zend_ini_boolean_displayer_cb),
        )
    };
}

/// Get the current version of an INI value, as a [`rsphp_sys::zend_long`].
#[doc = c_version!("#define INI_INT(name) zend_ini_long((name), strlen(name), 0)")]
#[macro_export]
macro_rules! INI_INT {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_long call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_long(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                0,
            )
        }
    };
}
/// Get the current version of an INI value, as an f64.
#[doc = c_version!("#define INI_FLT(name) zend_ini_double((name), strlen(name), 0)")]
#[macro_export]
macro_rules! INI_FLT {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_double call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_double(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                0,
            )
        }
    };
}
/// Get the current version of an INI value, as a character pointer.
#[doc = c_version!("#define INI_STR(name) zend_ini_string_ex((name), strlen(name), 0, NULL)")]
#[macro_export]
macro_rules! INI_STR {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_string_ex call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_string_ex(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                0,
                std::ptr::null_mut(),
            )
        }
    };
}
/// Get the current version of an INI value, as a boolean.
#[doc = c_version!("#define INI_BOOL(name) ((bool) INI_INT(name))")]
#[macro_export]
macro_rules! INI_BOOL {
    ($name:literal) => {
        $crate::INI_INT!($name) != 0
    };
}

/// Get the original version of an INI value, as a [`rsphp_sys::zend_long`].
#[doc = c_version!("#define INI_ORIG_INT(name)    zend_ini_long((name), strlen(name), 1)")]
#[macro_export]
macro_rules! INI_ORIG_INT {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_long call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_long(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                1,
            )
        }
    };
}
/// Get the original version of an INI value, as an f64.
#[doc = c_version!("#define INI_ORIG_FLT(name)    zend_ini_double((name), strlen(name), 1)")]
#[macro_export]
macro_rules! INI_ORIG_FLT {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_double call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_double(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                1,
            )
        }
    };
}
/// Get the original version of an INI value, as a character pointer.
#[doc = c_version!("#define INI_ORIG_STR(name)    zend_ini_string((name), strlen(name), 1)")]
#[macro_export]
macro_rules! INI_ORIG_STR {
    ($name:literal) => {
        // SAFETY: name does not need to be valid for the zend_ini_string call,
        // length provided is correct
        unsafe {
            rsphp_sys::zend_ini_string(
                $crate::c_str!($name).as_ptr(),
                $crate::c_str!($name).count_bytes(),
                1,
            )
        }
    };
}
/// Get the original version of an INI value, as a boolean.
#[doc = c_version!("#define INI_ORIG_BOOL(name) ((bool) INI_ORIG_INT(name))")]
#[macro_export]
macro_rules! INI_ORIG_BOOL {
    ($name:literal) => {
        $crate::INI_ORIG_INT!($name) != 0
    };
}
