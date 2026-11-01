//! Extension trait for [`rsphp_sys::zend_type`].
use rsphp_sys::{
    _IS_BOOL, _ZEND_TYPE_ARENA_BIT, _ZEND_TYPE_INTERSECTION_BIT, _ZEND_TYPE_ITERABLE_BIT,
    _ZEND_TYPE_KIND_MASK, _ZEND_TYPE_LIST_BIT, _ZEND_TYPE_LITERAL_NAME_BIT, _ZEND_TYPE_MASK,
    _ZEND_TYPE_MAY_BE_MASK, _ZEND_TYPE_NAME_BIT, _ZEND_TYPE_NULLABLE_BIT, _ZEND_TYPE_UNION_BIT,
    IS_ITERABLE, IS_MIXED, MAY_BE_ANY, MAY_BE_BOOL, zend_string, zend_type, zend_type_list,
};
use std::ffi::{c_char, c_void};

/// Extension trait for [`rsphp_sys::zend_type`].
pub trait ZendTypeExt {
    /// Create a new type for the given mask.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_MASK!()`].
    fn init_mask(mask: u32) -> Self;

    /// Create a new type for the given code and options.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_CODE!()`].
    fn init_code(code: u8, allow_null: bool, extra_flags: u32) -> Self;

    /// Create a new type for the given opaque pointer and given options.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_PTR!()`].
    fn init_ptr(ptr: *mut c_void, type_kind: u32, allow_null: bool, extra_flags: u32) -> Self;

    /// Create a new type as a union of the given type list, with the given options.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_UNION!()`].
    fn init_union(ptr: *mut zend_type_list, extra_flags: u32) -> Self;

    /// Create a new type as an intersection of the given type list, with the given options.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_INTERSECTION!()`].
    fn init_intersection(ptr: *mut zend_type_list, extra_flags: u32) -> Self;

    /// Create a new type representing the exact class whose name was provided, with
    /// the given options.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_INIT_CLASS!()`].
    fn init_class(ptr: *mut zend_string, allow_null: bool, extra_flags: u32) -> Self;

    /// Check if a type is set.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_SET!()`].
    fn is_set(&self) -> bool;

    /// Check if a type is complex, i.e. a list with a union or intersection, or
    /// a pointer to a class name.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_COMPLEX!()`].
    fn is_complex(&self) -> bool;

    /// Check if a type has a [`rsphp_sys::zend_string`] pointer to a name.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_HAS_NAME!()`].
    fn has_name(&self) -> bool;

    /// Check if a type has a [`std::ffi::c_char`] pointer to a name.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_HAS_LITERAL_NAME!()`].
    fn has_literal_name(&self) -> bool;

    /// Check if a type has an inner list of types.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_HAS_LIST!()`].
    fn has_list(&self) -> bool;

    /// Check if a type is the iterator fallback.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_ITERABLE_FALLBACK!()`].
    fn is_iterable_fallback(&self) -> bool;

    /// Check if a type is an intersection.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_INTERSECTION!()`].
    fn is_intersection(&self) -> bool;

    /// Check if a type is a union.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_UNION!()`].
    fn is_union(&self) -> bool;

    /// Check if a type is arena allocated.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_USES_ARENA!()`].
    fn uses_arena(&self) -> bool;

    /// Check if a has a type mask that only refers to standard types.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_IS_ONLY_MASK!()`].
    fn is_only_mask(&self) -> bool;

    /// Get the name of the type as a [`rsphp_sys::zend_string`] pointer to a name.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_NAME!()`].
    fn get_name(&mut self) -> *mut zend_string;

    /// Get the name of the type as a [`std::ffi::c_char`] pointer to a name.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_LITERAL_NAME!()`].
    fn get_literal_name(&self) -> *const c_char;

    /// Get the list of the type's inner types as a [`rsphp_sys::zend_type_list`] reference.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_LIST!()`].
    fn get_list(&self) -> &zend_type_list;

    /// Get the full mask of the type.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_FULL_MASK!()`].
    fn full_mask(&self) -> u32;

    /// Get the pure mask of the type, i.e. just the `MAY_BE_*` flags.
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_PURE_MASK!()`].
    fn pure_mask(&self) -> u32;

    /// Get the pure mask of the type, i.e. just the `MAY_BE_*` flags, excluding
    /// [`rsphp_sys::_ZEND_TYPE_NULLABLE_BIT`].
    ///
    /// This is equivalent to [`rsphp_macros_c::ZEND_TYPE_PURE_MASK_WITHOUT_NULL!()`].
    fn pure_mask_without_null(&self) -> u32;

    /// Check if this type is satisfied by `null`.
    ///
    /// This is eqivalent to [`rsphp_macros_c::ZEND_TYPE_ALLOW_NULL!()`].
    fn allows_null(&self) -> bool;
}

impl ZendTypeExt for zend_type {
    fn init_mask(mask: u32) -> Self {
        zend_type {
            ptr: std::ptr::null_mut(),
            type_mask: mask,
        }
    }
    fn init_code(code: u8, allow_null: bool, extra_flags: u32) -> Self {
        let mut mask = match code {
            _IS_BOOL => MAY_BE_BOOL,
            IS_ITERABLE => _ZEND_TYPE_ITERABLE_BIT,
            IS_MIXED => MAY_BE_ANY,
            _ => 1 << code,
        };
        if allow_null {
            mask |= _ZEND_TYPE_NULLABLE_BIT;
        }
        mask |= extra_flags;
        Self::init_mask(mask)
    }
    fn init_ptr(ptr: *mut c_void, type_kind: u32, allow_null: bool, extra_flags: u32) -> Self {
        assert!(!ptr.is_null(), "Must not be null");
        assert!(ptr.is_aligned(), "Must be aligned");
        let mut type_mask = type_kind | extra_flags;
        if allow_null {
            type_mask |= _ZEND_TYPE_NULLABLE_BIT;
        }
        zend_type { ptr, type_mask }
    }
    fn init_union(ptr: *mut zend_type_list, extra_flags: u32) -> Self {
        assert!(!ptr.is_null(), "Must not be null");
        assert!(ptr.is_aligned(), "Must be aligned");
        let type_mask = _ZEND_TYPE_LIST_BIT | _ZEND_TYPE_UNION_BIT | extra_flags;
        zend_type {
            ptr: ptr.cast::<c_void>(),
            type_mask,
        }
    }
    fn init_intersection(ptr: *mut zend_type_list, extra_flags: u32) -> Self {
        assert!(!ptr.is_null(), "Must not be null");
        assert!(ptr.is_aligned(), "Must be aligned");
        let type_mask = _ZEND_TYPE_LIST_BIT | _ZEND_TYPE_INTERSECTION_BIT | extra_flags;
        zend_type {
            ptr: ptr.cast::<c_void>(),
            type_mask,
        }
    }
    fn init_class(ptr: *mut zend_string, allow_null: bool, extra_flags: u32) -> Self {
        assert!(!ptr.is_null(), "Must not be null");
        assert!(ptr.is_aligned(), "Must be aligned");
        Self::init_ptr(
            ptr.cast::<c_void>(),
            _ZEND_TYPE_NAME_BIT,
            allow_null,
            extra_flags,
        )
    }

    fn is_set(&self) -> bool {
        self.type_mask & _ZEND_TYPE_MASK != 0
    }

    fn is_complex(&self) -> bool {
        self.type_mask & _ZEND_TYPE_KIND_MASK != 0
    }

    fn has_name(&self) -> bool {
        self.type_mask & _ZEND_TYPE_NAME_BIT != 0
    }

    fn has_literal_name(&self) -> bool {
        self.type_mask & _ZEND_TYPE_LITERAL_NAME_BIT != 0
    }

    fn has_list(&self) -> bool {
        self.type_mask & _ZEND_TYPE_LIST_BIT != 0
    }

    fn is_iterable_fallback(&self) -> bool {
        self.type_mask & _ZEND_TYPE_ITERABLE_BIT != 0
    }

    fn is_intersection(&self) -> bool {
        self.type_mask & _ZEND_TYPE_INTERSECTION_BIT != 0
    }

    fn is_union(&self) -> bool {
        self.type_mask & _ZEND_TYPE_UNION_BIT != 0
    }

    fn uses_arena(&self) -> bool {
        self.type_mask & _ZEND_TYPE_ARENA_BIT != 0
    }

    fn is_only_mask(&self) -> bool {
        self.is_set() && self.ptr.is_null()
    }

    fn get_name(&mut self) -> *mut zend_string {
        assert!(self.has_name(), "Must have a name");
        let ptr = self.ptr;
        assert!(!ptr.is_null(), "Pointer must not be null");
        let name_ptr: *mut zend_string = ptr.cast();
        assert!(name_ptr.is_aligned(), "Pointer must be aligned");
        name_ptr
    }

    fn get_literal_name(&self) -> *const c_char {
        assert!(self.has_literal_name(), "Must have a literal name");
        let ptr = self.ptr;
        assert!(!ptr.is_null(), "Pointer must not be null");
        let literal_name_ptr: *const c_char = ptr.cast();
        assert!(literal_name_ptr.is_aligned(), "Pointer must be aligned");
        literal_name_ptr
    }

    fn get_list(&self) -> &zend_type_list {
        assert!(self.has_list(), "Must be a list");
        let ptr = self.ptr;
        assert!(!ptr.is_null(), "Pointer must not be null");
        let list_ptr: *const zend_type_list = ptr.cast();
        assert!(list_ptr.is_aligned(), "Pointer must be aligned");
        // SAFETY: confirmed to be a list, must have a valid list pointer
        unsafe { list_ptr.as_ref().expect("Should be valid pointer") }
    }

    fn full_mask(&self) -> u32 {
        self.type_mask
    }

    fn pure_mask(&self) -> u32 {
        self.type_mask & _ZEND_TYPE_MAY_BE_MASK
    }

    fn pure_mask_without_null(&self) -> u32 {
        self.type_mask & _ZEND_TYPE_MAY_BE_MASK & !_ZEND_TYPE_NULLABLE_BIT
    }

    fn allows_null(&self) -> bool {
        self.type_mask & _ZEND_TYPE_NULLABLE_BIT != 0
    }
}
