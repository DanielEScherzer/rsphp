//! This module defines a number of extension traits that are implemented for
//! `rsphp_sys` types, to add extra functionality to those types.

mod hash_table_ext;
pub use hash_table_ext::HashTableExt;

mod zend_attribute_ext;
pub use zend_attribute_ext::ZendAttributeExt;

mod zend_arg_info_ext;
pub use zend_arg_info_ext::ZendArgInfoExt;

mod zend_class_constant_ext;
pub use zend_class_constant_ext::ZendClassConstantExt;

mod zend_class_entry_ext;
pub use zend_class_entry_ext::ZendClassEntryExt;

mod zend_constant_ext;
pub use zend_constant_ext::ZendConstantExt;

mod zend_extension_ext;
pub use zend_extension_ext::ZendExtensionExt;

mod zend_function_ext;
pub use zend_function_ext::ZendFunctionExt;

mod zend_module_dep_ext;
pub use zend_module_dep_ext::ZendModuleDepExt;

mod zend_module_entry_ext;
pub use zend_module_entry_ext::ZendModuleEntryExt;

mod zend_string_ext;
pub use zend_string_ext::ZendStringExt;

mod zend_type_ext;
pub use zend_type_ext::ZendTypeExt;

mod zend_type_list_ext;
pub use zend_type_list_ext::ZendTypeListExt;

mod zval_ext;
pub use zval_ext::ZvalExt;
