//! This crate holds utilities for PHP extensions written in Rust.

/// Module for `PHPObject` type.
mod php_object;
pub use php_object::PHPObject;

/// Module for `PHPValue` type and related.
mod php_value;
pub use php_value::PHPValue;

/// Implementation of `PHPExceptions`.
mod php_exceptions;
pub use php_exceptions::PHPExceptions;

pub mod extensions;
pub mod stack_types;
pub mod wrappers;
