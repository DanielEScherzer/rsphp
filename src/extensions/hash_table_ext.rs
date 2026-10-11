//! Extension trait for [`rsphp_sys::HashTable`].
use crate::PHPValue;
use rsphp_sys::{
    HashTable, zend_hash_exists, zend_hash_find, zend_hash_next_index_insert, zend_string, zval,
};
use std::ptr::NonNull;

pub trait HashTableExt {
    fn contains(&self, key: &mut zend_string) -> bool;
    fn next_index_insert<T>(&mut self, value: T) -> *mut zval
    where
        T: Into<PHPValue>;
    fn find(&self, key: &mut zend_string) -> Option<NonNull<zval>>;
    fn num_elements(&self) -> u32;
}

impl HashTableExt for HashTable {
    fn contains(&self, key: &mut zend_string) -> bool {
        // SAFETY: following the contract of zend_hash_exists()
        unsafe {
            zend_hash_exists(
                std::ptr::from_ref::<HashTable>(self),
                std::ptr::from_mut::<zend_string>(key),
            )
        }
    }
    fn next_index_insert<T>(&mut self, value: T) -> *mut zval
    where
        T: Into<PHPValue>,
    {
        // SAFETY: zvals can be initialized with zero bytes
        let mut obj_to_insert: zval = unsafe { std::mem::zeroed() };
        let val: PHPValue = value.into();
        // SAFETY: we have a valid zval pointer
        unsafe {
            val.apply_to(std::ptr::from_mut::<zval>(&mut obj_to_insert));
        }
        // SAFETY: following the contract of zend_hash_next_index_insert()
        unsafe {
            zend_hash_next_index_insert(
                std::ptr::from_mut::<HashTable>(self),
                std::ptr::from_mut::<zval>(&mut obj_to_insert),
            )
        }
    }
    fn find(&self, key: &mut zend_string) -> Option<NonNull<zval>> {
        // SAFETY: following the contract of zend_hash_find()
        let result = unsafe {
            zend_hash_find(
                std::ptr::from_ref::<HashTable>(self),
                std::ptr::from_mut::<zend_string>(key),
            )
        };
        NonNull::new(result)
    }
    fn num_elements(&self) -> u32 {
        // Avoid needing to cross FFI for just accessing a field
        self.nNumOfElements
    }
}
