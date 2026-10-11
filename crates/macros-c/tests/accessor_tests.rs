//! Tests for some of the macros for accessing zval values.

macro_rules! verify_field_accessors {
    ($($fn:ident => $macro:ident,)+) => {
        $(
            #[test]
            #[cfg_attr(feature = "verify_zval_types", should_panic = "Zval type expected to match")]
            fn $fn() {
                // SAFETY: zvals can be initialized with zero bytes
                let zval: rsphp_sys::zval = unsafe { std::mem::zeroed() };
                let _result = rsphp_macros_c::$macro!(zval);
            }
        )+
    }
}
macro_rules! verify_pointer_accessors {
    ($($fn:ident => $macro:ident,)+) => {
        $(
            #[test]
            #[cfg_attr(feature = "verify_zval_types", should_panic = "Zval type expected to match")]
            fn $fn() {
                // SAFETY: zvals can be initialized with zero bytes
                let zval: rsphp_sys::zval = unsafe { std::mem::zeroed() };
                // SAFETY: pointer is created from a reference and is valid to read from
                let _result = unsafe { rsphp_macros_c::$macro!(std::ptr::from_ref(&zval)) };
            }
        )+
    }
}

verify_field_accessors!(
    non_int => Z_LVAL,
    non_float => Z_DVAL,
    non_str => Z_STR,
    non_array => Z_ARR,
    non_object => Z_OBJ,
    non_ref => Z_REF,
    non_ast => Z_AST,
    non_indirect => Z_INDIRECT,
    non_ptr => Z_PTR,
);

verify_pointer_accessors!(
    ptr_non_int => Z_LVAL_P,
    ptr_non_float => Z_DVAL_P,
    ptr_non_str => Z_STR_P,
    ptr_non_array => Z_ARR_P,
    ptr_non_object => Z_OBJ_P,
    ptr_non_ref => Z_REF_P,
    ptr_non_ast => Z_AST_P,
    ptr_non_indirect => Z_INDIRECT_P,
    ptr_non_ptr => Z_PTR_P,
);
