//! Aliases for PHP 8.5.

/// The result type.
pub type ZendResultType = rsphp_sys::zend_result;
/// A failure.
pub const ZEND_RESULT_FAILURE: ZendResultType = rsphp_sys::ZEND_RESULT_CODE::FAILURE;
/// A success.
pub const ZEND_RESULT_SUCCESS: ZendResultType = rsphp_sys::ZEND_RESULT_CODE::SUCCESS;
