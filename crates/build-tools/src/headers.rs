/// The Zend headers to add.
const ZEND_HEADERS: &[&str] = &[
    "zend.h",
    "zend_API.h",
    "zend_alloc.h",
    "zend_alloc_sizes.h",
    "zend_arena.h",
    "zend_ast.h",
    "zend_atomic.h",
    "zend_attributes.h",
    "zend_bitset.h",
    "zend_build.h",
    "zend_builtin_functions.h",
    "zend_call_stack.h",
    "zend_closures.h",
    "zend_compile.h",
    "zend_config.h",
    "zend_constants.h",
    "zend_cpuinfo.h",
    "zend_dtrace.h",
    "zend_enum.h",
    "zend_errors.h",
    "zend_exceptions.h",
    "zend_execute.h",
    "zend_extensions.h",
    "zend_fibers.h",
    "zend_float.h",
    "zend_frameless_function.h",
    "zend_gc.h",
    "zend_gdb.h",
    "zend_generators.h",
    "zend_globals.h",
    "zend_globals_macros.h",
    "zend_hash.h",
    "zend_highlight.h",
    "zend_hrtime.h",
    "zend_inheritance.h",
    "zend_ini.h",
    "zend_ini_scanner.h",
    "zend_interfaces.h",
    "zend_iterators.h",
    "zend_language_scanner.h",
    "zend_lazy_objects.h",
    "zend_list.h",
    "zend_llist.h",
    "zend_long.h",
    "zend_map_ptr.h",
    "zend_max_execution_timer.h",
    "zend_mmap.h",
    "zend_modules.h",
    "zend_multibyte.h",
    "zend_multiply.h",
    "zend_object_handlers.h",
    "zend_objects.h",
    "zend_objects_API.h",
    "zend_observer.h",
    "zend_operators.h",
    "zend_portability.h",
    "zend_property_hooks.h",
    "zend_ptr_stack.h",
    "zend_range_check.h",
    "zend_signal.h",
    "zend_simd.h",
    "zend_smart_str.h",
    "zend_smart_str_public.h",
    "zend_smart_string.h",
    "zend_smart_string_public.h",
    "zend_sort.h",
    "zend_stack.h",
    "zend_stream.h",
    "zend_string.h",
    "zend_strtod.h",
    "zend_strtod_int.h",
    "zend_system_id.h",
    "zend_type_info.h",
    "zend_types.h",
    "zend_variables.h",
    "zend_virtual_cwd.h",
    "zend_vm.h",
    "zend_vm_handlers.h",
    "zend_vm_opcodes.h",
    "zend_weakrefs.h",
];

/// Helper to iterate over known Zend headers to add to bindgen.
#[derive(Debug)]
pub struct ZendHeaderIterator {
    /// `.../Zend` directory in a PHP installation.
    directory: String,
}

impl ZendHeaderIterator {
    #[must_use]
    /// Create a new iterator for the Zend headers, given the path to their
    /// directory.
    pub fn new(directory: &str) -> Self {
        ZendHeaderIterator {
            directory: directory.to_owned(),
        }
    }
}

impl IntoIterator for ZendHeaderIterator {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        ZEND_HEADERS
            .iter()
            .map(|header| self.directory.clone() + header)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
