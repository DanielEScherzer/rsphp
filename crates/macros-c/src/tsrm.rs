//! Macros corresponding to `TSRM.h` in php/php-src.
//!
//! Remember that these macros assume that the extension is *not* being
//! statically linked with PHP (i.e. that `ZEND_ENABLE_STATIC_TSRMLS_CACHE` is
//! not enabled for ZTS builds). Thus, `ZEND_TSRMG` is an alias for TSRMG.

use crate::{c_differences, c_version};

const _: () = {
    assert!(size_of::<usize>() == 8, "usize should be 8 bytes");
};
const _: () = {
    assert!(size_of::<isize>() == 8, "isize should be 8 bytes");
};
/// Get an aligned size for TSRM.
#[doc = c_version!(
    "# define TSRM_ALIGNED_SIZE(size)",
    "(((size) + INT64_C(15)) & ~INT64_C(15))"
)]
#[doc = c_differences!(
    "The C version is defined differently for `SIZEOF_SIZE_T` being 4 or 8, we \
        assert that it is always 8. Only the C version for the size being 8 is \
        shown above."
)]
#[macro_export]
#[cfg(any(doc, php_zend_zts))]
macro_rules! TSRM_ALIGNED_SIZE {
    ($size:expr) => {{
        #[expect(
            clippy::cast_possible_wrap,
            clippy::as_conversions,
            reason = "Matches C code"
        )]
        let size: i64 = $size as i64;
        let result_64: i64 = (size + 15_i64) & (!15_i64);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            reason = "Matches C code, isize is 8 bytes"
        )]
        let result: isize = result_64 as isize;
        result
    }};
}

/// TSRM id unshuffling.
#[doc = c_version!("#define TSRM_UNSHUFFLE_RSRC_ID(rsrc_id)        ((rsrc_id)-1)")]
#[macro_export]
#[cfg(any(doc, php_zend_zts))]
macro_rules! TSRM_UNSHUFFLE_RSRC_ID {
    ($rsrc_id:expr) => {
        $rsrc_id - 1
    };
}

/// Lookup of an element in a structure in the slow cache.
#[doc = c_version!("#define TSRMG(id, type, element)    (TSRMG_BULK(id, type)->element)")]
#[macro_export]
#[cfg(any(doc, php_zend_zts))]
macro_rules! TSRMG {
    ($id:expr, $ty:ty, $element:ident) => {{
        // These are unsafe and so the macro is unsafe
        let structure_ptr: *const $ty = $crate::TSRMG_BULK!($id, $ty);
        let structure_reference: &$ty = structure_ptr.as_ref().expect(concat!(
            "TSRMG: lookup of TSRMG_BULK(",
            stringify!($id),
            ", ",
            stringify!($ty),
            ") should be convertible to a mutable reference"
        ));
        structure_reference.$element
    }};
}

/// Bulk lookup of an overall structure in the slow cache.
#[doc = c_version!("#define TSRMG_BULK(id, type)    ((type) (*((void ***) tsrm_get_ls_cache()))[TSRM_UNSHUFFLE_RSRC_ID(id)])")]
#[macro_export]
#[cfg(any(doc, php_zend_zts))]
macro_rules! TSRMG_BULK {
    ($id:expr, $ty:ty) => {{
        // These are unsafe and so the macro is unsafe
        let cache_base: *const std::ffi::c_void = rsphp_sys::tsrm_get_ls_cache();
        let cache_triple_void: *const *const *const std::ffi::c_void =
            cache_base.cast::<*const *const std::ffi::c_void>();
        let cache_deref: *const *const std::ffi::c_void = *cache_triple_void;
        let cache_with_offset = cache_deref.add(
            $crate::TSRM_UNSHUFFLE_RSRC_ID!($id)
                .try_into()
                .expect("Should be valid usize"),
        );
        let at_offset: *const std::ffi::c_void = *cache_with_offset;
        at_offset.cast::<$ty>()
    }};
}

/// Lookup of an element in a structure in the TSRM cache.
#[doc = c_version!("#define TSRMG_FAST(offset, type, element)    (TSRMG_FAST_BULK(offset, type)->element)")]
#[macro_export]
#[cfg(any(doc, php_zend_zts))]
macro_rules! TSRMG_FAST {
    ($offset:expr, $ty:ty, $element:ident) => {{
        // These are unsafe and so the macro is unsafe
        let structure_ptr: *const $ty = $crate::TSRMG_FAST_BULK!($offset, $ty).cast_const();
        let structure_reference: &$ty = structure_ptr.as_ref().expect(concat!(
            "TSRMG_FAST: lookup of TSRMG_FAST_BULK(",
            stringify!($offset),
            ", ",
            stringify!($ty),
            ") should be convertible to a mutable reference"
        ));
        structure_reference.$element
    }};
}

/// Bulk lookup of an overall structure in the TSRM cache.
#[doc = c_version!("#define TSRMG_FAST_BULK(offset, type)    ((type) (((char*) tsrm_get_ls_cache())+(offset)))")]
#[macro_export]
#[cfg(any(doc, all(php_zend_zts, not(php_is_86))))]
macro_rules! TSRMG_FAST_BULK {
    ($offset:expr, $ty:ty) => {{
        // These are unsafe and so the macro is unsafe
        let cache_base: *mut std::ffi::c_void = rsphp_sys::tsrm_get_ls_cache();
        let at_offset: *mut std::ffi::c_void = cache_base.byte_add($offset);
        at_offset.cast::<$ty>()
    }};
}

/// Bulk lookup of an overall structure in the TSRM cache.
#[macro_export]
#[cfg(all(not(doc), php_zend_zts, php_is_86))]
macro_rules! TSRMG_FAST_BULK {
    ($offset:expr, $ty:ty) => {{
        // These are unsafe and so the macro is unsafe
        let cache_base: *mut std::ffi::c_void = rsphp_sys::tsrm_get_ls_cache();
        let at_offset: *mut std::ffi::c_void = cache_base.byte_offset($offset);
        at_offset.cast::<$ty>()
    }};
}
