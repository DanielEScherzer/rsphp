use crate::stack_types::StackZendString;
use rsphp_macros_c::{CG, EG, Z_OBJ, ZSTR_KNOWN, ZVAL_LONG, ZVAL_STR};
use rsphp_sys::{
    _zend_known_string_id::ZEND_STR_CODE, _zend_known_string_id::ZEND_STR_MESSAGE, E_ERROR,
    instanceof_function, object_init_ex, zend_ce_error, zend_ce_throwable, zend_class_entry,
    zend_error_noreturn, zend_throw_exception_internal, zend_update_property_ex, zval,
};

/// Method holder for methods relating to throwing exceptions and errors.
pub struct PHPExceptions;

impl PHPExceptions {
    /// Throw an exception (or error) with the given class and message.
    ///
    /// # SAFETY
    ///
    /// `ce` must be a valid pointer to a class entry to use.
    ///
    /// # Panics
    ///
    /// `ce` must be a subclass of `zend_ce_throwable`.
    pub unsafe fn throw(ce: *mut zend_class_entry, message: &str) {
        // SAFETY: upheld by caller
        unsafe { PHPExceptions::do_throw(ce, message, None) };
    }

    /// Throw an exception (or error) with the given class, message, and code.
    ///
    /// # SAFETY
    ///
    /// `ce` must be a valid pointer to a class entry to use.
    ///
    /// # Panics
    ///
    /// `ce` must be a subclass of `zend_ce_throwable`.
    pub unsafe fn throw_with_code(ce: *mut zend_class_entry, message: &str, code: i64) {
        // SAFETY: upheld by caller
        unsafe { PHPExceptions::do_throw(ce, message, Some(code)) };
    }

    /// # SAFETY
    ///
    /// `ce` must be a valid pointer to a class entry to use.
    ///
    /// # Panics
    ///
    /// `ce` must be a subclass of `zend_ce_throwable`.
    unsafe fn do_throw(ce: *mut zend_class_entry, message: &str, code_opt: Option<i64>) {
        // Valid UTF8 can contain interior null bytes, in which case the use
        // of strlen() in zend_throw_exception() will be problematic. But
        // even if there are no interior null bytes, Rust strings are not
        // null terminated and so strlen() would be problematic.
        let mut message_zend = StackZendString::new(message);

        // Unfortunately zend_throw_exception_zstr is static within
        // zend_exceptions.c so we cannot use it here, reimplement it
        // SAFETY: zend_ce_throwable is built in to PHP and always available
        let ce_throwable = unsafe { zend_ce_throwable };
        assert!(
            // SAFETY: both pointers are valid
            unsafe { instanceof_function(ce, ce_throwable) },
            "Exceptions must implement throwable"
        );

        // SAFETY: zvals can be initialized with zero bytes
        let mut exception_zv: zval = unsafe { std::mem::zeroed() };
        // SAFETY: object_init_ex() is given a valid class entry and a valid
        // zval pointer
        unsafe { object_init_ex(&raw mut exception_zv, ce) };

        {
            // SAFETY: zvals can be initialized with zero bytes
            let mut message_zv: zval = unsafe { std::mem::zeroed() };
            // SAFETY: message_zv is valid to write to
            unsafe { ZVAL_STR!(&raw mut message_zv, message_zend.mut_ptr()) };
            // SAFETY: zend_update_property_ex is given valid arguments, caller
            // must provide a valid `ce`
            unsafe {
                zend_update_property_ex(
                    ce,
                    Z_OBJ!(exception_zv),
                    ZSTR_KNOWN!(ZEND_STR_MESSAGE),
                    &raw mut message_zv,
                );
            }
        }

        if let Some(code) = code_opt {
            // SAFETY: zvals can be initialized with zero bytes
            let mut code_zv: zval = unsafe { std::mem::zeroed() };
            // SAFETY: code_zv is valid to write to
            unsafe { ZVAL_LONG!(&raw mut code_zv, code) };
            // SAFETY: zend_update_property_ex is given valid arguments, caller
            // must provide a valid `ce`
            unsafe {
                zend_update_property_ex(
                    ce,
                    Z_OBJ!(exception_zv),
                    ZSTR_KNOWN!(ZEND_STR_CODE),
                    &raw mut code_zv,
                );
            }
        }

        // SAFETY: object can be thrown
        unsafe { zend_throw_exception_internal(Z_OBJ!(exception_zv)) };
    }

    /// Matches the [`rsphp_sys::zend_throw_error()`] function in C.
    ///
    /// # Panics
    ///
    /// Panics if [`rsphp_sys::E_ERROR`] is defined to a number that does not
    /// fit into a [`std::ffi::c_int`]; this should never happen unless the
    /// definition for the Rust code has been adjusted manually.
    pub fn throw_error(message: &str) {
        if std::ptr::eq(EG!(exception), std::ptr::without_provenance(usize::MAX)) {
            return;
        }

        if !EG!(current_execute_data).is_null() && !CG!(in_compilation) {
            // SAFETY: zend_ce_error is built in to PHP and always available
            let ce_error = unsafe { zend_ce_error };
            // SAFETY: ce_error must be valid
            unsafe { PHPExceptions::throw(ce_error, message) };
        } else {
            // SAFETY: matches zend_throw_error
            unsafe {
                // Yes, this will cut off at a null byte
                // Yes, that is what the C version does too
                zend_error_noreturn(
                    E_ERROR
                        .try_into()
                        .expect("E_ERROR should be valid as c_int"),
                    c"%s".as_ptr(),
                    message.as_ptr(),
                )
            };
        }
    }
}
