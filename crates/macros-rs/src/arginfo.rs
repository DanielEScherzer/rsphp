use crate::{c_differences, c_version};

/// Create a fake arginfo entry for a function with a return type mask.
///
/// PHP uses `ZEND_BEGIN_ARG_WITH_RETURN_TYPE_MASK_EX2` to declare an array that
/// is then populated with the first entry being a fake arg info for the overall
/// function, this macro creates that fake entry.
#[macro_export]
macro_rules! ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_TYPE_MASK {
    ($return_reference:tt, $required_num_args:expr, $type:expr, $is_tentative_return_type:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: std::ptr::without_provenance($required_num_args),
            type_: rsphp_macros_c::ZEND_TYPE_INIT_MASK!(
                $type
                    | rsphp_macros_c::_ZEND_ARG_INFO_FLAGS!(
                        $return_reference,
                        0,
                        $is_tentative_return_type
                    )
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a fake arginfo entry for a function with return type information.
///
/// PHP uses `ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX` to declare an array that
/// is then populated with the first entry being a fake arg info for the overall
/// function, this macro creates that fake entry.
#[macro_export]
macro_rules! ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_TYPE_INFO {
    ($return_reference:tt, $required_num_args:expr, $type:expr, $allows_null:tt, $is_tentative_return_type:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: std::ptr::without_provenance($required_num_args),
            type_: rsphp_macros_c::ZEND_TYPE_INIT_CODE!(
                $type,
                $allows_null,
                rsphp_macros_c::_ZEND_ARG_INFO_FLAGS!(
                    $return_reference,
                    0,
                    $is_tentative_return_type
                )
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a fake arginfo entry for a function with return object information.
///
/// PHP uses `ZEND_BEGIN_ARG_WITH_RETURN_OBJ_INFO_EX2` to declare an array that
/// is then populated with the first entry being a fake arg info for the overall
/// function, this macro creates that fake entry.
#[macro_export]
macro_rules! ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_OBJ_INFO {
    ($return_reference:tt, $required_num_args:expr, $class_name:literal, $allow_null:tt, $is_tentative_return_type:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: std::ptr::without_provenance($required_num_args),
            type_: rsphp_macros_c::ZEND_TYPE_INIT_CLASS_CONST!(
                $class_name,
                $allow_null,
                rsphp_macros_c::_ZEND_ARG_INFO_FLAGS!(
                    $return_reference,
                    0,
                    $is_tentative_return_type
                )
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a fake arginfo entry for a function with return object type information.
///
/// PHP uses `ZEND_BEGIN_ARG_WITH_RETURN_OBJ_TYPE_MASK_EX2` to declare an array that
/// is then populated with the first entry being a fake arg info for the overall
/// function, this macro creates that fake entry.
#[macro_export]
macro_rules! ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_OBJ_TYPE_INFO {
    ($return_reference:tt, $required_num_args:expr, $class_name:literal, $type:tt, $is_tentative_return_type:tt) => {
        rsphp_sys::zend_internal_arg_info {
            name: std::ptr::without_provenance($required_num_args),
            type_: rsphp_macros_c::ZEND_TYPE_INIT_CLASS_CONST_MASK!(
                $class_name,
                $type
                    | rsphp_macros_c::_ZEND_ARG_INFO_FLAGS!(
                        $return_reference,
                        0,
                        $is_tentative_return_type
                    )
            ),
            default_value: std::ptr::null(),
        }
    };
}

/// Create a fake arginfo entry for a function.
///
/// PHP uses `ZEND_BEGIN_ARG_INFO_EX` to declare an array that
/// is then populated with the first entry being a fake arg info for the overall
/// function, this macro creates that fake entry.
#[macro_export]
macro_rules! ZEND_ARG_INFO_FOR_FUNCTION {
    ($return_reference:tt, $required_num_args:expr) => {
        rsphp_sys::zend_internal_arg_info {
            name: std::ptr::without_provenance($required_num_args),
            type_: rsphp_macros_c::ZEND_TYPE_INIT_NONE!(rsphp_macros_c::_ZEND_ARG_INFO_FLAGS!(
                $return_reference,
                0,
                0
            )),
            default_value: std::ptr::null(),
        }
    };
}

/// Define a global function handler entry.
#[doc = c_version!("#define ZEND_FE(name, arg_info)                        ZEND_RAW_FENTRY(#name, zif_##name, arg_info, 0, NULL, NULL)")]
#[doc = c_differences!(
    "Since Rust handlers are not added based on the expected function name, \
    this macro also accepts the handler that should be used."
)]
#[macro_export]
macro_rules! ZEND_FE {
    ($name:literal, $handler:expr, $arg_info:expr) => {
        rsphp_macros_c::ZEND_RAW_FENTRY!(
            $name,
            $handler,
            $arg_info,
            0,
            std::ptr::null(),
            std::ptr::null()
        )
    };
}

/// Define a class function handler entry.
#[doc = c_version!("#define ZEND_ME(classname, name, arg_info, flags)    ZEND_RAW_FENTRY(#name, zim_##classname##_##name, arg_info, flags, NULL, NULL)")]
#[doc = c_differences!(
    "Since Rust handlers are not added based on the expected function name, \
    this macro accepts the handler that should be used instead of the class name."
)]
#[macro_export]
macro_rules! ZEND_ME {
    ($name:literal, $handler:expr, $arg_info:expr, $flags:expr) => {
        rsphp_macros_c::ZEND_RAW_FENTRY!(
            $name,
            $handler,
            $arg_info,
            $flags,
            std::ptr::null(),
            std::ptr::null()
        )
    };
}

/// Define a class function handler entry for an abstract method.
///
/// The method is marked as abstract ([`rsphp_sys::ZEND_ACC_ABSTRACT`]) and
/// public ([`rsphp_sys::ZEND_ACC_PUBLIC`]).
#[doc = c_version!(
    "#define ZEND_ABSTRACT_ME(classname, name, arg_info)    \
        ZEND_RAW_FENTRY(#name, NULL, arg_info, ZEND_ACC_PUBLIC|ZEND_ACC_ABSTRACT, NULL, NULL)"
)]
#[doc = c_differences!(
    "Unlike the C version, this macro does not require the class name to be provided."
)]
#[macro_export]
macro_rules! ZEND_ABSTRACT_ME {
    ($name:literal, $arg_info:expr) => {
        rsphp_macros_c::ZEND_RAW_FENTRY!(
            $name,
            NULL,
            $arg_info,
            rsphp_sys::ZEND_ACC_ABSTRACT | rsphp_sys::ZEND_ACC_PUBLIC,
            std::ptr::null(),
            std::ptr::null()
        )
    };
}

/// Define a class function handler entry for an abstract method with the given
/// flags.
#[doc = c_version!(
    "#define ZEND_ABSTRACT_ME_WITH_FLAGS(classname, name, arg_info, flags)    \
        ZEND_RAW_FENTRY(#name, NULL, arg_info, flags, NULL, NULL)"
)]
#[doc = c_differences!(
    "Unlike the C version, this macro does not require the class name to be provided."
)]
#[macro_export]
macro_rules! ZEND_ABSTRACT_ME_WITH_FLAGS {
    ($name:literal, $arg_info:expr, $flags:expr) => {
        rsphp_macros_c::ZEND_RAW_FENTRY!(
            $name,
            NULL,
            $arg_info,
            $flags,
            std::ptr::null(),
            std::ptr::null()
        )
    };
}
