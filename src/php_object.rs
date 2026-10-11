use rsphp_macros_c::Z_OBJ;
use rsphp_sys::{ZEND_RESULT_CODE, object_init_ex, zend_class_entry, zend_object, zval};
use std::convert::{AsMut, AsRef};
use std::ptr::NonNull;

/// Wrapper around a [`rsphp_sys::zend_object`] that is known to exist and
/// be valid.
pub struct PHPObject {
    /// The wrapped object.
    ptr: NonNull<zend_object>,
}

impl PHPObject {
    /// Create a new `PHPObject` from a raw pointer.
    ///
    /// # Safety
    ///
    /// If not null, the provided pointer must be safe to read from and write to.
    pub unsafe fn wrap(object: *mut zend_object) -> Option<Self> {
        NonNull::new(object).map(|ptr| PHPObject { ptr })
    }

    /// Create a new `PHPObject` for the given class.
    ///
    /// # Safety
    ///
    /// The provided class pointer must be valid to read from and use.
    pub unsafe fn initialize(class_type_ptr: *mut zend_class_entry) -> Option<Self> {
        // SAFETY: class type must be valid
        let class_type: &mut zend_class_entry = unsafe { class_type_ptr.as_mut() }?;
        // SAFETY: zvals can be initialized with zero bytes
        let mut zval: zval = unsafe { std::mem::zeroed() };
        // SAFETY: object_init_ex() is called correctly with a pointer to a
        // zval to initialize and non-null pointer to a class entry
        let result = unsafe { object_init_ex(&raw mut zval, class_type) };
        if result == ZEND_RESULT_CODE::FAILURE {
            return None;
        }
        let ptr = NonNull::new(Z_OBJ!(zval))?;
        Some(PHPObject { ptr })
    }

    /// Get a reference to an object with the given index in the property table.
    ///
    /// # Panics
    ///
    /// Panics if the object does not have a valid class set, the index given is
    /// out of bounds of the default properties of that class, the index cannot
    /// be converted into a usize properly, or reading the specified property
    /// offset with the struct hack fails.
    #[must_use]
    pub fn get_prop_by_index(&self, index: i32) -> &zval {
        let class_entry = self.get_class().expect("object class entry should be set");
        assert!(
            index < class_entry.default_properties_count,
            "Index must be valid for the class entry"
        );
        let prop_table = &self.as_ref().properties_table;
        if index == 0 {
            &prop_table[0]
        } else {
            // Struct hack
            let start: *const zval = prop_table.as_ptr();
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let for_current =
                unsafe { start.add(index.try_into().expect("should be valid usize")) };
            // SAFETY: per assertion above the index is valid for this class
            unsafe {
                for_current
                    .as_ref()
                    .expect("should be valid property storage")
            }
        }
    }

    /// Get a mutable reference to an object with the given index in the
    /// property table.
    ///
    /// # Panics
    ///
    /// Panics if the object does not have a valid class set, the index given is
    /// out of bounds of the default properties of that class, the index cannot
    /// be converted into a usize properly, or reading the specified property
    /// offset with the struct hack fails.
    pub fn get_mut_prop_by_index(&mut self, index: i32) -> &mut zval {
        let class_entry = self.get_class().expect("object class entry should be set");
        assert!(
            index < class_entry.default_properties_count,
            "Index must be valid for the class entry"
        );
        let prop_table = &mut self.as_mut().properties_table;
        if index == 0 {
            &mut prop_table[0]
        } else {
            // Struct hack
            let start: *mut zval = prop_table.as_mut_ptr();
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let for_current =
                unsafe { start.add(index.try_into().expect("should be valid usize")) };
            // SAFETY: per assertion above the index is valid for this class
            unsafe {
                for_current
                    .as_mut()
                    .expect("should be valid property storage")
            }
        }
    }

    #[must_use]
    pub fn get_class(&self) -> Option<&zend_class_entry> {
        // SAFETY: pointer must be valid
        let obj_ref = unsafe { self.ptr.as_ref() };
        // SAFETY: all objects have a valid class entry set
        unsafe { obj_ref.ce.as_ref() }
    }

    #[must_use]
    pub fn ptr(&self) -> *mut zend_object {
        self.ptr.as_ptr()
    }
}

impl AsRef<zend_object> for PHPObject {
    fn as_ref(&self) -> &zend_object {
        // SAFETY: pointer must be valid
        unsafe { self.ptr.as_ref() }
    }
}

impl AsMut<zend_object> for PHPObject {
    fn as_mut(&mut self) -> &mut zend_object {
        // SAFETY: pointer must be valid
        unsafe { self.ptr.as_mut() }
    }
}
