//! This module defines some stack-based types to wrap around PHP's C heap-based
//! types, using implementations of the [`Drop`] trait to ensure that the
//! heap-based types are properly freed.

/// Module for `StackSmartStr`.
mod stack_smart_str;
pub use stack_smart_str::StackSmartStr;

/// Module for `StackZendString`.
mod stack_zend_string;
pub use stack_zend_string::StackZendString;

/// Module for `StackZval`.
mod stack_zval;
pub use stack_zval::StackZval;
