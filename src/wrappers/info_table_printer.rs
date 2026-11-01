use rsphp_sys::{php_info_print_table_end, php_info_print_table_row, php_info_print_table_start};

use std::ffi::CString;

/// A utility for printing out extension information tables.
#[derive(Debug)]
pub struct InfoTablePrinter {
    /// If nothing is printed then we don't need to open and close the table.
    have_printed: bool,
}

impl InfoTablePrinter {
    /// Print the given rows.
    ///
    /// See [`InfoTablePrinter::row()`] for the conditions under which this
    /// method may panic.
    pub fn print(rows: &[&[&str]]) {
        let mut printer = InfoTablePrinter::new();
        for row in rows {
            printer.row(row);
        }
    }

    #[must_use]
    /// Create a new information table printer.
    pub fn new() -> Self {
        InfoTablePrinter {
            have_printed: false,
        }
    }

    /// Print a row.
    ///
    /// # Panics
    ///
    /// Panics if any string contains interior null bytes, which cannot be
    /// handled by a C function that uses null-terminated strings.
    ///
    /// # Caveats
    ///
    /// Given that [`rsphp_sys::php_info_print_table_row()`] is variadic and
    /// the Rust code here needs to call it with a known number of arguments,
    /// if more than two values in the row are desired this method will print
    /// the first two values and then a warning; do your own printing or request
    /// that the limit be increased if needed.
    #[expect(clippy::missing_asserts_for_indexing, reason = "Length checked")]
    pub fn row(&mut self, values: &[&str]) {
        if values.is_empty() {
            return;
        }
        if !self.have_printed {
            // SAFETY: php_info_print_table_start() has no preconditions
            unsafe { php_info_print_table_start() };
            self.have_printed = true;
        }
        // No way to specify a variadic call, so hard-code some options
        let str1 = CString::new(values[0]).expect("Should be valid");
        if values.len() == 1 {
            // SAFETY: all pointers are null-terminated strings, and the number
            // given in the first parameter matches the number of subsequent
            // arguments
            unsafe { php_info_print_table_row(1, str1.as_ptr()) };
            return;
        }
        let str2 = CString::new(values[1]).expect("Should be valid");
        if values.len() >= 2 {
            // SAFETY: all pointers are null-terminated strings, and the number
            // given in the first parameter matches the number of subsequent
            // arguments
            unsafe { php_info_print_table_row(2, str1.as_ptr(), str2.as_ptr()) };
            return;
        }
        if values.len() > 2 {
            // Safety: pointer is for a null-terminated string, correct number
            // of arguments given
            unsafe {
                php_info_print_table_row(
                    1,
                    c"WARNING: rsphp::wrappers::InfoTablePrinter::row() truncated some values",
                );
            }
        }
    }
}

impl Default for InfoTablePrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for InfoTablePrinter {
    fn drop(&mut self) {
        if self.have_printed {
            // SAFETY: php_info_print_table_end() has no preconditions
            unsafe { php_info_print_table_end() };
        }
    }
}
