//! Extension trait for [`rsphp_sys::zend_module_dep`].
use rsphp_sys::zend_module_dep;
use std::ffi::CStr;

pub trait ZendModuleDepExt {
    fn get_relation(&self) -> Option<&str>;
    fn get_version(&self) -> Option<&str>;
}

impl ZendModuleDepExt for zend_module_dep {
    fn get_relation(&self) -> Option<&str> {
        if self.rel.is_null() {
            return None;
        }
        assert!(self.rel.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let relation = unsafe { CStr::from_ptr(self.rel) };
        Some(relation.to_str().expect("Relations should be valid UTF8"))
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
