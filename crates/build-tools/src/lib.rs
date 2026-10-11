//! This crate holds various tools for building up the `rsphp-sys` crate, so
//! that they can be reused when overriding that crate on a per-project basis.

/// Module for the `AnonymousTypeNames` callbacks.
mod anonymous_type_names;
pub use anonymous_type_names::AnonymousTypeNames;

/// Module for the `BuildIdCallbacks` callbacks.
mod build_id;
pub use build_id::BuildIdCallbacks;

/// Module for the `CommentProcessor` callbacks.
///
/// Only needed if you want to have rustdoc run on the generated bindings and
/// not have any complaints.
mod comment_processor;
pub use comment_processor::CommentProcessor;

/// Module for the `ZendHeaderIterator` utility.
mod headers;
pub use headers::ZendHeaderIterator;

/// Module for the `IntTypeCastCallbacks` callbacks.
mod int_casts;
pub use int_casts::IntTypeCastCallbacks;

/// Module for the `ExplicitIntTypesCallbacks` callbacks.
mod int_types;
pub use int_types::ExplicitIntTypesCallbacks;
