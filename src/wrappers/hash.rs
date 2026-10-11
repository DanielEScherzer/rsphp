//! Types for dealing with [`rsphp_sys::HashTable`] values.
use rsphp_macros_c::{
    HT_IS_PACKED, Z_TYPE, Z_TYPE_P, ZEND_HASH_ELEMENT_EX, ZEND_HASH_ELEMENT_SIZE,
};
use rsphp_sys::{Bucket, HashTable, IS_UNDEF, zend_string, zval};

/// Representation of a *non packed* hash table, for read-only iteration
/// though the underlying values can be modified.
pub struct HashMapRepr<'table> {
    /// The underlying table.
    _raw: *const HashTable,
    /// The one-past-the-end bucket.
    end: *const Bucket,
    /// The next bucket to read from.
    next: *const Bucket,
    /// Marker for lifetime enforcement.
    _marker: std::marker::PhantomData<&'table HashTable>,
}

impl HashMapRepr<'_> {
    /// Create a new `HashMapRepr` for the given table.
    ///
    /// # Panics
    ///
    /// Panics if the table is packed ([`rsphp_macros_c::HT_IS_PACKED!()`]) or
    /// if the number of elements in the table cannot be converted to a
    /// [`usize`].
    #[must_use]
    pub fn from_reference(reference: &HashTable) -> Self {
        assert!(
            // SAFETY: pointer created from a reference is valid
            unsafe { !HT_IS_PACKED!(std::ptr::from_ref::<HashTable>(reference)) },
            "Must not be a packed HashTable"
        );
        // SAFETY: confirmed not to be packed
        let data_ptr = unsafe { reference.__bindgen_anon_1.arData };
        HashMapRepr {
            _raw: std::ptr::from_ref::<HashTable>(reference),
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            end: unsafe {
                data_ptr.add(
                    reference
                        .nNumUsed
                        .try_into()
                        .expect("Should be valid usize"),
                )
            },
            next: data_ptr,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new `HashMapRepr` for the table pointed to with the given
    /// pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid for reads and convertible to a reference if
    /// it is not null. This is a helper that simply uses
    /// [`pointer::as_ref()`][pointer#method.as_ref] to get a reference.
    ///
    /// # Panics
    ///
    /// Panics if the pointer given is null, as well as for any condition where
    /// [`Self::from_reference()`] panics.
    #[must_use]
    pub unsafe fn from_pointer(ptr: *const HashTable) -> Self {
        // SAFETY: based on the provided pointer being valid
        unsafe { HashMapRepr::from_reference(ptr.as_ref().expect("Should be valid pointer")) }
    }
}

// ZEND_HASH_MAP_FOREACH_STR_KEY_VAL
impl<'table> Iterator for HashMapRepr<'table> {
    type Item = (&'table mut zend_string, zval);

    fn next(&mut self) -> Option<Self::Item> {
        // From ZEND_HASH_MAP_FOREACH_FROM
        if self.next == self.end {
            return None;
        }
        // SAFETY: until we get to the end pointers are valid
        let bucket_p: Bucket = unsafe { *self.next };
        let __z: zval = bucket_p.val;
        // SAFETY: pointer addition fits in isize and is part of the same
        // allocation
        self.next = unsafe { self.next.add(1) };
        if Z_TYPE!(&__z) == IS_UNDEF {
            self.next()
        } else {
            // Now, from ZEND_HASH_MAP_FOREACH_STR_KEY_VAL
            Some((
                #[expect(
                    clippy::unwrap_in_result,
                    reason = "Unwrap is unrelated to returned value"
                )]
                // SAFETY: known to be a valid entry in the table
                unsafe { bucket_p.key.as_mut() }.expect("Should have a key"),
                __z,
            ))
        }
    }
}

/// Representation of a *potentially packed* hash table, for read-only iteration
/// though the underlying values can be modified.
pub struct HashRepr<'table> {
    /// The underlying table.
    raw: *const HashTable,
    /// The current index.
    index: u64,
    /// The number of remaining elements to read.
    count_remaining: u32,
    /// The next element to read from.
    next_elem: *mut zval,
    /// Marker for lifetime enforcement.
    _marker: std::marker::PhantomData<&'table HashTable>,
}

impl HashRepr<'_> {
    #[must_use]
    pub fn from_reference(reference: &HashTable) -> Self {
        // SAFETY: pointer created from a reference is valid
        let size = unsafe { ZEND_HASH_ELEMENT_SIZE!(std::ptr::from_ref::<HashTable>(reference)) };
        let first: *mut zval =
            // SAFETY: pointer created from a reference is valid
            unsafe { ZEND_HASH_ELEMENT_EX!(std::ptr::from_ref::<HashTable>(reference), 0, size) };
        HashRepr {
            raw: std::ptr::from_ref::<HashTable>(reference),
            index: 0,
            count_remaining: reference.nNumUsed,
            next_elem: first,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new `HashRepr` for the table pointed to with the given pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid for reads and convertible to a reference if
    /// it is not null. This is a helper that simply uses
    /// [`pointer::as_ref()`][pointer#method.as_ref] to get a reference.
    ///
    /// # Panics
    ///
    /// Panics if the pointer given is null.
    #[must_use]
    pub unsafe fn from_pointer(ptr: *const HashTable) -> Self {
        // SAFETY: based on the provided pointer being valid
        unsafe { HashRepr::from_reference(ptr.as_ref().expect("Should be valid pointer")) }
    }
}

// ZEND_HASH_FOREACH_KEY_VAL
impl<'table> Iterator for HashRepr<'table> {
    type Item = (u64, Option<&'table mut zend_string>, zval);

    fn next(&mut self) -> Option<Self::Item> {
        // From ZEND_HASH_FOREACH
        if self.count_remaining == 0 {
            return None;
        }
        let current_val: *mut zval = self.next_elem;
        let hash: u64;
        let key: *mut zend_string;
        // SAFETY: pointer must be valid
        if unsafe { HT_IS_PACKED!(self.raw) } {
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            self.next_elem = unsafe { self.next_elem.add(1) };
            hash = self.index;
            self.index += 1;
            key = std::ptr::null_mut();
        } else {
            let bucket: *mut Bucket = self.next_elem.cast::<Bucket>();
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let next_bucket = unsafe { bucket.add(1) };
            // SAFETY: known to be a valid bucket in the table
            self.next_elem = unsafe { std::ptr::from_mut::<zval>(&mut (*next_bucket).val) };
            // SAFETY: known to be a valid bucket in the table
            hash = unsafe { (*bucket).h };
            // SAFETY: known to be a valid bucket in the table
            key = unsafe { (*bucket).key };
        }

        self.count_remaining -= 1;
        // SAFETY: hash tables should only hold valid values
        if unsafe { Z_TYPE_P!(current_val) } == IS_UNDEF {
            self.next()
        } else {
            // Now, from ZEND_HASH_FOREACH_KEY_VAL
            // SAFETY: known to be a valid entry in the table
            Some((hash, unsafe { key.as_mut() }, unsafe { *current_val }))
        }
    }
}

/// Representation of a *definitely packed* hash table, for read-only iteration
/// though the underlying values can be modified.
pub struct HashPackedRepr<'table> {
    /// The underlying table.
    _raw: *const HashTable,
    /// The one-past-the-end value.
    end: *const zval,
    /// The next value to read.
    next: *const zval,
    /// The next index to report.
    next_idx: u64,
    /// Marker for lifetime enforcement.
    _marker: std::marker::PhantomData<&'table HashTable>,
}

impl HashPackedRepr<'_> {
    /// Create a new `HashPackedRepr` for the given table.
    ///
    /// # Panics
    ///
    /// Panics if the table is not packed ([`rsphp_macros_c::HT_IS_PACKED!()`])
    /// or if the number of elements in the table cannot be converted to a
    /// [`usize`].
    #[must_use]
    pub fn from_reference(reference: &HashTable) -> Self {
        assert!(
            // SAFETY: pointer created from a reference is valid
            unsafe { HT_IS_PACKED!(std::ptr::from_ref::<HashTable>(reference)) },
            "Must be a packed HashTable"
        );
        // SAFETY: confirmed to be a packed table
        let packed_ptr = unsafe { reference.__bindgen_anon_1.arPacked };
        HashPackedRepr {
            _raw: std::ptr::from_ref::<HashTable>(reference),
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            end: unsafe {
                packed_ptr.add(
                    reference
                        .nNumUsed
                        .try_into()
                        .expect("Should be valid usize"),
                )
            },
            next: packed_ptr,
            next_idx: 0,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new `HashPackedRepr` for the table pointed to with the given
    /// pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid for reads and convertible to a reference if
    /// it is not null. This is a helper that simply uses
    /// [`pointer::as_ref()`][pointer#method.as_ref] to get a reference.
    ///
    /// # Panics
    ///
    /// Panics if the pointer given is null, as well as for any condition where
    /// [`Self::from_reference()`] panics.
    #[must_use]
    pub unsafe fn from_pointer(ptr: *const HashTable) -> Self {
        // SAFETY: based on the provided pointer being valid
        unsafe { HashPackedRepr::from_reference(ptr.as_ref().expect("Should be valid pointer")) }
    }
}

// ZEND_HASH_PACKED_FOREACH_KEY_VAL
impl Iterator for HashPackedRepr<'_> {
    type Item = (u64, zval);

    fn next(&mut self) -> Option<Self::Item> {
        // From ZEND_HASH_PACKED_FOREACH_FROM
        if self.next == self.end {
            return None;
        }
        // SAFETY: until we get to the end pointers are valid
        let __z: zval = unsafe { *self.next };
        // SAFETY: pointer addition fits in isize and is part of the same
        // allocation
        self.next = unsafe { self.next.add(1) };
        let idx = self.next_idx;
        self.next_idx += 1;
        if Z_TYPE!(&__z) == IS_UNDEF {
            self.next()
        } else {
            Some((idx, __z))
        }
    }
}
