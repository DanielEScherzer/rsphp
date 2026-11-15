//! Macros corresponding to `zend_globals.h` in php/php-src.

/// Compile time offset of compiler globals in the reserved region before the
/// cache pointer.
///
/// Only defined for PHP 8.6, and only for ZTS.
///
/// # C version
///
/// ```c
/// # define ZEND_CG_OFFSET   (-(ptrdiff_t) TSRM_ALIGNED_SIZE(sizeof(zend_compiler_globals)))
/// ```
///
/// Not actually a macro, but computed using macros that cannot be easily
/// reimplemented for use in bindgen.
#[cfg(any(doc, all(php_zend_zts, php_is_86)))]
pub const ZEND_CG_OFFSET: isize =
    -crate::TSRM_ALIGNED_SIZE!(size_of::<rsphp_sys::zend_compiler_globals>());

/// Compile time offset of executor globals in the reserved region before the
/// cache pointer.
///
/// Only defined for PHP 8.6.
///
/// Not actually a macro, but computed using macros that cannot be easily
/// reimplemented for use in bindgen.
#[cfg(any(doc, all(php_zend_zts, php_is_86)))]
pub const ZEND_EG_OFFSET: isize =
    ZEND_CG_OFFSET - crate::TSRM_ALIGNED_SIZE!(size_of::<rsphp_sys::zend_executor_globals>());
