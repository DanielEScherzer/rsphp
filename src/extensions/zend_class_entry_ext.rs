//! Extension trait for [`rsphp_sys::zend_class_entry`].
use rsphp_sys::{instanceof_function, zend_class_entry, zend_class_name};
use std::ops::Range;

pub trait ZendClassEntryExt {
    fn get_trait_names(&self) -> ZendClassEntryTraitNamesIter<'_>;
    fn instance_of(&self, other: &zend_class_entry) -> bool;
}

impl ZendClassEntryExt for &zend_class_entry {
    fn get_trait_names(&self) -> ZendClassEntryTraitNamesIter<'_> {
        ZendClassEntryTraitNamesIter {
            raw: self,
            range: 0..(self.num_traits.try_into().expect("Valid as usize")),
        }
    }
    fn instance_of(&self, other: &zend_class_entry) -> bool {
        // SAFETY: valid pointers are passed to instanceof_function()
        unsafe { instanceof_function(*self, other) }
    }
}

impl ZendClassEntryExt for &mut zend_class_entry {
    fn get_trait_names(&self) -> ZendClassEntryTraitNamesIter<'_> {
        ZendClassEntryTraitNamesIter {
            raw: self,
            range: 0..(self.num_traits.try_into().expect("Valid as usize")),
        }
    }
    fn instance_of(&self, other: &zend_class_entry) -> bool {
        // SAFETY: valid pointers are passed to instanceof_function()
        unsafe { instanceof_function(*self, other) }
    }
}

/// Iterator over the trait names of a [`rsphp_sys::zend_class_entry`].
pub struct ZendClassEntryTraitNamesIter<'cls> {
    /// The target [`rsphp_sys::zend_class_entry`].
    raw: &'cls zend_class_entry,
    /// The range of trait names left to process.
    range: Range<usize>,
}

impl Iterator for ZendClassEntryTraitNamesIter<'_> {
    type Item = zend_class_name;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| {
            let list_ptr = self.raw.trait_names;
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let item_ptr = unsafe { list_ptr.add(index) };
            // SAFETY: num_traits records the size of the list and the list
            // elements must be valid
            unsafe { *item_ptr }
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}
