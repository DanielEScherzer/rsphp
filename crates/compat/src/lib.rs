//! Compatability aliases for things that change names in different PHP versions.

#[cfg(not(php_is_86))]
mod php_85;
#[cfg(not(php_is_86))]
pub use php_85::*;

#[cfg(php_is_86)]
mod php_86;
#[cfg(php_is_86)]
pub use php_86::*;
