//! Build script for [`rsphp_sys`] that runs bindgen and CC.

use std::path::Path;

use bindgen::EnumVariation;

use rsphp_build_tools::{
    AnonymousTypeNames, BuildIdCallbacks, CommentProcessor, ExplicitIntTypesCallbacks,
    IntTypeCastCallbacks, ZendHeaderIterator,
};

fn main() {
    // For docs don't try to rebuild
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    let include_dir_output = std::process::Command::new("php-config")
        .arg("--include-dir")
        .output()
        .expect("php-config --include-dir should succeed");
    let include_dir_full = std::str::from_utf8(&include_dir_output.stdout)
        .expect("php-config --include-dir should result in UTF8");
    let include_dir = include_dir_full.trim();

    let includes_output = std::process::Command::new("php-config")
        .arg("--includes")
        .output()
        .expect("php-config --includes should succeed");
    let includes_full = std::str::from_utf8(&includes_output.stdout)
        .expect("php-config --includes should result in UTF8");
    let includes = includes_full.trim();

    let mut builder = bindgen::Builder::default();

    for include in includes.split(' ') {
        builder = builder.clang_arg(include);
    }

    let bindings = builder
        .headers(ZendHeaderIterator::new(&format!("{include_dir}/Zend/")))
        .header(format!("{include_dir}/main/php.h"))
        .header(format!("{include_dir}/main/php_output.h"))
        .header(format!("{include_dir}/ext/standard/info.h"))
        .header(format!("{include_dir}/ext/standard/php_var.h"))
        // Include static/inline functions
        .wrap_static_fns(true)
        .wrap_static_fns_path("reexport.c")
        // Reduce some system functions - filter what headers are allowed
        .allowlist_file(format!("{include_dir}/.*.h"))
        // Build-specific
        .blocklist_var("HAVE_.*")
        .blocklist_var("PHP_HAVE_.*")
        // Doesn't seem to get exported, "undefined symbol: zend_visibility_string"
        // when trying to load dynamic library, even though it exists just fine
        // in GDB. Blocklist it so that rust triggers an error if trying to bind
        // to it
        .blocklist_function("zend_visibility_string")
        // Doesn't seem to be exported, not marked with ZEND_API so maybe that
        // is why, blocklist it
        .blocklist_function("zend_attribute_attribute_get_flags")
        // zend_lazy_object_get_instance also isn't exported, but cannot be
        // recreated in rust because internally it uses the zend_lazy_object_info
        // type which is only declared in a C file. Also cannot manually reexport
        // so just block it entirely
        .blocklist_function("zend_lazy_object_get_instance")
        .default_enum_style(EnumVariation::ModuleConsts)
        .merge_extern_blocks(true)
        // SLOW and didn't actually add anything
        // .clang_macro_fallback()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Convert some stuff to correct types.
        .parse_callbacks(Box::new(ExplicitIntTypesCallbacks::new()))
        // Some stuff bindgen doesn't understand
        .parse_callbacks(Box::new(BuildIdCallbacks::new()))
        // Nicer type names.
        .parse_callbacks(Box::new(AnonymousTypeNames::new()))
        // Comment cleanup
        .parse_callbacks(Box::new(CommentProcessor::new()))
        // Integer type cast handling
        .parse_callbacks(Box::new(IntTypeCastCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Unclear how to do this with bindgen
    // We want _zend_ast_kind to be a u16 not a c_uint
    // https://github.com/rust-lang/rust-bindgen/pull/3400
    let binding_content = bindings.to_string();
    let updated_binding_content = binding_content.replace(
        "pub mod _zend_ast_kind {\n    #[allow(unused_imports)]\n    use super::*;\n    pub type Type = ::std::os::raw::c_uint;",
        "pub mod _zend_ast_kind {\n    #[allow(unused_imports)]\n    use super::*;\n    pub type Type = u16;",
    );
    std::fs::write("bindings.rs", updated_binding_content).expect("Couldn't write bindings");

    // Use the `cc` crate to build a C file and statically link it.
    let mut cc_builder = cc::Build::new();
    for include in includes.split(' ') {
        // Updates the reference and returns it, we don't need to use the return
        let _: &mut cc::Build = cc_builder.include(Path::new(include.trim_start_matches("-I")));
    }
    cc_builder
        .file("reexport.c")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-deprecated-declarations")
        .compile("reexport");
}
