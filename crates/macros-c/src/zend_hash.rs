//! Macros corresponding to `zend_hash.h` in php/php-src.

use crate::c_version;

/// Access the flags of a [`rsphp_sys::HashTable`] pointer.
#[doc = c_version!("#define HT_FLAGS(ht) (ht)->u.flags")]
#[macro_export]
macro_rules! HT_FLAGS {
    ($ht:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        (*$ht).u.flags
    };
}

/// Check if a [`rsphp_sys::HashTable`] pointer is packed.
#[doc = c_version!(
    "#define HT_IS_PACKED(ht)",
    "((HT_FLAGS(ht) & HASH_FLAG_PACKED) != 0)"
)]
#[macro_export]
macro_rules! HT_IS_PACKED {
    ($ht:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        $crate::HT_FLAGS!($ht) & rsphp_sys::HASH_FLAG_PACKED != 0
    };
}

/// Set a [`rsphp_sys::zval`] pointer to point to the empty array.
///
/// # C version
///
/// ```c
/// #define ZVAL_EMPTY_ARRAY(z) do {                        \
///         zval *__z = (z);                                \
///         Z_ARR_P(__z) = (zend_array*)&zend_empty_array;    \
///         Z_TYPE_INFO_P(__z) = IS_ARRAY; \
///     } while (0)
/// ```
#[macro_export]
macro_rules! ZVAL_EMPTY_ARRAY {
    ($z:expr) => {{
        let __z: *mut rsphp_sys::zval = $z;
        // cannot assign through macros
        // The following is unsafe, and is the reason this macro is unsafe
        let _z_ref: &mut rsphp_sys::zval = &mut *(__z);
        _z_ref.value.arr = &rsphp_sys::zend_empty_array as *const _ as *mut _;
        _z_ref.u1.type_info = rsphp_sys::IS_ARRAY as u32;
    }};
}

/// Get the size of elements in an array.
#[doc = c_version!(
    "# define ZEND_HASH_ELEMENT_SIZE(__ht)",
    "(sizeof(zval) + (~HT_FLAGS(__ht) & HASH_FLAG_PACKED) * ((sizeof(Bucket)-sizeof(zval))/HASH_FLAG_PACKED))"
)]
#[macro_export]
macro_rules! ZEND_HASH_ELEMENT_SIZE {
    ($ht:expr) => {
        std::mem::size_of::<rsphp_sys::zval>()
            + ((!$crate::HT_FLAGS!($ht) & rsphp_sys::HASH_FLAG_PACKED) as usize)
                * ((std::mem::size_of::<rsphp_sys::Bucket>()
                    - std::mem::size_of::<rsphp_sys::zval>())
                    / (rsphp_sys::HASH_FLAG_PACKED as usize))
    };
}

/// Get the element at a specific index in an array.
#[doc = c_version!(
    "#define ZEND_HASH_ELEMENT_EX(__ht, _idx, _size)",
    "((zval*)(((char*)(__ht)->arPacked) + ((_idx) * (_size))))"
)]
#[macro_export]
macro_rules! ZEND_HASH_ELEMENT_EX {
    ($ht_p:expr, $idx:expr, $size:expr) => {
        // The following is unsafe, and is the reason this macro is unsafe
        (*$ht_p).__bindgen_anon_1.arPacked.byte_add($idx * $size)
    };
}
