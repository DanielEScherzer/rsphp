# Changelog

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
