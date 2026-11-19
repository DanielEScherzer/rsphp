//! Aliases for PHP 8.6.

/// The result type.
pub type ZendResultType = rsphp_sys::zend_result::Type;
/// A failure.
pub const ZEND_RESULT_FAILURE: ZendResultType = rsphp_sys::zend_result::FAILURE;
/// A success.
pub const ZEND_RESULT_SUCCESS: ZendResultType = rsphp_sys::zend_result::SUCCESS;
