//! Macros corresponding to `zend_api.h` in php/php-src.

use crate::c_version;

/// Create a namespaced name.
#[doc = c_version!("#define ZEND_NS_NAME(ns, name)            ns \"\\\" name")]
#[macro_export]
macro_rules! ZEND_NS_NAME {
    ($ns:literal, $name:literal) => {
        concat!($ns, "\\", $name)
    };
}

/// Create a [`rsphp_sys::zend_function_entry`] for arg info for a function.
#[doc = c_version!("#define ZEND_RAW_FENTRY(zend_name, name, arg_info, flags, frameless_function_infos, doc_comment)   { zend_name, name, arg_info, (uint32_t) (sizeof(arg_info)/sizeof(struct _zend_internal_arg_info)-1), flags, frameless_function_infos, doc_comment },")]
#[macro_export]
macro_rules! ZEND_RAW_FENTRY {
    ($name:literal, NULL, $arginfo:expr, $flags:expr, $frameless_function_infos:expr, $doc_comment:expr) => {
        rsphp_sys::zend_function_entry {
            fname: $crate::c_str!($name).as_ptr(),
            handler: None,
            arg_info: $arginfo.as_ptr(),
            num_args: ($arginfo.len() - 1) as u32,
            flags: $flags,
            frameless_function_infos: $frameless_function_infos,
            doc_comment: $doc_comment,
        }
    };
    ($name:literal, $handler:expr, $arginfo:expr, $flags:expr, $frameless_function_infos:expr, $doc_comment:expr) => {
        rsphp_sys::zend_function_entry {
            fname: $crate::c_str!($name).as_ptr(),
            handler: Some($handler),
            arg_info: $arginfo.as_ptr(),
            num_args: ($arginfo.len() - 1) as u32,
            flags: $flags,
            frameless_function_infos: $frameless_function_infos,
            doc_comment: $doc_comment,
        }
    };
}

/// Create a [`rsphp_sys::zend_function_entry`] indicating the end of the function list.
#[doc = c_version!("#define ZEND_FE_END            { NULL, NULL, NULL, 0, 0, NULL, NULL }")]
#[macro_export]
macro_rules! ZEND_FE_END {
    () => {
        rsphp_sys::zend_function_entry {
            fname: std::ptr::null(),
            handler: None,
            arg_info: std::ptr::null(),
            num_args: 0,
            flags: 0,
            frameless_function_infos: std::ptr::null(),
            doc_comment: std::ptr::null(),
        }
    };
}

/// Build flags for arg info.
#[doc = c_version!(
    "#define _ZEND_ARG_INFO_FLAGS(pass_by_ref, is_variadic, is_tentative)",
    "(((pass_by_ref) << _ZEND_SEND_MODE_SHIFT) | ((is_variadic) ? _ZEND_IS_VARIADIC_BIT : 0) | ((is_tentative) ? _ZEND_IS_TENTATIVE_BIT : 0))"
)]
#[macro_export]
#[rustfmt::skip]
macro_rules! _ZEND_ARG_INFO_FLAGS {
    (0, 0, 0) => { 0 };
    (0, 0, 1) => { rsphp_sys::_ZEND_IS_TENTATIVE_BIT };
    (0, 1, 0) => { rsphp_sys::_ZEND_IS_VARIADIC_BIT };
    (0, 1, 1) => { rsphp_sys::_ZEND_IS_VARIADIC_BIT | rsphp_sys::_ZEND_IS_TENTATIVE_BIT };
    (1, 0, 0) => { (1 << rsphp_sys::_ZEND_SEND_MODE_SHIFT) };
    (1, 0, 1) => { (1 << rsphp_sys::_ZEND_SEND_MODE_SHIFT) | rsphp_sys::_ZEND_IS_TENTATIVE_BIT };
    (1, 1, 0) => { (1 << rsphp_sys::_ZEND_SEND_MODE_SHIFT) | rsphp_sys::_ZEND_IS_VARIADIC_BIT };
    (1, 1, 1) => {
        (1 << rsphp_sys::_ZEND_SEND_MODE_SHIFT)
            | rsphp_sys::_ZEND_IS_VARIADIC_BIT
            | rsphp_sys::_ZEND_IS_TENTATIVE_BIT
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with no
/// arg/return type information.
#[doc = c_version!(
    "#define ZEND_ARG_INFO(pass_by_ref, name)",
    "{ #name, ZEND_TYPE_INIT_NONE(_ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_INFO {
    ($pass_by_ref:tt, $name:literal) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_NONE!($crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a default value.
#[doc = c_version!(
    "#define ZEND_ARG_INFO_WITH_DEFAULT_VALUE(pass_by_ref, name, default_value)",
    "{ #name, ZEND_TYPE_INIT_NONE(_ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), default_value },"
)]
#[macro_export]
macro_rules! ZEND_ARG_INFO_WITH_DEFAULT_VALUE {
    ($pass_by_ref:tt, $name:literal, $default_value:literal) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_NONE!($crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)),
            default_value: $crate::c_str!($default_value).as_ptr(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with a
/// variadic argument.
#[doc = c_version!(
    "#define ZEND_ARG_VARIADIC_INFO(pass_by_ref, name)",
    "{ #name, ZEND_TYPE_INIT_NONE(_ZEND_ARG_INFO_FLAGS(pass_by_ref, 1, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_VARIADIC_INFO {
    ($pass_by_ref:tt, $name:literal) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_NONE!($crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 1, 0)),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a simple arg/return type.
#[doc = c_version!(
    "#define ZEND_ARG_TYPE_INFO(pass_by_ref, name, type_hint, allow_null)",
    "{ #name, ZEND_TYPE_INIT_CODE(type_hint, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_TYPE_INFO {
    ($pass_by_ref:tt, $name:expr, $type_hint:expr, $allows_null:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CODE!(
                $type_hint,
                $allows_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a simple variadic arg type.
#[doc = c_version!(
    "#define ZEND_ARG_VARIADIC_TYPE_INFO(pass_by_ref, name, type_hint, allow_null)",
    "{ #name, ZEND_TYPE_INIT_CODE(type_hint, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 1, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_VARIADIC_TYPE_INFO {
    ($pass_by_ref:tt, $name:expr, $type_hint:expr, $allows_null:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CODE!(
                $type_hint,
                $allows_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 1, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a simple arg/return type and with a default value.
#[doc = c_version!(
    "#define ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(pass_by_ref, name, type_hint, allow_null, default_value)",
    "{ #name, ZEND_TYPE_INIT_CODE(type_hint, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), default_value },"
)]
#[macro_export]
macro_rules! ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE {
    ($pass_by_ref:tt, $name:literal, $type_hint:ident, $allows_null:tt, $default_value:literal) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            // type_ until https://github.com/rust-lang/rust-bindgen/issues/3333
            type_: $crate::ZEND_TYPE_INIT_CODE!(
                $type_hint,
                $allows_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: $crate::c_str!($default_value).as_ptr(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex arg/return type and with a default value.
#[doc = c_version!(
    "#define ZEND_ARG_OBJ_INFO_WITH_DEFAULT_VALUE(pass_by_ref, name, class_name, allow_null, default_value)",
    "{ #name, ZEND_TYPE_INIT_CLASS_CONST(#class_name, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), default_value },"
)]
#[macro_export]
macro_rules! ZEND_ARG_OBJ_INFO_WITH_DEFAULT_VALUE {
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $allows_null:tt, $default_value:literal) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            // type_ until https://github.com/rust-lang/rust-bindgen/issues/3333
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST!(
                $class_name,
                $allows_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: $crate::c_str!($default_value).as_ptr(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex arg type that does not include objects, and a default value.
#[doc = c_version!(
    "#define ZEND_ARG_TYPE_MASK(pass_by_ref, name, type_mask, default_value)",
    "{ #name, ZEND_TYPE_INIT_MASK(type_mask | _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), default_value },"
)]
#[macro_export]
macro_rules! ZEND_ARG_TYPE_MASK {
    ($pass_by_ref:tt, $name:literal, $type_mask:expr, $default_value:expr) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_MASK!(
                $type_mask | $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: $default_value,
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex arg/return type that does include objects, and a default value.
#[doc = c_version!(
    "#define ZEND_ARG_OBJ_TYPE_MASK(pass_by_ref, name, class_name, type_mask, default_value)",
    "{ #name, ZEND_TYPE_INIT_CLASS_CONST_MASK(#class_name, type_mask | _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), default_value },"
)]
#[macro_export]
macro_rules! ZEND_ARG_OBJ_TYPE_MASK {
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $type_mask:expr, NULL) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST_MASK!(
                $class_name,
                $type_mask | $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $type_mask:expr, $default_value:expr) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST_MASK!(
                $class_name,
                $type_mask | $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: $default_value,
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex variadic arg type that does include objects.
#[doc = c_version!(
    "#define ZEND_ARG_VARIADIC_OBJ_TYPE_MASK(pass_by_ref, name, class_name, type_mask)",
    "{ #name, ZEND_TYPE_INIT_CLASS_CONST_MASK(#class_name, type_mask | _ZEND_ARG_INFO_FLAGS(pass_by_ref, 1, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_VARIADIC_OBJ_TYPE_MASK {
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $type_mask:expr, NULL) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST_MASK!(
                $class_name,
                $type_mask | $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 1, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex arg/return type that only includes a (potentially nullable) object.
#[doc = c_version!(
    "#define ZEND_ARG_OBJ_INFO(pass_by_ref, name, class_name, allow_null)",
    "{ #name, ZEND_TYPE_INIT_CLASS_CONST(#class_name, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 0, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_OBJ_INFO {
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $allow_null:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST!(
                $class_name,
                $allow_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 0, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a [`rsphp_sys::zend_internal_arg_info`] for a global function with
/// a complex variadic arg type that only includes a (potentially nullable) object.
#[doc = c_version!(
    "#define ZEND_ARG_VARIADIC_OBJ_INFO(pass_by_ref, name, class_name, allow_null)",
    "{ #name, ZEND_TYPE_INIT_CLASS_CONST(#class_name, allow_null, _ZEND_ARG_INFO_FLAGS(pass_by_ref, 1, 0)), NULL },"
)]
#[macro_export]
macro_rules! ZEND_ARG_VARIADIC_OBJ_INFO {
    ($pass_by_ref:tt, $name:literal, $class_name:literal, $allow_null:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: $crate::c_str!($name).as_ptr(),
            type_: $crate::ZEND_TYPE_INIT_CLASS_CONST!(
                $class_name,
                $allow_null,
                $crate::_ZEND_ARG_INFO_FLAGS!($pass_by_ref, 1, 0)
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Initialize a [`rsphp_sys::zend_class_entry`] with the given name and set of
/// functions and with the [`rsphp_sys::std_object_handlers`] object handlers.
///
/// # C version
///
/// ```c
/// #define INIT_CLASS_ENTRY(class_container, class_name, functions) \
///     INIT_CLASS_ENTRY_EX(class_container, class_name, strlen(class_name), functions)
/// #define INIT_CLASS_ENTRY_EX(class_container, class_name, class_name_len, functions) \
///     {                                                            \
///         memset(&class_container, 0, sizeof(zend_class_entry)); \
///         class_container.name = zend_string_init_interned(class_name, class_name_len, 1); \
///         class_container.default_object_handlers = &std_object_handlers;    \
///         class_container.info.internal.builtin_functions = functions;    \
///     }
/// ```
#[macro_export]
macro_rules! INIT_CLASS_ENTRY {
    // $class_name might be the result of concat!() from INIT_NS_CLASS_ENTRY
    ($class_container:ident, $class_name:expr, NULL) => {
        $class_container.name = unsafe {
            // Bindgen thinks zend_string_init_interned might be null and so
            // wraps it in an option
            (rsphp_sys::zend_string_init_interned.expect("zend_string_init_interned should be set"))(
                $class_name.as_bytes().as_ptr().cast::<std::ffi::c_char>(),
                $class_name.len(),
                true,
            )
        };
        $class_container.default_object_handlers = unsafe { &rsphp_sys::std_object_handlers };
        unsafe {
            let internal = &mut $class_container.info.internal;
            internal.builtin_functions = std::ptr::null_mut();
        }
    };
    // $class_name might be the result of concat!() from INIT_NS_CLASS_ENTRY
    ($class_container:ident, $class_name:expr, $functions:ident) => {
        $class_container.name = unsafe {
            // Bindgen thinks zend_string_init_interned might be null and so
            // wraps it in an option
            (rsphp_sys::zend_string_init_interned.expect("zend_string_init_interned should be set"))(
                $class_name.as_bytes().as_ptr().cast::<std::ffi::c_char>(),
                $class_name.len(),
                true,
            )
        };
        $class_container.default_object_handlers = unsafe { &rsphp_sys::std_object_handlers };
        unsafe {
            let internal = &mut $class_container.info.internal;
            internal.builtin_functions = $functions.as_ptr();
        }
    };
}

/// Initialize a namespaced class entry.
///
/// See [`INIT_CLASS_ENTRY`] for details.
#[doc = c_version!(
    "#define INIT_NS_CLASS_ENTRY(class_container, ns, class_name, functions)",
    "INIT_CLASS_ENTRY(class_container, ZEND_NS_NAME(ns, class_name), functions)"
)]
#[macro_export]
macro_rules! INIT_NS_CLASS_ENTRY {
    ($class_container:ident, $ns:literal, $class_name:literal, NULL) => {
        $crate::INIT_CLASS_ENTRY!(
            $class_container,
            $crate::ZEND_NS_NAME!($ns, $class_name),
            NULL
        )
    };
    ($class_container:ident, $ns:literal, $class_name:literal, $functions:ident) => {
        $crate::INIT_CLASS_ENTRY!(
            $class_container,
            $crate::ZEND_NS_NAME!($ns, $class_name),
            $functions
        )
    };
}

const _: () = {
    assert!(
        rsphp_sys::ZEND_MAP_PTR_KIND == rsphp_sys::ZEND_MAP_PTR_KIND_PTR_OR_OFFSET,
        "ZEND_MAP_PTR_KIND and ZEND_MAP_PTR_KIND_PTR_OR_OFFSET should match"
    );
};

/// Access the static properties table of a [`rsphp_sys::zend_class_entry`].
#[doc = c_version!("#define CE_STATIC_MEMBERS(ce) ((zval*)ZEND_MAP_PTR_GET((ce)->static_members_table))")]
#[macro_export]
macro_rules! CE_STATIC_MEMBERS {
    ($ce:expr) => { {
        // ZEND_MAP_PTR_GET would do some identity magic with ZEND_MAP_PTR to
        // convert static_members_table to static_members_table__ptr, but rust
        // doesn't support that
        let inner_ptr: *mut rsphp_sys::zval = (*$ce).static_members_table__ptr;
        // # define ZEND_MAP_PTR_IS_OFFSET(ptr) (((uintptr_t)ZEND_MAP_PTR(ptr)) & 1L)
        let is_offset: bool = inner_ptr.addr() & 1 != 0;
        // # define ZEND_MAP_PTR_GET(ptr) \
        //  ((ZEND_MAP_PTR_IS_OFFSET(ptr) ? \
        //  ZEND_MAP_PTR_GET_IMM(ptr) : \
        //  ((void*)(ZEND_MAP_PTR(ptr)))))
        let mapped_ptr: *mut c_void = if is_offset {
            // # define ZEND_MAP_PTR_GET_IMM(ptr) (*ZEND_MAP_PTR_OFFSET2PTR((intptr_t)ZEND_MAP_PTR(ptr)))
            // #define ZEND_MAP_PTR_OFFSET2PTR(offset) ((void**)((char*)CG(map_ptr_base) + offset))
            let base: *mut c_void = $crate::CG!(map_ptr_base);
            let after_offset: *mut c_void = base.byte_add(inner_ptr.addr());
            let result_double_void: *mut *mut c_void = after_offset.cast::<*mut c_void>();
            *result_double_void
        } else {
            inner_ptr.cast::<c_void>()
        };
        mapped_ptr.cast::<rsphp_sys::zval>()
    } };
}

/// Access the class constants table of a [`rsphp_sys::zend_class_entry`].
#[doc = c_version!("#define CE_CONSTANTS_TABLE(ce) zend_class_constants_table(ce)")]
#[macro_export]
macro_rules! CE_CONSTANTS_TABLE {
    ($ce:expr) => {
        rsphp_sys::zend_class_constants_table($ce)
    };
}

/// Initialize a [`rsphp_sys::zval`] to be an empty array.
#[doc = c_version!("#define array_init(arg)                ZVAL_ARR((arg), zend_new_array(0))")]
#[macro_export]
macro_rules! array_init {
    ($arg:expr) => {
        $crate::ZVAL_ARR!($arg, rsphp_sys::_zend_new_array(0))
    };
}

/// Set a [`rsphp_sys::zval`] to be a new string with the given length.
///
/// # C version
///
/// ```c
/// #define ZVAL_STRINGL(z, s, l) do {                \
///         ZVAL_NEW_STR(z, zend_string_init(s, l, 0));        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_STRINGL {
    ($z:expr, $s:expr, $l:expr) => {
        $crate::ZVAL_NEW_STR!($z, rsphp_sys::zend_string_init($s, $l, false))
    };
}

/// Set a [`rsphp_sys::zval`] to be a new string with the length computed.
///
/// # C version
///
/// ```c
/// ZVAL_STRING(z, s) do {                    \
///         const char *_s = (s);                    \
///         ZVAL_STRINGL(z, _s, strlen(_s));        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_STRING {
    ($z:expr, $s:expr) => {
        $crate::ZVAL_STRINGL!($z, $s, unsafe {
            std::ffi::CStr::from_ptr($s).count_bytes()
        })
    };
}

/// Set a [`rsphp_sys::zval`] to be an empty string.
///
/// # C version
///
/// ```c
/// #define ZVAL_EMPTY_STRING(z) do {                \
///         ZVAL_INTERNED_STR(z, ZSTR_EMPTY_ALLOC());        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_EMPTY_STRING {
    ($z:expr) => {
        $crate::ZVAL_INTERNED_STR!($z, rsphp_sys::zend_empty_string)
    };
}

/// Verify that the `exception` is set in [`rsphp_sys::executor_globals`] and
/// return.
#[doc = c_version!("#define RETURN_THROWS()                    do { ZEND_ASSERT(EG(exception)); (void) return_value; return; } while (0)")]
#[macro_export]
macro_rules! RETURN_THROWS {
    () => {
        // replacing ZEND_ASSERT with a rust assert
        assert!(!$crate::EG!(exception).is_null());
        // ().into() satisfies both unit returns and nice handlers with
        // RSZValue returns
        return ().into();
    };
}
