//! Extension trait for [`rsphp_sys::zend_type_list`].
use rsphp_sys::{zend_type, zend_type_list};
use std::ops::Range;

// Cannot implement `Iterator` directly on `zend_type_list` so add a method instead
pub trait ZendTypeListExt {
    fn iter(&self) -> ZendTypeListIter<'_>;
}

impl ZendTypeListExt for &zend_type_list {
    fn iter(&self) -> ZendTypeListIter<'_> {
        ZendTypeListIter {
            raw: self,
            range: 0..(self.num_types.try_into().expect("Valid as usize")),
        }
    }
}

/// Iterator over the types in a [`rsphp_sys::zend_type_list`].
pub struct ZendTypeListIter<'list> {
    /// The target [`rsphp_sys::zend_type_list`].
    raw: &'list zend_type_list,
    /// The range of types left to process.
    range: Range<usize>,
}

// ZEND_TYPE_LIST_FOREACH
impl Iterator for ZendTypeListIter<'_> {
    type Item = zend_type;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| {
            let list_ptr = self.raw.types.as_ptr();
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let item_ptr = unsafe { list_ptr.add(index) };
            // SAFETY: zend_type_list records the size of the list and the
            // list elements must be valid
            unsafe { *item_ptr }
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}
