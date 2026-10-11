use rsphp_sys::{zval, zval_ptr_dtor, zval_ptr_dtor_nogc};
use std::convert::AsRef;
use std::ops::Deref;

pub struct StackZval {
    /// The wrapped zval.
    inner: zval,
    /// Whether the `drop` handler should use [`rsphp_sys::zval_ptr_dtor()`]
    /// (if true) or [`rsphp_sys::zval_ptr_dtor_nogc()`] (if false).
    gc: bool,
}

/// Rust handling of stack-allocated [`rsphp_sys::zval`] values.
///
/// Can be dereferenced into the underlying zval.
///
/// Calls [`rsphp_sys::zval_ptr_dtor`] automatically on drop.
#[expect(clippy::new_without_default, reason = "Default is not appropriate")]
impl StackZval {
    /// Create a new uninitialized [`rsphp_sys::zval`].
    ///
    /// Because [`rsphp_sys::IS_UNDEF`] has the value of 0, using [`std::mem::zeroed`]
    /// results in a zval marked as `IS_UNDEF`.
    #[must_use]
    pub fn new() -> Self {
        StackZval {
            // SAFETY: zvals can be initialized with zero bytes
            inner: unsafe { std::mem::zeroed() },
            gc: true,
        }
    }

    /// Access a constant pointer to the contained instance, for passing to FFI.
    pub fn ptr(&mut self) -> *const zval {
        std::ptr::from_ref(&self.inner)
    }

    /// Access a mutable pointer to the contained instance, for passing to FFI.
    pub fn mut_ptr(&mut self) -> *mut zval {
        std::ptr::from_mut(&mut self.inner)
    }

    pub fn set_no_gc_dtor(&mut self) {
        self.gc = false;
    }
}

impl Drop for StackZval {
    fn drop(&mut self) {
        if self.gc {
            // SAFETY: inner is a valid zval pointer
            unsafe { zval_ptr_dtor(std::ptr::from_mut(&mut self.inner)) };
        } else {
            // SAFETY: inner is a valid zval pointer
            unsafe { zval_ptr_dtor_nogc(std::ptr::from_mut(&mut self.inner)) };
        }
    }
}

impl Deref for StackZval {
    type Target = zval;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<zval> for StackZval {
    /// Access an immutable reference to the contained instance.
    fn as_ref(&self) -> &zval {
        &self.inner
    }
}
