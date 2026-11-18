# Changelog

## 0.1.3 - 2026.11.18

### Fixed

- `[rsphp-macros-rs]` The ZPP implementation now marks the parameters for
  `Z_PARAM_LONG` and `Z_PARAM_LONG_OR_NULL` as `zend_long`s to guide type
  inference, since the default of `i32` leads to a 4-byte value when PHP treats
  the provided pointer as an 8-byte value.
  ([#11](https://github.com/DanielEScherzer/rsphp/pull/11))

### Added

- `[rsphp]` The following types are now exported under the `extensions` module:
    - `ZendAttributeArgIter`, the iterator returned by `ZendAttributeExt::args()`
    - `ZendClassEntryTraitNamesIter`, the iterator returned by
      `ZendClassEntryExt::get_trait_names()`
    - `ZendTypeListIter`, the iterator returned by `ZendTypeListExt::iter()`

  and `PHPStringWrapper` is exported under the root `rsphp` crate.
  ([#12](https://github.com/DanielEScherzer/rsphp/pull/12))
- `[rsphp]` The following types now implement `std::fmt::Debug`
  ([#12](https://github.com/DanielEScherzer/rsphp/pull/12)):
    - `extensions::ZendAttributeArgIter`
    - `extensions::ZendClassEntryTraitNamesIter`
    - `extensions::ZendTypeListIter`
    - `PHPExceptions`
    - `PHPObject`
    - `PHPStringSource`
    - `PHPStringWrapper`
    - `PHPValue`
    - `stack_types::StackSmartStr`
    - `stack_types::StackZendString`
    - `stack_types::StackZval`
    - `wrappers::HashMapRepr`
    - `wrappers::HashRepr`
    - `wrappers::HashPackedRepr`
    - `wrappers::InfoTablePrinter`
- `[rsphp]` `PHPExceptions` now implements `Copy` and `Clone`.
  ([#12](https://github.com/DanielEScherzer/rsphp/pull/12))
- `[rsphp-stub-parser]` Added support for negative integer class constants.
  ([#14](https://github.com/DanielEScherzer/rsphp/pull/14))
- `[rsphp-stub-parser]` Added support for methods on enums.
  ([#15](https://github.com/DanielEScherzer/rsphp/pull/15))
- `[rsphp-build-tools]` Added `PhpConfigRunner` for running `php-config`
  commands. ([#16](https://github.com/DanielEScherzer/rsphp/pull/16))

### Changed

- `[rsphp]` The following types are now marked as `#[non_exhaustive]`
  ([#12](https://github.com/DanielEScherzer/rsphp/pull/12)):
  - `PHPExceptions`
  - `PHPValue`
- `[rsphp]` `wrappers::InfoTablePrinter` no longer panics when trying to print
  rows with too many values; instead, the supported values are printed, followed
  by a warning. ([#12](https://github.com/DanielEScherzer/rsphp/pull/12))

### Documentation

- The contents of the `rsphp` crate are now fully documented, and lints have
  been enabled to ensure that future additions are properly documented.
  ([#12](https://github.com/DanielEScherzer/rsphp/pull/12))

## 0.1.2 - 2026.10.26

### Fixed

- `[rsphp]` Fixed pointer casts in `PHPValue::apply_to()` to be valid on
  AArch64 platforms. ([#6](https://github.com/DanielEScherzer/rsphp/pull/6))
- `[rsphp-stub-parser]` Fixed missing class and property flags: classes are now
  properly marked with:
    - `ZEND_ACC_ABSTRACT`
    - `ZEND_ACC_ALLOW_DYNAMIC_PROPERTIES`
    - `ZEND_ACC_READONLY_CLASS`

  as appropriate, and properties are now properly marked with:
    - `ZEND_ACC_FINAL`
    - `ZEND_ACC_PUBLIC_SET` / `ZEND_ACC_PROTECTED_SET` / `ZEND_ACC_PRIVATE_SET`
    - `ZEND_ACC_READONLY`
    - `ZEND_ACC_STATIC`

  as appropriate. ([#7](https://github.com/DanielEScherzer/rsphp/pull/7))

### Changed

- `[rsphp-stub-parser]` Avoid wildcard imports in generated arg info files.
  ([#5](https://github.com/DanielEScherzer/rsphp/pull/5))
- `[rsphp-stub-parser]` Clean up whitespace in generated arg info files.
  ([#9](https://github.com/DanielEScherzer/rsphp/pull/9))

### Documentation

- The `rsphp-sys` bundled and shown on [docs.rs](https://docs.rs) were updated
  to PHP 8.5.12. ([#5](https://github.com/DanielEScherzer/rsphp/pull/5))

## 0.1.1 - 2026.10.11

### Fixed

- `[rsphp-macros-c]` Fixed `Z_OBJ_HANDLER_P` macro missing its second parameter.
  ([#2](https://github.com/DanielEScherzer/rsphp/pull/2))

### Documentation

- Added metadata to tell [docs.rs](https://docs.rs) to show documentation for
  private items.
  ([#1](https://github.com/DanielEScherzer/rsphp/pull/1))

## 0.1.0 - 2026.10.11

Initial public release!
