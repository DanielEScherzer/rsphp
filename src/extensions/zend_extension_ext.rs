//! Extension trait for [`rsphp_sys::zend_extension`].
use rsphp_sys::zend_extension;
use std::ffi::CStr;

pub trait ZendExtensionExt {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> Option<&str>;
    fn get_author(&self) -> Option<&str>;
    fn get_url(&self) -> Option<&str>;
    fn get_copyright(&self) -> Option<&str>;
}

impl ZendExtensionExt for &zend_extension {
    fn get_name(&self) -> &str {
        assert!(!self.name.is_null(), "Must have a name");
        assert!(self.name.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let result = unsafe { CStr::from_ptr(self.name) };
        result.to_str().expect("Names should be valid UTF8")
    }
    fn get_version(&self) -> Option<&str> {
        if self.version.is_null() {
            return None;
        }
        assert!(self.version.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let result = unsafe { CStr::from_ptr(self.version) };
        Some(result.to_str().expect("Versions should be valid UTF8"))
    }
    fn get_author(&self) -> Option<&str> {
        if self.author.is_null() {
            return None;
        }
        assert!(self.author.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let result = unsafe { CStr::from_ptr(self.author) };
        Some(result.to_str().expect("Authors should be valid UTF8"))
    }
    fn get_url(&self) -> Option<&str> {
        if self.URL.is_null() {
            return None;
        }
        assert!(self.URL.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let result = unsafe { CStr::from_ptr(self.URL) };
        Some(result.to_str().expect("URLs should be valid UTF8"))
    }
    fn get_copyright(&self) -> Option<&str> {
        if self.copyright.is_null() {
            return None;
        }
        assert!(self.copyright.is_aligned(), "Must be aligned");
        // SAFETY: pointer is valid for cstring reads
        let result = unsafe { CStr::from_ptr(self.copyright) };
        Some(result.to_str().expect("Copyrights should be valid UTF8"))
    }
}
