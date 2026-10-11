/// Try and mimic ZPP but actually just wrap around [`rsphp_sys::zend_parse_parameters()`]
/// because the actual ZPP macros use way too much global state to easily
/// recreated.
///
/// # Supported fast ZPP macros
///
/// As of PHP 8.5, the fast ZPP system offers the following macros which are
/// also possible to implement with the slow ZPP `zend_parse_parameters()`. The
/// table below excludes the internal `*_EX` variants that are implementation
/// details of the fast ZPP macros.
///
/// Parameters are passed as a mutable reference, i.e. mutable pointers to
/// variables of the specified types. The exceptions to that are:
/// * for `Z_PARAM_OBJECT_OF_CLASS` and similar, the pointer to the actual class
///   entry is passed, rather than a mutable pointer to a variable to assign.
/// * for `Z_PARAM_VARIADIC`, the first parameter is the literal character
///   `+` or `*` rather than a pointer to a character.
///
/// | C macro | Parameters | ZPP specifier | Notes |
/// | -------- | - | - | - |
/// | `Z_PARAM_OPTIONAL` | - | `\|` | |
/// | `Z_PARAM_ARRAY` | `zval *` | `a` | |
/// | `Z_PARAM_ARRAY_OR_NULL` | `zval *` | `a!` | |
/// | `Z_PARAM_ARRAY_OR_OBJECT` | `zval *` | `A` | |
/// | `Z_PARAM_ARRAY_HT` | `HashTable *` | `h` | |
/// | `Z_PARAM_ARRAY_HT_OR_NULL` | `HashTable *` | `h!` | |
/// | `Z_PARAM_ARRAY_OR_OBJECT_HT` | `HashTable *` | `H` | |
/// | `Z_PARAM_BOOL` | `bool` | `b` | |
/// | `Z_PARAM_BOOL_OR_NULL` | `bool`, `bool` | `b!` | Second boolean reflects if null was passed |
/// | `Z_PARAM_CLASS` | `zend_class_entry *` | `C` | |
/// | `Z_PARAM_CLASS_OR_NULL` | `zend_class_entry *` | `C!` | |
/// | `Z_PARAM_DOUBLE` | `double` | `d` | |
/// | `Z_PARAM_DOUBLE_OR_NULL` | `double`, `bool` | `d!` | |
/// | `Z_PARAM_FUNC` | `zend_fcall_info`, `zend_fcall_info_cache` | `f` | |
/// | `Z_PARAM_FUNC_OR_NULL` | `zend_fcall_info`, `zend_fcall_info_cache` | `f!` | |
/// | `Z_PARAM_LONG` | `zend_long` | `l` | |
/// | `Z_PARAM_LONG_OR_NULL` | `zend_long`, `bool` | `l!` | |
/// | `Z_PARAM_NUMBER` | `zval *` | `n` | |
/// | `Z_PARAM_NUMBER_OR_NULL` | `zval *` | `n!` | |
/// | `Z_PARAM_OBJECT` | `zval *` | `o` | |
/// | `Z_PARAM_OBJECT_OR_NULL` | `zval *` | `o!` | |
/// | `Z_PARAM_OBJECT_OF_CLASS` | `zval *`, `zend_class_entry` | `O` | |
/// | `Z_PARAM_OBJECT_OF_CLASS_OR_NULL` | `zval *`, `zend_class_entry` | `O!` | |
/// | `Z_PARAM_PATH` | `char *`, `size_t` | `p` | |
/// | `Z_PARAM_PATH_OR_NULL` | `char *`, `size_t` | `p!` | |
/// | `Z_PARAM_PATH_STR` | `zend_string *` | `P` | |
/// | `Z_PARAM_PATH_STR_OR_NULL` | `zend_string *` | `P!` | |
/// | `Z_PARAM_RESOURCE` | `zval *` | `r` | |
/// | `Z_PARAM_RESOURCE_OR_NULL` | `zval *` | `r!` | |
/// | `Z_PARAM_STRING` | `char *`, `size_t` | `s` | |
/// | `Z_PARAM_STRING_OR_NULL` | `char *`, `size_t` | `s!` | |
/// | `Z_PARAM_STR` | `zend_string *` | `S` | |
/// | `Z_PARAM_STR_OR_NULL` | `zend_string *` | `S!` | |
/// | `Z_PARAM_VARIADIC` | `char`, `zval *`, `uint_32` | `+` or `*` | The character is either `+` or `*` |
/// | `Z_PARAM_ZVAL` | `zval *` | `z` | |
/// | `Z_PARAM_ZVAL_OR_NULL` | `zval *` | `z!` | |
///
/// # Unsupported macros
///
/// The fast ZPP system also has the following macros that are not supported by
/// the [`rsphp_sys::zend_parse_parameters()`] specification:
///
/// | C macro | Parameters |
/// | -------- | - |
/// | `Z_PARAM_ARRAY_HT_OR_LONG` | `HashTable *`, `zend_long` |
/// | `Z_PARAM_ARRAY_HT_OR_LONG_OR_NULL` | `HashTable *`, `zend_long` |
/// | `Z_PARAM_ARRAY_HT_OR_STR` | `HashTable *`, `zend_string *` |
/// | `Z_PARAM_ARRAY_HT_OR_STR_OR_NULL` | `HashTable *`, `zend_string *` |
/// | `Z_PARAM_FUNC_NO_TRAMPOLINE_FREE` | `zend_fcall_info`, `zend_fcall_info_cache` |
/// | `Z_PARAM_FUNC_NO_TRAMPOLINE_FREE_OR_NULL` | `zend_fcall_info`, `zend_fcall_info_cache` |
/// | `Z_PARAM_FUNC_OR_NULL_WITH_ZVAL` | `zend_fcall_info`, `zend_fcall_info_cache`, `zval *` |
/// | `Z_PARAM_ITERABLE` | `zval *` |
/// | `Z_PARAM_ITERABLE_OR_NULL` | `zval *` |
/// | `Z_PARAM_NUMBER_OR_STR` | `zval *` |
/// | `Z_PARAM_NUMBER_OR_STR_OR_NULL` | `zval *` |
/// | `Z_PARAM_OBJ` | `zend_object *` |
/// | `Z_PARAM_OBJ_OR_NULL` | `zend_object *` |
/// | `Z_PARAM_OBJ_OF_CLASS` | `zend_object *`, `zend_class_entry` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_NULL` | `zend_object *`, `zend_class_entry` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_LONG` | `zend_object *`, `zend_class_entry`, `zend_long` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_LONG_OR_NULL` | `zend_object *`, `zend_class_entry`, `zend_long` |
/// | `Z_PARAM_OBJ_OR_CLASS_NAME` | `zend_class_entry *` |
/// | `Z_PARAM_OBJ_OR_CLASS_NAME_OR_NULL` | `zend_class_entry *` |
/// | `Z_PARAM_OBJ_OR_STR` | `zend_object *`, `zend_string *` |
/// | `Z_PARAM_OBJ_OR_STR_OR_NULL` | `zend_object *`, `zend_string *` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_STR` | `zend_object *`, `zend_class_entry`, `zend_string *` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_STR_OR_NULL` | `zend_object *`, `zend_class_entry`, `zend_string *` |
/// | `Z_PARAM_STR_OR_LONG` | `zend_string *`, `zend_long` |
/// | `Z_PARAM_STR_OR_LONG_OR_NULL` | `zend_string *`, `zend_long`, `bool` |
/// | `Z_PARAM_VARIADIC_WITH_NAMED` | `zval *`, `uint_32`, `HashTable *` |
///
/// ## Workarounds
///
/// For the C macros used for [`rsphp_sys::zend_object`] pointers, some have
/// equivalent macros with [`rsphp_sys::zval`] pointers, and *those* could be
/// reproduced with the slow ZPP system.
///
/// | C macro with `zend_object *` | C macros with `zval *` |
/// | -------- | - |
/// | `Z_PARAM_OBJ` | `Z_PARAM_OBJECT` |
/// | `Z_PARAM_OBJ_OR_NULL` | `Z_PARAM_OBJECT_OR_NULL` |
/// | `Z_PARAM_OBJ_OF_CLASS` | `Z_PARAM_OBJECT_OF_CLASS` |
/// | `Z_PARAM_OBJ_OF_CLASS_OR_NULL` | `Z_PARAM_OBJECT_OF_CLASS_OR_NULL` |
///
/// For these, it is suggested to use the `Z_PARAM_OBJECT*` variants, and then
/// manually extract the `zend_object` pointer with [`rsphp_macros_c::Z_OBJ_P`]
/// if applicable.
///
/// For other unsupported ZPP macros, use `Z_PARAM_ZVAL` or `Z_PARAM_ZVAL_OR_NULL`
/// and manually handle the supported cases. If a function would use one of the
/// unsupported C macros and is instead implemented manually, when argument
/// validation fails consider using the same error messages that the fast ZPP
/// system uses. See
///
/// * [`rsphp_sys::zend_wrong_parameter_error()`]
/// * [`rsphp_sys::_zend_expected_type`]
/// * the [`rsphp_sys`] `ZPP_ERROR_*` constants
///
/// ## Variadic with named
///
/// Note that using `Z_PARAM_ZVAL` does not work for replacing
/// `Z_PARAM_VARIADIC_WITH_NAMED`, since the parameter parsing logic in PHP
/// unconditionally a [`ArgumentCountError`] when named extra arguments are
/// present and not expected. Instead, you will need to remove the
/// [`rsphp_sys::ZEND_CALL_HAS_EXTRA_NAMED_PARAMS`] flag from the call
/// information before processing and add it back afterwards, making sure that
/// the flag is restore even if the processing fails to avoid memory leaks.
///
/// For example, the function
///
/// ```php
/// function dump_named_variadic_args(int $initial, mixed ...$args): void {}
/// ```
///
/// Might use fast ZPP as follows:
///
/// ```
/// # extern crate rsphp_macros_rs;
/// # use rsphp_macros_c::{ZEND_ADD_CALL_FLAG, ZEND_CALL_INFO, ZEND_DEL_CALL_FLAG};
/// # use rsphp_macros_rs::ZEND_PARSE_ARGS;
/// # use rsphp_sys::{HashTable, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS, zend_execute_data, zval};
/// # extern "C" fn dump_named_variadic_args(
/// #     execute_data: *mut zend_execute_data,
/// #     _return_value: *mut zval
/// # ) {
/// let mut initial: i64 = 0;
/// let mut num_args: u32 = 0;
/// let mut params: *mut zval = std::ptr::null_mut();
/// let mut named_params: *mut HashTable = std::ptr::null_mut();
/// // SAFETY: PHP ensures that the given execute_data pointer is valid
/// let flags = unsafe { ZEND_CALL_INFO!(execute_data) };
/// let has_extra_named = flags & ZEND_CALL_HAS_EXTRA_NAMED_PARAMS != 0;
/// let mut drop_guard = None;
/// if has_extra_named {
///     // SAFETY: PHP ensures that the given execute_data pointer is valid
///     named_params = unsafe { *execute_data }.extra_named_params;
///     unsafe { ZEND_DEL_CALL_FLAG!(execute_data, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS) };
///     struct DropGuard<F: FnMut()>(F);
///     impl<F: FnMut()> Drop for DropGuard<F> {
///         fn drop(&mut self) {
///             (self.0)();
///         }
///     }
///     // SAFETY: PHP ensures that the given execute_data pointer is valid
///     drop_guard = Some(DropGuard(|| unsafe {
///         ZEND_ADD_CALL_FLAG!(execute_data, ZEND_CALL_HAS_EXTRA_NAMED_PARAMS)
///     }));
/// }
/// // SAFETY: PHP ensures that the given execute_data pointer is valid; correct
/// // argument types are passed for the fast ZPP macro.
/// unsafe {
///     ZEND_PARSE_ARGS!(
///         execute_data
///         Z_PARAM_LONG(initial)
///         Z_PARAM_VARIADIC('*', params, num_args)
///     );
/// }
/// drop(drop_guard);
///
/// # // ...
/// # }
/// ```
///
/// [`ArgumentCountError`]: https://www.php.net/manual/en/class.argumentcounterror.php
#[macro_export]
macro_rules! ZEND_PARSE_ARGS {
    ($execute_data:ident $( $args:tt )+ ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = ( $crate::__impl__ZEND_PARSE_ARGS!( @make_spec [ "" ] [ $( $args )+ ] ) ),
            _ed = $execute_data,
            _vars = [],
            _pending = [ $( $args )+ ],
        )
    };
}

/// Implementation details of the ZEND_PARSE_ARGS macro, not part of the public
/// interface though.
///
/// Using a separate macro means that the output of the `macro_rules!` block
/// in the documentation isn't cluttered needing to show all of these cases
#[doc(hidden)]
#[macro_export]
macro_rules! __impl__ZEND_PARSE_ARGS {
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "a" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY_OR_NULL( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "a!" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY_OR_OBJECT( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "A" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY_HT( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "h" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY_HT_OR_NULL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "h!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ARRAY_OR_OBJECT_HT( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "H" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_BOOL( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "b" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_BOOL_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "b!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_CLASS( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "C" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_CLASS_OR_NULL( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "C!" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_FUNC( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "f" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_FUNC_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "f!" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_LONG( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "l" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_LONG_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "l!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_DOUBLE( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "d" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_DOUBLE_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "d!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_RESOURCE( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "r" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_RESOURCE_OR_NULL( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "r!" ] [ $( $args )* ] )
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_STRING( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "s" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_STRING_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "s!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_STR( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "S" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_STR_OR_NULL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "S!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_PATH( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "p" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_PATH_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "p!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_PATH_STR( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "P" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_PATH_STR_OR_NULL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "P!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_NUMBER( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "n" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_NUMBER_OR_NULL( $v:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "n!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OBJECT( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "o" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OBJECT_OF_CLASS( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "O" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OBJECT_OF_CLASS_OR_NULL( $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "O!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OBJ( $v1:expr ) $( $args:tt )* ] ) => {
        compile_error!("No slow ZPP version of Z_PARAM_OBJ, use Z_PARAM_OBJECT");
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_VARIADIC( $spec:literal, $v1:expr, $v2:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ $spec ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_VARIADIC_WITH_NAMED( $v1:expr ) $( $args:tt )* ] ) => {
        compile_error!("No slow ZPP version of Z_PARAM_VARIADIC_WITH_NAMED, remove the ZEND_CALL_HAS_EXTRA_NAMED_PARAMS flag and use Z_PARAM_VARIADIC");
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OBJECT_OR_NULL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "o!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ZVAL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "z" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_ZVAL_OR_NULL( $v1:expr ) $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "z!" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [ Z_PARAM_OPTIONAL $( $args:tt )* ] ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(@make_spec [ $( $p )+ "|" ] [ $( $args )* ])
    };
    ( @make_spec [ $( $p:expr )+ ] [] ) => {
        concat!( $( $p ),+, "\0" )
    };
    ( @make_spec [ $( $p:expr )+ ] $remainder:tt ) => {
        compile_error!("Unhandled @make_spec");
    };
    // We CANNOT do a similar @make_args to extract the arguments because macros
    // are evaluated outside in and we need to have the arguments extracted before
    // the overall zend_parse_parameters() call can be built
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )+ ],
        _pending = [],
    ) => {
        // The following is unsafe, and is the reason this macro is unsafe
        // SAFETY:
        // - $execute_data should be the execution data pointer passed into
        //   the function; such pointers are provided by PHP as valid and can
        //   be dereferenced
        // - num_args is expected to be the active field of the zval in This,
        //   and is always safe to access given that all of the union fields
        //   are u32 and the union is `repr(C)`
        let num_args = (*$execute_data).This.u2.num_args;
        // SAFETY:
        // - spec and processed arguments should be valid
        let result = rsphp_sys::zend_parse_parameters(
            num_args,
            $spec.as_ptr().cast::<std::ffi::c_char>(),
            $( $processed_var ),+
        );
        if result == rsphp_sys::ZEND_RESULT_CODE::FAILURE {
            rsphp_macros_c::RETURN_THROWS!();
        }
    };
    // We pass `&mut ...` parameters to zend_parse_parameters() for the most
    // part, EXCEPT for Z_PARAM_OBJECT_OF_CLASS where we want to pass the actual
    // pointer to the class - `$processed_var` will be token trees of `()` with
    // the contents of the parameter, either `&mut ...` or the pointer, see
    // Z_PARAM_OBJECT_OF_CLASS special handling below
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ Z_PARAM_OPTIONAL $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ $has_one_arg:ident ( $binding_var:expr ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $binding_var) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ Z_PARAM_OBJECT_OF_CLASS ( $dest:ident, $ce:ident ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $dest) ($ce) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ Z_PARAM_OBJECT_OF_CLASS_OR_NULL ( $dest:ident, $ce:ident ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $dest) ($ce) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ $has_two_args:ident ( $var1:ident, $var2:ident ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $var1) (&mut $var2) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ Z_PARAM_VARIADIC ( $v_spec:literal, $var1:ident, $var2:ident ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $var1) (&mut $var2) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = [ $has_three_args:ident ( $var1:ident, $var2:ident, $var3:ident ) $( $args:tt )* ],
    ) => {
        $crate::__impl__ZEND_PARSE_ARGS!(
            _spec = $spec,
            _ed = $execute_data,
            _vars = [ $( $processed_var )* (&mut $var1) (&mut $var2) (&mut $var3) ],
            _pending = [ $( $args )* ],
        )
    };
    (
        _spec = $spec:tt,
        _ed = $execute_data:ident,
        _vars = [ $( $processed_var:tt )* ],
        _pending = $pending:tt
    ) => {
        compile_error!(
            "Handled pending: spec={}, vars={}, pending={}",
            stringify!($spec),
            stringify!($( $processed_var ),*),
            stringify!($pending)
        );
    };
    ( _spec $( $remainder:tt )+ ) => {
        compile_error!(
            "Unsupported ZPP: {}"
        );
    };
}
