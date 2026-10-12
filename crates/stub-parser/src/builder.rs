use crate::VerificationMode;
use crate::types::ArgInfoTypeRepr;
use crate::verification::{VERIFICATION_RSPHP_SYS_IMPORTS, VerificationData};
use php_ast::ast::{BuiltinType, Visibility};
use php_ast::owned::{
    Attribute, ClassConstDecl, ClassDecl, ClassMemberKind, Comment, EnumCase, EnumDecl,
    EnumMemberKind, Expr, ExprKind, FunctionDecl, Ident, InterfaceDecl, MethodDecl, Name,
    NamespaceBody, NamespaceDecl, Param, PropertyDecl, Stmt, StmtKind, TypeHint, TypeHintKind,
};
use php_ast::span::Span;
use regex::Regex;
use std::assert_matches;
use std::collections::HashSet;
use std::fmt::Write as _;

/// Wrapper around `EnumDecl`, `ClassDecl`, or `InterfaceDecl` to present a
/// common interface for class-like declarations.
#[derive(Debug)]
enum ClassLikeType {
    /// A PHP enum declaration.
    PhpEnum(EnumDecl),
    /// A PHP class declaration.
    Class(ClassDecl),
    /// A PHP interface declaration.
    Interface(InterfaceDecl),
}
/// Wrapper around `EnumCase`, `PropertyDecl`, `MethodDecl`, or `ClassConstDecl`
/// to present a common interface for class-member declarations.
enum ClassLikeMember {
    /// An enum case.
    EnumCase(EnumCase),
    /// A class or interface property.
    Property(PropertyDecl),
    /// A class, interface or enum method.
    Method(MethodDecl),
    /// A class, interface or enum constant.
    ClassConst(ClassConstDecl),
}
impl TryFrom<ClassMemberKind> for ClassLikeMember {
    type Error = &'static str;

    fn try_from(orig: ClassMemberKind) -> Result<Self, Self::Error> {
        match orig {
            ClassMemberKind::Property(property) => Ok(ClassLikeMember::Property(property)),
            ClassMemberKind::Method(method) => Ok(ClassLikeMember::Method(method)),
            ClassMemberKind::ClassConst(class_const) => {
                Ok(ClassLikeMember::ClassConst(class_const))
            }
            ClassMemberKind::TraitUse(_) => Err("PHP does not support traits on internal classes"),
        }
    }
}
impl TryFrom<EnumMemberKind> for ClassLikeMember {
    type Error = &'static str;

    fn try_from(orig: EnumMemberKind) -> Result<Self, Self::Error> {
        match orig {
            EnumMemberKind::Case(case) => Ok(ClassLikeMember::EnumCase(case)),
            EnumMemberKind::Method(method) => Ok(ClassLikeMember::Method(method)),
            EnumMemberKind::ClassConst(class_const) => Ok(ClassLikeMember::ClassConst(class_const)),
            EnumMemberKind::TraitUse(_) => Err("PHP does not support traits on internal classes"),
        }
    }
}

#[expect(
    unused_results,
    reason = "HashSet used to prevent duplication, just need to include the import"
)]
impl ClassLikeType {
    /// Name of the class/interface/enum.
    fn name(&self) -> &str {
        let ident: &Ident = match self {
            ClassLikeType::PhpEnum(php_enum) => &php_enum.name,
            ClassLikeType::Interface(interface) => &interface.name,
            ClassLikeType::Class(class) => class.name.as_ref().expect("Not an anonymous class"),
        };
        ident.as_ref().expect("Valid name")
    }
    /// Body of the class/interface/enum.
    fn body_members(&self) -> Vec<ClassLikeMember> {
        match self {
            ClassLikeType::PhpEnum(php_enum) => php_enum
                .body
                .members
                .clone()
                .into_iter()
                .map(|member| member.kind.try_into().expect("Valid enum member"))
                .collect(),
            ClassLikeType::Interface(interface) => interface
                .body
                .members
                .clone()
                .into_iter()
                .map(|member| member.kind.try_into().expect("Valid interface member"))
                .collect(),
            ClassLikeType::Class(class) => class
                .body
                .members
                .clone()
                .into_iter()
                .map(|member| member.kind.try_into().expect("Valid class member"))
                .collect(),
        }
    }
    /// Attributes on the overall class/interface/enum.
    fn attributes(&self) -> &[Attribute] {
        match self {
            ClassLikeType::PhpEnum(php_enum) => &php_enum.attributes,
            ClassLikeType::Interface(interface) => &interface.attributes,
            ClassLikeType::Class(class) => &class.attributes,
        }
    }
    /// Helper to create variable names for references to zend class entries.
    fn make_ce_variable_name(type_name: &Name) -> String {
        "zend_ce_".to_owned()
            + &type_name
                .parts
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .join("_")
    }
    /// Name of a class's parent, if present.
    fn parent_class_var(&self) -> Option<String> {
        match self {
            ClassLikeType::Class(class) => class.extends.as_ref().map(Self::make_ce_variable_name),
            _ => None,
        }
    }
    /// List of a class's or enum's interfaces, or the interfaces than an
    /// interface extends, as variable names from `make_ce_variable_name()`.
    fn interface_variables(&self) -> Vec<String> {
        let interface_names: &[Name] = match self {
            ClassLikeType::PhpEnum(php_enum) => &php_enum.implements,
            ClassLikeType::Interface(interface) => &interface.extends,
            ClassLikeType::Class(class) => &class.implements,
        };
        interface_names
            .iter()
            .map(Self::make_ce_variable_name)
            .collect()
    }
    /// Code snippet to make this class/interface/enum implement/extend the
    /// interfaces that it should, with the assumption that the interfaces
    /// are available under the names from [`Self::interface_variables()`], and the
    /// overall class entry is available under the name `class_entry`.
    fn interfaces_applied_code(&self, scope: &mut ImportsUsed) -> String {
        let interface_ptrs = self.interface_variables();
        if interface_ptrs.is_empty() {
            String::new()
        } else {
            scope.import_sys.insert("zend_class_implements".to_owned());
            format!(
                "\n{space}unsafe {{\
                \n{space}{space}zend_class_implements(\
                \n{space}{space}{space}class_entry,\
                \n{space}{space}{space}{count},\
                \n{space}{space}{space}{iface_names},\
                \n{space}{space})\
                \n{space}}};",
                count = interface_ptrs.len(),
                iface_names = interface_ptrs.join(",\n            "),
                space = "    "
            )
        }
    }
    /// Code snippet to register this class/interface/enum, where the class has
    /// the given number of methods, is in the given namespace, and the overall
    /// program where the class is declared is passed to allow fetching
    /// unprocessed details.
    #[expect(clippy::too_many_lines, reason = "It's a complicated function")]
    fn init_and_register_code(
        &self,
        scope: &mut ImportsUsed,
        namespace: Option<&String>,
        method_count: usize,
        program: &str,
    ) -> Result<String, String> {
        let name = self.name();
        let ns_name_segment = match namespace {
            Some(ns) => format!("{ns}\\\\"),
            None => String::new(),
        };
        let add_interfaces = self.interfaces_applied_code(scope);
        let (macro_name, macro_args) = match (namespace, method_count) {
            (Some(ns), 0) => (
                "INIT_NS_CLASS_ENTRY",
                format!("(ce, \"{ns}\", \"{name}\", NULL)"),
            ),
            (Some(ns), _) => (
                "INIT_NS_CLASS_ENTRY",
                format!("(ce, \"{ns}\", \"{name}\", methods)"),
            ),
            (None, 0) => ("INIT_CLASS_ENTRY", format!("(ce, \"{name}\", NULL)")),
            (None, _) => ("INIT_CLASS_ENTRY", format!("(ce, \"{name}\", methods)")),
        };
        let init_macro = format!("{macro_name}!{macro_args};");

        match self {
            ClassLikeType::Class(class) => {
                let extends_param = self
                    .parent_class_var()
                    .unwrap_or_else(|| "std::ptr::null_mut()".to_owned());
                let mut flags = vec![];
                if class.modifiers.is_final {
                    flags.push("ZEND_ACC_FINAL");
                    scope.import_sys.insert("ZEND_ACC_FINAL".to_owned());
                }
                match class.doc_comment.as_ref() {
                    None => (),
                    Some(comment) => {
                        if comment.text.contains("@strict-properties") {
                            flags.push("ZEND_ACC_NO_DYNAMIC_PROPERTIES");
                            scope
                                .import_sys
                                .insert("ZEND_ACC_NO_DYNAMIC_PROPERTIES".to_owned());
                        }
                        if comment.text.contains("@not-serializable") {
                            flags.push("ZEND_ACC_NOT_SERIALIZABLE");
                            scope
                                .import_sys
                                .insert("ZEND_ACC_NOT_SERIALIZABLE".to_owned());
                        }
                    }
                }
                let flags_str = if flags.is_empty() {
                    "0".to_owned()
                } else {
                    flags.join("|")
                };
                scope.import_sys.insert("zend_class_entry".to_owned());
                scope
                    .import_sys
                    .insert("zend_register_internal_class_with_flags".to_owned());
                scope.import_macros_c.insert(macro_name.to_owned());
                Ok(format!(
                    "\n{space}let mut ce: zend_class_entry = unsafe {{ std::mem::zeroed() }};\
                    \n{space}{init_macro}\
                    \n{space}let class_entry: *mut zend_class_entry = unsafe {{\
                    \n{space}{space}zend_register_internal_class_with_flags(\
                    \n{space}{space}{space}std::ptr::from_mut::<zend_class_entry>(&mut ce),\
                    \n{space}{space}{space}{extends_param},\
                    \n{space}{space}{space}{flags_str},\
                    \n{space}{space})\
                    \n{space}}};\
                    {add_interfaces}",
                    space = "    ",
                ))
            }
            ClassLikeType::PhpEnum(php_enum) => {
                if method_count != 0 {
                    return Err("Enum methods not handled yet".to_owned());
                }
                let get_raw_code = |span: Span| -> &str {
                    let start: usize = span.start.try_into().expect("Valid usize");
                    let end: usize = span.end.try_into().expect("Valid usize");
                    program.get(start..end).expect("Valid byte range for span")
                };
                let backing = php_enum
                    .scalar_type
                    .as_ref()
                    .map(|type_name| match get_raw_code(type_name.span) {
                        "string" => Ok(BuiltinType::String),
                        "int" => Ok(BuiltinType::Int),
                        _ => Err(format!("Unhandled enum backing type for enum {self:#?}")),
                    })
                    .transpose()?;
                let backing_type_code = match backing {
                    None => "IS_UNDEF",
                    Some(BuiltinType::String) => "IS_STRING",
                    Some(BuiltinType::Int) => "IS_LONG",
                    Some(backing_type) => {
                        return Err(format!("Invalid backing type: {backing_type:?}"));
                    }
                };
                scope.import_sys.insert("zend_class_entry".to_owned());
                scope.import_sys.insert(backing_type_code.to_owned());
                scope
                    .import_sys
                    .insert("zend_register_internal_enum".to_owned());
                Ok(format!(
                    "\n{space}let class_entry: *mut zend_class_entry = unsafe {{\
                    \n{space}{space}zend_register_internal_enum(\
                    \n{space}{space}{space}c\"{ns_name_segment}{name}\".as_ptr(),\
                    \n{space}{space}{space}{backing_type_code},\
                    \n{space}{space}{space}std::ptr::null_mut(),\
                    \n{space}{space})\
                    \n{space}}};\
                    {add_interfaces}",
                    space = "    ",
                ))
            }
            ClassLikeType::Interface(_) => {
                scope.import_sys.insert("zend_class_entry".to_owned());
                scope
                    .import_sys
                    .insert("zend_register_internal_interface".to_owned());
                scope.import_macros_c.insert(macro_name.to_owned());
                Ok(format!(
                    "\n{space}let mut ce: zend_class_entry = unsafe {{ std::mem::zeroed() }};\
                        \n{space}{init_macro}\
                        \n{space}let class_entry: *mut zend_class_entry = unsafe {{\
                        \n{space}{space}zend_register_internal_interface(\
                        \n{space}{space}{space}std::ptr::from_mut::<zend_class_entry>(&mut ce),\
                        \n{space}{space})\
                        \n{space}}};\
                        {add_interfaces}",
                    space = "    ",
                ))
            }
        }
    }
    /// The kind of class-like-type this is, one of "enum", "interface", or
    /// "class".
    fn kind(&self) -> &'static str {
        match self {
            ClassLikeType::PhpEnum(_) => "enum",
            ClassLikeType::Interface(_) => "interface",
            ClassLikeType::Class(_) => "class",
        }
    }
    /// The scalar type backing this enum, if this is indeed an enum and has
    /// a scalar backing.
    fn enum_scalar_type(&self) -> Option<&Name> {
        match self {
            ClassLikeType::PhpEnum(php_enum) => php_enum.scalar_type.as_ref(),
            _ => None,
        }
    }
}

/// Tracker for what imports need to be added to a given scope.
#[derive(Default)]
#[expect(
    clippy::field_scoped_visibility_modifiers,
    clippy::struct_field_names,
    reason = "To be refactored"
)]
pub(crate) struct ImportsUsed {
    /// Imports needed from `rsphp_sys`.
    pub(crate) import_sys: HashSet<String>,
    /// Imports needed from `rsphp_macros_c`.
    pub(crate) import_macros_c: HashSet<String>,
    /// Imports needed from `rsphp_macros_rs`.
    pub(crate) import_macros_rs: HashSet<String>,
}

impl ImportsUsed {
    /// Create a new empty set of imports.
    fn new() -> Self {
        Self {
            import_sys: HashSet::new(),
            import_macros_c: HashSet::new(),
            import_macros_rs: HashSet::new(),
        }
    }

    /// Format all imports.
    fn as_uses(&self) -> String {
        vec![
            Self::as_use_single(&self.import_macros_c, "rsphp_macros_c"),
            Self::as_use_single(&self.import_macros_rs, "rsphp_macros_rs"),
            Self::as_use_single(&self.import_sys, "rsphp_sys"),
        ]
        .into_iter()
        .flatten()
        .collect::<String>()
    }

    /// Format imports for a single module.
    fn as_use_single(used: &HashSet<String>, module: &str) -> Option<String> {
        if used.is_empty() {
            return None;
        }
        let mut used_names: Vec<String> = used.clone().into_iter().collect();
        if used_names.len() == 1 {
            return Some(format!("use {module}::{};\n", used_names[0]));
        }
        used_names.sort();
        Some(format!("use {module}::{{{}}};\n", used_names.join(", ")))
    }
}

/// Stateful tool for building up argument information for a program.
pub(crate) struct ArgInfoBuilder<'program> {
    /// The program text.
    program: &'program str,
    /// The code generated so far.
    parts: Vec<String>,
    /// The current namespace.
    namespace: Option<String>,
    /// How registration functions should verify their arguments.
    verification: VerificationMode,
    /// Whether any class/enum/interface/trait is present in the stub; if not
    /// then there is nothing to verify and the verification code doesn't need
    /// to be added.
    needs_verification: bool,
    /// File-level imports used.
    file_uses: ImportsUsed,
}

impl<'program> ArgInfoBuilder<'program> {
    /// Create a new builder for the given program with a specified verification
    /// mode.
    pub(crate) fn new(program: &'program str, verification: VerificationMode) -> Self {
        ArgInfoBuilder {
            program,
            parts: vec![],
            namespace: None,
            verification,
            needs_verification: false,
            file_uses: ImportsUsed::new(),
        }
    }
}

#[expect(
    clippy::unused_self,
    clippy::needless_pass_by_ref_mut,
    reason = "Non-wildcard imports not set up yet"
)]
#[expect(
    unused_results,
    reason = "HashSet used to prevent duplication, just need to include the import"
)]
#[expect(clippy::too_many_lines, reason = "Many complicated functions")]
impl ArgInfoBuilder<'_> {
    /// Get a snippet of the raw code in the program at the location specified
    /// in the span.
    fn get_raw_code(&self, span: Span) -> &str {
        let start: usize = span.start.try_into().expect("Valid usize");
        let end: usize = span.end.try_into().expect("Valid usize");
        self.program
            .get(start..end)
            .expect("Valid byte range for span")
    }
    /// Process statements and update the state accordingly.
    pub(crate) fn statements(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            match &stmt.kind {
                StmtKind::Function(function) => self.function(function.as_ref())?,
                StmtKind::Class(class) => self.class_like(&ClassLikeType::Class(*class.clone()))?,
                StmtKind::Enum(php_enum) => {
                    self.class_like(&ClassLikeType::PhpEnum(*php_enum.clone()))?;
                }
                StmtKind::Interface(interface) => {
                    self.class_like(&ClassLikeType::Interface(*interface.clone()))?;
                }
                StmtKind::Namespace(namespace) => self.namespace(namespace.as_ref())?,
                _ => (),
            }
        }
        Ok(())
    }
    /// Process a namespace statement, either adjusting the current namespace or
    /// processing the contents of a braced namespace declaration with the
    /// namespace overridden.
    fn namespace(&mut self, namespace: &NamespaceDecl) -> Result<(), String> {
        let new_namespace = namespace.name.as_ref().map(|name| {
            name.parts
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .join("\\")
        });
        match &namespace.body {
            NamespaceBody::Simple => {
                self.namespace = new_namespace;
            }
            NamespaceBody::Braced(body) => {
                let old_namespace = self.namespace.clone();
                self.namespace = new_namespace;
                let result = self.statements(&body.stmts);
                self.namespace = old_namespace;
                result?;
            }
        }
        Ok(())
    }
    /// Generate code for a function parameter.
    fn param(&self, scope: &mut ImportsUsed, param: &Param) -> Result<String, String> {
        let by_ref = u8::from(param.by_ref);
        let param_type_info_opt = param.type_hint.as_ref().map(|type_hint| {
            ArgInfoTypeRepr::try_from(type_hint.kind.clone(), self.namespace.as_ref())
                .expect("Type representation should be valid")
                .typehint(self.namespace.as_ref())
                .expect("Type representation should work for parameters")
        });
        let name = param
            .name
            .as_ref()
            .ok_or_else(|| "Parameter name is missing".to_owned())?;
        let mut default = param
            .default
            .as_ref()
            .map(|Expr { kind: _, span }| self.get_raw_code(*span).escape_default().to_string());
        // Handle namespace
        if let Some(Expr { kind, span: _ }) = &param.default
            && let ExprKind::ClassConstAccess(_) = kind
            && let Some(ref dflt) = default
            && !dflt.starts_with('\\')
            && let Some(namespace) = &self.namespace
        {
            default = Some(namespace.clone() + "\\\\" + dflt);
        }
        if default == Some("UNKNOWN".to_owned()) {
            default = None;
        }
        let maybe_variadic = if param.variadic { "VARIADIC_" } else { "" };
        let Some(param_type_info) = param_type_info_opt else {
            let (macro_name, macro_args) = match default {
                None => (
                    format!("ZEND_ARG_{maybe_variadic}INFO"),
                    format!("({by_ref}, \"{name}\")"),
                ),
                Some(dflt) => (
                    "ZEND_ARG_INFO_WITH_DEFAULT_VALUE".to_owned(),
                    format!("({by_ref}, \"{name}\", \"{dflt}\")"),
                ),
            };
            scope.import_macros_c.insert(macro_name.clone());
            return Ok(format!("    {macro_name}!{macro_args},\n"));
        };
        let (macro_name, macro_args) = match (
            (
                param_type_info.class_info,
                param_type_info.mask_info,
                param_type_info.return_info,
                param_type_info.is_nullable,
            ),
            default,
        ) {
            ((None, None, _, _), _) => {
                return Err(format!("No class or type mask for {param:?}"));
            }
            ((None, Some(param_mask), param_info, nullable), None) => {
                if param_mask.contains('|') {
                    scope.import_sys.extend(param_type_info.mask_uses);
                    (
                        "ZEND_ARG_TYPE_MASK".to_owned(),
                        format!("({by_ref}, \"{name}\", {param_mask}, std::ptr::null())"),
                    )
                } else {
                    scope.import_sys.extend(param_type_info.return_uses);
                    (
                        format!("ZEND_ARG_{maybe_variadic}TYPE_INFO"),
                        format!(
                            "({by_ref}, \"{name}\", {}, {})",
                            param_info.expect("Should be set"),
                            u8::from(nullable),
                        ),
                    )
                }
            }
            ((Some(param_class), None, None, nullable), None) => (
                format!("ZEND_ARG_{maybe_variadic}OBJ_INFO"),
                format!(
                    "({by_ref}, \"{name}\", \"{param_class}\", {})",
                    u8::from(nullable),
                ),
            ),
            ((Some(param_class), Some(param_mask), _, _), dflt) => {
                scope.import_sys.extend(param_type_info.mask_uses);
                (
                    format!("ZEND_ARG_{maybe_variadic}OBJ_TYPE_MASK"),
                    format!(
                        "({by_ref}, \"{name}\", \"{param_class}\", {param_mask}, {})",
                        match dflt {
                            Some(default_repr) => default_repr,
                            None => "NULL".to_owned(),
                        },
                    ),
                )
            }
            ((None, _, Some(param_info), nullable), Some(dflt)) => {
                scope.import_sys.extend(param_type_info.return_uses);
                (
                    "ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE".to_owned(),
                    format!(
                        "({by_ref}, \"{name}\", {param_info}, {}, \"{dflt}\")",
                        u8::from(nullable),
                    ),
                )
            }
            ((Some(param_class), None, None, nullable), Some(dflt)) if !param.variadic => (
                "ZEND_ARG_OBJ_INFO_WITH_DEFAULT_VALUE".to_owned(),
                format!(
                    "({by_ref}, \"{name}\", \"{param_class}\", {}, \"{dflt}\")",
                    u8::from(nullable),
                ),
            ),
            _ => return Err(format!("Unhandled parameter: {param:?}")),
        };
        scope.import_macros_c.insert(macro_name.clone());
        Ok(format!("    {macro_name}!{macro_args},\n"))
    }

    /// Generate code for a global function or class method.
    #[expect(clippy::too_many_arguments, reason = "To be refactored")]
    fn build_function(
        &self,
        scope: &mut ImportsUsed,
        params: &[Param],
        comment_opt: Option<&Comment>,
        name_constant: &str,
        return_reference: bool,
        return_type: Option<&TypeHint>,
        doc_name: &str,
    ) -> Result<String, String> {
        let mut required_params = 0;
        for param in params {
            if param.default.is_none() && !param.variadic {
                required_params += 1;
            }
        }
        let param_code: Vec<String> = params
            .iter()
            .map(|parameter| self.param(scope, parameter))
            .collect::<Result<Vec<_>, _>>()?;
        let return_reference_int: u8 = u8::from(return_reference);
        let Some(return_info) = return_type else {
            scope.import_sys.insert("zend_internal_arg_info".to_owned());
            scope
                .import_macros_rs
                .insert("ZEND_ARG_INFO_FOR_FUNCTION".to_owned());
            return Ok(format!(
                "/// Argument information for `{doc_name}()`.\
                \npub(crate) const {name_constant}: [zend_internal_arg_info; {}] = [\
                \n{space}ZEND_ARG_INFO_FOR_FUNCTION!({return_reference_int}, {required_params}),\n\
                {}];\n",
                param_code.len() + 1,
                param_code.join(""),
                space = "    "
            ));
        };
        // Has a return declaration
        let ret_info_repr =
            ArgInfoTypeRepr::try_from(return_info.kind.clone(), self.namespace.as_ref())?;
        let typehint = ret_info_repr.typehint(self.namespace.as_ref())?;
        let ret_nullable = u8::from(typehint.is_nullable);
        let tentative = match comment_opt {
            Some(comment) => comment.text.contains("@tentative-return-type"),
            None => false,
        };
        let tentative_int = u8::from(tentative);
        #[expect(clippy::uninlined_format_args, reason = "Clearer to not inline")]
        let (fn_arginfo_macro, fn_arginfo_params) = match (
            typehint.class_info,
            typehint.mask_info,
            typehint.return_info,
        ) {
            (None, None, None) => return Err(format!("No return info for {return_info:?}")),
            (Some(class), None, None) => (
                "ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_OBJ_INFO",
                format!(
                    "({}, {}, \"{}\", {}, {})",
                    return_reference_int, required_params, class, ret_nullable, tentative_int,
                ),
            ),
            (None, _, Some(info)) if typehint.is_nullable => {
                scope.import_sys.extend(typehint.return_uses);
                (
                    "ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_TYPE_INFO",
                    format!(
                        "({}, {}, {}, 1, {})",
                        return_reference_int, required_params, info, tentative_int,
                    ),
                )
            }
            (None, Some(mask), _) => {
                scope.import_sys.extend(typehint.mask_uses);
                (
                    "ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_TYPE_MASK",
                    format!(
                        "({}, {}, {}, {})",
                        return_reference_int, required_params, mask, tentative_int,
                    ),
                )
            }
            (Some(class), Some(mask), _) => {
                scope.import_sys.extend(typehint.mask_uses);
                (
                    "ZEND_ARG_INFO_FOR_FUNCTION_WITH_RETURN_OBJ_TYPE_INFO",
                    format!(
                        "({}, {}, \"{}\", {}, {})",
                        return_reference_int, required_params, class, mask, tentative_int,
                    ),
                )
            }
            _ => return Err(format!("Unhandled return info: {return_info:?}")),
        };
        scope.import_sys.insert("zend_internal_arg_info".to_owned());
        scope.import_macros_rs.insert(fn_arginfo_macro.to_owned());
        Ok(format!(
            "/// Argument information for `{doc_name}()`.\
            \npub(crate) const {name_constant}: [zend_internal_arg_info; {}] = [\
            \n{space}{fn_arginfo_macro}!{fn_arginfo_params},\n\
            {}];\n",
            param_code.len() + 1,
            param_code.join(""),
            space = "    "
        ))
    }

    /// Generate code for a global function.
    fn function(&mut self, func: &FunctionDecl) -> Result<(), String> {
        let fn_name = func.name.as_ref().expect("No errors");
        let ns_segment = match &self.namespace {
            Some(ns) => format!("{}_", ns.to_uppercase()),
            None => String::new(),
        };
        let doc_name = match &self.namespace {
            Some(ns) => format!("{ns}\\{fn_name}"),
            None => fn_name.to_string(),
        };
        // Can't borrow from self.file_uses as mutable while calling a function
        let mut file_uses = std::mem::take(&mut self.file_uses);
        let fn_result = self.build_function(
            &mut file_uses,
            &func.params,
            func.doc_comment.as_ref(),
            &format!("ARGINFO_{ns_segment}{}", fn_name.to_uppercase()),
            func.by_ref,
            func.return_type.as_ref(),
            &doc_name,
        );
        self.file_uses = file_uses;
        self.parts.push(fn_result?);
        Ok(())
    }

    /// Generate code for a class property.
    fn build_property(
        &self,
        scope: &mut ImportsUsed,
        prop: &PropertyDecl,
    ) -> Result<String, String> {
        scope.import_sys.insert("zval".to_owned());
        scope.import_sys.insert("zend_class_entry".to_owned());
        let prop_name = prop.name.as_ref().ok_or("Property name missing")?;
        let prop_default_create = format!(
            "{space}let mut property_{name}_default_value: zval = unsafe {{ std::mem::zeroed() }};",
            name = prop_name,
            space = "    "
        );
        // rsphp-macros-c macro to call, Option with an extra argument, and
        // Option with extra code to put before the initialization
        let (init_macro, init_arg, init_extra_code) = match &prop.default {
            None => ("ZVAL_UNDEF", None, None),
            Some(Expr {
                kind: ExprKind::Null,
                span: _,
            }) => ("ZVAL_NULL", None, None),
            Some(Expr {
                kind: ExprKind::Bool(false),
                span: _,
            }) => ("ZVAL_FALSE", None, None),
            Some(Expr {
                kind: ExprKind::Bool(true),
                span: _,
            }) => ("ZVAL_TRUE", None, None),
            Some(Expr {
                kind: ExprKind::String(string),
                span: _,
            }) if **string == *"" => ("ZVAL_EMPTY_STRING", None, None),
            Some(Expr {
                kind: ExprKind::Array(array),
                span: _,
            }) if array.is_empty() => ("ZVAL_EMPTY_ARRAY", None, None),
            Some(Expr {
                kind: ExprKind::Int(num),
                span: _,
            }) => ("ZVAL_LONG", Some(num.to_string()), None),
            Some(Expr {
                kind: ExprKind::Identifier(ident),
                span: _,
            }) if **ident == *"E_ERROR" => {
                scope.import_sys.insert("E_ERROR".to_owned());
                ("ZVAL_LONG", Some("E_ERROR".to_owned()), None)
            }
            Some(Expr {
                kind: ExprKind::String(string),
                span: _,
            }) => {
                scope.import_sys.insert("zend_string_init".to_owned());
                (
                    "ZVAL_INTERNED_STR",
                    Some(format!("property_{prop_name}_default_str")),
                    Some(format!(
                        "{space}let property_{name}_default_str = unsafe {{ zend_string_init(c\"{s}\".as_ptr(), \"{s}\".len(), true) }};\n",
                        name = prop_name,
                        space = "    ",
                        s = string
                    )),
                )
            }
            _ => return Err(format!("Unhandled property default: {prop:?}")),
        };
        scope.import_macros_c.insert(init_macro.to_owned());
        let prop_name_init = format!(
            "{space}let property_{name}_name: *mut zend_string = unsafe {{\
            \n{space}{space}zend_string_init(\
            \n{space}{space}{space}c\"{name}\".as_ptr(),\
            \n{space}{space}{space}\"{name}\".len(),\
            \n{space}{space}{space}true,\
            \n{space}{space})\
            \n{space}}};",
            name = prop_name,
            space = "    "
        );
        scope.import_sys.insert("zend_string".to_owned());
        scope.import_sys.insert("zend_string_init".to_owned());
        let vis = match prop.visibility {
            None | Some(Visibility::Public) => "ZEND_ACC_PUBLIC",
            Some(Visibility::Protected) => "ZEND_ACC_PROTECTED",
            Some(Visibility::Private) => "ZEND_ACC_PRIVATE",
        };
        scope.import_sys.insert(vis.to_owned());
        let (type_declaration, type_code) = match prop.type_hint.as_ref() {
            Some(type_hint) => {
                ArgInfoTypeRepr::try_from(type_hint.kind.clone(), self.namespace.as_ref())?
                    .property_type_code(scope, self.namespace.as_ref(), prop_name)?
            }
            None => (None, "ZEND_TYPE_INIT_NONE!(0)".to_owned()),
        };
        let ptr_for_default =
            format!("std::ptr::from_mut::<zval>(&mut property_{prop_name}_default_value)");
        let prop_decl = format!(
            "{space}unsafe {{\
            \n{space}{space}zend_declare_typed_property(\
            \n{space}{space}{space}class_entry,\
            \n{space}{space}{space}property_{name}_name,\
            \n{space}{space}{space}{ptr_for_default},\
            \n{space}{space}{space}{vis} as i32,\
            \n{space}{space}{space}std::ptr::null_mut(),\
            \n{space}{space}{space}{typemask},\
            \n{space}{space});\
            \n{space}}}",
            name = prop_name,
            space = "    ",
            typemask = type_code,
        );
        let prop_name_free = format!(
            "{space}unsafe {{\
            \n{space}{space}zend_string_release(property_{name}_name);\
            \n{space}}}",
            name = prop_name,
            space = "    "
        );
        scope
            .import_sys
            .insert("zend_declare_typed_property".to_owned());
        scope.import_sys.insert("zend_string_release".to_owned());
        let mut parts = vec![prop_default_create];
        match init_extra_code {
            None => (),
            Some(code) => parts.push(code),
        }
        parts.push(format!(
            "{space}unsafe {{ {init_macro}!({ptr_for_default}{sep}{extra_arg}); }}",
            space = "    ",
            sep = if init_arg.is_some() { ", " } else { "" },
            extra_arg = init_arg.unwrap_or_default()
        ));
        parts.push(prop_name_init);
        match type_declaration {
            None => (),
            Some(code) => parts.push(code),
        }
        parts.push(prop_decl);
        parts.push(prop_name_free);
        Ok(format!("\n{}", parts.join("\n")))
    }

    /// Generate code for a class, enum, or interface declaration.
    fn class_like(&mut self, wrapped: &ClassLikeType) -> Result<(), String> {
        let mut scope = ImportsUsed::new();
        let class_name = wrapped.name();
        let mut properties = vec![];
        let mut constants = vec![];
        let doc_name = match &self.namespace {
            Some(ns) => format!("{ns}\\{class_name}"),
            None => class_name.to_owned(),
        };
        let mut attribute_setup = vec![];
        let mut method_verifications = vec![];
        let mut module_parts = vec![];
        let mut enum_cases = vec![];
        let enum_backing = wrapped
            .enum_scalar_type()
            .map(|type_name| match self.get_raw_code(type_name.span) {
                "string" => Ok(BuiltinType::String),
                "int" => Ok(BuiltinType::Int),
                _ => Err(format!("Unhandled enum backing type for enum {wrapped:#?}")),
            })
            .transpose()?;
        for member in wrapped.body_members() {
            match member {
                ClassLikeMember::Method(method) => {
                    let fn_name = method.name.as_ref().expect("No errors");
                    let name_constant = format!("ARGINFO_{}", fn_name.to_uppercase());
                    module_parts.push(self.build_function(
                        &mut scope,
                        &method.params,
                        method.doc_comment.as_ref(),
                        &name_constant,
                        method.by_ref,
                        method.return_type.as_ref(),
                        &format!("{doc_name}::{fn_name}"),
                    )?);
                    let mut verification =
                        VerificationData::new(fn_name.to_string(), name_constant);
                    verification.add_visibility(method.visibility.as_ref());
                    verification.add_flag_if(method.is_static, "ZEND_ACC_STATIC");
                    verification.add_flag_if(
                        matches!(wrapped, ClassLikeType::Interface(_)),
                        "ZEND_ACC_ABSTRACT",
                    );
                    for attrib in &method.attributes {
                        attribute_setup.push(self.build_method_attribute(
                            &mut scope,
                            fn_name,
                            attrib,
                            &mut verification,
                        )?);
                    }
                    method_verifications.push(verification);
                    self.needs_verification = true;
                }
                ClassLikeMember::Property(property) => {
                    properties.push(self.build_property(&mut scope, &property)?);
                }
                ClassLikeMember::ClassConst(class_const) => {
                    constants.push(self.build_constant(&mut scope, &class_const)?);
                }
                ClassLikeMember::EnumCase(enum_case) => {
                    enum_cases.push(self.build_case(
                        &mut scope,
                        &enum_case,
                        enum_backing.as_ref(),
                    )?);
                }
            }
        }

        let mut class_attributes = wrapped
            .attributes()
            .iter()
            .map(|attribute| self.class_attribute(&mut scope, attribute))
            .collect::<Result<Vec<_>, _>>()?;
        class_attributes.extend(attribute_setup);
        let class_attributes_str = if class_attributes.is_empty() {
            String::new()
        } else {
            "\n".to_owned() + &class_attributes.join("\n")
        };

        let enum_cases_code = if enum_cases.is_empty() {
            String::new()
        } else {
            assert_matches!(wrapped, ClassLikeType::PhpEnum(_), "Only enums have cases");
            format!(
                "\n{space}unsafe {{\
                \n{space}{}\
                \n{space}}};",
                enum_cases.join("\n    "),
                space = "    "
            )
        };

        let mut registration_params = vec![];

        if let Some(parent_var) = wrapped.parent_class_var() {
            registration_params.push(format!("{parent_var}: *mut zend_class_entry"));
        }
        let interface_ptrs = wrapped.interface_variables();
        for interface in &interface_ptrs {
            registration_params.push(format!("{interface}: *mut zend_class_entry"));
        }
        if !method_verifications.is_empty() {
            scope.import_sys.insert("zend_function_entry".to_owned());
            registration_params.push("methods: &[zend_function_entry]".to_owned());
        }
        // Registration function
        let methods_verify = match &self.namespace {
            Some(ns) => self
                .verification
                .apply_to_methods(&format!("{ns}\\\\{class_name}"), &method_verifications),
            None => self
                .verification
                .apply_to_methods(class_name, &method_verifications),
        };
        let kind = wrapped.kind();
        let mut module_content = format!(
            "{use_verify}\n\
            {module_parts}\n\
            /// Register the `{doc_name}` {kind}.\
            \npub(crate) fn register_{kind}({methods_param}) -> *mut zend_class_entry {{\
            {methods_verify}\
            {init_and_register_code}\
            {enum_cases_code}\
            \n{}\
            \n{}\
            {class_attributes_str}\
            \n{space}class_entry\
            \n}}",
            properties.join("\n    "),
            constants.join("\n"),
            methods_param = registration_params.join(", "),
            module_parts = module_parts.join("\n"),
            init_and_register_code = wrapped.init_and_register_code(
                &mut scope,
                self.namespace.as_ref(),
                method_verifications.len(),
                self.program
            )?,
            space = "    ",
            use_verify = self.verification.apply_to_import(
                "use super::verify_registration;\n",
                method_verifications.is_empty()
            ),
        );
        // Some extra imports might be conditional
        let flags_in_verification = VerificationData::get_all_flags(&method_verifications);
        match self.verification {
            VerificationMode::Never => (),
            VerificationMode::Always => {
                scope.import_sys.extend(flags_in_verification);
            }
            VerificationMode::DebugAssert => {
                let extra_flags_in_verification = &flags_in_verification - &scope.import_sys;
                if let Some(extra_use) =
                    ImportsUsed::as_use_single(&extra_flags_in_verification, "rsphp_sys")
                {
                    module_content =
                        format!("#[cfg(debug_assertions)]\n{extra_use}") + &module_content;
                }
            }
        }
        module_content = scope.as_uses() + &module_content;
        let module_content_indented = module_content
            .split('\n')
            .map(|line| {
                if line.is_empty() {
                    String::new()
                } else {
                    "    ".to_owned() + line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let module = format!(
            "/// Module for `{doc_name}` {kind}.\n\
            pub(crate) mod {ns_segment}{class_name} {{\n\
            {module_content_indented}\n\
            }}",
            ns_segment = self
                .namespace
                .as_ref()
                .map(|segment| format!("{segment}_"))
                .unwrap_or_default()
        );
        self.parts.push(module);
        Ok(())
    }

    /// Generate code for registering an attribute on a class method.
    fn build_method_attribute(
        &self,
        scope: &mut ImportsUsed,
        name: &str,
        attrib: &Attribute,
        verification: &mut VerificationData,
    ) -> Result<String, String> {
        let attrib_name = attrib
            .name
            .parts
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>()
            .join("\\");

        let mut param_init = vec![];
        for (index, arg) in attrib.args.iter().enumerate() {
            if let Some(arg_name_full) = &arg.name {
                if arg_name_full.parts.len() != 1 {
                    return Err(format!("Argument name has multiple parts: {arg:?}"));
                }
                let arg_name = &arg_name_full.parts[0];
                let known_name = match arg_name.as_ref() {
                    "since" => "_zend_known_string_id::ZEND_STR_SINCE",
                    "message" => "_zend_known_string_id::ZEND_STR_MESSAGE",
                    _ => return Err(format!("Unhandled attribute parameter name: {name}")),
                };
                scope.import_sys.insert("_zend_known_string_id".to_owned());
                let init = format!(
                    "(*(*attrib).args.as_mut_ptr().add({index})).name = ZSTR_KNOWN!({known_name});",
                );
                scope.import_macros_c.insert("ZSTR_KNOWN".to_owned());
                param_init.push(init);
            }
            match &arg
                .value
                .as_ref()
                .ok_or("Missing argument value in attribute")?
                .kind
            {
                ExprKind::String(string) => {
                    let alloc = format!(
                        "let arg_{index}_value = zend_string_init(c\"{string}\".as_ptr(), \
                            \"{string}\".len(), true);",
                    );
                    scope.import_sys.insert("zend_string_init".to_owned());
                    param_init.push(alloc);
                    let param_use = format!(
                        "ZVAL_STR!(std::ptr::from_mut::<zval>(&mut (*(*attrib).args.as_mut_ptr().add({index})).value), arg_{index}_value);",
                    );
                    scope.import_sys.insert("zval".to_owned());
                    scope.import_macros_c.insert("ZVAL_STR".to_owned());
                    param_init.push(param_use);
                }
                _ => return Err(format!("Unhandled attribute value: {arg:#?}")),
            }
        }

        if attrib_name == "Deprecated" {
            verification.add_flag("ZEND_ACC_DEPRECATED");
        }
        #[expect(clippy::single_match_else, reason = "Clearer as a match")]
        let (name_alloc, name_use, name_free) = match attrib_name.as_str() {
            "Deprecated" => {
                scope.import_macros_c.insert("ZSTR_LEN".to_owned());
                scope.import_macros_c.insert("ZSTR_KNOWN".to_owned());
                scope.import_sys.insert("_zend_known_string_id".to_owned());
                (
                    "\n        assert_eq!(10, ZSTR_LEN!(ZSTR_KNOWN!(_zend_known_string_id::ZEND_STR_DEPRECATED_CAPITALIZED)));".to_owned(),
                    "ZSTR_KNOWN!(_zend_known_string_id::ZEND_STR_DEPRECATED_CAPITALIZED)".to_owned(),
                    String::new()
                )
            }
            _ => {
                scope.import_sys.insert("zend_string".to_owned());
                scope
                    .import_sys
                    .insert("zend_string_init_interned".to_owned());
                scope.import_sys.insert("zend_string_release_ex".to_owned());
                (
                    format!(
                        "\n{space}{space}let attrib_name: *mut zend_string = (zend_string_init_interned.expect(\"zend_string_init_interned should be set\"))(\
                            c\"{name}\".as_ptr(),\
                            \"{name}\".len(),\
                            true);",
                        space = "    ",
                        name = attrib_name
                    ),
                    "attrib_name".to_owned(),
                    "\n        zend_string_release_ex(attrib_name, true);".to_owned(),
                )
            }
        };
        scope.import_sys.insert("zend_function".to_owned());
        scope.import_sys.insert("zend_hash_str_find_ptr".to_owned());
        scope.import_sys.insert("HashTable".to_owned());
        scope.import_sys.insert("zend_attribute".to_owned());
        scope
            .import_sys
            .insert("zend_add_function_attribute".to_owned());
        Ok(format!(
            "{space}unsafe {{\
            \n{space}{space}let fn_ptr: *mut zend_function = zend_hash_str_find_ptr(\
            \n{space3}std::ptr::from_ref::<HashTable>(&(*class_entry).function_table),\
            \n{space3}c\"{fn_name}\".as_ptr(),\
            \n{space3}\"{fn_name}\".len(),\
            \n{space}{space}).cast::<zend_function>();\
            {name_alloc}\
            \n{space}{space}let attrib: *mut zend_attribute = zend_add_function_attribute(fn_ptr, {name_use}, {param_count});\
            {name_free}\
            {param_init}\
            \n{space}}}",
            fn_name = name.to_lowercase(),
            param_count = attrib.args.len(),
            param_init = param_init
                .into_iter()
                .fold(String::new(), |mut result, init_code| {
                    write!(result, "\n        {init_code}").expect("String writes are infallible");
                    result
                }),
            space = "    ",
            space3 = "            ",
        ))
    }

    /// Generate code for registering a class constant.
    fn build_constant(
        &self,
        scope: &mut ImportsUsed,
        constant: &ClassConstDecl,
    ) -> Result<String, String> {
        let constant_name = constant
            .name
            .as_ref()
            .ok_or_else(|| "Constant name is missing".to_owned())?;
        let vis = match constant.visibility {
            None | Some(Visibility::Public) => "ZEND_ACC_PUBLIC",
            Some(Visibility::Protected) => "ZEND_ACC_PROTECTED",
            Some(Visibility::Private) => "ZEND_ACC_PRIVATE",
        };
        scope.import_sys.insert(vis.to_owned());
        let type_repr = ArgInfoTypeRepr::try_from(
            constant.type_hint.as_ref().map_or_else(
                || {
                    let comment = constant
                        .doc_comment
                        .as_ref()
                        .expect("Doc comment needed if class constant is missing a type");
                    let type_re = Regex::new(r"@var (\S+)").expect("Valid regex");
                    match type_re
                        .captures(&comment.text)
                        .ok_or_else(|| format!("Missing @var for constant: {constant:?}"))?
                        .get(1)
                        .expect("Should have match value")
                        .as_str()
                    {
                        "int" => Ok(TypeHintKind::Keyword(BuiltinType::Int, Span::DUMMY)),
                        _ => Err(format!("Unhandled @var for constant: {constant:?}")),
                    }
                },
                |th| Ok(th.kind.clone()),
            )?,
            self.namespace.as_ref(),
        )?;
        let (_type_declaration, type_code) =
            type_repr.property_type_code(scope, self.namespace.as_ref(), constant_name)?;
        let init_code = match &constant.value.kind {
            ExprKind::Int(num) => {
                scope.import_macros_c.insert("ZVAL_LONG".to_owned());
                format!("ZVAL_LONG!(&mut value, {num});")
            }
            ExprKind::Null => {
                scope.import_macros_c.insert("ZVAL_NULL".to_owned());
                "ZVAL_NULL!(&mut value);".to_owned()
            }
            ExprKind::Identifier(ident) if *ident == "UNKNOWN".into() => {
                let cvalue_re = Regex::new(r"@cvalue (\S+)").map_err(|err| err.to_string())?;
                let mut cvalue_val = cvalue_re
                    .captures(
                        &constant
                            .doc_comment
                            .as_ref()
                            .expect("Should have doc comment")
                            .text,
                    )
                    .expect("Should have @cvalue")
                    .get(1)
                    .expect("Should have match value")
                    .as_str()
                    .to_owned();
                // Might not be something defined in core
                if cvalue_val.starts_with("ZEND_") || cvalue_val.starts_with("PHP_") {
                    scope.import_sys.insert(cvalue_val.clone());
                } else {
                    cvalue_val = "crate::".to_owned() + &cvalue_val;
                }
                match type_repr {
                    ArgInfoTypeRepr::Builtin(BuiltinType::Int) => {
                        scope.import_macros_c.insert("ZVAL_LONG".to_owned());
                        format!("ZVAL_LONG!(&mut value, {cvalue_val} as i64);")
                    }
                    ArgInfoTypeRepr::Builtin(BuiltinType::Float) => {
                        scope.import_macros_c.insert("ZVAL_DOUBLE".to_owned());
                        format!("ZVAL_DOUBLE!(&mut value, {cvalue_val} as f64);")
                    }
                    ArgInfoTypeRepr::Builtin(BuiltinType::String) => {
                        scope.import_sys.insert("zend_string".to_owned());
                        scope.import_sys.insert("zend_string_init".to_owned());
                        scope.import_macros_c.insert("ZVAL_STR".to_owned());
                        format!(
                            "let value_str: *mut zend_string = zend_string_init(\
                            \n{space3}{cvalue_val}.as_ptr().cast::<std::ffi::c_char>(),\
                            \n{space3}std::ffi::CStr::from_ptr({cvalue_val}.as_ptr().cast::<std::ffi::c_char>()).count_bytes(),\
                            \n{space3}true);\
                            \n{space}{space}ZVAL_STR!(&mut value, value_str);",
                            space = "    ",
                            space3 = "            "
                        )
                    }
                    _ => {
                        return Err(format!(
                            "Unhandled constant initialization: {constant:#?}, {type_repr:?}"
                        ));
                    }
                }
            }
            _ => return Err(format!("Unhandled constant initialization: {constant:#?}")),
        };
        scope.import_sys.insert("zval".to_owned());
        scope.import_sys.insert("zend_string".to_owned());
        scope
            .import_sys
            .insert("zend_string_init_interned".to_owned());
        scope
            .import_sys
            .insert("zend_declare_typed_class_constant".to_owned());
        scope.import_sys.insert("zend_string_release_ex".to_owned());
        Ok(format!(
            "{space}unsafe {{\
            \n{space}{space}let mut value: zval = std::mem::zeroed();\
            \n{space}{space}{init_code}\
            \n{space}{space}let name: *mut zend_string = (zend_string_init_interned.expect(\"zend_string_init_interned should be set\"))(\
            \n{space}{space}{space}c\"{constant_name}\".as_ptr(),\
            \n{space}{space}{space}\"{constant_name}\".len(),\
            \n{space}{space}{space}true,\
            \n{space}{space});\
            \n{space}{space}zend_declare_typed_class_constant(\
            \n{space}{space}{space}class_entry, name, std::ptr::from_mut::<zval>(&mut value), {vis} as i32,\
            \n{space}{space}{space}std::ptr::null_mut(), {type_code}\
            \n{space}{space});\
            \n{space}{space}zend_string_release_ex(name, true);\
            \n{space}}}",
            space = "    ",
        ))
    }

    /// Generate code for registering an attribute for the overall class/interface/enum.
    fn class_attribute(
        &self,
        scope: &mut ImportsUsed,
        attrib: &Attribute,
    ) -> Result<String, String> {
        let attrib_name = attrib
            .name
            .parts
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>()
            .join("\\");

        let mut param_init = vec![];
        for (index, arg) in attrib.args.iter().enumerate() {
            if arg.name.is_some() {
                return Err("Attributes with named arguments are not supported yet".to_owned());
            }
            let value = self.get_raw_code(arg.value.as_ref().ok_or("Only missing with PFA")?.span);
            if attrib_name == "Attribute" || attrib_name == "\\Attribute" {
                // We can handle these
                let flags: Vec<&'static str> = value
                    .split('|')
                    .map(|flag| match flag {
                        "Attribute::TARGET_CLASS" | "\\Attribute::TARGET_CLASS" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_CLASS")
                        }
                        "Attribute::TARGET_FUNCTION" | "\\Attribute::TARGET_FUNCTION" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_FUNCTION")
                        }
                        "Attribute::TARGET_METHOD" | "\\Attribute::TARGET_METHOD" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_METHOD")
                        }
                        "Attribute::TARGET_PROPERTY" | "\\Attribute::TARGET_PROPERTY" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_PROPERTY")
                        }
                        "Attribute::TARGET_CLASS_CONSTANT"
                        | "\\Attribute::TARGET_CLASS_CONSTANT" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_CLASS_CONST")
                        }
                        "Attribute::TARGET_PARAMETER" | "\\Attribute::TARGET_PARAMETER" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_PARAMETER")
                        }
                        "Attribute::TARGET_CONSTANT" | "\\Attribute::TARGET_CONSTANT" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_CONST")
                        }
                        "Attribute::TARGET_ALL" | "\\Attribute::TARGET_ALL" => {
                            Ok("ZEND_ATTRIBUTE_TARGET_ALL")
                        }
                        "Attribute::IS_REPEATABLE" | "\\Attribute::IS_REPEATABLE" => {
                            Ok("ZEND_ATTRIBUTE_IS_REPEATABLE")
                        }
                        _ => Err(format!("Unhandled attribute constant: {flag}")),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                for flag in &flags {
                    scope.import_sys.insert(flag.to_string());
                }
                scope.import_macros_c.insert("ZVAL_LONG".to_owned());
                let init = format!(
                    "ZVAL_LONG!(&mut (*attrib).args[{index}].value, {flags} as i64);",
                    flags = flags.join("|")
                );
                param_init.push(init);
            } else {
                return Err(format!("Unsupported class attribute: {attrib:?}"));
            }
        }
        scope.import_sys.insert("zend_string".to_owned());
        scope
            .import_sys
            .insert("zend_string_init_interned".to_owned());
        scope.import_sys.insert("zend_attribute".to_owned());
        scope
            .import_sys
            .insert("zend_add_class_attribute".to_owned());
        scope.import_sys.insert("zend_string_release_ex".to_owned());
        Ok(format!(
            "{space}unsafe {{\
            \n{space}{space}let name: *mut zend_string = (zend_string_init_interned.expect(\"zend_string_init_interned should be set\"))(\
            \n{space}{space}{space}c\"{attrib_name}\".as_ptr(),\
            \n{space}{space}{space}\"{attrib_name}\".len(),\
            \n{space}{space}{space}true,\
            \n{space}{space});\
            \n{space}{space}let attrib: *mut zend_attribute = zend_add_class_attribute(class_entry, name, {param_count});\
            \n{space}{space}zend_string_release_ex(name, true);\
            {param_init}\
            \n{space}}}",
            param_count = attrib.args.len(),
            param_init = param_init
                .into_iter()
                .fold(String::new(), |mut result, init_code| {
                    write!(result, "\n        {init_code}").expect("String writes are infallible");
                    result
                }),
            space = "    ",
        ))
    }

    /// Generate code for registering an enum case.
    fn build_case(
        &mut self,
        scope: &mut ImportsUsed,
        case: &EnumCase,
        backing: Option<&BuiltinType>,
    ) -> Result<String, String> {
        let case_name = case.name.as_ref().expect("No errors");
        if let Some(value) = &case.value {
            if backing.is_none() {
                return Err(format!(
                    "Enum case {case:?} has backed value but enum is not backed"
                ));
            }
            return match &value.kind {
                ExprKind::String(string) => {
                    scope.import_sys.insert("zval".to_owned());
                    scope.import_sys.insert("zend_string".to_owned());
                    scope.import_sys.insert("zend_string_init".to_owned());
                    scope
                        .import_sys
                        .insert("zend_enum_add_case_cstr".to_owned());
                    scope.import_macros_c.insert("ZVAL_STR".to_owned());
                    Ok(format!(
                        "{space}{{\
                        \n{space3}let mut value: zval = std::mem::zeroed();\
                        \n{space3}let value_str: *mut zend_string = zend_string_init(\
                        \n{space3}{space}c\"{val}\".as_ptr(),\
                        \n{space3}{space}c\"{val}\".count_bytes(),\
                        \n{space3}{space}true,\
                        \n{space3});\
                        \n{space3}ZVAL_STR!(std::ptr::from_mut::<zval>(&mut value), value_str);\
                        \n{space3}zend_enum_add_case_cstr(class_entry, c\"{name}\".as_ptr(), std::ptr::from_mut::<zval>(&mut value));\
                        \n{space}{space}}}",
                        name = case_name,
                        val = string,
                        space = "    ",
                        space3 = "            ",
                    ))
                }
                ExprKind::Int(num) => {
                    scope.import_sys.insert("zval".to_owned());
                    scope
                        .import_sys
                        .insert("zend_enum_add_case_cstr".to_owned());
                    scope.import_macros_c.insert("ZVAL_LONG".to_owned());
                    Ok(format!(
                        "{space}{{\
                        \n{space3}let mut value: zval = std::mem::zeroed();\
                        \n{space3}ZVAL_LONG!(std::ptr::from_mut::<zval>(&mut value), {val});\
                        \n{space3}zend_enum_add_case_cstr(class_entry, c\"{name}\".as_ptr(), std::ptr::from_mut::<zval>(&mut value));\
                        \n{space}{space}}}",
                        name = case_name,
                        val = num,
                        space = "    ",
                        space3 = "            ",
                    ))
                }
                _ => Err(format!("Unhandled enum backing: {case:#?}")),
            };
        }
        scope
            .import_sys
            .insert("zend_enum_add_case_cstr".to_owned());
        Ok(format!(
            "{space}zend_enum_add_case_cstr(class_entry, c\"{name}\".as_ptr(), std::ptr::null_mut());",
            name = case_name,
            space = "    "
        ))
    }

    /// Combine all of the code parts into the arginfo file contents.
    pub(crate) fn assemble(&mut self) -> String {
        // Some extra imports might be conditional
        let mut extra_uses_code = String::new();
        let verification_code = if self.needs_verification {
            let verification_imports = VERIFICATION_RSPHP_SYS_IMPORTS;
            match self.verification {
                VerificationMode::Never => (),
                VerificationMode::Always => {
                    self.file_uses.import_sys.extend(
                        verification_imports
                            .iter()
                            .map(|import| (*import).to_owned()),
                    );
                }
                VerificationMode::DebugAssert => {
                    let extra_verification_imports = &verification_imports
                        .iter()
                        .map(|import| (*import).to_owned())
                        .collect::<HashSet<_>>()
                        - &self.file_uses.import_sys;
                    if let Some(extra_use) =
                        ImportsUsed::as_use_single(&extra_verification_imports, "rsphp_sys")
                    {
                        extra_uses_code = format!("#[cfg(debug_assertions)]\n{extra_use}");
                    }
                }
            }
            self.verification.get_infra()
        } else {
            String::new()
        };
        format!(
            "{}{extra_uses_code}{}\n{}\n",
            self.file_uses.as_uses(),
            verification_code,
            self.parts.join("\n")
        )
    }
}
