use std::collections::HashSet;

/// Tracker for what imports need to be added to a given scope.
#[derive(Default)]
pub(crate) struct ImportsUsed {
    /// Imports needed from `rsphp_sys`.
    sys: HashSet<String>,
    /// Imports needed from `rsphp_macros_c`.
    macros_c: HashSet<String>,
    /// Imports needed from `rsphp_macros_rs`.
    macros_rs: HashSet<String>,
}

impl ImportsUsed {
    /// Create a new empty set of imports.
    pub(crate) fn new() -> Self {
        Self {
            sys: HashSet::new(),
            macros_c: HashSet::new(),
            macros_rs: HashSet::new(),
        }
    }

    /// Add a new use.
    #[expect(
        clippy::panic,
        reason = "Indicates a bug in the library, surfaced at build time"
    )]
    pub(crate) fn add_import<T>(&mut self, module: &str, name: T) -> bool
    where
        T: Into<String>,
    {
        match module {
            "rsphp_sys" => self.sys.insert(name.into()),
            "rsphp_macros_c" => self.macros_c.insert(name.into()),
            "rsphp_macros_rs" => self.macros_rs.insert(name.into()),
            _ => panic!("Invalid use for module {module}"),
        }
    }

    /// Add a few more imports for `rsphp_sys`.
    pub(crate) fn extend_sys_imports<T>(&mut self, names: T)
    where
        T: IntoIterator<Item = String>,
    {
        self.sys.extend(names);
    }

    /// Identify the imports in the provided `HashSet` that are *not* in
    /// the current imports for `rsphp_sys`.
    pub(crate) fn excluding_existing_sys_imports(
        &self,
        target: &HashSet<String>,
    ) -> HashSet<String> {
        target - &self.sys
    }

    /// Format all imports.
    pub(crate) fn as_uses(&self) -> String {
        vec![
            Self::as_use_single(&self.macros_c, "rsphp_macros_c"),
            Self::as_use_single(&self.macros_rs, "rsphp_macros_rs"),
            Self::as_use_single(&self.sys, "rsphp_sys"),
        ]
        .into_iter()
        .flatten()
        .collect::<String>()
    }

    /// Format imports for a single module.
    pub(crate) fn as_use_single(used: &HashSet<String>, module: &str) -> Option<String> {
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
