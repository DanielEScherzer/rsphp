#![expect(non_snake_case, non_upper_case_globals)]

use bstr::BStr;
use rsphp::wrappers::{HashRepr, InfoTablePrinter};
use rsphp_macros_c::{
    HT_IS_PACKED, RETURN_THROWS, Z_ARR_P, Z_DVAL_P, Z_LVAL_P, Z_OBJCE_P, Z_STR_P, Z_TYPE_P,
    ZEND_ADD_CALL_FLAG, ZEND_CALL_INFO, ZEND_DEL_CALL_FLAG, ZEND_FE_END, ZSTR_LEN, ZSTR_VAL,
    ZVAL_STR,
};
use rsphp_macros_rs::{ZEND_FE, ZEND_PARSE_ARGS, ZEND_PARSE_PARAMETERS_NONE};
use rsphp_sys::{
    HashTable, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS, ZEND_RESULT_CODE, display_ini_entries,
    php_output_write, php_printf, php_var_dump, zend_execute_data, zend_function_entry,
    zend_module_entry, zend_parse_parameters, zend_register_ini_entries_ex, zend_result,
    zend_string, zend_string_init, zend_unregister_ini_entries_ex, zval,
};
use std::ffi::{c_char, c_int};

mod demo;
use demo::{class_RSDemo_methods, rs_demo_ptr};

mod opaquedata;
use opaquedata::{
    class_opaquedata_methods, init_opaquedata_object_handlers, opaquedata_object_handlers,
    opaquedata_object_new, opaquedata_ptr,
};

mod ini_demo;
use ini_demo::{class_ini_demo_methods, ini_demo_ptr};

#[rustfmt::skip]
#[expect(clippy::cast_possible_wrap)]
mod rustdemo_arginfo;
#[rustfmt::skip]
mod opaquedata_arginfo;
#[rustfmt::skip]
mod ini_demo_arginfo;

extern "C" fn my_module_info(entry: *mut zend_module_entry) {
    InfoTablePrinter::print(&[&["RustDemo extension", "enabled"]]);

    unsafe {
        display_ini_entries(entry);
    }
}

extern "C" fn my_module_init(type_: c_int, module_number: c_int) -> zend_result {
    unsafe {
        rs_demo_ptr = rustdemo_arginfo::RSDemo::register_class(&class_RSDemo_methods);
        opaquedata_ptr =
            opaquedata_arginfo::RustDemo_OpaqueData::register_class(&class_opaquedata_methods);
        init_opaquedata_object_handlers();
        (*opaquedata_ptr).default_object_handlers = &raw const opaquedata_object_handlers;
        (*opaquedata_ptr).__bindgen_anon_2.create_object = Some(opaquedata_object_new);

        ini_demo_ptr = ini_demo_arginfo::IniDemo::register_class(&class_ini_demo_methods);

        zend_register_ini_entries_ex(ini_demo::demo_ini_entries.as_ptr(), module_number, type_);
    }
    ZEND_RESULT_CODE::SUCCESS
}

extern "C" fn my_module_shutdown(type_: c_int, module_number: c_int) -> zend_result {
    unsafe {
        zend_unregister_ini_entries_ex(module_number, type_);
    }
    ZEND_RESULT_CODE::SUCCESS
}

extern "C" fn test1(exd: *mut zend_execute_data, _return_value: *mut zval) {
    // SAFETY: exd is valid and has $this, this is a function
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(exd) };
    let to_print = c"The extension rustdemo is loaded and working!\r\n";
    unsafe {
        php_output_write(to_print.as_ptr(), to_print.count_bytes());
    }
}
extern "C" fn test2(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let mut str_ptr: *const c_char = c"World".as_ptr();
    let mut str_len: usize = 5;
    let result = unsafe {
        zend_parse_parameters(
            (*execute_data).This.u2.num_args,
            c"|s".as_ptr(),
            &mut str_ptr,
            &mut str_len,
        )
    };
    if result == ZEND_RESULT_CODE::FAILURE {
        RETURN_THROWS!();
    }
    assert_eq!(result, ZEND_RESULT_CODE::SUCCESS, "Must be success");
    let str_slice: &[u8] = unsafe { std::slice::from_raw_parts(str_ptr.cast::<u8>(), str_len) };
    let rust_str = BStr::new(str_slice);
    // CANNOT use CString since PHP strings can have null bytes in the middle
    let result = format!("Hello {rust_str}");
    let result: *mut zend_string = unsafe {
        zend_string_init(
            result.as_bytes().as_ptr().cast::<c_char>(),
            result.len(),
            false,
        )
    };
    unsafe { ZVAL_STR!(return_value, result) };
}
extern "C" fn test3(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let mut str_ptr: *mut zend_string = std::ptr::null_mut();
    let result = unsafe {
        zend_parse_parameters(
            (*execute_data).This.u2.num_args,
            c"S".as_ptr(),
            &mut str_ptr,
        )
    };
    if result == ZEND_RESULT_CODE::FAILURE {
        RETURN_THROWS!();
    }
    assert_eq!(result, ZEND_RESULT_CODE::SUCCESS, "Must be success");
    let str_slice: &[u8] =
        unsafe { std::slice::from_raw_parts((*str_ptr).val.as_ptr().cast::<u8>(), (*str_ptr).len) };
    let rust_str = BStr::new(str_slice);
    // CANNOT use CString since PHP strings can have null bytes in the middle
    let result = format!("Hello3 {rust_str}");
    let result: *mut zend_string = unsafe {
        zend_string_init(
            result.as_bytes().as_ptr().cast::<c_char>(),
            result.len(),
            false,
        )
    };
    unsafe { ZVAL_STR!(return_value, result) };
}
extern "C" fn dump_named_variadic_args(
    execute_data: *mut zend_execute_data,
    _return_value: *mut zval,
) {
    struct DropGuard<F: FnMut()>(F);
    impl<F: FnMut()> Drop for DropGuard<F> {
        fn drop(&mut self) {
            (self.0)();
        }
    }

    let mut initial: i64 = 0;
    let mut num_args: u32 = 0;
    let mut params: *mut zval = std::ptr::null_mut();
    let mut named_params: *mut HashTable = std::ptr::null_mut();
    // SAFETY: PHP ensures that the given execute_data pointer is valid
    let flags = unsafe { ZEND_CALL_INFO!(execute_data) };
    let has_extra_named = flags & ZEND_CALL_HAS_EXTRA_NAMED_PARAMS != 0;
    let mut drop_guard = None;
    if has_extra_named {
        // SAFETY: PHP ensures that the given execute_data pointer is valid
        named_params = unsafe { *execute_data }.extra_named_params;
        unsafe { ZEND_DEL_CALL_FLAG!(execute_data, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS) };
        // SAFETY: PHP ensures that the given execute_data pointer is valid
        drop_guard = Some(DropGuard(|| unsafe {
            ZEND_ADD_CALL_FLAG!(execute_data, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS);
        }));
    }
    // SAFETY: PHP ensures that the given execute_data pointer is valid; correct
    // argument types are passed for the fast ZPP macro.
    unsafe {
        ZEND_PARSE_ARGS!(
            execute_data
            Z_PARAM_LONG(initial)
            Z_PARAM_VARIADIC('*', params, num_args)
        );
    }
    drop(drop_guard);

    let to_print = format!("Initial value: {initial}\n");
    unsafe {
        php_output_write(to_print.as_ptr().cast::<c_char>(), to_print.len());
    }
    for idx in 0..num_args {
        let param_p = unsafe { params.add(idx.try_into().unwrap()) };
        unsafe {
            php_var_dump(param_p, 1);
        }
    }
    if has_extra_named {
        let repr = unsafe { HashRepr::from_pointer(named_params) };
        for (_idx, key_opt, mut value) in repr {
            if let Some(key) = key_opt {
                unsafe {
                    php_printf(c"key:%S\n".as_ptr(), std::ptr::from_ref(key));
                }
            } else {
                unsafe {
                    php_printf(c"MISSING KEY??\n".as_ptr());
                }
            }
            unsafe { php_var_dump(std::ptr::from_mut(&mut value), 1) };
        }
    }
}
extern "C" fn zpp_testing(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let mut bool_val = true;
    let mut nullable_bool_val = true;
    let mut nullable_bool_null = false;
    let mut long_val = 1;
    let mut nullable_long_val = 1;
    let mut nullable_long_null = false;
    let mut double_val = 1.0;
    let mut nullable_double_val = 1.0;
    let mut nullable_double_null = false;
    let mut string_ptr: *const c_char = std::ptr::null_mut();
    let mut string_len: usize = 0;
    let mut nullable_string_ptr: *const c_char = std::ptr::null_mut();
    let mut nullable_string_len: usize = 0;

    let result = unsafe {
        zend_parse_parameters(
            (*execute_data).This.u2.num_args,
            c"bb!ll!dd!ss!".as_ptr(),
            &mut bool_val,
            &mut nullable_bool_val,
            &mut nullable_bool_null,
            &mut long_val,
            &mut nullable_long_val,
            &mut nullable_long_null,
            &mut double_val,
            &mut nullable_double_val,
            &mut nullable_double_null,
            &mut string_ptr,
            &mut string_len,
            &mut nullable_string_ptr,
            &mut nullable_string_len,
        )
    };
    if result == ZEND_RESULT_CODE::FAILURE {
        RETURN_THROWS!();
    }
    let string_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(string_ptr.cast::<u8>(), string_len) };
    let string_rust = BStr::new(string_slice);

    let mut nullable_string_rust = BStr::new("null");
    if !nullable_string_ptr.is_null() {
        let string_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(nullable_string_ptr.cast::<u8>(), nullable_string_len)
        };
        nullable_string_rust = BStr::new(string_slice);
    }

    let result = format!(
        "bool={}, ?bool={}, long={long_val}, ?long={}, double={double_val}, ?double={}, string={string_rust}, ?string={nullable_string_rust}",
        if bool_val { "true" } else { "false" },
        if nullable_bool_null {
            "null"
        } else if nullable_bool_val {
            "true"
        } else {
            "false"
        },
        if nullable_long_null {
            "null".to_string()
        } else {
            nullable_long_val.to_string()
        },
        if nullable_double_null {
            "null".to_string()
        } else {
            nullable_double_val.to_string()
        },
    );
    let result: *mut zend_string = unsafe {
        zend_string_init(
            result.as_bytes().as_ptr().cast::<c_char>(),
            result.len(),
            false,
        )
    };
    unsafe { ZVAL_STR!(return_value, result) };
}
extern "C" fn zpp_testing_macro(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    let mut bool_val = true;
    let mut nullable_bool_val = true;
    let mut nullable_bool_null = false;
    let mut long_val = 1;
    let mut nullable_long_val = 1;
    let mut nullable_long_null = false;
    let mut double_val = 1.0;
    let mut nullable_double_val = 1.0;
    let mut nullable_double_null = false;
    let mut string_ptr: *const c_char = std::ptr::null_mut();
    let mut string_len: usize = 0;
    let mut nullable_string_ptr: *const c_char = std::ptr::null_mut();
    let mut nullable_string_len: usize = 0;
    // SAFETY: rust implementation of fast ZPP is used properly
    unsafe {
        rsphp_macros_rs::ZEND_PARSE_ARGS!(
            execute_data
            Z_PARAM_BOOL(bool_val)
            Z_PARAM_BOOL_OR_NULL(nullable_bool_val, nullable_bool_null)
            Z_PARAM_LONG(long_val)
            Z_PARAM_LONG_OR_NULL(nullable_long_val, nullable_long_null)
            Z_PARAM_DOUBLE(double_val)
            Z_PARAM_DOUBLE_OR_NULL(nullable_double_val, nullable_double_null)
            Z_PARAM_STRING(string_ptr, string_len)
            Z_PARAM_STRING_OR_NULL(nullable_string_ptr, nullable_string_len)
        );
    }
    let string_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(string_ptr.cast::<u8>(), string_len) };
    let string_rust = BStr::new(string_slice);

    let mut nullable_string_rust = BStr::new("null");
    if !nullable_string_ptr.is_null() {
        let string_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(nullable_string_ptr.cast::<u8>(), nullable_string_len)
        };
        nullable_string_rust = BStr::new(string_slice);
    }
    let result = format!(
        "bool={}, ?bool={}, long={}, ?long={}, double={}, ?double={}, string={string_rust}, ?string={nullable_string_rust}",
        if bool_val { "true" } else { "false" },
        if nullable_bool_null {
            "null"
        } else if nullable_bool_val {
            "true"
        } else {
            "false"
        },
        long_val,
        if nullable_long_null {
            "null".to_string()
        } else {
            nullable_long_val.to_string()
        },
        double_val,
        if nullable_double_null {
            "null".to_string()
        } else {
            nullable_double_val.to_string()
        },
    );
    let result: *mut zend_string = unsafe {
        zend_string_init(
            result.as_bytes().as_ptr().cast::<c_char>(),
            result.len(),
            false,
        )
    };
    unsafe { ZVAL_STR!(return_value, result) };
}

extern "C" fn display_value(execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    let mut value: *mut zval = std::ptr::null_mut();
    let result = unsafe {
        zend_parse_parameters((*execute_data).This.u2.num_args, c"z".as_ptr(), &mut value)
    };
    if result == ZEND_RESULT_CODE::FAILURE {
        RETURN_THROWS!();
    }
    let mut description = match unsafe { Z_TYPE_P!(value) } {
        rsphp_sys::IS_UNDEF => "undef???".to_string(),
        rsphp_sys::IS_NULL => "null".to_string(),
        rsphp_sys::IS_FALSE => "false".to_string(),
        rsphp_sys::IS_TRUE => "true".to_string(),
        rsphp_sys::IS_LONG => unsafe { Z_LVAL_P!(value) }.to_string(),
        rsphp_sys::IS_DOUBLE => format!("{:e}", unsafe { Z_DVAL_P!(value) }),
        rsphp_sys::IS_STRING => {
            let content = unsafe { Z_STR_P!(value) };
            let string_slice: &[u8] = unsafe {
                std::slice::from_raw_parts(ZSTR_VAL!(content).cast::<u8>(), ZSTR_LEN!(content))
            };
            format!("string: \"{}\"", BStr::new(string_slice))
        }
        rsphp_sys::IS_ARRAY => {
            let ht = unsafe { Z_ARR_P!(value) };
            let is_packed = unsafe { HT_IS_PACKED!(ht) };
            let num_used = unsafe { (*ht).nNumUsed };
            let num_elements = unsafe { (*ht).nNumOfElements };
            format!(
                "{} array, num_used = {num_used}, num_elements = {num_elements}",
                if is_packed { "packed" } else { "non-packed" },
            )
        }
        rsphp_sys::IS_OBJECT => {
            let ce = unsafe { Z_OBJCE_P!(value) };
            let name = unsafe { (*ce).name };
            let string_slice: &[u8] = unsafe {
                std::slice::from_raw_parts(ZSTR_VAL!(name).cast::<u8>(), ZSTR_LEN!(name))
            };
            format!("Instance of {}", BStr::new(string_slice))
        }
        _ => "unknown???".to_string(),
    };
    description += "\n";
    unsafe {
        php_output_write(description.as_ptr().cast::<c_char>(), description.len());
    }
}

const EXT_FUNCTIONS: [zend_function_entry; 8] = [
    ZEND_FE!("rd_test1", test1, rustdemo_arginfo::ARGINFO_RD_TEST1),
    ZEND_FE!("rd_test2", test2, rustdemo_arginfo::ARGINFO_RD_TEST2),
    ZEND_FE!("rd_test3", test3, rustdemo_arginfo::ARGINFO_RD_TEST3),
    ZEND_FE!(
        "display_value",
        display_value,
        rustdemo_arginfo::ARGINFO_DISPLAY_VALUE
    ),
    ZEND_FE!(
        "zpp_testing",
        zpp_testing,
        rustdemo_arginfo::ARGINFO_ZPP_TESTING
    ),
    ZEND_FE!(
        "zpp_testing_macro",
        zpp_testing_macro,
        rustdemo_arginfo::ARGINFO_ZPP_TESTING_MACRO
    ),
    ZEND_FE!(
        "dump_named_variadic_args",
        dump_named_variadic_args,
        rustdemo_arginfo::ARGINFO_DUMP_NAMED_VARIADIC_ARGS
    ),
    ZEND_FE_END!(),
];

static mut MY_MODULE_ENTRY: zend_module_entry = zend_module_entry {
    // TODO STANDARD_MODULE_HEADER
    // This matches the C STANDARD_MODULE_HEADER
    #[expect(clippy::cast_possible_truncation)]
    size: std::mem::size_of::<zend_module_entry>() as u16,
    zend_api: rsphp_sys::ZEND_MODULE_API_NO,
    zend_debug: rsphp_sys::ZEND_DEBUG,
    zts: rsphp_sys::USING_ZTS,
    ini_entry: std::ptr::null_mut(),
    deps: std::ptr::null_mut(),
    name: c"rustdemo".as_ptr(),
    functions: EXT_FUNCTIONS.as_ptr(),
    module_startup_func: Some(my_module_init),
    module_shutdown_func: Some(my_module_shutdown),
    request_startup_func: None,
    request_shutdown_func: None,
    info_func: Some(my_module_info),
    version: c"0.1.0".as_ptr(),
    globals_size: 0,
    #[cfg(not(php_zend_zts))]
    globals_ptr: std::ptr::null_mut(),
    #[cfg(php_zend_zts)]
    globals_id_ptr: std::ptr::null_mut(),
    globals_ctor: None,
    globals_dtor: None,
    post_deactivate_func: None,
    module_started: 0,
    type_: 0,
    handle: std::ptr::null_mut(),
    module_number: 0,
    build_id: rsphp_sys::ZEND_MODULE_BUILD_ID.as_ptr().cast::<c_char>(),
};

#[unsafe(no_mangle)]
pub extern "C" fn get_module() -> *mut zend_module_entry {
    &raw mut MY_MODULE_ENTRY
}
