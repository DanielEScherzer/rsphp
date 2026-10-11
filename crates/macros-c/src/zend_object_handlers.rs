//! Macros corresponding to `zend_object_handlers.h` in php/php-src.

use crate::c_version;

/// Check if an integer is a valid property offset.
#[doc = c_version!("#define IS_VALID_PROPERTY_OFFSET(offset)           ((intptr_t)(offset) >= ZEND_FIRST_PROPERTY_OFFSET)")]
#[macro_export]
macro_rules! IS_VALID_PROPERTY_OFFSET {
    ($offset:expr) => {
        $offset >= rsphp_sys::ZEND_FIRST_PROPERTY_OFFSET
    };
}

/// Free a trampoline function.
///
/// # C version
///
/// ```c
/// #define zend_free_trampoline(func) do { \
///         if ((func) == &EG(trampoline)) { \
///             EG(trampoline).common.attributes = NULL; \
///             EG(trampoline).common.function_name = NULL; \
///         } else { \
///             efree(func); \
///         } \
///     } while (0)
/// ```
#[macro_export]
macro_rules! zend_free_trampoline {
    ($func:expr) => {{
        let global_trampoline = &mut $crate::EG!(trampoline);
        if $func == global_trampoline as *mut _ {
            global_trampoline.common.attributes = std::ptr::null_mut();
            global_trampoline.common.function_name = std::ptr::null_mut();
        } else {
            $crate::efree!($func as *mut c_void);
        }
    }};
}
