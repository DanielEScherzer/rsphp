# RSPHP

## Organization

The primary `rsphp` crate contains utilities for use at runtime in PHP
extensions. This workplace also includes the following crates:

- `rsphp-build-tools`
- `rsphp-macros-c`
- `rsphp-macros-rs`
- `rsphp-stub-parser`
- `rsphp-stub-parser-cli`
- `rsphp-sys`

The `rsphp-macros-c` and `rsphp-macros-rs` crates provide macros to help easy
development - the former contains macros that closely match PHP's C macros,
and the latter provides macros that are less closely tied to a C macro but are
still expected to be useful, or which have a different interface from the
corresponding C macro. Not all macros are recreated - if there are any that
you would like added, let me know.

The `rsphp-stub-parser` crate provides support for automatically generating
argument info for extension classes, interfaces, traits, enums, functions, and
constants. It is inspired by PHP's `gen_stub.php` build script.

The `rsphp-stub-parser-cli` crate provides a command line interface for using
the stub parser; it is provided for use during the development process and for
testing.

The `rsphp-sys` crate provides bindings for PHP's C ABI, and the
`rsphp-build-tools` crate provides tools for generating those bindings.

## Argument information

When using the `rsphp-stub-parser` crate for generating argument information
from stub files, the resulting Rust code depends only on the `rsphp-macros-c`,
`rsphp-macros-rs`, and `rsphp-sys` crates, and not on the stub parser itself,
nor on the main `rsphp` crate. If the features from the `rsphp` crate are not
needed, there is no need to install it.

## rsphp-sys bindings

The `rsphp-sys` bindings are generated using `bindgen` on a single machine, for
a single PHP version. These are provided to simplify initial development of
extensions, but it is expected that developers will want to override these
bindings with new ones that include additional headers, or are configured
differently, or to account for platform and architecture differences. This is
*expected*, and supported. The `rsphp-build-tools` crate provides tool to
simplify the process. Not everything in that crate will be needed, however,
depending on the extension being written.

See Cargo's [`[patch]` section][cargo-patch] for how to override the `rsphp-sys`
crate with a custom version.

If the `rsphp-sys` crate is overridden locally, care must be taken to ensure
that the macro crates and the `rsphp` crate continue to work. Be sure that the
overrides:

* use the "module constants" variation of enum generation for the `zend_result`
and `_zend_known_string_id` enums, i.e. set the default style to be
`EnumVariation::ModuleConsts`, or use Bindgen's `constified_enum_module` builder
function.
* rename the `_zend_function__bindgen_ty_1` type to `_zend_function__common`
(this can be accomplished with the `AnonymousTypeNames` parser callback from
the build tools crate).
* adjust the types of integer constants (as done by the `ExplicitIntTypesCallbacks`
parser callback from the build tools crate).

[cargo-patch]: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section
