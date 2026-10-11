use rsphp_macros_c::{RETURN_THROWS, Z_OBJ_P, ZEND_FE_END, ZVAL_BOOL};
use rsphp_macros_rs::{ZEND_ME, ZEND_PARSE_PARAMETERS_NONE, ZEND_THIS};
use rsphp_sys::{
    ZEND_ACC_PUBLIC, ZEND_RESULT_CODE, object_properties_init, std_object_handlers,
    zend_class_entry, zend_execute_data, zend_function_entry, zend_object, zend_object_alloc,
    zend_object_handlers, zend_object_std_init, zend_parse_parameters, zval,
};

pub(crate) static mut opaquedata_ptr: *mut zend_class_entry = std::ptr::null_mut();
pub(crate) static mut opaquedata_object_handlers: zend_object_handlers =
    unsafe { std::mem::zeroed() };

extern "C" fn OpaqueDataConstructor(
    execute_data: *mut zend_execute_data,
    _return_value: *mut zval,
) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
    let mut object = Z_OBJ_P!(unsafe { ZEND_THIS!(execute_data) });
    let data = OpaqueDataObj::from_zend_obj_ptr(&mut object);
    data.opaque_bool = false;
}
extern "C" fn OpaqueDataDestructor(execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };
}
extern "C" fn OpaqueDataGetBool(execute_data: *mut zend_execute_data, return_value: *mut zval) {
    // SAFETY: execute_data is valid and has $this, this is a class method
    unsafe { ZEND_PARSE_PARAMETERS_NONE!(execute_data) };

    let mut object = Z_OBJ_P!(unsafe { ZEND_THIS!(execute_data) });
    let data = OpaqueDataObj::from_zend_obj_ptr(&mut object);
    unsafe {
        ZVAL_BOOL!(return_value, data.opaque_bool);
    }
}
extern "C" fn OpaqueDataSetBool(execute_data: *mut zend_execute_data, _return_value: *mut zval) {
    let mut to_set: bool = false;
    let result = unsafe {
        zend_parse_parameters((*execute_data).This.u2.num_args, c"b".as_ptr(), &mut to_set)
    };
    if result == ZEND_RESULT_CODE::FAILURE {
        RETURN_THROWS!();
    }
    assert_eq!(result, ZEND_RESULT_CODE::SUCCESS, "Must be success");
    let mut object = Z_OBJ_P!(unsafe { ZEND_THIS!(execute_data) });
    let data = OpaqueDataObj::from_zend_obj_ptr(&mut object);
    data.opaque_bool = to_set;
}

pub(crate) const class_opaquedata_methods: [zend_function_entry; 5] = [
    ZEND_ME!(
        "__construct",
        OpaqueDataConstructor,
        crate::opaquedata_arginfo::RustDemo_OpaqueData::ARGINFO___CONSTRUCT,
        ZEND_ACC_PUBLIC
    ),
    ZEND_ME!(
        "__destruct",
        OpaqueDataDestructor,
        crate::opaquedata_arginfo::RustDemo_OpaqueData::ARGINFO___DESTRUCT,
        ZEND_ACC_PUBLIC
    ),
    ZEND_ME!(
        "getBool",
        OpaqueDataGetBool,
        crate::opaquedata_arginfo::RustDemo_OpaqueData::ARGINFO_GETBOOL,
        ZEND_ACC_PUBLIC
    ),
    ZEND_ME!(
        "setBool",
        OpaqueDataSetBool,
        crate::opaquedata_arginfo::RustDemo_OpaqueData::ARGINFO_SETBOOL,
        ZEND_ACC_PUBLIC
    ),
    ZEND_FE_END!(),
];

#[repr(C)]
struct OpaqueDataObj {
    opaque_bool: bool,
    zo: zend_object,
}

impl OpaqueDataObj {
    fn from_zend_obj_ptr(obj: &mut *mut zend_object) -> &mut OpaqueDataObj {
        let offset = std::mem::offset_of!(OpaqueDataObj, zo);
        let opaque_ptr: *mut OpaqueDataObj =
            unsafe { obj.byte_sub(offset) }.cast::<OpaqueDataObj>();
        unsafe { opaque_ptr.as_mut() }.expect("Valid object pointer")
    }
}

pub(crate) unsafe fn init_opaquedata_object_handlers() {
    unsafe {
        std::ptr::copy_nonoverlapping(
            &raw const std_object_handlers,
            &raw mut opaquedata_object_handlers,
            1,
        );
        opaquedata_object_handlers.offset =
            i32::try_from(std::mem::offset_of!(OpaqueDataObj, zo)).expect("Valid i32");
    }
}

pub(crate) extern "C" fn opaquedata_object_new(ce: *mut zend_class_entry) -> *mut zend_object {
    let size = std::mem::size_of::<OpaqueDataObj>();
    let ptr: *mut OpaqueDataObj = unsafe { zend_object_alloc(size, ce) }.cast::<OpaqueDataObj>();
    unsafe {
        zend_object_std_init(&raw mut (*ptr).zo, ce);
        object_properties_init(&raw mut (*ptr).zo, ce);
    }
    unsafe { &raw mut (*ptr).zo }
}
