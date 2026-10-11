//! Extension trait for [`rsphp_sys::zend_type`].
use rsphp_sys::{
    _IS_BOOL, _ZEND_TYPE_ARENA_BIT, _ZEND_TYPE_INTERSECTION_BIT, _ZEND_TYPE_ITERABLE_BIT,
    _ZEND_TYPE_KIND_MASK, _ZEND_TYPE_LIST_BIT, _ZEND_TYPE_LITERAL_NAME_BIT, _ZEND_TYPE_MASK,
    _ZEND_TYPE_MAY_BE_MASK, _ZEND_TYPE_NAME_BIT, _ZEND_TYPE_NULLABLE_BIT, _ZEND_TYPE_UNION_BIT,
    IS_ITERABLE, IS_MIXED, MAY_BE_ANY, MAY_BE_BOOL, zend_string, zend_type, zend_type_list,
};
use std::ffi::{c_char, c_void};

pub trait ZendTypeExt {
    fn init_mask(mask: u32) -> Self;
    fn init_code(code: u8, allow_null: bool, extra_flags: u32) -> Self;
    fn init_ptr(ptr: *mut c_void, type_kind: u32, allow_null: bool, extra_flags: u32) -> Self;
    fn init_union(ptr: *mut zend_type_list, extra_flags: u32) -> Self;
    fn init_intersection(ptr: *mut zend_type_list, extra_flags: u32) -> Self;
    fn init_class(ptr: *mut zend_string, allow_null: bool, extra_flags: u32) -> Self;

    fn is_set(&self) -> bool;
    fn is_complex(&self) -> bool;
    fn has_name(&self) -> bool;
    fn has_literal_name(&self) -> bool;
    fn has_list(&self) -> bool;
    fn is_iterable_fallback(&self) -> bool;
    fn is_intersection(&self) -> bool;
    fn is_union(&self) -> bool;
    fn uses_arena(&self) -> bool;
    fn is_only_mask(&self) -> bool;
    fn get_name(&mut self) -> *mut zend_string;
    fn get_literal_name(&self) -> *const c_char;
    fn get_list(&self) -> &zend_type_list;
    fn full_mask(&self) -> u32;
    fn pure_mask(&self) -> u32;
    fn pure_mask_without_null(&self) -> u32;
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
