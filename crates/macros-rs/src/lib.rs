//! This crate holds rust macros that do not closely match C macros used in the
//! PHP source, but are expected to be useful for multiple extensions. For
//! macros do match C macros, see the `rsphp-macros-c` crate.

/// Utilities for this crate.
pub(crate) mod utils;
pub(crate) use utils::c_differences;
pub(crate) use utils::c_version;

/// Macros to mimic parts of the arginfo system from PHP's C macros.
mod arginfo;
/// General macros to mostly replace C macros.
mod macros;
/// Fast-ZPP-like interface for processing arguments using the slow ZPP
/// [`rsphp_sys::zend_parse_parameters()`].
mod zpp;
