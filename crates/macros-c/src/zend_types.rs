//! Macros corresponding to `zend_types.h` in php/php-src.

use crate::{c_differences, c_version};

/// Check if a [`rsphp_sys::zend_type`] has any type mask set.
#[doc = c_version!("#define ZEND_TYPE_IS_SET(t) (((t).type_mask & _ZEND_TYPE_MASK) != 0)")]
#[macro_export]
macro_rules! ZEND_TYPE_IS_SET {
    ($t:expr) => {
        $t.type_mask & rsphp_sys::_ZEND_TYPE_MASK != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] has a type mask that only refers to
/// standard types.
#[doc = c_version!("#define ZEND_TYPE_IS_ONLY_MASK(t) (ZEND_TYPE_IS_SET(t) && (t).ptr == NULL)")]
#[macro_export]
macro_rules! ZEND_TYPE_IS_ONLY_MASK {
    ($t:expr) => {
        $crate::ZEND_TYPE_IS_SET!($t) && $t.ptr.is_null()
    };
}

/// Check if a [`rsphp_sys::zend_type`] is complex (a list with a union or
/// intersection or a pointer to a class name).
#[doc = c_version!(
    "#define ZEND_TYPE_IS_COMPLEX(t)",
    "((((t).type_mask) & _ZEND_TYPE_KIND_MASK) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_IS_COMPLEX {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_KIND_MASK) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] has a name ([`rsphp_sys::zend_string`] pointer).
#[doc = c_version!(
    "#define ZEND_TYPE_HAS_NAME(t)",
    "((((t).type_mask) & _ZEND_TYPE_NAME_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_HAS_NAME {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_NAME_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] has a literal name ([`std::ffi::c_char`] pointer).
#[doc = c_version!(
    "#define ZEND_TYPE_HAS_LITERAL_NAME(t)",
    "((((t).type_mask) & _ZEND_TYPE_LITERAL_NAME_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_HAS_LITERAL_NAME {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_LITERAL_NAME_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] has a list.
#[doc = c_version!(
    "#define ZEND_TYPE_HAS_LIST(t)",
    "((((t).type_mask) & _ZEND_TYPE_LIST_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_HAS_LIST {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_LIST_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] is the iterator fallback.
#[doc = c_version!(
    "#define ZEND_TYPE_IS_ITERABLE_FALLBACK(t)",
    "((((t).type_mask) & _ZEND_TYPE_ITERABLE_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_IS_ITERABLE_FALLBACK {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_ITERABLE_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] is an intersection.
#[doc = c_version!(
    "#define ZEND_TYPE_IS_INTERSECTION(t)",
    "((((t).type_mask) & _ZEND_TYPE_INTERSECTION_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_IS_INTERSECTION {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_INTERSECTION_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] is a union.
#[doc = c_version!(
    "#define ZEND_TYPE_IS_UNION(t)",
    "((((t).type_mask) & _ZEND_TYPE_UNION_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_IS_UNION {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_UNION_BIT) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] is arena allocated.
#[doc = c_version!(
    "#define ZEND_TYPE_USES_ARENA(t)",
    "((((t).type_mask) & _ZEND_TYPE_ARENA_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_USES_ARENA {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_ARENA_BIT) != 0
    };
}

/// Access the type name of a [`rsphp_sys::zend_type`].
#[doc = c_version!("#define ZEND_TYPE_NAME(t) ((zend_string *) (t).ptr)")]
#[macro_export]
macro_rules! ZEND_TYPE_NAME {
    ($t:expr) => {
        $t.ptr.cast::<rsphp_sys::zend_string>()
    };
}

/// Access the literal type name of a [`rsphp_sys::zend_type`].
#[doc = c_version!("#define ZEND_TYPE_LITERAL_NAME(t) ((const char *) (t).ptr)")]
#[macro_export]
macro_rules! ZEND_TYPE_LITERAL_NAME {
    ($t:expr) => {
        $t.ptr.cast::<const std::ffi::char>()
    };
}

/// Access the list ([`rsphp_sys::zend_type_list`]) contained in a
/// [`rsphp_sys::zend_type`].
#[doc = c_version!("#define ZEND_TYPE_LIST(t) ((zend_type_list *) (t).ptr)")]
#[macro_export]
macro_rules! ZEND_TYPE_LIST {
    ($t:expr) => {
        $t.ptr.cast::<rsphp_sys::zend_type_list>()
    };
}

/// Identify the size of a [`rsphp_sys::zend_type_list`] using the struct hack.
#[doc = c_version!(
    "#define ZEND_TYPE_LIST_SIZE(num_types)",
    "(sizeof(zend_type_list) + ((num_types) - 1) * sizeof(zend_type))"
)]
#[macro_export]
macro_rules! ZEND_TYPE_LIST_SIZE {
    ($num_types:expr) => {
        std::mem::size_of::<rsphp_sys::zend_type_list>()
            + (($num_types - 1) * std::mem::size_of::<rsphp_sys::zend_type>())
    };
}

/// Set the pointer of a [`rsphp_sys::zend_type`].
#[doc = c_version!(
    "#define ZEND_TYPE_SET_PTR(t, _ptr)",
    "((t).ptr = (_ptr))"
)]
#[macro_export]
macro_rules! ZEND_TYPE_SET_PTR {
    ($t:expr, $ptr:expr) => {
        $t.ptr = $ptr.cast::<std::ffi::c_void>();
    };
}

/// Set the pointer and kind of a [`rsphp_sys::zend_type`].
///
/// # C version
///
/// ```c
/// #define ZEND_TYPE_SET_PTR_AND_KIND(t, _ptr, kind_bit) do { \
///     (t).ptr = (_ptr); \
///     (t).type_mask &= ~_ZEND_TYPE_KIND_MASK; \
///     (t).type_mask |= (kind_bit); \
/// } while (0)
/// ```
#[macro_export]
macro_rules! ZEND_TYPE_SET_PTR_AND_KIND {
    ($t:expr, $ptr:expr, $kind_bit:expr) => {{
        $t.ptr = $ptr.cast::<std::ffi::c_void>();
        $t.type_mask &= !(rsphp_sys::_ZEND_TYPE_KIND_MASK);
        $t.type_mask |= $kind_bit;
    }};
}

/// Set the pointer of a [`rsphp_sys::zend_type`] to be the given
/// [`rsphp_sys::zend_type_list`] pointer, and set the kind accordingly.
#[doc = c_version!(
    "#define ZEND_TYPE_SET_LIST(t, list)",
    "ZEND_TYPE_SET_PTR_AND_KIND(t, list, _ZEND_TYPE_LIST_BIT)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_SET_LIST {
    ($t:expr, $list:expr) => {
        $crate::ZEND_TYPE_SET_PTR_AND_KIND($t, $list, rsphp_sys::_ZEND_TYPE_LIST_BIT);
    };
}

/// Access the full type mask of a [`rsphp_sys::zend_type`].
#[doc = c_version!("#define ZEND_TYPE_FULL_MASK(t) ((t).type_mask)")]
#[macro_export]
macro_rules! ZEND_TYPE_FULL_MASK {
    ($t:expr) => {
        $t.type_mask
    };
}

/// Access the pure type mask of a [`rsphp_sys::zend_type`], i.e. just the
/// `MAY_BE_*` flags.
#[doc = c_version!("#define ZEND_TYPE_PURE_MASK(t) ((t).type_mask & _ZEND_TYPE_MAY_BE_MASK)")]
#[macro_export]
macro_rules! ZEND_TYPE_PURE_MASK {
    ($t:expr) => {
        $t.type_mask & rsphp_sys::_ZEND_TYPE_MAY_BE_MASK
    };
}

/// Access the full type mask of a [`rsphp_sys::zend_type`], excluding the
/// nullable bit.
#[doc = c_version!(
    "#define ZEND_TYPE_FULL_MASK_WITHOUT_NULL(t)",
    "((t).type_mask & ~_ZEND_TYPE_NULLABLE_BIT)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_FULL_MASK_WITHOUT_NULL {
    ($t:expr) => {
        $t.type_mask & !rsphp_sys::_ZEND_TYPE_NULLABLE_BIT
    };
}

/// Access the pure type mask of a [`rsphp_sys::zend_type`], i.e. just the
/// `MAY_BE_*` flags, excluding the nullable bit.
#[doc = c_version!(
    "#define ZEND_TYPE_PURE_MASK_WITHOUT_NULL(t)",
    "((t).type_mask & _ZEND_TYPE_MAY_BE_MASK & ~_ZEND_TYPE_NULLABLE_BIT)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_PURE_MASK_WITHOUT_NULL {
    ($t:expr) => {
        $t.type_mask & rsphp_sys::_ZEND_TYPE_MAY_BE_MASK & !rsphp_sys::_ZEND_TYPE_NULLABLE_BIT
    };
}

/// Check if a [`rsphp_sys::zend_type`] allows the given type code.
#[doc = c_version!(
    "#define ZEND_TYPE_CONTAINS_CODE(t, code)",
    "(((t).type_mask & (1u << (code))) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_CONTAINS_CODE {
    ($t:expr, $code:expr) => {
        ($t.type_mask & (1 << ($code as u32))) != 0
    };
}

/// Check if a [`rsphp_sys::zend_type`] allows null.
#[doc = c_version!(
    "#define ZEND_TYPE_ALLOW_NULL(t)",
    "(((t).type_mask & _ZEND_TYPE_NULLABLE_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_ALLOW_NULL {
    ($t:expr) => {
        ($t.type_mask & rsphp_sys::_ZEND_TYPE_NULLABLE_BIT) != 0
    };
}

/// Initialize a [`rsphp_sys::zend_type`] with the given flags.
///
/// # C version
///
/// ```c
/// #if defined(__cplusplus) && defined(_MSC_VER)
/// # define _ZEND_TYPE_PREFIX zend_type
/// #else
/// # define _ZEND_TYPE_PREFIX
/// #endif
///
/// #define ZEND_TYPE_INIT_NONE(extra_flags) \
///     _ZEND_TYPE_PREFIX { NULL, (extra_flags) }
/// ```
#[macro_export]
macro_rules! ZEND_TYPE_INIT_NONE {
    ($flags:expr) => {
        rsphp_sys::zend_type {
            ptr: std::ptr::null_mut(),
            type_mask: $flags,
        }
    };
}

/// Initiliaze a [`rsphp_sys::zend_type`] with the given mask.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_MASK(_type_mask)",
    "_ZEND_TYPE_PREFIX { NULL, (_type_mask) }"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_MASK {
    ($flags:expr) => {
        rsphp_sys::zend_type {
            ptr: std::ptr::null_mut(),
            type_mask: $flags as u32,
        }
    };
}

/// Initialize a [`rsphp_sys::zend_type`] with the given code as well as the
/// options given in `allow_null` and `extra_flags`.
///
/// # C version
///
/// ```c
/// #define ZEND_TYPE_INIT_CODE(code, allow_null, extra_flags) \
///     ZEND_TYPE_INIT_MASK(((code) == _IS_BOOL ? MAY_BE_BOOL : ( (code) == IS_ITERABLE ? _ZEND_TYPE_ITERABLE_BIT : ((code) == IS_MIXED ? MAY_BE_ANY : (1 << (code))))) \
///         | ((allow_null) ? _ZEND_TYPE_NULLABLE_BIT : 0) | (extra_flags))
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! ZEND_TYPE_INIT_CODE {
    (_IS_BOOL, 0, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::MAY_BE_BOOL | $flags) };
    (_IS_BOOL, 1, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::MAY_BE_BOOL | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $flags) };
    (IS_ITERABLE, 0, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::_ZEND_TYPE_ITERABLE_BIT | $flags) };
    (IS_ITERABLE, 1, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::_ZEND_TYPE_ITERABLE_BIT | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $flags) };
    (IS_MIXED, 0, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::MAY_BE_ANY | $flags) };
    (IS_MIXED, 1, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(rsphp_sys::MAY_BE_ANY | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $flags) };
    // Opaque expr might still be _IS_BOOL / IS_ITERABLE / IS_MIXED
    ($type:expr, 0, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(
        match $type as u8 {
            rsphp_sys::_IS_BOOL => rsphp_sys::MAY_BE_BOOL,
            rsphp_sys::IS_ITERABLE => rsphp_sys::_ZEND_TYPE_ITERABLE_BIT,
            rsphp_sys::IS_MIXED => rsphp_sys::MAY_BE_ANY,
            _ => 1 << $type
        }
        | $flags
    ) };
    ($type:expr, 1, $flags:expr) => { $crate::ZEND_TYPE_INIT_MASK!(
        match $type as u8 {
            rsphp_sys::_IS_BOOL => rsphp_sys::MAY_BE_BOOL,
            rsphp_sys::IS_ITERABLE => rsphp_sys::_ZEND_TYPE_ITERABLE_BIT,
            rsphp_sys::IS_MIXED => rsphp_sys::MAY_BE_ANY,
            _ => 1 << $type
        }
        | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $flags
    ) };
}

/// Initialize a [`rsphp_sys::zend_type`] with the given pointer and type
/// information.
///
/// # C version
///
/// ```c
/// #define ZEND_TYPE_INIT_PTR(ptr, type_kind, allow_null, extra_flags) \
///     _ZEND_TYPE_PREFIX { (void *) (ptr), \
///         (type_kind) | ((allow_null) ? _ZEND_TYPE_NULLABLE_BIT : 0) | (extra_flags) }
/// ```
#[doc = c_differences!(
    "In C the `ptr` is generally given as a literal string that is then treated \
        as a `const char *`; this macro accepts an expression for the class name \
        as the first parameter rather than a painter, and converts it to a \
        pointer to a c-string with that content."
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_PTR {
    // Literal is for raw strings to convert to char pointers
    ($class_name:literal, $type_kind:expr, 0, $extra_flags:expr) => {
        rsphp_sys::zend_type {
            ptr: $crate::c_str!($class_name).as_ptr() as *mut _,
            type_mask: $type_kind | $extra_flags,
        }
    };
    ($class_name:literal, $type_kind:expr, 1, $extra_flags:expr) => {
        rsphp_sys::zend_type {
            ptr: $crate::c_str!($class_name).as_ptr() as *mut _,
            type_mask: $type_kind | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $extra_flags,
        }
    };
    // Expr is for existing variables/expressions that are pointers
    ($class_name:expr, $type_kind:expr, 0, $extra_flags:expr) => {
        rsphp_sys::zend_type {
            ptr: $class_name as *mut _,
            type_mask: $type_kind | $extra_flags,
        }
    };
    ($class_name:expr, $type_kind:expr, 1, $extra_flags:expr) => {
        rsphp_sys::zend_type {
            ptr: $class_name as *mut _,
            type_mask: $type_kind | rsphp_sys::_ZEND_TYPE_NULLABLE_BIT | $extra_flags,
        }
    };
}

/// Initialize a [`rsphp_sys::zend_type`] with the given pointer and type mask.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_PTR_MASK(ptr, type_mask)",
    "_ZEND_TYPE_PREFIX { (void *) (ptr), (type_mask) }"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_PTR_MASK {
    ($ptr:expr, $type_mask:expr) => {
        rsphp_sys::zend_type {
            ptr: $ptr as *mut _,
            type_mask: $type_mask as u32,
        }
    };
}

/// Initialize a [`rsphp_sys::zend_type`] as a union with the given types and
/// extra flags.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_UNION(ptr, extra_flags)",
    "_ZEND_TYPE_PREFIX { (void *) (ptr), (_ZEND_TYPE_LIST_BIT|_ZEND_TYPE_UNION_BIT) | (extra_flags) }"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_UNION {
    ($ptr:expr, $type_mask:expr) => {
        rsphp_sys::zend_type {
            ptr: $ptr as *mut _,
            type_mask: rsphp_sys::_ZEND_TYPE_LIST_BIT
                | rsphp_sys::_ZEND_TYPE_UNION_BIT
                | ($type_mask as u32),
        }
    };
}

/// Initialize a [`rsphp_sys::zend_type`] as an intersection with the given
/// types and extra flags.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_INTERSECTION(ptr, extra_flags)",
    "_ZEND_TYPE_PREFIX { (void *) (ptr), (_ZEND_TYPE_LIST_BIT|_ZEND_TYPE_INTERSECTION_BIT) | (extra_flags) }"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_INTERSECTION {
    ($ptr:expr, $type_mask:expr) => {
        rsphp_sys::zend_type {
            ptr: $ptr as *mut _,
            type_mask: rsphp_sys::_ZEND_TYPE_LIST_BIT
                | rsphp_sys::_ZEND_TYPE_INTERSECTION_BIT
                | ($type_mask as u32),
        }
    };
}

/// Initialize a [`rsphp_sys::zend_type`] for the exact class given.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_CLASS(class_name, allow_null, extra_flags)",
    "ZEND_TYPE_INIT_PTR(class_name, _ZEND_TYPE_NAME_BIT, allow_null, extra_flags)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_CLASS {
    ($class_name:ident, 0, $extra_flags:expr) => {
        $crate::ZEND_TYPE_INIT_PTR!($class_name, rsphp_sys::_ZEND_TYPE_NAME_BIT, 0, $extra_flags)
    };
    ($class_name:ident, 1, $extra_flags:expr) => {
        $crate::ZEND_TYPE_INIT_PTR!($class_name, rsphp_sys::_ZEND_TYPE_NAME_BIT, 1, $extra_flags)
    };
}

/// Initialize a [`rsphp_sys::zend_type`] for the exact class given and with
/// the given type mask.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_CLASS_MASK(class_name, type_mask)",
    "ZEND_TYPE_INIT_PTR_MASK(class_name, _ZEND_TYPE_NAME_BIT | (type_mask))"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_CLASS_MASK {
    ($class_name:ident, $type_mask:expr) => {
        $crate::ZEND_TYPE_INIT_PTR_MASK!(
            $class_name,
            rsphp_sys::_ZEND_TYPE_NAME_BIT | ($type_mask as u32)
        )
    };
}

/// Initialize a [`rsphp_sys::zend_type`] for a class, potentially marked as
/// nullable and with extra flags applied.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_CLASS_CONST(class_name, allow_null, extra_flags)",
    "ZEND_TYPE_INIT_PTR(class_name, _ZEND_TYPE_LITERAL_NAME_BIT, allow_null, extra_flags)"
)]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_CLASS_CONST {
    ($class_name:literal, 0, $extra_flags:expr) => {
        $crate::ZEND_TYPE_INIT_PTR!(
            $class_name,
            rsphp_sys::_ZEND_TYPE_LITERAL_NAME_BIT,
            0,
            $extra_flags
        )
    };
    ($class_name:literal, 1, $extra_flags:expr) => {
        $crate::ZEND_TYPE_INIT_PTR!(
            $class_name,
            rsphp_sys::_ZEND_TYPE_LITERAL_NAME_BIT,
            1,
            $extra_flags
        )
    };
}

/// Initialize a [`rsphp_sys::zend_type`] for the given class and type mask.
#[doc = c_version!(
    "#define ZEND_TYPE_INIT_CLASS_CONST_MASK(class_name, type_mask)",
    "ZEND_TYPE_INIT_PTR_MASK(class_name, (_ZEND_TYPE_LITERAL_NAME_BIT | (type_mask)))"
)]
#[doc = c_differences!("Like [`ZEND_TYPE_INIT_PTR`] this converts a class name \
    into a pointer to a c-string with that name")]
#[macro_export]
macro_rules! ZEND_TYPE_INIT_CLASS_CONST_MASK {
    ($class_name:literal, $type_mask:expr) => {
        $crate::ZEND_TYPE_INIT_PTR_MASK!(
            $crate::c_str!($class_name).as_ptr(),
            rsphp_sys::_ZEND_TYPE_LITERAL_NAME_BIT | $type_mask
        )
    };
}

/// Access the type information of a [`rsphp_sys::zval`].
#[doc = c_version!("#define Z_TYPE_INFO(zval)            (zval).u1.type_info")]
#[macro_export]
macro_rules! Z_TYPE_INFO {
    ($zval:expr) => {{
        let u1 = $zval.u1;
        // SAFETY: interpreting the union as a full u32 is fine, it is a union
        // to allow setting multiple fields at once
        unsafe { u1.type_info }
    }};
}

/// Macro generation for pointer versions of macros that just forward to the
/// non-pointer version.
///
/// Must have C definitions like:
///
/// ```c
/// #define Z_REFCOUNTED_P(zval_p)        Z_REFCOUNTED(*(zval_p))
/// ```
///
/// with only the names of the macros changed.
macro_rules! make_pointer_version {
    ( $ptr_name:ident => $name:ident ) => {
        make_pointer_version!( $ptr_name => $name, $ );
    };
    ( $ptr_name:ident => $name:ident, $dollar:tt ) => {
        #[doc = concat!("Version of [`", stringify!($name), "!`][crate::", stringify!($name), "] for [`rsphp_sys::zval`] pointers.")]
        #[doc = c_version!(
            concat!(
                "#define ",
                stringify!($ptr_name),
                "(zval_p)    ",
                stringify!($name),
                "(*(zval_p))\n"
            )
        )]
        #[macro_export]
        macro_rules! $ptr_name {
            ($dollar zval_p:expr) => {
                $crate::$name!(* $dollar zval_p)
            }
        }
    };
}

make_pointer_version!( Z_TYPE_INFO_P => Z_TYPE_INFO );

/// Macro generation for zval u2 field access, with optional type checking.
macro_rules! generate_u2_accessors {
    ( $( [
        $name:ident,
        $ptr_name:ident,
        $holding:expr,
        $field_name:ident
    ], )+ ) => {
        generate_u2_accessors!(
            $(
                [$name, $ptr_name, $holding, $field_name, $],
            )+
        );
    };
    ( $( [
        $name:ident,
        $ptr_name:ident,
        $holding:expr,
        $field_name:ident,
        $dollar:tt
    ], )+ ) => {
        $(
            #[doc = concat!(
                "Access the extra `u2` information of a [`rsphp_sys::zval`] ",
                "(a [`rsphp_sys::_zval_u2`]), treating it as ",
                $holding,
                ".",
            )]
            #[doc = c_version!(concat!(
                "#define ", stringify!($name), "(zval)    (zval).u2.", stringify!($field_name)
            ))]
            #[macro_export]
            macro_rules! $name {
                ($dollar zval:expr) => {
                    {
                        // Outside of `unsafe` block
                        let u2 = $dollar zval.u2;
                        // SAFETY: all of the union fields are u32, interpreting
                        // the union as one variant or another is always safe
                        unsafe { u2.$field_name }
                    }
                }
            }

            make_pointer_version!( $ptr_name => $name );
        )+
    };
}

generate_u2_accessors!(
    [Z_NEXT, Z_NEXT_P, "the next index for iteration", next],
    [
        Z_CACHE_SLOT,
        Z_CACHE_SLOT_P,
        "a cache index for the [`rsphp_sys::ZEND_RECV_INIT`] opcode",
        cache_slot
    ],
    [
        Z_LINENO,
        Z_LINENO_P,
        "the line number of part of the abstract syntax tree",
        lineno
    ],
    [
        Z_OPLINE_NUM,
        Z_OPLINE_NUM_P,
        "the opline to use for the [`rsphp_sys::ZEND_FAST_CALL`] opcode",
        opline_num
    ],
    [
        Z_FE_POS,
        Z_FE_POS_P,
        "the position in foreach iteration",
        fe_pos
    ],
    [
        Z_FE_ITER,
        Z_FE_ITER_P,
        "the foreach iterator index in the global [`rsphp_sys::zend_executor_globals`] `ht_iterators` array",
        fe_iter_idx
    ],
    [Z_GUARD, Z_GUARD_P, "property and recursion guards", guard],
    [
        Z_CONSTANT_FLAGS,
        Z_CONSTANT_FLAGS_P,
        "flags for a constant",
        constant_flags
    ],
    [Z_EXTRA, Z_EXTRA_P, "an arbitrary `u32`", extra],
);

/// Add one to a reference counted value's count.
#[doc = c_version!("#define GC_ADDREF(p)                zend_gc_addref(&(p)->gc)")]
#[macro_export]
macro_rules! GC_ADDREF {
    ($p:expr) => {
        rsphp_sys::zend_gc_addref(std::ptr::from_mut(&mut (*$p).gc));
    };
}

/// Access the type information of a garbage collected value.
///
/// The value is expected to have a field named `gc` holding a
/// [`rsphp_sys::zend_refcounted_h`].
#[doc = c_version!("#define GC_TYPE_INFO(p)                (p)->gc.u.type_info")]
#[macro_export]
macro_rules! GC_TYPE_INFO {
    ($p:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        (*$p).gc.u.type_info
    };
}

/// Access the type mask information of a garbage collected value.
///
/// The value is expected to have a field named `gc` holding a
/// [`rsphp_sys::zend_refcounted_h`].
#[doc = c_version!("#define GC_TYPE(p)                    zval_gc_type(GC_TYPE_INFO(p))")]
#[macro_export]
macro_rules! GC_TYPE {
    ($p:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        rsphp_sys::zval_gc_type($crate::GC_TYPE_INFO!($p))
    };
}

/// Get the flags of a garbage collected value.
#[doc = c_version!("#define GC_FLAGS(p)                    zval_gc_flags(GC_TYPE_INFO(p))")]
#[macro_export]
macro_rules! GC_FLAGS {
    ($p:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        rsphp_sys::zval_gc_flags($crate::GC_TYPE_INFO!($p))
    };
}

/// Check if a type's values are reference counted.
#[doc = c_version!("# define Z_TYPE_INFO_REFCOUNTED(t)    (((t) & Z_TYPE_FLAGS_MASK) != 0)")]
#[macro_export]
macro_rules! Z_TYPE_INFO_REFCOUNTED {
    ($t:expr) => {
        ($t & rsphp_sys::Z_TYPE_FLAGS_MASK) != 0
    };
}

/// Check if a [`rsphp_sys::zval`]'s type is refcounted.
#[doc = c_version!("#define Z_REFCOUNTED(zval)            (Z_TYPE_FLAGS(zval) != 0)")]
#[macro_export]
macro_rules! Z_REFCOUNTED {
    ($zval:expr) => {
        $crate::Z_TYPE_FLAGS!($zval) != 0
    };
}

make_pointer_version!( Z_REFCOUNTED_P => Z_REFCOUNTED );

/// Get the type mask of type info.
#[doc = c_version!("#define Z_OPT_TYPE(zval)            (Z_TYPE_INFO(zval) & Z_TYPE_MASK)")]
#[macro_export]
macro_rules! Z_OPT_TYPE {
    ($zval:expr) => {
        $crate::Z_TYPE_INFO!($zval) & rsphp_sys::Z_TYPE_MASK
    };
}

make_pointer_version!( Z_OPT_TYPE_P => Z_OPT_TYPE );

/// Check if a [`rsphp_sys::zval`] is reference counted.
#[doc = c_version!("#define Z_OPT_REFCOUNTED(zval)        Z_TYPE_INFO_REFCOUNTED(Z_TYPE_INFO(zval))")]
#[macro_export]
macro_rules! Z_OPT_REFCOUNTED {
    ($zval:expr) => {
        $crate::Z_TYPE_INFO_REFCOUNTED!($crate::Z_TYPE_INFO!($zval))
    };
}

make_pointer_version!( Z_OPT_REFCOUNTED_P => Z_OPT_REFCOUNTED );

/// Check if a [`rsphp_sys::zval`] type mask is [`rsphp_sys::IS_REFERENCE`].
#[doc = c_version!("#define Z_OPT_ISREF(zval)            (Z_OPT_TYPE(zval) == IS_REFERENCE)")]
#[macro_export]
macro_rules! Z_OPT_ISREF {
    ($zval:expr) => {
        $crate::Z_OPT_TYPE!($zval) == rsphp_sys::IS_REFERENCE as u32
    };
}

make_pointer_version!( Z_OPT_ISREF_P => Z_OPT_ISREF );

/// Check if a [`rsphp_sys::zval`] is a reference ([`rsphp_sys::IS_REFERENCE`]).
#[doc = c_version!("#define Z_ISREF(zval)                (Z_TYPE(zval) == IS_REFERENCE)")]
#[macro_export]
macro_rules! Z_ISREF {
    ($zval:expr) => {
        $crate::Z_TYPE!($zval) == rsphp_sys::IS_REFERENCE
    };
}

make_pointer_version!( Z_ISREF_P => Z_ISREF );

/// Check if a [`rsphp_sys::zval`] is undefined ([`rsphp_sys::IS_UNDEF`]).
#[doc = c_version!("#define Z_ISUNDEF(zval)                (Z_TYPE(zval) == IS_UNDEF)")]
#[macro_export]
macro_rules! Z_ISUNDEF {
    ($zval:expr) => {
        $crate::Z_TYPE!($zval) == rsphp_sys::IS_UNDEF
    };
}

make_pointer_version!( Z_ISUNDEF_P => Z_ISUNDEF );

/// Check if a [`rsphp_sys::zval`] is null ([`rsphp_sys::IS_NULL`]).
#[doc = c_version!("#define Z_ISNULL(zval)                (Z_TYPE(zval) == IS_NULL)")]
#[macro_export]
macro_rules! Z_ISNULL {
    ($zval:expr) => {
        $crate::Z_TYPE!($zval) == rsphp_sys::IS_NULL
    };
}

make_pointer_version!( Z_ISNULL_P => Z_ISNULL );

/// Check if a [`rsphp_sys::zval`] is an error ([`rsphp_sys::_IS_ERROR`]).
#[doc = c_version!("#define Z_ISERROR(zval)                (Z_TYPE(zval) == _IS_ERROR)")]
#[macro_export]
macro_rules! Z_ISERROR {
    ($zval:expr) => {
        $crate::Z_TYPE!($zval) == rsphp_sys::_IS_ERROR
    };
}

make_pointer_version!( Z_ISERROR_P => Z_ISERROR );

/// Compare two types to see if they match.
///
/// # C version
///
/// ```c
/// #define ZEND_SAME_FAKE_TYPE(faketype, realtype) ( \
///     (faketype) == (realtype) \
///     || ((faketype) == _IS_BOOL && ((realtype) == IS_TRUE || (realtype) == IS_FALSE)) \
/// )
/// ```
#[macro_export]
macro_rules! ZEND_SAME_FAKE_TYPE {
    ($faketype:expr, $realtype:expr) => {
        ($faketype as u8 == $realtype as u8)
            || ($faketype as u8 == rsphp_sys::_IS_BOOL
                && ($realtype as u8 == rsphp_sys::IS_TRUE
                    || $realtype as u8 == rsphp_sys::IS_FALSE))
    };
}

/// Access the type of a [`rsphp_sys::zval`].
#[doc = c_version!("#define Z_TYPE(zval)                zval_get_type(&(zval))")]
#[macro_export]
macro_rules! Z_TYPE {
    ($zval:expr) => {{
        // Avoid FFI call for simple accessor
        let u1 = $zval.u1;
        // SAFETY: interpreting the union as a set of different fields is fine,
        // it is a union to allow setting multiple fields at once
        unsafe { u1.v.type_ }
    }};
}

make_pointer_version!( Z_TYPE_P => Z_TYPE );

/// Access the type flags of a [`rsphp_sys::zval`].
#[doc = c_version!("#define Z_TYPE_FLAGS(zval)            (zval).u1.v.type_flags")]
#[macro_export]
macro_rules! Z_TYPE_FLAGS {
    ($zval:expr) => {{
        let u1 = $zval.u1;
        // SAFETY: interpreting the union as a set of different fields is fine,
        // it is a union to allow setting multiple fields at once
        unsafe { u1.v.type_flags }
    }};
}

make_pointer_version!( Z_TYPE_FLAGS_P => Z_TYPE_FLAGS );

/// Macro generation for zval accessing, with optional type checking.
macro_rules! generate_zval_accessors {
    ( $( [
        $name:ident,
        $ptr_name:ident,
        $holding:expr,
        $expect_type:pat,
        $field_name:ident,
        $c_field_name:ident
    ], )+ ) => {
        generate_zval_accessors!(
            $(
                [$name, $ptr_name, $holding, $expect_type, $field_name, $c_field_name, "value", $],
            )+
        );
    };
    ( $( [
        $name:ident,
        $ptr_name:ident,
        $holding:expr,
        $expect_type:pat,
        $field_name:ident,
        $c_field_name:ident,
        $access_noun:literal,
        $dollar:tt
    ], )+ ) => {
        $(
            #[cfg(feature = "verify_zval_types")]
            #[doc = generate_zval_accessors!(@doc_field [$access_noun, $holding, $name, $c_field_name])]
            #[macro_export]
            macro_rules! $name {
                ($dollar zval:expr) => {
                    {
                        let u1 = $dollar zval.u1;
                        std::assert_matches!(
                            // SAFETY: always okay to read from the `v` field, union is
                            // to allow setting more information at once
                            unsafe { u1.v.type_ },
                            $expect_type,
                            concat!("Zval type expected to match ", stringify!($expect_type))
                        );
                        // Outside of `unsafe` block
                        let value = $dollar zval.value;
                        // SAFETY: type verified above
                        unsafe { value.$field_name }
                    }
                }
            }

            #[cfg(not(feature = "verify_zval_types"))]
            #[doc = generate_zval_accessors!(@doc_field [$access_noun, $holding, $name, $c_field_name])]
            #[macro_export]
            macro_rules! $name {
                ($dollar zval:expr) => {
                    {
                        let value = $dollar zval.value;
                        // SAFETY: the following is unsafe, and is the reason
                        // the macro is unsafe
                        value.$field_name
                    }
                }
            }

            make_pointer_version!( $ptr_name => $name );
        )+
    };
    (@doc_field [$access_noun:literal, $holding:expr, $name:ident, $c_field_name:ident]) => {
        concat!(
            "Access the ", $access_noun, " of a [`rsphp_sys::zval`] holding ", $holding, ".\n",
            c_version!(concat!(
                "#define ", stringify!($name), "(zval)    (zval).value.", stringify!($c_field_name)
            ))
        )
    };
}

generate_zval_accessors!(
    [Z_LVAL, Z_LVAL_P, "an integer", rsphp_sys::IS_LONG, lval, lval],
    [Z_DVAL, Z_DVAL_P, "a double", rsphp_sys::IS_DOUBLE, dval, dval],
    [Z_STR, Z_STR_P, "a [`rsphp_sys::zend_string`] pointer", rsphp_sys::IS_STRING, str_, str],
    [Z_ARR, Z_ARR_P, "a [`rsphp_sys::HashTable`] pointer", rsphp_sys::IS_ARRAY, arr, arr],
    [Z_OBJ, Z_OBJ_P, "a [`rsphp_sys::zend_object`] pointer", rsphp_sys::IS_OBJECT, obj, obj],
    [Z_RES, Z_RES_P, "a [`rsphp_sys::zend_resource`] pointer", rsphp_sys::IS_RESOURCE, res, res],
    [Z_REF, Z_REF_P, "a [`rsphp_sys::zend_reference`] pointer", rsphp_sys::IS_REFERENCE, ref_, ref],
    [Z_AST, Z_AST_P, "a [`rsphp_sys::zend_ast_ref`] pointer", rsphp_sys::IS_CONSTANT_AST, ast, ast],
    [Z_INDIRECT, Z_INDIRECT_P, "a [`rsphp_sys::zval`] pointer indirectly", rsphp_sys::IS_INDIRECT, zv, zv],
    [Z_CE, Z_CE_P, "a [`rsphp_sys::zend_class_entry`] pointer", rsphp_sys::IS_PTR | rsphp_sys::IS_ALIAS_PTR, ce, ce],
    [Z_FUNC, Z_FUNC_P, "a [`rsphp_sys::zend_function`] pointer", rsphp_sys::IS_PTR, func, func],
    [Z_PTR, Z_PTR_P, "an opaque pointer", rsphp_sys::IS_PTR | rsphp_sys::IS_ALIAS_PTR, ptr, ptr],
);
generate_zval_accessors!(
    [
        Z_COUNTED,
        Z_COUNTED_P,
        concat!(
            "some reference counted value, i.e. one of:\n\n",
            "* a [`rsphp_sys::zend_string`] pointer\n",
            "* a [`rsphp_sys::HashTable`] pointer\n",
            "* a [`rsphp_sys::zend_object`] pointer\n",
            "* a [`rsphp_sys::zend_resource`] pointer\n",
            "* a [`rsphp_sys::zend_reference`] pointer\n",
            "* a [`rsphp_sys::zend_ast_ref`] pointer\n",
            "\nall of which start with a [`rsphp_sys::_zend_refcounted_h`] field"
        ),
        rsphp_sys::IS_STRING
            | rsphp_sys::IS_ARRAY
            | rsphp_sys::IS_OBJECT
            | rsphp_sys::IS_RESOURCE
            | rsphp_sys::IS_REFERENCE
            | rsphp_sys::IS_CONSTANT_AST,
        counted,
        counted,
        "reference counting information",
        $
    ],
);

/// Access the inner char array of a [`rsphp_sys::zval`] holding a
/// [`rsphp_sys::zend_string`] pointer.
#[doc = c_version!("#define Z_STRVAL(zval)                ZSTR_VAL(Z_STR(zval))")]
#[macro_export]
macro_rules! Z_STRVAL {
    ($zval:expr) => {
        $crate::ZSTR_VAL!($crate::Z_STR!($zval))
    };
}

make_pointer_version!( Z_STRVAL_P => Z_STRVAL );

/// Access the value of a [`rsphp_sys::zval`] holding a [`rsphp_sys::HashTable`]
/// pointer.
#[doc = c_version!("#define Z_ARRVAL(zval)                Z_ARR(zval)")]
#[macro_export]
macro_rules! Z_ARRVAL {
    ($zval:expr) => {
        $crate::Z_ARR!($zval)
    };
}

make_pointer_version!( Z_ARRVAL_P => Z_ARRVAL );

/// Access the [`rsphp_sys::zend_object_handlers`] of a [`rsphp_sys::zval`]
/// holding a [`rsphp_sys::zend_object`] pointer.
#[doc = c_version!("#define Z_OBJ_HT(zval)                Z_OBJ(zval)->handlers")]
#[macro_export]
macro_rules! Z_OBJ_HT {
    ($zval:expr) => {
        (*$crate::Z_OBJ!($zval)).handlers
    };
}

make_pointer_version!( Z_OBJ_HT_P => Z_OBJ_HT );

/// Access one of the handlers in the [`rsphp_sys::zend_object_handlers`] of
/// a [`rsphp_sys::zval`] holding a [`rsphp_sys::zend_object`] pointer.
#[doc = c_version!("#define Z_OBJ_HANDLER(zval, hf)        Z_OBJ_HT((zval))->hf")]
#[macro_export]
macro_rules! Z_OBJ_HANDLER {
    ($zval:expr, $hf:ident) => {
        (*$crate::Z_OBJ_HT!($zval)).$hf
    };
}

/// Access one of the handlers in the [`rsphp_sys::zend_object_handlers`] of
/// a [`rsphp_sys::zval`] pointer holding a [`rsphp_sys::zend_object`] pointer.
#[doc = c_version!("#define Z_OBJ_HANDLER_P(zv_p, hf)    Z_OBJ_HANDLER(*(zv_p), hf)")]
#[macro_export]
macro_rules! Z_OBJ_HANDLER_P {
    ($zval_p:expr, $hf:ident) => {
        $crate::Z_OBJ_HANDLER!(*$zval_p, $hf)
    };
}

/// Access the [`rsphp_sys::zend_class_entry`] of a [`rsphp_sys::zval`] holding a
/// [`rsphp_sys::zend_object`] pointer.
#[doc = c_version!("#define Z_OBJCE(zval)                (Z_OBJ(zval)->ce)")]
#[macro_export]
macro_rules! Z_OBJCE {
    ($zval:expr) => {
        (*$crate::Z_OBJ!($zval)).ce
    };
}

make_pointer_version!( Z_OBJCE_P => Z_OBJCE );

/// Access the underlying [`rsphp_sys::zval`] of a [`rsphp_sys::zval`] holding a
/// [`rsphp_sys::zend_reference`] pointer.
#[doc = c_version!("#define Z_REFVAL(zval)                &Z_REF(zval)->val")]
#[macro_export]
macro_rules! Z_REFVAL {
    ($zval:expr) => {
        &mut (*$crate::Z_REF!($zval)).val
    };
}

make_pointer_version!( Z_REFVAL_P => Z_REFVAL );

/// Access the [`rsphp_sys::zend_ast`] pointer that follows a
/// [`rsphp_sys::zend_ast_ref`].
#[doc = c_version!("#define GC_AST(p)                    ((zend_ast*)(((char*)p) + sizeof(zend_ast_ref)))")]
#[macro_export]
macro_rules! GC_AST {
    ($p:expr) => {
        $p.byte_add(std::mem::size_of::<rsphp_sys::zend_ast_ref>())
            .cast::<rsphp_sys::zend_ast>()
    };
}

/// Access the [`rsphp_sys::zend_ast`] pointer that follows the
/// [`rsphp_sys::zend_ast_ref`] held in a [`rsphp_sys::zval`].
#[doc = c_version!("#define Z_ASTVAL(zval)                GC_AST(Z_AST(zval))")]
#[macro_export]
macro_rules! Z_ASTVAL {
    ($zval:expr) => {
        $crate::GC_AST!($crate::Z_AST!($zval))
    };
}

make_pointer_version!( Z_ASTVAL_P => Z_ASTVAL );

/// Set a [`rsphp_sys::zval`] pointer to be undefined ([`rsphp_sys::IS_UNDEF`]).
///
/// # C version
///
/// ```c
/// #define ZVAL_UNDEF(z) do {                \
///         Z_TYPE_INFO_P(z) = IS_UNDEF;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_UNDEF {
    ($z:expr) => {
        // cannot assign through macros
        (*$z).u1.type_info = rsphp_sys::IS_UNDEF as u32;
    };
}

/// Set a [`rsphp_sys::zval`] pointer to be null ([`rsphp_sys::IS_NULL`]).
///
/// # C version
///
/// ```c
/// #define ZVAL_NULL(z) do {                \
///         Z_TYPE_INFO_P(z) = IS_NULL;        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_NULL {
    ($z:expr) => {
        // cannot assign through macros
        (*$z).u1.type_info = rsphp_sys::IS_NULL as u32;
    };
}

/// Set a [`rsphp_sys::zval`] pointer to be false ([`rsphp_sys::IS_FALSE`]).
///
/// # C version
///
/// ```c
/// #define ZVAL_FALSE(z) do {                \
///         Z_TYPE_INFO_P(z) = IS_FALSE;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_FALSE {
    ($z:expr) => {
        // cannot assign through macros
        (*$z).u1.type_info = rsphp_sys::IS_FALSE as u32;
    };
}

/// Set a [`rsphp_sys::zval`] pointer to be true ([`rsphp_sys::IS_TRUE`]).
///
/// # C version
///
/// ```c
/// #define ZVAL_TRUE(z) do {                \
///         Z_TYPE_INFO_P(z) = IS_TRUE;        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_TRUE {
    ($z:expr) => {
        // cannot assign through macros
        (*$z).u1.type_info = rsphp_sys::IS_TRUE as u32;
    };
}

/// Set a [`rsphp_sys::zval`] pointer to be true ([`rsphp_sys::IS_TRUE`]) or false
/// ([`rsphp_sys::IS_FALSE`]) based on the given boolean.
///
/// # C version
///
/// ```c
/// #define ZVAL_BOOL(z, b) do {            \
///         Z_TYPE_INFO_P(z) =                \
///             (b) ? IS_TRUE : IS_FALSE;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_BOOL {
    ($z:expr, $b:expr) => {
        // cannot assign through macros
        (*$z).u1.type_info = if $b {
            rsphp_sys::IS_TRUE
        } else {
            rsphp_sys::IS_FALSE
        } as u32;
    };
}

/// Set a [`rsphp_sys::zval`] pointer to be the given integer.
///
/// # C version
///
/// ```c
/// #define ZVAL_LONG(z, l) do {            \
///         zval *__z = (z);                \
///         Z_LVAL_P(__z) = l;                \
///         Z_TYPE_INFO_P(__z) = IS_LONG;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_LONG {
    ($z:expr, $l:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.lval = $l;
        _z_ref.u1.type_info = rsphp_sys::IS_LONG as u32;
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be the given float value.
///
/// # C version
///
/// ```c
/// #define ZVAL_DOUBLE(z, d) do {            \
///         zval *__z = (z);                \
///         Z_DVAL_P(__z) = d;                \
///         Z_TYPE_INFO_P(__z) = IS_DOUBLE;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_DOUBLE {
    ($z:expr, $d:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.dval = $d;
        _z_ref.u1.type_info = rsphp_sys::IS_DOUBLE as u32;
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be a string with the given
/// [`rsphp_sys::zend_string`] pointer.
///
/// # C version
///
/// ```c
/// #define ZVAL_STR(z, s) do {                        \
///         zval *__z = (z);                        \
///         zend_string *__s = (s);                    \
///         Z_STR_P(__z) = __s;                        \
///         /* interned strings support */            \
///         Z_TYPE_INFO_P(__z) = ZSTR_IS_INTERNED(__s) ? \
///             IS_INTERNED_STRING_EX :             \
///             IS_STRING_EX;                        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_STR {
    ($z:expr, $s:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        let __s: *mut rsphp_sys::zend_string = $s;
        // cannot assign through macros
        // The following is unsafe, and is one reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.str_ = __s;
        // The following is unsafe, and is one reason this macro is unsafe
        if $crate::ZSTR_IS_INTERNED!(&*(__s)) {
            _z_ref.u1.type_info = rsphp_sys::IS_INTERNED_STRING_EX;
        } else {
            _z_ref.u1.type_info = rsphp_sys::IS_STRING_EX;
        }
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be a string with the given
/// [`rsphp_sys::zend_string`] pointer, where the string is known to be interned.
///
/// # C version
///
/// ```c
/// #define ZVAL_INTERNED_STR(z, s) do {                \
///         zval *__z = (z);                            \
///         zend_string *__s = (s);                        \
///         Z_STR_P(__z) = __s;                            \
///         Z_TYPE_INFO_P(__z) = IS_INTERNED_STRING_EX;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_INTERNED_STR {
    ($z:expr, $s:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        let __s: *mut rsphp_sys::zend_string = $s;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.str_ = __s;
        _z_ref.u1.type_info = rsphp_sys::IS_INTERNED_STRING_EX;
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be a string with the given
/// [`rsphp_sys::zend_string`] pointer, where the string is known to not be interned.
///
/// # C version
///
/// ```c
/// #define ZVAL_NEW_STR(z, s) do {                \
///         zval *__z = (z);                            \
///         zend_string *__s = (s);                        \
///         Z_STR_P(__z) = __s;                            \
///         Z_TYPE_INFO_P(__z) = IS_STRING_EX;    \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_NEW_STR {
    ($z:expr, $s:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        let __s: *mut rsphp_sys::zend_string = $s;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.str_ = __s;
        _z_ref.u1.type_info = rsphp_sys::IS_STRING_EX;
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be a string that is a copy of the given
/// [`rsphp_sys::zend_string`] pointer.
///
/// # C version
///
/// ```c
/// #define ZVAL_STR_COPY(z, s) do {                        \
///         zval *__z = (z);                                \
///         zend_string *__s = (s);                            \
///         Z_STR_P(__z) = __s;                                \
///         /* interned strings support */                    \
///         if (ZSTR_IS_INTERNED(__s)) {                    \
///             Z_TYPE_INFO_P(__z) = IS_INTERNED_STRING_EX;    \
///         } else {                                        \
///             GC_ADDREF(__s);                                \
///             Z_TYPE_INFO_P(__z) = IS_STRING_EX;            \
///         }                                                \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_STR_COPY {
    ($z:expr, $s:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        let __s: *mut rsphp_sys::zend_string = $s;
        // cannot assign through macros
        // The following is unsafe, and is one reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.str_ = __s;
        // The following is unsafe, and is one reason this macro is unsafe
        if $crate::ZSTR_IS_INTERNED!(&*(__s)) {
            _z_ref.u1.type_info = rsphp_sys::IS_INTERNED_STRING_EX;
        } else {
            $crate::GC_ADDREF!(__s);
            _z_ref.u1.type_info = rsphp_sys::IS_STRING_EX;
        }
    }};
}

/// Set a [`rsphp_sys::zval`] pointer to be an array with the given
/// [`rsphp_sys::zend_array`] pointer.
///
/// # C version
///
/// ```c
/// #define ZVAL_ARR(z, a) do {                        \
///         zend_array *__arr = (a);                \
///         zval *__z = (z);                        \
///         Z_ARR_P(__z) = __arr;                    \
///         Z_TYPE_INFO_P(__z) = IS_ARRAY_EX;        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_ARR {
    ($z:expr, $a:expr) => {{
        let __arr: *mut rsphp_sys::zend_array = $a;
        let __z: *mut rsphp_sys::zval = $z;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.arr = __arr;
        _z_ref.u1.type_info = rsphp_sys::IS_ARRAY_EX;
    }};
}

/// Add a reference count to a reference counted [`rsphp_sys::zval`] pointer.
#[doc = c_version!("#define Z_ADDREF_P(pz)                zval_addref_p(pz)")]
#[macro_export]
macro_rules! Z_ADDREF_P {
    ($pz:expr) => {
        rsphp_sys::zval_addref_p($pz)
    };
}

/// Add a reference count if the provided [`rsphp_sys::zval`] pointer is reference
/// counted.
///
/// # C version
///
/// ```c
/// #define Z_TRY_ADDREF_P(pz) do {        \
///     zval *_pz = (pz);                \
///     if (Z_REFCOUNTED_P(_pz)) {        \
///         Z_ADDREF_P(_pz);            \
///     }                                \
/// } while (0)
/// ```
#[macro_export]
macro_rules! Z_TRY_ADDREF_P {
    ($pz:expr) => {{
        let _pz: *mut zval = $pz;
        if $crate::Z_REFCOUNTED_P!(_pz) {
            $crate::Z_ADDREF_P!(_pz);
        }
    }};
}

/// Add a reference count if the provided [`rsphp_sys::zval`] is reference counted.
#[doc = c_version!("#define Z_TRY_ADDREF(z)                Z_TRY_ADDREF_P(&(z))")]
#[macro_export]
macro_rules! Z_TRY_ADDREF {
    ($z:expr) => {
        $crate::Z_TRY_ADDREF_P!(&mut $z)
    };
}

/// Set a [`rsphp_sys::zval`] pointer to hold a [`rsphp_sys::zend_object`].
///
/// # C version
/// ```c
/// #define ZVAL_OBJ(z, o) do {                        \
///         zval *__z = (z);                        \
///         Z_OBJ_P(__z) = (o);                        \
///         Z_TYPE_INFO_P(__z) = IS_OBJECT_EX;        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_OBJ {
    ($z:expr, $o:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.obj = $o;
        _z_ref.u1.type_info = rsphp_sys::IS_OBJECT_EX;
    }};
}

/// Copy a [`rsphp_sys::zend_object`] into a [`rsphp_sys::zval`] pointer.
///
/// # C version
///
/// ```c
/// #define ZVAL_OBJ_COPY(z, o) do {                \
///         zval *__z = (z);                        \
///         zend_object *__o = (o);                    \
///         GC_ADDREF(__o);                            \
///         Z_OBJ_P(__z) = __o;                        \
///         Z_TYPE_INFO_P(__z) = IS_OBJECT_EX;        \
///     } while (0)
#[macro_export]
macro_rules! ZVAL_OBJ_COPY {
    ($z:expr, $o:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        let __o: *mut rsphp_sys::zend_object = $o;
        $crate::GC_ADDREF!(__o);
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.obj = __o;
        _z_ref.u1.type_info = rsphp_sys::IS_OBJECT_EX;
    }};
}

const _: () = {
    assert!(size_of::<usize>() == 8, "usize should be 8 bytes");
};
/// Copy a [`rsphp_sys::zval`] into a [`rsphp_sys::zval`] pointer, including
/// extra information.
///
/// # C version
///
/// ```c
/// # define ZVAL_COPY_VALUE_EX(z, v, gc, t)                \
///     do {                                                \
///         Z_COUNTED_P(z) = gc;                            \
///         Z_TYPE_INFO_P(z) = t;                            \
///     } while (0)
/// ```
#[doc = c_differences!(
    "The C version is defined differently for `SIZEOF_SIZE_T` being 4 or 8, we \
        assert that it is always 8. Only the C version for the size being 8 is \
        shown above."
)]
#[macro_export]
macro_rules! ZVAL_COPY_VALUE_EX {
    ($z:expr, $_v:expr, $gc:expr, $t:expr) => {
        // Cannot assign through macros
        let __z: *mut rsphp_sys::zval = $z;
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.counted = $gc;
        _z_ref.u1.type_info = $t;
    };
}

/// Copy a [`rsphp_sys::zval`] pointer's value into another [`rsphp_sys::zval`]
/// pointer, including extra information.
///
/// # C version
///
/// ```c
/// #define ZVAL_COPY_VALUE(z, v)                            \
///     do {                                                \
///         zval *_z1 = (z);                                \
///         const zval *_z2 = (v);                            \
///         zend_refcounted *_gc = Z_COUNTED_P(_z2);        \
///         uint32_t _t = Z_TYPE_INFO_P(_z2);                \
///         ZVAL_COPY_VALUE_EX(_z1, _z2, _gc, _t);            \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_COPY_VALUE {
    ($z:expr, $v:expr) => { {
        // Cannot assign through macros
        let _z1: *mut rsphp_sys::zval = $z;
        let _z2: *const rsphp_sys::zval = $v;
        let _gc: *mut rsphp_sys::zend_refcounted = $crate::Z_COUNTED_P!(_z2);
        let _t: u32 = $crate::Z_TYPE_INFO_P!(_z2);
        // The following is unsafe, and is the reason this macro is unsafe
        $crate::ZVAL_COPY_VALUE_EX!(_z1, _z2, _gc, _t);
    } }
}

/// Copy a [`rsphp_sys::zval`] pointer's value into another [`rsphp_sys::zval`]
/// pointer, including extra information; if the value is reference counted,
/// increment the count.
///
/// # C version
///
/// ```c
/// #define ZVAL_COPY(z, v)                                    \
///     do {                                                \
///         zval *_z1 = (z);                                \
///         const zval *_z2 = (v);                            \
///         zend_refcounted *_gc = Z_COUNTED_P(_z2);        \
///         uint32_t _t = Z_TYPE_INFO_P(_z2);                \
///         ZVAL_COPY_VALUE_EX(_z1, _z2, _gc, _t);            \
///         if (Z_TYPE_INFO_REFCOUNTED(_t)) {                \
///             GC_ADDREF(_gc);                                \
///         }                                                \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_COPY {
    ($z:expr, $v:expr) => { {
        // Cannot assign through macros
        let _z1: *mut rsphp_sys::zval = $z;
        let _z2: *const rsphp_sys::zval = $v;
        let _gc: *mut rsphp_sys::zend_refcounted = $crate::Z_COUNTED_P!(_z2);
        let _t: u32 = $crate::Z_TYPE_INFO_P!(_z2);
        // The following is unsafe, and is the reason this macro is unsafe
        $crate::ZVAL_COPY_VALUE_EX!(_z1, _z2, _gc, _t);
        if ($crate::Z_TYPE_INFO_REFCOUNTED!(_t)) {
            $crate::GC_ADDREF!(_gc);
        }
    } }
}

/// Copy a [`rsphp_sys::zval`] pointer's value into another [`rsphp_sys::zval`]
/// pointer, duplicating it or updating the reference count.
///
/// The value is duplicated if the source is a persistent zval; if not, and the
/// source value is reference counted, the reference count is incremented.
///
/// # C version
///
/// ```c
/// #define ZVAL_COPY_OR_DUP(z, v)                                            \
///     do {                                                                \
///         zval *_z1 = (z);                                                \
///         const zval *_z2 = (v);                                            \
///         zend_refcounted *_gc = Z_COUNTED_P(_z2);                        \
///         uint32_t _t = Z_TYPE_INFO_P(_z2);                                \
///         ZVAL_COPY_VALUE_EX(_z1, _z2, _gc, _t);                            \
///         if (Z_TYPE_INFO_REFCOUNTED(_t)) {                                \
///             /* Objects reuse PERSISTENT as WEAKLY_REFERENCED */            \
///             if (EXPECTED(!(GC_FLAGS(_gc) & GC_PERSISTENT)                \
///                     || GC_TYPE(_gc) == IS_OBJECT)) {                    \
///                 GC_ADDREF(_gc);                                            \
///             } else {                                                    \
///                 zval_copy_ctor_func(_z1);                                \
///             }                                                            \
///         }                                                                \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_COPY_OR_DUP {
    ($z:expr, $v:expr) => { {
        // Cannot assign through macros
        let _z1: *mut rsphp_sys::zval = $z;
        let _z2: *const rsphp_sys::zval = $v;
        let _gc: *mut rsphp_sys::zend_refcounted = $crate::Z_COUNTED_P!(_z2);
        let _t: u32 = $crate::Z_TYPE_INFO_P!(_z2);
        // The following is unsafe, and is the reason this macro is unsafe
        $crate::ZVAL_COPY_VALUE_EX!(_z1, _z2, _gc, _t);
        if ($crate::Z_TYPE_INFO_REFCOUNTED!(_t)) {
            if ($crate::GC_FLAGS!(_gc) & rsphp_sys::GC_PERSISTENT) == 0
                || $crate::GC_TYPE!(_gc) == rsphp_sys::IS_OBJECT
            {
                $crate::GC_ADDREF!(_gc);
            } else {
                rsphp_sys::zval_copy_ctor_func(_z1);
            }
        }

    } }
}

/// Unwrap a [`rsphp_sys::zval`] pointer if it is [`rsphp_sys::IS_REFERENCE`].
///
/// # C version
///
/// ```c
/// #define ZVAL_DEREF(z) do {                                \
///         if (UNEXPECTED(Z_ISREF_P(z))) {                    \
///             (z) = Z_REFVAL_P(z);                        \
///         }                                                \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_DEREF {
    ($zval_p:ident) => {{
        if $crate::Z_ISREF_P!($zval_p) {
            $zval_p = $crate::Z_REFVAL_P!($zval_p);
        }
    }};
}

/// Unwrap a [`rsphp_sys::zval`] pointer if it is [`rsphp_sys::IS_INDIRECT`].
///
/// # C version
///
/// ```c
/// ZVAL_DEINDIRECT(z) do {                            \
///         if (Z_TYPE_P(z) == IS_INDIRECT) {                \
///             (z) = Z_INDIRECT_P(z);                        \
///         }                                                \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_DEINDIRECT {
    ($zval_p:ident) => {{
        if $crate::Z_TYPE_P!($zval_p) == rsphp_sys::IS_INDIRECT {
            $zval_p = $crate::Z_INDIRECT_P!($zval_p);
        }
    }};
}

/// Copy a [`rsphp_sys::zval`] pointer's value into another [`rsphp_sys::zval`]
/// pointer, and also dereference it.
///
/// # C version
///
/// ```c
/// #define ZVAL_COPY_DEREF(z, v) do {                        \
///         zval *_z3 = (v);                                \
///         if (Z_OPT_REFCOUNTED_P(_z3)) {                    \
///             if (UNEXPECTED(Z_OPT_ISREF_P(_z3))) {        \
///                 _z3 = Z_REFVAL_P(_z3);                    \
///                 if (Z_OPT_REFCOUNTED_P(_z3)) {            \
///                     Z_ADDREF_P(_z3);                    \
///                 }                                        \
///             } else {                                    \
///                 Z_ADDREF_P(_z3);                        \
///             }                                            \
///         }                                                \
///         ZVAL_COPY_VALUE(z, _z3);                        \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_COPY_DEREF {
    ($z:expr, $v:expr) => { {
        // Cannot assign through macros
        let mut _z3: *mut rsphp_sys::zval = $v;
        if $crate::Z_OPT_REFCOUNTED_P!(_z3) {
            if $crate::Z_OPT_ISREF_P!(_z3) {
                _z3 = $crate::Z_REFVAL_P!(_z3);
                if $crate::Z_OPT_REFCOUNTED_P!(_z3) {
                    $crate::Z_ADDREF_P!(_z3);
                }
            } else {
                $crate::Z_ADDREF_P!(_z3);
            }
        }
        // The following is unsafe, and is the reason this macro is unsafe
        $crate::ZVAL_COPY_VALUE!($z, _z3)
    } }
}

/// Get flags for a property stored in a [`rsphp_sys::zval`] pointer.
#[doc = c_version!("#define Z_PROP_FLAG_P(z) Z_EXTRA_P(z)")]
#[macro_export]
macro_rules! Z_PROP_FLAG_P {
    ($z:expr) => {
        $crate::Z_EXTRA_P!($z)
    };
}
