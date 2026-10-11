use rsphp_sys::{php_info_print_table_end, php_info_print_table_row, php_info_print_table_start};

use std::ffi::CString;

pub struct InfoTablePrinter {
    /// If nothing is printed then we don't need to open and close the table.
    have_printed: bool,
}

impl InfoTablePrinter {
    pub fn print(rows: &[&[&str]]) {
        let mut printer = InfoTablePrinter::new();
        for row in rows {
            printer.row(row);
        }
    }

    #[must_use]
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
    /// handled by a C function that uses null-terminated strings, or if
    /// more than two values are printed for the same row.
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
        if values.len() == 2 {
            // SAFETY: all pointers are null-terminated strings, and the number
            // given in the first parameter matches the number of subsequent
            // arguments
            unsafe { php_info_print_table_row(2, str1.as_ptr(), str2.as_ptr()) };
            return;
        }
        panic!("Unhandled number of values: {}", values.len());
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
