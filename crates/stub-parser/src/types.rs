use php_ast::ast::{BuiltinType, NameKind};
use php_ast::owned::{Name, TypeHintKind};

/// Information for a typehint for a function or parameter.
#[derive(Debug)]
#[expect(
    clippy::field_scoped_visibility_modifiers,
    reason = "Only used as plain old data, no invariants"
)]
pub(crate) struct FunctionTypehint {
    /// Info for classes/objects.
    pub(crate) class_info: Option<String>,
    /// Info for type masks.
    pub(crate) mask_info: Option<String>,
    /// Info for return types.
    pub(crate) return_info: Option<String>,
    /// Whether the type is nullable.
    pub(crate) is_nullable: bool,
}

/// Representation of a PHP type.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ArgInfoTypeRepr {
    /// Some nullable type.
    Nullable(Box<ArgInfoTypeRepr>),
    /// A builtin type.
    Builtin(BuiltinType),
    /// A named type.
    Named(String),
    /// A named type with a fully qualified name.
    FullyQualifiedNamed(String),
    /// A union of types to include.
    Union(Vec<ArgInfoTypeRepr>),
}

impl ArgInfoTypeRepr {
    /// Create an `ArgInfoTypeRepr` for the given `TypeHintKind`, resolving
    /// names in the given namespace.
    pub(crate) fn try_from(
        value: TypeHintKind,
        namespace: Option<&String>,
    ) -> Result<Self, String> {
        match value {
            TypeHintKind::Keyword(keyword, _) => Ok(ArgInfoTypeRepr::Builtin(keyword)),
            TypeHintKind::Named(Name {
                ref parts,
                kind,
                span: _,
            }) => {
                let joined = parts.as_ref().join("\\\\");
                match (kind, namespace) {
                    (NameKind::Unqualified, _) => Ok(ArgInfoTypeRepr::Named(joined)),
                    (NameKind::FullyQualified, _)
                    | (NameKind::Qualified | NameKind::Relative, None) => {
                        Ok(ArgInfoTypeRepr::FullyQualifiedNamed(joined))
                    }
                    (NameKind::Qualified | NameKind::Relative, Some(ns)) => Ok(
                        ArgInfoTypeRepr::FullyQualifiedNamed(ns.to_owned() + "\\" + &joined),
                    ),
                    (NameKind::Error, _) => Err(format!("Type hint name error: {value:?}")),
                }
            }
            TypeHintKind::Nullable(inner) => Ok(ArgInfoTypeRepr::Nullable(Box::new(
                ArgInfoTypeRepr::try_from(inner.as_ref().kind.clone(), namespace)?,
            ))),
            TypeHintKind::Union(inner) => Ok(ArgInfoTypeRepr::Union(
                inner
                    .as_ref()
                    .iter()
                    .map(|inner_type| ArgInfoTypeRepr::try_from(inner_type.kind.clone(), namespace))
                    .collect::<Result<Vec<_>, String>>()?,
            )),
            TypeHintKind::Intersection(_) => {
                Err("Intersection type hints not yet supported".to_owned())
            }
        }
    }

    /// First string is the extra code to add at the start for the initialization
    /// second is the string to actually use.
    pub(crate) fn property_type_code(
        &self,
        namespace: Option<&String>,
        name_str: &str,
    ) -> Result<(Option<String>, String), String> {
        let namespace_prefix = match namespace {
            Some(ns) => format!("{ns}\\\\"),
            None => String::new(),
        };
        match self {
            ArgInfoTypeRepr::Builtin(_) => Ok((
                None,
                format!("ZEND_TYPE_INIT_MASK!({})", self.type_info()?.1),
            )),
            ArgInfoTypeRepr::Nullable(inner) => match **inner {
                ArgInfoTypeRepr::Builtin(_) => Ok((
                    None,
                    format!("ZEND_TYPE_INIT_MASK!({}|MAY_BE_NULL)", inner.type_info()?.1),
                )),
                ArgInfoTypeRepr::Named(_) | ArgInfoTypeRepr::FullyQualifiedNamed(_) => Ok((
                    inner.property_type_code(namespace, name_str)?.0,
                    format!("ZEND_TYPE_INIT_CLASS!(property_{name_str}_type, 0, MAY_BE_NULL)"),
                )),
                _ => Err(format!("Unhandled type code: {self:?}")),
            },
            ArgInfoTypeRepr::Named(raw_name) => Ok((
                Some(format!(
                    "{space}let property_{name_str}_type: *mut zend_string = unsafe {{\
                        \n{space}{space}zend_string_init(\
                        \n{space}{space}{space}c\"{name}\".as_ptr(),\
                        \n{space}{space}{space}\"{name}\".len(),\
                        \n{space}{space}{space}true,\
                        \n{space}{space})\
                        \n{space}}};",
                    name = format!("{namespace_prefix}{raw_name}"),
                    space = "    "
                )),
                format!("ZEND_TYPE_INIT_CLASS!(property_{name_str}_type, 0, 0)"),
            )),
            ArgInfoTypeRepr::FullyQualifiedNamed(name) => Ok((
                Some(format!(
                    "{space}let property_{name_str}_type: *mut zend_string = unsafe {{\
                        \n{space}{space}zend_string_init(\
                        \n{space}{space}{space}c\"{name}\".as_ptr(),\
                        \n{space}{space}{space}\"{name}\".len(),\
                        \n{space}{space}{space}true,\
                        \n{space}{space})\
                        \n{space}}};",
                    space = "    "
                )),
                format!("ZEND_TYPE_INIT_CLASS!(property_{name_str}_type, 0, 0)"),
            )),
            ArgInfoTypeRepr::Union(inner) => {
                if inner
                    .iter()
                    .all(|inner_type| matches!(inner_type, ArgInfoTypeRepr::Builtin(_)))
                {
                    let parts: Vec<_> = inner
                        .iter()
                        .map(|inner_type| inner_type.type_info().map(|inner_info| inner_info.1))
                        .collect::<Result<_, _>>()?;
                    Ok((None, format!("ZEND_TYPE_INIT_MASK!({})", parts.join("|"))))
                } else {
                    Err(format!("Unhandled union: {inner:?}"))
                }
            }
        }
    }

    /// Get function typehint information.
    pub(crate) fn typehint(&self, namespace: Option<&String>) -> Result<FunctionTypehint, String> {
        let mut is_nullable = false;
        let without_null = match self {
            ArgInfoTypeRepr::Nullable(inner) => {
                is_nullable = true;
                inner
            }
            other => other,
        };
        let namespace_prefix = match namespace {
            Some(ns) => format!("{ns}\\\\"),
            None => String::new(),
        };
        let (class_type, mask, for_info) = match without_null {
            ArgInfoTypeRepr::Builtin(_) => (
                None,
                Some(without_null.type_info()?.1.to_owned()),
                Some(without_null.type_info()?.0.to_owned()),
            ),
            ArgInfoTypeRepr::Named(name) => (Some(format!("{namespace_prefix}{name}")), None, None),
            ArgInfoTypeRepr::FullyQualifiedNamed(name) => (Some(name.clone()), None, None),
            ArgInfoTypeRepr::Union(inner) => {
                let inner_repr: Vec<_> = inner
                    .iter()
                    .map(|inner_type| inner_type.typehint(namespace))
                    .collect::<Result<Vec<_>, String>>()?;
                // should not be nullable
                if inner_repr.iter().any(|repr| repr.is_nullable) {
                    return Err(format!("Union {inner:#?} contains nullable types"));
                }
                let mask_parts = inner_repr
                    .iter()
                    .filter_map(|repr| repr.mask_info.clone())
                    .collect::<Vec<_>>();
                let info_parts = inner_repr
                    .iter()
                    .filter_map(|repr| repr.return_info.clone())
                    .collect::<Vec<_>>();
                let name_parts = inner_repr
                    .iter()
                    .filter_map(|repr| repr.class_info.clone())
                    .collect::<Vec<_>>();
                // Might only contain basic types
                if name_parts.is_empty() {
                    (None, Some(mask_parts.join("|")), Some(info_parts.join("|")))
                } else if mask_parts.is_empty() {
                    (Some(name_parts.join("|")), None, None)
                } else {
                    (
                        Some(name_parts.join("|")),
                        Some(mask_parts.join("|")),
                        Some(info_parts.join("|")),
                    )
                }
            }
            ArgInfoTypeRepr::Nullable(_) => {
                return Err(format!("Unhandled type hint: {without_null:?}"));
            }
        };
        Ok(FunctionTypehint {
            class_info: class_type,
            mask_info: mask,
            return_info: for_info,
            is_nullable,
        })
    }

    /// For builtin types, get the `IS_*` and `MAY_BE_*` flags.
    pub(crate) fn type_info(&self) -> Result<(&str, &str), String> {
        let ArgInfoTypeRepr::Builtin(php_type) = self else {
            return Err(format!("type_info() called for {self:#?}"));
        };
        match php_type {
            BuiltinType::Int | BuiltinType::Integer => Ok(("IS_LONG", "MAY_BE_LONG")),
            BuiltinType::Float | BuiltinType::Double => Ok(("IS_DOUBLE", "MAY_BE_DOUBLE")),
            BuiltinType::String => Ok(("IS_STRING", "MAY_BE_STRING")),
            BuiltinType::Bool | BuiltinType::Boolean => Ok(("_IS_BOOL", "MAY_BE_BOOL")),
            BuiltinType::Void => Ok(("IS_VOID", "MAY_BE_VOID")),
            BuiltinType::Never => Ok(("IS_NEVER", "MAY_BE_NEVER")),
            BuiltinType::Mixed => Ok(("IS_MIXED", "MAY_BE_ANY")),
            BuiltinType::Object => Ok(("IS_OBJECT", "MAY_BE_OBJECT")),
            BuiltinType::Iterable => {
                Err("Iterable should not be used as a typehint in stubs".to_owned())
            }
            BuiltinType::Callable => Ok(("IS_CALLABLE", "MAY_BE_CALLABLE")),
            BuiltinType::Array => Ok(("IS_ARRAY", "MAY_BE_ARRAY")),
            BuiltinType::Self_ => Err("Self should not be used as a typehint in stubs".to_owned()),
            BuiltinType::Parent_ => {
                Err("Parent should not be used as a typehint in stubs".to_owned())
            }
            BuiltinType::Static => Ok(("IS_STATIC", "MAY_BE_STATIC")),
            BuiltinType::Null => Ok(("IS_NULL", "MAY_BE_NULL")),
            BuiltinType::True => Ok(("IS_TRUE", "MAY_BE_TRUE")),
            BuiltinType::False => Ok(("IS_FALSE", "MAY_BE_FALSE")),
        }
    }
}
