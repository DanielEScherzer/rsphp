//! This crate defines bindings for PHP's C ABI.

#![expect(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_qualifications,
    unused_results,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::decimal_literal_representation,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::exhaustive_structs,
    clippy::inline_modules,
    clippy::integer_division,
    clippy::manual_div_ceil,
    clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_safety_doc,
    clippy::multiple_unsafe_ops_per_block,
    clippy::must_use_candidate,
    clippy::ptr_as_ptr,
    clippy::ptr_offset_with_cast,
    clippy::pub_underscore_fields,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_arguments,
    clippy::undocumented_unsafe_blocks,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::used_underscore_binding,
    clippy::useless_transmute,
    reason = "Generated code"
)]
#![cfg_attr(
    not(test),
    expect(
        clippy::missing_docs_in_private_items,
        reason = "rust-lang/rust-clippy#16934"
    )
)]
#![allow(
    clippy::type_complexity,
    reason = "stream operations only complex in debug mode"
)]

// Include the generated bindings
include!("../bindings.rs");
