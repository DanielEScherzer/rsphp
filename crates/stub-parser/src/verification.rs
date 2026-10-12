use php_ast::ast::Visibility;
use std::collections::HashSet;
use std::fmt::Write as _;

/// How should calls to `register_class_*` methods be verified for correctness?
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VerificationMode {
    /// Always run assertions.
    Always,
    /// Only run full assertions when debug assertions are enabled.
    DebugAssert,
    /// Don't add extra assertions (beyond the number of methods, which is
    /// always checked).
    Never,
}

impl VerificationMode {
    /// Get the code to set up verification, with the current mode applied.
    pub(crate) fn get_infra(self) -> String {
        match self {
            VerificationMode::Always => VERIFICATION_INFRASTRUCTURE.join("\n"),
            VerificationMode::DebugAssert => VERIFICATION_INFRASTRUCTURE
                .iter()
                .map(|part: &&str| match part.split_once("\nfn ") {
                    Some((docs, fn_decl)) => {
                        format!("{docs}\n#[cfg(debug_assertions)]\nfn {fn_decl}")
                    }
                    None => format!("#[cfg(debug_assertions)]{part}"),
                })
                .collect::<Vec<_>>()
                .join("\n"),
            VerificationMode::Never => String::new(),
        }
    }

    /// Get the code to apply to a rust import, with the current verification
    /// mode applied.
    ///
    /// If `omit` is true then an empty string is always returned.
    pub(crate) fn apply_to_import(self, import: &str, omit: bool) -> String {
        if omit {
            return String::new();
        }
        match self {
            VerificationMode::Always => import.to_owned(),
            VerificationMode::Never => String::new(),
            VerificationMode::DebugAssert => format!("#[cfg(debug_assertions)]\n{import}"),
        }
    }

    /// Get the code to apply to the methods of a class/interface/enum with the
    /// given name, with the current verification mode applied.
    pub(crate) fn apply_to_methods(self, name: &str, methods: &[VerificationData]) -> String {
        if methods.is_empty() {
            return String::new();
        }
        let method_count = methods.len();
        let count_assert = format!(
            "\n    assert_eq!({}, methods.len(), \"{name} should have {method_count} methods\");",
            method_count + 1
        );
        let full_verification = format!(
            "\n    verify_registration(\n        \"{name}\",\n        &[{expected}\n        ],\n        methods,\n    );",
            expected = methods.iter().fold(String::new(), |mut output, method| {
                write!(
                    output,
                    "\n            {},",
                    method.as_verification_parameter()
                )
                .expect("String writes are infallible");
                output
            })
        );
        match self {
            VerificationMode::Never => count_assert + "\n",
            VerificationMode::DebugAssert => format!(
                "\n    #[cfg(debug_assertions)]{full_verification}\n    #[cfg(not(debug_assertions))]{count_assert}\n"
            ),
            VerificationMode::Always => full_verification + "\n",
        }
    }
}

/// Data to apply for verification of class registration.
pub(crate) struct VerificationData {
    /// Name of method to register.
    method_name: String,
    /// Name of the `ARGINFO_*` constant to compare with.
    arginfo_name: String,
    /// Expected `ZEND_ACC_*` flags the registration should include.
    flags: Vec<String>,
}

impl VerificationData {
    /// Create a new `VerificationData` for the given method name and arginfo
    /// constant name.
    pub(crate) fn new(method_name: String, arginfo_name: String) -> Self {
        VerificationData {
            method_name,
            arginfo_name,
            flags: vec![],
        }
    }

    /// Get all of the flags that any of the methods use, for adding `use`
    /// statements.
    pub(crate) fn get_all_flags(methods: &[VerificationData]) -> HashSet<String> {
        let mut result = HashSet::new();
        for method in methods {
            result.extend(method.flags.clone());
        }
        result
    }

    /// Add a `ZEND_ACC_*` flag based on the visibility to the expected flags.
    pub(crate) fn add_visibility(&mut self, vis: Option<&Visibility>) {
        let flag = match vis {
            None | Some(Visibility::Public) => "ZEND_ACC_PUBLIC",
            Some(Visibility::Protected) => "ZEND_ACC_PROTECTED",
            Some(Visibility::Private) => "ZEND_ACC_PRIVATE",
        };
        self.add_flag(flag);
    }

    /// Add the specified flag to the expected flags.
    pub(crate) fn add_flag(&mut self, flag: &str) {
        self.flags.push(flag.to_owned());
    }

    /// If the condition is true, add the specified flag to the expected flags.
    pub(crate) fn add_flag_if(&mut self, cond: bool, flag: &str) {
        if cond {
            self.flags.push(flag.to_owned());
        }
    }

    /// Format the verification data as a parameter to the `verify_registration()`
    /// function.
    fn as_verification_parameter(&self) -> String {
        format!(
            "(\"{}\", &{}, {})",
            self.method_name,
            self.arginfo_name,
            self.flags.join("|")
        )
    }
}

/// Imports needed for infrastructure, at a file level.
pub(crate) const VERIFICATION_RSPHP_SYS_IMPORTS: &[&str] = &[
    "_ZEND_TYPE_LIST_BIT",
    "_ZEND_TYPE_LITERAL_NAME_BIT",
    "_ZEND_TYPE_NAME_BIT",
    "zend_function_entry",
    "zend_internal_arg_info",
    "zend_string",
    "zend_type",
    "zend_type_list",
];

/// Infrastructure for registration verification.
const VERIFICATION_INFRASTRUCTURE: &[&str] = &[
    "\nuse std::ffi::{c_char, CStr};",
    r#"
/// Check if two type lists match.
fn type_lists_match(expected: &zend_type_list, actual: &zend_type_list) -> bool {
    if expected.num_types != actual.num_types {
        return false;
    }
    let expected_types: &[zend_type] = unsafe {
        std::slice::from_raw_parts(
            expected.types.as_ptr(),
            expected.num_types.try_into().expect("Valid as a usize"),
        )
    };
    let actual_types: &[zend_type] = unsafe {
        std::slice::from_raw_parts(actual.types.as_ptr(), actual.num_types.try_into().expect("Valid as a usize"))
    };
    expected_types.iter().zip(actual_types).all(|(e, a)| types_match(e, a).is_none())
}"#,
    r#"
/// Check if two types match, returning an error message with some details if
/// they do not.
#[expect(clippy::unwrap_in_result, reason = "Failures are Some(s) rather than None, and startup code should panic")]
fn types_match(expected: &zend_type, actual: &zend_type) -> Option<String> {
    if expected.type_mask != actual.type_mask {
        return Some(format!("Expected type mask 0x{:x}, found 0x{:x}", expected.type_mask, actual.type_mask));
    }
    if expected.ptr.is_null() {
        if actual.ptr.is_null() {
            return None;
        }
        return Some("Expected no pointer to be set in actual type".to_owned());
    }
    if actual.ptr.is_null() {
        return Some("Expected a pointer to be set in the actual type".to_owned());
    }
    // Compare the values depending on the flags
    if expected.type_mask & _ZEND_TYPE_LITERAL_NAME_BIT != 0 {
        // CString pointers
        let expected_name = unsafe { CStr::from_ptr(expected.ptr.cast::<c_char>()) };
        let actual_name = unsafe { CStr::from_ptr(actual.ptr.cast::<c_char>()) };
        if expected_name == actual_name {
            return None;
        }
        return Some(format!("Expected literal name {expected_name:?}, found {actual_name:?}"));
    }
    if expected.type_mask & _ZEND_TYPE_NAME_BIT != 0 {
        // zend_string pointers, compare them by comparing the lengths and values
        let expected_name: &zend_string = unsafe { expected.ptr.cast::<zend_string>().as_ref().expect("Valid pointer") };
        let actual_name: &zend_string = unsafe { actual.ptr.cast::<zend_string>().as_ref().expect("Valid pointer") };
        if expected_name.len != actual_name.len {
            return Some(format!("Expected name of length {}, found name of length {}", expected_name.len, actual_name.len));
        }
        let expected_slice: &[c_char] = unsafe { std::slice::from_raw_parts(expected_name.val.as_ptr(), expected_name.len) };
        let actual_slice: &[c_char] = unsafe { std::slice::from_raw_parts(actual_name.val.as_ptr(), actual_name.len) };
        if expected_slice == actual_slice {
            return None;
        }
        return Some(format!("Expected name {expected_slice:?}, found {actual_slice:?}"));
    }
    if expected.type_mask & _ZEND_TYPE_LIST_BIT != 0 {
        // zend_type_list pointers
        let expected_list: &zend_type_list = unsafe { expected.ptr.cast::<zend_type_list>().as_ref().expect("Valid pointer") };
        let acutal_list: &zend_type_list = unsafe { actual.ptr.cast::<zend_type_list>().as_ref().expect("Valid pointer") };
        if type_lists_match(expected_list, acutal_list) {
            return None;
        }
        return Some("Type lists differ".to_owned());
    }
    panic!("Unhandled zend types, unknown mask type: {}", expected.type_mask);
}"#,
    r#"
/// Verify a function argument information matches the expected information,
/// returning an error of errors.
fn verify_arg(
    idx: usize,
    method: &str,
    expected: &zend_internal_arg_info,
    actual: &zend_internal_arg_info,
) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    if idx == 0 {
        // Dummy entry at the start with information for the overall function
        assert!(expected.default_value.is_null(), "{method}: expected arg info for function registration cannot have a default value");
        if !actual.default_value.is_null() {
            errors.push(format!("{method}: Arg info for function registration has an unexpected default value"));
        }
        // `name` is abused as the number of required arguments
        let expected_req_args = expected.name.addr();
        let actual_req_args = actual.name.addr();
        if expected_req_args != actual_req_args {
            errors.push(format!("{method}: expected {expected_req_args} required arguments, found {actual_req_args}"));
        }
        if let Some(error) = types_match(&expected.type_, &actual.type_) {
            errors.push(format!("{method}: return types differ: {error}"));
        }
        return errors;
    }
    let expected_name = unsafe { CStr::from_ptr(expected.name) };
    if actual.name.is_null() {
        errors.push(format!("{method}: argument {} is missing a name", idx - 1));
    } else {
        let actual_name = unsafe { CStr::from_ptr(actual.name) };
        if expected_name != actual_name {
            errors.push(format!("{method}: argument {} expected to be named {expected_name:?}, actually named {actual_name:?}", idx - 1));
        }
    }
    if let Some(error) = types_match(&expected.type_, &actual.type_) {
        errors.push(format!("{method}: argument {} type does not match expected: {error}", idx - 1));
    }
    match (expected.default_value.is_null(), actual.default_value.is_null()) {
        (true, true) => (),
        (true, false) => {
            let actual_default = unsafe { CStr::from_ptr(actual.default_value) };
            errors.push(format!("{method}: argument {} has unexpected default: {actual_default:?}", idx - 1));
        }
        (false, true) => {
            let expected_default = unsafe { CStr::from_ptr(expected.default_value) };
            errors.push(format!("{method}: argument {} missing expected default: {expected_default:?}", idx - 1));
        }
        (false, false) => {
            let actual_default = unsafe { CStr::from_ptr(actual.default_value) };
            let expected_default = unsafe { CStr::from_ptr(expected.default_value) };
            if actual_default != expected_default {
                errors.push(format!(
                    "{method}: argument {} expected to have default {expected_default:?}, found {actual_default:?}",
                    idx - 1
                ));
            }
        }
    }
    errors
}"#,
    r#"
/// Verify the registration of a function or method.
fn verify_registration(
    debug_name: &str,
    expected_methods: &[(&str, &[zend_internal_arg_info], u32)],
    actual_methods: &[zend_function_entry],
) {
    let mut remaining_methods = actual_methods.to_vec();
    // Must end with the results of ZEND_FE_END
    // So that we can use unwrapping below, do these assertions immediately
    let last = remaining_methods.pop().unwrap_or_else(|| panic!("{debug_name} registration should end with ZEND_FE_END"));
    assert!(last.fname.is_null(), "{debug_name} registration should end with ZEND_FE_END");
    assert!(last.handler.is_none(), "{debug_name} registration should end with ZEND_FE_END");
    assert!(last.arg_info.is_null(), "{debug_name} registration should end with ZEND_FE_END");
    assert_eq!(0, last.num_args, "{debug_name} registration should end with ZEND_FE_END");
    assert_eq!(0, last.flags, "{debug_name} registration should end with ZEND_FE_END");
    assert!(last.frameless_function_infos.is_null(), "{debug_name} registration should end with ZEND_FE_END");
    assert!(last.doc_comment.is_null(), "{debug_name} registration should end with ZEND_FE_END");

    // So that we don't need to keep converting from char pointers
    let mut methods_and_names: Vec<_> = remaining_methods.into_iter().map(
        |fn_entry| (
            unsafe { CStr::from_ptr(fn_entry.fname) }.to_str().expect("Should be a valid method name"),
            fn_entry
        )
    ).collect();
    let mut errors: Vec<String> = vec![];
    for &(name, expect_args, expect_flags) in expected_methods {
        let index_opt = methods_and_names
            .iter()
            .position(|&(fn_name, _)| fn_name == name);
        let Some(index) = index_opt else {
            errors.push(format!("Missing registration of method {name}"));
            continue;
        };
        let (_, fn_entry) = methods_and_names.remove(index);
        if fn_entry.flags != expect_flags {
            errors.push(format!(
                "Registration of method {name} has wrong flags, expected {expect_flags}, found {}",
                fn_entry.flags
            ));
        }
        if fn_entry.arg_info == expect_args.as_ptr() {
            continue;
        }
        // Rust global `const` values are *not* guaranteed to refer to the same
        // location in memory because they might be inlined; compare the actual values
        if fn_entry.num_args + 1 != expect_args.len().try_into().expect("Valid as a usize") {
            errors.push(
                format!("Registration of method {name} has wrong arg info, expected {} args, found {}",
                expect_args.len() - 1, fn_entry.num_args)
            );
            continue;
        }
        let fn_entry_arginfo = unsafe {
            std::slice::from_raw_parts(
                fn_entry.arg_info,
                (fn_entry.num_args + 1).try_into().expect("Valid as a usize"),
            )
        };
        for (idx, (expected, found)) in expect_args.iter().zip(fn_entry_arginfo).enumerate() {
            errors.extend(verify_arg(idx, name, expected, found));
        }
    }
    for (name, _) in methods_and_names {
        errors.push(format!("Unexpected registration of method {name}"));
    }
    let expected: &[String] = &[];
    assert_eq!(expected, errors, "Registration of {debug_name} should be correct");
}"#,
];
