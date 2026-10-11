//! Macros corresponding to `zend_compile.h` in php/php-src.

use crate::{c_differences, c_version};

/// Get the property with the given offset from a [`rsphp_sys::zend_object`]
/// pointer.
#[doc = c_version!(
    "#define OBJ_PROP(obj, offset)",
    "((zval*)((char*)(obj) + offset))"
)]
#[macro_export]
macro_rules! OBJ_PROP {
    ($obj:expr, $offset:expr) => {
        $obj.byte_add($offset).cast::<zval>()
    };
}

/// Identify the offset in bytes from a [`rsphp_sys::zend_object`] pointer to
/// the property with the given offset number.
#[doc = c_version!(
    "#define OBJ_PROP_TO_OFFSET(num)",
    "((uint32_t)(XtOffsetOf(zend_object, properties_table) + sizeof(zval) * (num)))"
)]
#[macro_export]
macro_rules! OBJ_PROP_TO_OFFSET {
    ($num:expr) => {
        std::mem::offset_of!(rsphp_sys::zend_object, properties_table)
            + (std::mem::size_of::<rsphp_sys::zval>() * $num)
    };
}

/// Identify the offset (in units of [`rsphp_sys::zval`]) from a
/// [`rsphp_sys::zend_object`] start of the property table to the property
/// with the given offset number.
#[doc = c_version!(
    "#define OBJ_PROP_TO_NUM(offset)",
    "(((offset) - OBJ_PROP_TO_OFFSET(0)) / sizeof(zval))"
)]
#[macro_export]
macro_rules! OBJ_PROP_TO_NUM {
    ($offset:expr) => {
        ($offset - $crate::OBJ_PROP_TO_OFFSET!(0)) / std::mem::size_of::<rsphp_sys::zval>()
    };
}

/// Access the flags of a [`rsphp_sys::zend_class_constant`] pointer.
#[doc = c_version!("#define ZEND_CLASS_CONST_FLAGS(c) Z_CONSTANT_FLAGS((c)->value)")]
#[macro_export]
macro_rules! ZEND_CLASS_CONST_FLAGS {
    ($c:expr) => {
        $crate::Z_CONSTANT_FLAGS!((*$c).value)
    };
}

/// Access the call information of a [`rsphp_sys::zend_execute_data`] pointer.
#[doc = c_version!("#define ZEND_CALL_INFO(call)", "Z_TYPE_INFO((call)->This)")]
#[macro_export]
macro_rules! ZEND_CALL_INFO {
    ($call:expr) => {
        $crate::Z_TYPE_INFO!((*$call).This)
    };
}

/// Get the kind information flags from the overall call information.
#[doc = c_version!("#define ZEND_CALL_KIND_EX(call_info)", "(call_info & (ZEND_CALL_CODE | ZEND_CALL_TOP))")]
#[macro_export]
macro_rules! ZEND_CALL_KIND_EX {
    ($call_info:expr) => {
        $call_info & (rsphp_sys::ZEND_CALL_CODE | rsphp_sys::ZEND_CALL_TOP)
    };
}

/// Get the kind information flags of a [`rsphp_sys::zend_execute_data`] pointer.
#[doc = c_version!("#define ZEND_CALL_KIND(call)", "ZEND_CALL_KIND_EX(ZEND_CALL_INFO(call))")]
#[macro_export]
macro_rules! ZEND_CALL_KIND {
    ($call:expr) => {
        $crate::ZEND_CALL_KIND_EX!($crate::ZEND_CALL_INFO!($call))
    };
}

/// Add a flag to the call information of a [`rsphp_sys::zend_execute_data`]
/// pointer.
///
/// # C version
///
/// ```c
/// #define ZEND_ADD_CALL_FLAG_EX(call_info, flag) do { \
///         call_info |= (flag); \
///     } while (0)
/// #define ZEND_ADD_CALL_FLAG(call, flag) do { \
///         ZEND_ADD_CALL_FLAG_EX(Z_TYPE_INFO((call)->This), flag); \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZEND_ADD_CALL_FLAG {
    ($call:expr, $flag:expr) => {{
        /// Cannot assign through macros
        let _z: &mut rsphp_sys::zval = &mut ((*$call).This);
        _z.u1.type_info |= $flag;
    }};
}

/// Remove a flag from the call information of a
/// [`rsphp_sys::zend_execute_data`] pointer.
///
/// # C version
///
/// ```c
/// #define ZEND_DEL_CALL_FLAG_EX(call_info, flag) do { \
///         call_info &= ~(flag); \
///     } while (0)
/// #define ZEND_DEL_CALL_FLAG(call, flag) do { \
///         ZEND_DEL_CALL_FLAG_EX(Z_TYPE_INFO((call)->This), flag); \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZEND_DEL_CALL_FLAG {
    ($call:expr, $flag:expr) => {{
        /// Cannot assign through macros
        let _z: &mut rsphp_sys::zval = &mut ((*$call).This);
        _z.u1.type_info &= !$flag;
    }};
}

/// Access the number of arguments in the call information of a
/// [`rsphp_sys::zend_execute_data`] pointer.
#[doc = c_version!("#define ZEND_CALL_NUM_ARGS(call)", "(call)->This.u2.num_args")]
#[macro_export]
macro_rules! ZEND_CALL_NUM_ARGS {
    ($call:expr) => {
        (*$call).This.u2.num_args
    };
}

const _: () = {
    assert!(size_of::<usize>() == 8, "usize should be 8 bytes");
};
/// Extract a run-time constant parameter after a [`rsphp_sys::zend_op`] pointer.
#[doc = c_version!(
    "#define RT_CONSTANT(opline, node)",
    "((zval*)(((char*)(opline)) + (int32_t)(node).constant))"
)]
#[doc = c_differences!(
    "The C version is defined differently for `SIZEOF_SIZE_T` being 4 or 8, we \
        assert that it is always 8. Only the C version for the size being 8 is \
        shown above."
)]
#[macro_export]
macro_rules! RT_CONSTANT {
    ($opline:expr, $node_op:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        $opline
            .byte_add($node_op.constant as usize)
            .cast::<rsphp_sys::zval>()
    };
}

/// Check if code corresponds to user code.
#[doc = c_version!("#define ZEND_USER_CODE(type)        ((type) != ZEND_INTERNAL_FUNCTION)")]
#[macro_export]
macro_rules! ZEND_USER_CODE {
    ($type:expr) => {
        $type != rsphp_sys::ZEND_INTERNAL_FUNCTION
    };
}

/// Get the send mode of a function argument ([`rsphp_sys::zend_arg_info`]).
#[doc = c_version!(
    "#define ZEND_ARG_SEND_MODE(arg_info)",
    "((ZEND_TYPE_FULL_MASK((arg_info)->type) >> _ZEND_SEND_MODE_SHIFT) & 3)"
)]
#[macro_export]
macro_rules! ZEND_ARG_SEND_MODE {
    ($arg_info:expr) => {
        ($crate::ZEND_TYPE_FULL_MASK!((*$arg_info).type_) >> rsphp_sys::_ZEND_SEND_MODE_SHIFT) & 3
    };
}

/// Check if a function argument ([`rsphp_sys::zend_arg_info`]) is marked as
/// variadic.
#[doc = c_version!(
    "#define ZEND_ARG_IS_VARIADIC(arg_info)",
    "((ZEND_TYPE_FULL_MASK((arg_info)->type) & _ZEND_IS_VARIADIC_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_ARG_IS_VARIADIC {
    ($arg_info:expr) => {
        ($crate::ZEND_TYPE_FULL_MASK!((*$arg_info).type_) & rsphp_sys::_ZEND_IS_VARIADIC_BIT) != 0
    };
}

/// Check if a function argument ([`rsphp_sys::zend_arg_info`]) is marked as
/// promoted.
#[doc = c_version!(
    "#define ZEND_ARG_IS_PROMOTED(arg_info)",
    "((ZEND_TYPE_FULL_MASK((arg_info)->type) & _ZEND_IS_PROMOTED_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_ARG_IS_PROMOTED {
    ($arg_info:expr) => {
        ($crate::ZEND_TYPE_FULL_MASK!((*$arg_info).type_) & rsphp_sys::_ZEND_IS_PROMOTED_BIT) != 0
    };
}

/// Check if a function argument (used for return type information) is marked
/// as a tentative return type.
#[doc = c_version!(
    "#define ZEND_ARG_TYPE_IS_TENTATIVE(arg_info)",
    "((ZEND_TYPE_FULL_MASK((arg_info)->type) & _ZEND_IS_TENTATIVE_BIT) != 0)"
)]
#[macro_export]
macro_rules! ZEND_ARG_TYPE_IS_TENTATIVE {
    ($arg_info:expr) => {
        $crate::ZEND_TYPE_FULL_MASK!((*$arg_info).type_) & rsphp_sys::_ZEND_IS_TENTATIVE_BIT != 0
    };
}
