//! Rust CXX bridge code generation
//!
//! Generates a unified #[cxx::bridge] module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, Type};
use crate::type_mapping::{map_return_type_in_context, map_type_in_context, TypeContext};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// Rust keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Convert a parameter name to a safe Rust identifier string.
/// Rust keywords are renamed with trailing underscore (e.g., "where" -> "where_")
/// because CXX doesn't support r#keyword syntax in FFI declarations.
fn safe_param_name(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

/// Types collected from class interfaces
pub struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    pub classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    pub handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // This ensures the Handle type is declared even if not used in method signatures
        if class.is_handle_type && !class.has_protected_destructor {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, streams, void ptrs, etc.)
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "char" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// =============================================================================
// UNIFIED FFI MODULE GENERATION
// =============================================================================
//
// These functions generate a single unified FFI module containing ALL types,
// plus per-module re-export files. This avoids cross-module type filtering
// issues and simplifies the architecture.

/// Generate the unified ffi.rs file containing ALL types from all modules
///
/// This generates a single #[cxx::bridge] with all types using full C++ names
/// (e.g., gp_Pnt, TopoDS_Shape) to avoid collisions and make the mapping obvious.
///
/// Returns the generated Rust code as a String.
pub fn generate_unified_ffi(
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
) -> String {
    // Build sets for type context
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collections.iter().map(|c| c.typedef_name.clone()));
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect classes that will have Handle<T> declarations generated
    // These are classes that are handle types (inherit from Standard_Transient)
    // and don't have protected destructors
    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Create type context for unified module (no specific module)
    let type_ctx = TypeContext {
        current_module: "unified",  // Special marker for unified generation
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
    };

    // Get all classes with protected destructors
    let protected_destructor_class_names = symbol_table.protected_destructor_class_names();

    // Collect collection type names to exclude from class generation
    // Collections are generated separately with specialized wrappers
    let collection_type_names: HashSet<String> = collections.iter()
        .map(|c| c.typedef_name.clone())
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !b.has_protected_destructor)
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions (using pre-computed names from resolver)
    let function_items = generate_unified_functions(symbol_table, &type_ctx);

    // Generate Handle type declarations
    let handle_decls = generate_unified_handle_declarations(all_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes);
    let opaque_type_decls = generate_unified_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
    );

    // Generate UniquePtr impl blocks
    let unique_ptr_impls = generate_unified_unique_ptr_impls(all_classes);

    // Build the output
    let mut out = String::new();

    // File header
    let header_count = all_headers.len();
    writeln!(out, "//! Unified CXX bridge for OpenCASCADE").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! This file was automatically generated by opencascade-binding-generator").unwrap();
    writeln!(out, "//! from {} OCCT headers.", header_count).unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! Do not edit this file directly.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_snake_case)]").unwrap();
    writeln!(out, "#![allow(clippy::missing_safety_doc)]").unwrap();
    writeln!(out, "#[cxx::bridge]").unwrap();
    writeln!(out, "mod ffi {{").unwrap();
    writeln!(out, "    unsafe extern \"C++\" {{").unwrap();
    writeln!(out, "        include!(\"wrappers.hxx\");").unwrap();

    // Handle types section
    if !handle_decls.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out, "        // Handle types").unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&handle_decls);
    }

    // All types and methods section
    if !class_items.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out, "        // All types and methods").unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&class_items);
    }

    // Free functions section
    if !function_items.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out, "        // Free functions").unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&function_items);
    }

    // Referenced types (opaque) section
    if !opaque_type_decls.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out, "        // Referenced types (opaque)").unwrap();
        writeln!(out, "        // ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&opaque_type_decls);
    }

    // Insert collection FFI code if there are collections
    if !collections.is_empty() {
        let (coll_type_aliases, coll_ffi_decls) =
            super::collections::generate_unified_rust_ffi_collections(collections);
        out.push_str(&coll_type_aliases);
        out.push_str(&coll_ffi_decls);
    }

    // Close unsafe extern "C++" block
    writeln!(out, "    }}").unwrap();

    // UniquePtr impl blocks
    if !unique_ptr_impls.is_empty() {
        writeln!(out).unwrap();
        out.push_str(&unique_ptr_impls);
    }

    // Close mod ffi
    writeln!(out, "}}").unwrap();

    // Collection impl blocks are generated in per-module re-export files,
    // not here in ffi.rs, to avoid duplicate method definitions.

    // Add re-export of all FFI types
    writeln!(out).unwrap();
    writeln!(out, "// Re-export all FFI types to crate::ffi namespace").unwrap();
    writeln!(out, "pub use ffi::*;").unwrap();

    out
}

/// Generate free function declarations for unified mode.
///
/// Uses pre-computed names and filtering from the resolver's SymbolTable,
/// ensuring consistency between ffi.rs and wrappers.hxx.
fn generate_unified_functions(
    symbol_table: &crate::resolver::SymbolTable,
    ctx: &TypeContext,
) -> String {
    let mut out = String::new();

    for func in symbol_table.all_included_functions() {
        generate_unified_function(&mut out, func, ctx);
    }

    out
}

/// Generate a single free function for unified mode
fn generate_unified_function(
    out: &mut String,
    func: &crate::resolver::ResolvedFunction,
    ctx: &TypeContext,
) {
    let params_str: String = func.params.iter().map(|p| {
        let name = safe_param_name(&p.name);
        let ty = map_type_in_context(&p.ty.original, ctx);
        format!("{}: {}", name, ty.rust_type)
    }).collect::<Vec<_>>().join(", ");

    let ret_str = func.return_type.as_ref().map(|rt| {
        let mapped = map_return_type_in_context(&rt.original, ctx);
        format!(" -> {}", mapped.rust_type)
    }).unwrap_or_default();

    let source_attr = format_source_attribution(
        &func.source_header,
        None,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    writeln!(out, "        /// {}", source_attr).unwrap();
    if let Some(ref comment) = func.doc_comment {
        writeln!(out, "        ///").unwrap();
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(out, "        ///").unwrap();
            } else {
                writeln!(out, "        /// {}", trimmed).unwrap();
            }
        }
    }
    writeln!(out, "        #[cxx_name = \"{}\"]", func.cpp_wrapper_name).unwrap();
    writeln!(out, "        fn {}({}){};", func.rust_ffi_name, params_str, ret_str).unwrap();
}

/// Generate Handle type declarations for unified mode
fn generate_unified_handle_declarations(classes: &[&ParsedClass]) -> String {
    let mut handles = BTreeSet::new();

    for class in classes {
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
        }
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
        writeln!(out, "        /// Handle to {}", class_name).unwrap();
        writeln!(out, "        type {};", handle_type_name).unwrap();
    }
    out
}

/// Generate opaque type declarations for unified mode
fn generate_unified_opaque_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    all_enum_names: &HashSet<String>,
    protected_destructor_classes: &HashSet<String>,
    collection_type_names: &HashSet<String>,
) -> String {
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();
    let mut out = String::new();

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) {
            continue;
        }
        if all_enum_names.contains(type_name) {
            continue;
        }
        if protected_destructor_classes.contains(type_name) {
            continue;
        }
        if is_primitive_type(type_name) {
            continue;
        }
        // Skip collection types - they're generated separately
        if collection_type_names.contains(type_name) {
            continue;
        }

        writeln!(out, "        /// Referenced type from C++").unwrap();
        writeln!(out, "        type {};", type_name).unwrap();
    }

    out
}

/// Generate UniquePtr impl blocks for unified mode
fn generate_unified_unique_ptr_impls(classes: &[&ParsedClass]) -> String {
    let mut out = String::new();
    for class in classes {
        if !class.has_protected_destructor {
            writeln!(out, "    impl UniquePtr<{}> {{}}", class.name).unwrap();
        }
    }
    out
}

/// Emit a Rust `#[repr(i32)]` enum definition with TryFrom/From impls
fn emit_rust_enum(output: &mut String, resolved: &crate::resolver::ResolvedEnum) {
    // Doc comment
    if let Some(ref comment) = resolved.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }
    writeln!(output, "/// C++ enum: `{}`", resolved.cpp_name).unwrap();

    // Collect unique variants (skip duplicated values — C++ allows alias enum values, Rust doesn't)
    let mut seen_values = std::collections::HashSet::new();
    let mut unique_variants = Vec::new();
    let mut next_value: i64 = 0;
    for variant in &resolved.variants {
        let value = variant.value.unwrap_or(next_value);
        if seen_values.insert(value) {
            unique_variants.push((variant, value));
        }
        next_value = value + 1;
    }

    writeln!(output, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(output, "#[repr(i32)]").unwrap();
    writeln!(output, "pub enum {} {{", resolved.rust_name).unwrap();

    for (variant, value) in &unique_variants {
        if let Some(ref comment) = variant.doc_comment {
            for line in comment.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    writeln!(output, "    ///").unwrap();
                } else {
                    writeln!(output, "    /// {}", trimmed).unwrap();
                }
            }
        }
        writeln!(output, "    {} = {},", variant.rust_name, value).unwrap();
    }
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    // Generate From<EnumName> for i32
    let name = &resolved.rust_name;
    writeln!(output, "impl From<{}> for i32 {{", name).unwrap();
    writeln!(output, "    fn from(value: {}) -> Self {{", name).unwrap();
    writeln!(output, "        value as i32").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    // Generate TryFrom<i32> for EnumName
    // Use explicit error type to avoid ambiguity if enum has an "Error" variant
    writeln!(output, "impl TryFrom<i32> for {} {{", name).unwrap();
    writeln!(output, "    type Error = i32;").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "    fn try_from(value: i32) -> Result<Self, i32> {{").unwrap();
    writeln!(output, "        match value {{").unwrap();
    for (variant, value) in &unique_variants {
        writeln!(output, "            {} => Ok({}::{}),", value, name, variant.rust_name).unwrap();
    }
    writeln!(output, "            _ => Err(value),").unwrap();
    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();
}

/// Generate a module re-export file for the unified architecture
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");

    // Generate re-exports for functions using pre-computed names from the resolver.
    // The resolver assigns globally-unique rust_ffi_name values, so we just look them up.
    let rust_module = crate::module_graph::module_to_rust_name(module_name);
    let module_functions = symbol_table.included_functions_for_module(&rust_module);
    for func in &module_functions {
        output.push_str(&format!("pub use crate::ffi::{};\n", func.rust_ffi_name));
    }

    if !module_functions.is_empty() {
        output.push('\n');
    }

    // Generate Rust enum definitions for enums in this module
    if let Some(enum_ids) = symbol_table.enums_by_module.get(&rust_module) {
        for enum_id in enum_ids {
            if let Some(resolved_enum) = symbol_table.enums.get(enum_id) {
                if !matches!(resolved_enum.status, crate::resolver::BindingStatus::Included) {
                    continue;
                }
                emit_rust_enum(&mut output, resolved_enum);
            }
        }
    }

    // Re-export collection types belonging to this module
    for coll in collections {
        output.push_str(&format!(
            "pub use crate::ffi::{} as {};\n",
            coll.typedef_name, coll.short_name
        ));
    }
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect base handle types referenced by upcast methods that need re-exporting.
    // These are handle types for base classes (e.g. HandleGeomSurface, HandleGeomCurve)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Base handle type re-exports (targets of handle upcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");

        for bindings in header_bindings {
            output.push_str(&super::bindings::emit_reexport_class(bindings, module_name));
        }
    }

    // Re-export additional types (handles, opaque references, collection iterators)
    // that appear in ffi.rs but aren't covered by ClassBindings or collections.
    // Skip types already re-exported by ClassBindings (directly_exported_handles or base_handle_reexports).
    if !extra_types.is_empty() {
        let mut extra_lines = Vec::new();
        for (ffi_name, short_name) in extra_types {
            // Skip handle types that are already re-exported by emit_reexport_class (has_to_handle or has_handle_get)
            // or by the base handle re-exports section above.
            if directly_exported_handles.contains(ffi_name.as_str())
                || base_handle_reexports.contains(ffi_name.as_str())
            {
                continue;
            }
            if ffi_name == short_name {
                extra_lines.push(format!("pub use crate::ffi::{};\n", ffi_name));
            } else {
                extra_lines.push(format!(
                    "pub use crate::ffi::{} as {};\n",
                    ffi_name, short_name
                ));
            }
        }
        if !extra_lines.is_empty() {
            output.push_str("// ========================\n");
            output.push_str("// Additional type re-exports\n");
            output.push_str("// ========================\n\n");
            for line in &extra_lines {
                output.push_str(line);
            }
            output.push('\n');
        }
    }

    output
}
/// Generate an `impl` block for a collection type, re-exporting its FFI helper functions as methods.
fn emit_collection_impl(coll: &super::collections::CollectionInfo) -> String {
    use super::collections::CollectionKind;
    let mut out = String::new();
    let coll_name = &coll.typedef_name;
    let short = &coll.short_name;
    let elem = &coll.element_type;

    out.push_str(&format!("impl {} {{\n", short));

    match coll.kind {
        CollectionKind::List | CollectionKind::Sequence => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                short, coll_name
            ));
            // size
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        crate::ffi::{}_size(self)\n    }}\n\n",
                coll_name
            ));
            // clear
            out.push_str(&format!(
                "    /// Remove all elements\n    pub fn clear(self: std::pin::Pin<&mut Self>) {{\n        crate::ffi::{}_clear(self)\n    }}\n\n",
                coll_name
            ));
            // append
            out.push_str(&format!(
                "    /// Append an element\n    pub fn append(self: std::pin::Pin<&mut Self>, item: &crate::ffi::{}) {{\n        crate::ffi::{}_append(self, item)\n    }}\n\n",
                elem, coll_name
            ));
            if coll.kind == CollectionKind::List {
                // prepend
                out.push_str(&format!(
                    "    /// Prepend an element\n    pub fn prepend(self: std::pin::Pin<&mut Self>, item: &crate::ffi::{}) {{\n        crate::ffi::{}_prepend(self, item)\n    }}\n\n",
                    elem, coll_name
                ));
            }
            if coll.kind == CollectionKind::Sequence {
                // value (1-based index)
                out.push_str(&format!(
                    "    /// Get element at 1-based index\n    pub fn value(&self, index: i32) -> &crate::ffi::{} {{\n        crate::ffi::{}_value(self, index)\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                short, coll_name
            ));
            // size
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        crate::ffi::{}_size(self)\n    }}\n\n",
                coll_name
            ));
            // clear
            out.push_str(&format!(
                "    /// Remove all elements\n    pub fn clear(self: std::pin::Pin<&mut Self>) {{\n        crate::ffi::{}_clear(self)\n    }}\n\n",
                coll_name
            ));
            // add
            out.push_str(&format!(
                "    /// Add an element, returns index\n    pub fn add(self: std::pin::Pin<&mut Self>, item: &crate::ffi::{}) -> i32 {{\n        crate::ffi::{}_add(self, item)\n    }}\n\n",
                elem, coll_name
            ));
            if coll.kind == CollectionKind::IndexedMap {
                // find_key (1-based)
                out.push_str(&format!(
                    "    /// Get element at 1-based index\n    pub fn find_key(&self, index: i32) -> &crate::ffi::{} {{\n        crate::ffi::{}_find_key(self, index)\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::DataMap => {
            if let Some(ref value_type) = coll.value_type {
                // new
                out.push_str(&format!(
                    "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                    short, coll_name
                ));
                // size
                out.push_str(&format!(
                    "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        crate::ffi::{}_size(self)\n    }}\n\n",
                    coll_name
                ));
                // clear
                out.push_str(&format!(
                    "    /// Remove all elements\n    pub fn clear(self: std::pin::Pin<&mut Self>) {{\n        crate::ffi::{}_clear(self)\n    }}\n\n",
                    coll_name
                ));
                // bind
                out.push_str(&format!(
                    "    /// Bind a key to a value\n    pub fn bind(self: std::pin::Pin<&mut Self>, key: &crate::ffi::{}, value: &crate::ffi::{}) -> bool {{\n        crate::ffi::{}_bind(self, key, value)\n    }}\n\n",
                    elem, value_type, coll_name
                ));
                // find
                out.push_str(&format!(
                    "    /// Find a value by key (returns nullptr if not found)\n    pub fn find(&self, key: &crate::ffi::{}) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}_find(self, key)\n    }}\n\n",
                    elem, value_type, coll_name
                ));
                // contains
                out.push_str(&format!(
                    "    /// Check if key exists\n    pub fn contains(&self, key: &crate::ffi::{}) -> bool {{\n        crate::ffi::{}_contains(self, key)\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(ref value_type) = coll.value_type {
                // new
                out.push_str(&format!(
                    "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                    short, coll_name
                ));
                // size
                out.push_str(&format!(
                    "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        crate::ffi::{}_size(self)\n    }}\n\n",
                    coll_name
                ));
                // clear
                out.push_str(&format!(
                    "    /// Remove all elements\n    pub fn clear(self: std::pin::Pin<&mut Self>) {{\n        crate::ffi::{}_clear(self)\n    }}\n\n",
                    coll_name
                ));
                // add
                out.push_str(&format!(
                    "    /// Add a key-value pair, returns index\n    pub fn add(self: std::pin::Pin<&mut Self>, key: &crate::ffi::{}, value: &crate::ffi::{}) -> i32 {{\n        crate::ffi::{}_add(self, key, value)\n    }}\n\n",
                    elem, value_type, coll_name
                ));
                // find_from_key
                out.push_str(&format!(
                    "    /// Find value by key\n    pub fn find_from_key<'a>(&'a self, key: &crate::ffi::{}) -> &'a crate::ffi::{} {{\n        crate::ffi::{}_find_from_key(self, key)\n    }}\n\n",
                    elem, value_type, coll_name
                ));
                // find_from_index
                out.push_str(&format!(
                    "    /// Find value by 1-based index\n    pub fn find_from_index<'a>(&'a self, index: i32) -> &'a crate::ffi::{} {{\n        crate::ffi::{}_find_from_index(self, index)\n    }}\n\n",
                    value_type, coll_name
                ));
                // find_key
                out.push_str(&format!(
                    "    /// Find key by 1-based index\n    pub fn find_key(&self, index: i32) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}_find_key(self, index)\n    }}\n\n",
                    elem, coll_name
                ));
                // find_index
                out.push_str(&format!(
                    "    /// Find index by key (returns 0 if not found)\n    pub fn find_index(&self, key: &crate::ffi::{}) -> i32 {{\n        crate::ffi::{}_find_index(self, key)\n    }}\n\n",
                    elem, coll_name
                ));
                // contains
                out.push_str(&format!(
                    "    /// Check if key exists\n    pub fn contains(&self, key: &crate::ffi::{}) -> bool {{\n        crate::ffi::{}_contains(self, key)\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::Array1 => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                short, coll_name
            ));
            // ctor with bounds
            out.push_str(&format!(
                "    /// Create with lower and upper bounds\n    pub fn new_int2(lower: i32, upper: i32) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_ctor_int2(lower, upper)\n    }}\n\n",
                coll_name
            ));
            // ctor with bounds and init value
            out.push_str(&format!(
                "    /// Create with bounds, all elements initialized\n    pub fn new_int2_value(lower: i32, upper: i32, value: &crate::ffi::{}) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_ctor_int2_value(lower, upper, value)\n    }}\n\n",
                elem, coll_name
            ));
            // length
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn length(&self) -> i32 {{\n        crate::ffi::{}_length(self)\n    }}\n\n",
                coll_name
            ));
            // lower
            out.push_str(&format!(
                "    /// Get lower bound index\n    pub fn lower(&self) -> i32 {{\n        crate::ffi::{}_lower(self)\n    }}\n\n",
                coll_name
            ));
            // upper
            out.push_str(&format!(
                "    /// Get upper bound index\n    pub fn upper(&self) -> i32 {{\n        crate::ffi::{}_upper(self)\n    }}\n\n",
                coll_name
            ));
            // value
            out.push_str(&format!(
                "    /// Get element at index\n    pub fn value(&self, index: i32) -> &crate::ffi::{} {{\n        crate::ffi::{}_value(self, index)\n    }}\n\n",
                elem, coll_name
            ));
            // set_value
            out.push_str(&format!(
                "    /// Set element at index\n    pub fn set_value(self: std::pin::Pin<&mut Self>, index: i32, item: &crate::ffi::{}) {{\n        crate::ffi::{}_set_value(self, index, item)\n    }}\n\n",
                elem, coll_name
            ));
        }
        CollectionKind::Array2 => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_new()\n    }}\n\n",
                short, coll_name
            ));
            // ctor with bounds
            out.push_str(&format!(
                "    /// Create with row and column bounds\n    pub fn new_int4(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_ctor_int4(row_lower, row_upper, col_lower, col_upper)\n    }}\n\n",
                coll_name
            ));
            // ctor with bounds and init value
            out.push_str(&format!(
                "    /// Create with bounds, all elements initialized\n    pub fn new_int4_value(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32, value: &crate::ffi::{}) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}_ctor_int4_value(row_lower, row_upper, col_lower, col_upper, value)\n    }}\n\n",
                elem, coll_name
            ));
            // nb_rows / nb_columns
            out.push_str(&format!(
                "    /// Get number of rows\n    pub fn nb_rows(&self) -> i32 {{\n        crate::ffi::{}_nb_rows(self)\n    }}\n\n",
                coll_name
            ));
            out.push_str(&format!(
                "    /// Get number of columns\n    pub fn nb_columns(&self) -> i32 {{\n        crate::ffi::{}_nb_columns(self)\n    }}\n\n",
                coll_name
            ));
            // value
            out.push_str(&format!(
                "    /// Get element at (row, col)\n    pub fn value(&self, row: i32, col: i32) -> &crate::ffi::{} {{\n        crate::ffi::{}_value(self, row, col)\n    }}\n\n",
                elem, coll_name
            ));
            // set_value
            out.push_str(&format!(
                "    /// Set element at (row, col)\n    pub fn set_value(self: std::pin::Pin<&mut Self>, row: i32, col: i32, item: &crate::ffi::{}) {{\n        crate::ffi::{}_set_value(self, row, col, item)\n    }}\n\n",
                elem, coll_name
            ));
        }
    }

    // Add iter() method for non-array collection types
    match coll.kind {
        CollectionKind::Array1 | CollectionKind::Array2 => {}
        _ => {
            let iter_type = format!("{}Iterator", coll.short_name);
            out.push_str(&format!(
                "    /// Create an iterator over the collection\n    pub fn iter(&self) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}_iter(self)\n    }}\n\n",
                iter_type, coll_name
            ));
        }
    }

    out.push_str("}\n\n");

    // Add iterator impl block with next() for non-array collection types
    match coll.kind {
        CollectionKind::Array1 | CollectionKind::Array2 => {}
        _ => {
            let iter_type = format!("{}Iterator", coll.short_name);
            let next_suffix = match coll.kind {
                CollectionKind::DataMap | CollectionKind::IndexedDataMap => "_next_key",
                _ => "_next",
            };
            let next_fn = format!("{}{}", iter_type, next_suffix);
            out.push_str(&format!("impl {} {{\n", iter_type));
            out.push_str(&format!(
                "    /// Get next element (returns null UniquePtr when done)\n    pub fn next(self: std::pin::Pin<&mut Self>) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}(self)\n    }}\n",
                elem, next_fn
            ));
            out.push_str("}\n\n");
        }
    }

    out
}