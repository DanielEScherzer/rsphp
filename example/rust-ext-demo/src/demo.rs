use rsphp::extensions::ZendStringExt as _;
use rsphp_compat::{ZEND_RESULT_FAILURE, ZEND_RESULT_SUCCESS};
use rsphp_macros_c::{
    RETURN_THROWS, Z_LVAL_P, Z_OBJ_P, Z_OBJCE_P, Z_STR_P, ZEND_FE_END, ZVAL_BOOL, ZVAL_STR,
};
use rsphp_macros_rs::{ZEND_ME, ZEND_PARSE_PARAMETERS_NONE, ZEND_THIS};
use rsphp_sys::{
    ZEND_ACC_PUBLIC, zend_ce_arithmetic_error, zend_class_entry, zend_enum_fetch_case_name,
    zend_execute_data, zend_function_entry, zend_object, zend_parse_parameters, zend_read_property,
    zend_string, zend_string_init, zend_throw_exception, zend_update_property_long, zval,
};
use std::ffi::c_char;

pub(crate) static mut rs_demo_ptr: *mut zend_class_entry = std::ptr::null_mut();
pub(crate) static mut color_ptr: *mut zend_class_entry = std::ptr::null_mut();

extern "C" fn RSDemoConstructor(execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    let mut initial_value: i64 = -1;
    let result = unsafe {
        zend_parse_parameters(
            (*execute_data).This.u2.num_args,
            c"l".as_ptr(),
            &mut initial_value,
        )
    };
    if result == ZEND_RESULT_FAILURE {
        RETURN_THROWS!();
    }
    assert_eq!(result, ZEND_RESULT_SUCCESS, "Must be success");
    unsafe {
        zend_update_property_long(
            rs_demo_ptr,
            Z_OBJ_P!(ZEND_THIS!(execute_data)),
            c"inner".as_ptr(),
            5,
            initial_value,
        );
    }
}

extern "C" fn RSDemoIncrement(execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let ptr = std::ptr::null_mut();
    let current_zval = unsafe {
        zend_read_property(
            Z_OBJCE_P!(ZEND_THIS!(execute_data)),
            Z_OBJ_P!(ZEND_THIS!(execute_data)),
            c"inner".as_ptr(),
            c"inner".count_bytes(),
            true,
            ptr,
        )
    };
    let current = unsafe { Z_LVAL_P!(current_zval) };
    let Some(new) = current.checked_add(1) else {
        unsafe {
            zend_throw_exception(
                zend_ce_arithmetic_error,
                c"Attempting to increment beyond maximum value".as_ptr(),
                0,
            )
        };
        RETURN_THROWS!();
    };
    unsafe {
        zend_update_property_long(
            rs_demo_ptr,
            Z_OBJ_P!(ZEND_THIS!(execute_data)),
            c"inner".as_ptr(),
            5,
            new,
        );
    }
}

extern "C" fn RSDemoAsString(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let ptr = std::ptr::null_mut();
    let current_zval = unsafe {
        zend_read_property(
            Z_OBJCE_P!(ZEND_THIS!(execute_data)),
            Z_OBJ_P!(ZEND_THIS!(execute_data)),
            c"inner".as_ptr(),
            c"inner".count_bytes(),
            true,
            ptr,
        )
    };
    let current = unsafe { Z_LVAL_P!(current_zval) };
    let result = current.to_string();
    let result: *mut zend_string = unsafe {
        zend_string_init(
            result.as_bytes().as_ptr().cast::<c_char>(),
            result.len(),
            false,
        )
    };
    unsafe { ZVAL_STR!(return_value, result) };
}

extern "C" fn ColorIsGreyScale(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };

    let enum_obj: *mut zend_object = unsafe { Z_OBJ_P!(ZEND_THIS!(execute_data)) };
    let name_obj: *const zval = unsafe { zend_enum_fetch_case_name(enum_obj) };
    let name: *const zend_string = unsafe { Z_STR_P!(name_obj) };
    let name_ref = unsafe { name.as_ref() }.expect("Valid reference");
    let name_str = name_ref.as_bstr();
    let result = name_str == "White" || name_str == "Black";
    unsafe {
        ZVAL_BOOL!(return_value, result);
    }
}

pub(crate) const class_RSDemo_methods: [zend_function_entry; 4] = [
    ZEND_ME!(
        "__construct",
        RSDemoConstructor,
        crate::rustdemo_arginfo::RSDemo::ARGINFO___CONSTRUCT,
        ZEND_ACC_PUBLIC
    ),
    ZEND_ME!(
        "increment",
        RSDemoIncrement,
        crate::rustdemo_arginfo::RSDemo::ARGINFO_INCREMENT,
        ZEND_ACC_PUBLIC
    ),
    ZEND_ME!(
        "asString",
        RSDemoAsString,
        crate::rustdemo_arginfo::RSDemo::ARGINFO_ASSTRING,
        ZEND_ACC_PUBLIC
    ),
    ZEND_FE_END!(),
];

pub(crate) const enum_Color_methods: &[zend_function_entry] = &[
    ZEND_ME!(
        "isGreyScale",
        ColorIsGreyScale,
        crate::rustdemo_arginfo::Color::ARGINFO_ISGREYSCALE,
        ZEND_ACC_PUBLIC
    ),
    ZEND_FE_END!(),
];
