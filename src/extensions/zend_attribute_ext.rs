//! Extension trait for [`rsphp_sys::zend_attribute`].
use rsphp_sys::{zend_attribute, zend_attribute_arg};
use std::ops::Range;

pub trait ZendAttributeExt {
    fn args(&self) -> ZendAttributeArgIter<'_>;
}

impl ZendAttributeExt for &zend_attribute {
    fn args(&self) -> ZendAttributeArgIter<'_> {
        ZendAttributeArgIter {
            raw: self,
            range: 0..(self.argc.try_into().expect("Valid as usize")),
        }
    }
}

/// Iterator over the arguments of a [`rsphp_sys::zend_attribute`].
pub struct ZendAttributeArgIter<'attr> {
    /// The target [`rsphp_sys::zend_attribute`].
    raw: &'attr zend_attribute,
    /// The range of arguments left to process.
    range: Range<usize>,
}

impl Iterator for ZendAttributeArgIter<'_> {
    type Item = zend_attribute_arg;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| {
            let list_ptr = self.raw.args.as_ptr();
            // SAFETY: pointer addition fits in isize and is part of the same
            // allocation
            let item_ptr = unsafe { list_ptr.add(index) };
            // SAFETY: zend_attribute records the size of the list and the
            // list elements must be valid
            unsafe { *item_ptr }
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}
