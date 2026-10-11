use rsphp_macros_c::{
    INI_BOOL, INI_FLT, INI_INT, INI_STR, ZEND_FE_END, ZEND_INI_END, ZEND_INI_ENTRY, ZVAL_BOOL,
    ZVAL_DOUBLE, ZVAL_LONG, ZVAL_STR_COPY, ZVAL_STRING,
};
use rsphp_macros_rs::{ZEND_ME, ZEND_PARSE_PARAMETERS_NONE};
use rsphp_sys::{
    ZEND_ACC_PUBLIC, ZEND_ACC_STATIC, zend_class_entry, zend_execute_data, zend_function_entry,
    zend_ini_entry_def, zend_ini_str, zval,
};

pub(crate) static mut ini_demo_ptr: *mut zend_class_entry = std::ptr::null_mut();

extern "C" fn ini_demo_get_int(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let result = INI_INT!("rustdemo.ini_setting");
    unsafe { ZVAL_LONG!(return_value, result) };
}

extern "C" fn ini_demo_get_float(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let result = INI_FLT!("rustdemo.ini_setting");
    unsafe { ZVAL_DOUBLE!(return_value, result) };
}

extern "C" fn ini_demo_get_string(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let result = INI_STR!("rustdemo.ini_setting");
    unsafe { ZVAL_STRING!(return_value, result) };
}

extern "C" fn ini_demo_get_string_untruncated(
    execute_data: *mut zend_execute_data,
    return_value: *mut zval,
) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    // SAFETY: length passed is correct
    let result = unsafe {
        zend_ini_str(
            c"rustdemo.ini_setting".as_ptr(),
            c"rustdemo.ini_setting".count_bytes(),
            false,
        )
    };
    unsafe { ZVAL_STR_COPY!(return_value, result) };
}

extern "C" fn ini_demo_get_bool(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let result = INI_BOOL!("rustdemo.ini_setting");
    unsafe {
        ZVAL_BOOL!(return_value, result);
    };
}

pub(crate) const class_ini_demo_methods: [zend_function_entry; 6] = [
    ZEND_ME!(
        "get_int",
        ini_demo_get_int,
        crate::ini_demo_arginfo::IniDemo::ARGINFO_GET_INT,
        ZEND_ACC_PUBLIC | ZEND_ACC_STATIC
    ),
    ZEND_ME!(
        "get_float",
        ini_demo_get_float,
        crate::ini_demo_arginfo::IniDemo::ARGINFO_GET_FLOAT,
        ZEND_ACC_PUBLIC | ZEND_ACC_STATIC
    ),
    ZEND_ME!(
        "get_string",
        ini_demo_get_string,
        crate::ini_demo_arginfo::IniDemo::ARGINFO_GET_STRING,
        ZEND_ACC_PUBLIC | ZEND_ACC_STATIC
    ),
    ZEND_ME!(
        "get_string_untruncated",
        ini_demo_get_string_untruncated,
        crate::ini_demo_arginfo::IniDemo::ARGINFO_GET_STRING_UNTRUNCATED,
        ZEND_ACC_PUBLIC | ZEND_ACC_STATIC
    ),
    ZEND_ME!(
        "get_bool",
        ini_demo_get_bool,
        crate::ini_demo_arginfo::IniDemo::ARGINFO_GET_BOOL,
        ZEND_ACC_PUBLIC | ZEND_ACC_STATIC
    ),
    ZEND_FE_END!(),
];

pub(crate) const demo_ini_entries: [zend_ini_entry_def; 2] = [
    ZEND_INI_ENTRY!(
        "rustdemo.ini_setting",
        "1234",
        rsphp_sys::ZEND_INI_ALL,
        None
    ),
    ZEND_INI_END!(),
];
