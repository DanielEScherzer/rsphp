use crate::c_differences;

/// Verify that the execute data has no arguments, throwing a PHP error and
/// return early if there are arguments.
///
/// # Corresponding C macro
///
/// ```c
/// #define ZEND_PARSE_PARAMETERS_NONE() do { \
///         if (UNEXPECTED(ZEND_NUM_ARGS() != 0)) { \
///             zend_wrong_parameters_none_error(); \
///             return; \
///         } \
///     } while (0)
/// ```
#[doc = c_differences!(
    "Unlike the C version, this macro requires the current execute data to be \
        passed in."
)]
#[macro_export]
macro_rules! ZEND_PARSE_PARAMETERS_NONE {
    ($execute_data:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        if (*$execute_data).This.u2.num_args != 0 {
            // SAFETY: we are in a function
            rsphp_sys::zend_wrong_parameters_none_error();
            // ().into() satisfies both unit returns and nice handlers with
            // PHPValue returns
            return ().into();
        }
    };
}

/// Access the `This` pointer of the current execute data.
///
/// # Corresponding C macros
///
/// ```c
/// #define EX(element)             ((execute_data)->element)
/// ```
///
/// ```c
/// #define ZEND_THIS                           (&EX(This))
/// ```
#[doc = c_differences!(
    "Unlike the C version, this macro requires the current execute data to be \
        passed in."
)]
#[macro_export]
macro_rules! ZEND_THIS {
    ($execute_data:expr) => {
        &mut ((*$execute_data).This)
    };
}

/// Set an executor global, accounting for ZTS changes.
///
/// [`rsphp_macros_c::EG`] cannot be used in a write context in Rust, but the
/// corresponding C macro could be used in C; this macro simplifies setting
/// executor globals and accounts for changes when ZTS is enabled.
#[macro_export]
#[cfg(any(doc, not(php_zend_zts)))]
macro_rules! EG_SET {
    ($field:ident, $val:expr) => {
        unsafe { rsphp_sys::executor_globals.$field = $val }
    };
}

/// Set an executor global, accounting for ZTS changes.
///
/// [`rsphp_macros_c::EG`] cannot be used in a write context in Rust, but the
/// corresponding C macro could be used in C; this macro simplifies setting
/// executor globals and accounts for changes when ZTS is enabled.
#[macro_export]
#[cfg(all(not(doc), php_zend_zts))]
macro_rules! EG_SET {
    ($field:ident, $val:expr) => {{
        // Cannot assign through a macro
        // EG!($field) = $val;
        // rsphp_macros_c::TSRMG_FAST!(
        //      rsphp_sys::executor_globals_offset,
        //      rsphp_sys::zend_executor_globals,
        //      $field
        // ) = $val;
        unsafe {
            let all_eg: *mut rsphp_sys::zend_executor_globals = rsphp_macros_c::TSRMG_FAST_BULK!(
                rsphp_sys::executor_globals_offset,
                rsphp_sys::zend_executor_globals
            );
            let all_eg_ref: &mut rsphp_sys::zend_executor_globals = all_eg.as_mut().expect(
                "EG_SET: TSRMG_FAST_BULK pointer should be convertible to mutable reference",
            );
            all_eg_ref.$field = $val;
        }
    }};
}
