//! This crate holds rust macros that match (or at least closely imitate) C
//! macros used in the PHP source. For macros that do not match the C version
//! as closely, see the `rsphp-macros-rs` crate.
//!
//! These macros assume that the extension is *not* being statically linked
//! with PHP (i.e. that `ZEND_ENABLE_STATIC_TSRMLS_CACHE` is not enabled for
//! ZTS builds).

#[doc(no_inline)]
pub use c_str_macro::c_str;

/// Utilities for this crate.
pub(crate) mod utils;
pub(crate) use utils::c_differences;
pub(crate) use utils::c_version;

#[cfg(any(doc, php_zend_zts))]
mod tsrm;
mod zend_alloc;
mod zend_api;
mod zend_closures;
mod zend_compile;
mod zend_constants;
mod zend_globals_macros;
mod zend_hash;
mod zend_ini;
mod zend_object_handlers;
mod zend_string;
mod zend_types;
