//! Extension trait for [`rsphp_sys::zend_module_entry`].
use rsphp_sys::zend_module_entry;
use std::ffi::CStr;

pub trait ZendModuleEntryExt {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> Option<&str>;
}

impl ZendModuleEntryExt for zend_module_entry {
    fn get_name(&self) -> &str {
        assert!(!self.name.is_null(), "Must not be null");
        assert!(self.name.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let name = unsafe { CStr::from_ptr(self.name) };
        name.to_str().expect("Names should be valid UTF8")
    }

    fn get_version(&self) -> Option<&str> {
        if self.version.is_null() {
            return None;
        }
        assert!(self.version.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let version = unsafe { CStr::from_ptr(self.version) };
        Some(version.to_str().expect("Versions should be valid UTF8"))
    }
}
