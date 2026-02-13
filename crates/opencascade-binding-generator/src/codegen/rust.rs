//! Rust CXX bridge code generation
//!
//! Generates a unified #[cxx::bridge] module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, ParsedFunction, Type};
use crate::type_mapping::{map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use std::collections::{BTreeSet, HashMap, HashSet};
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
struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
fn collect_referenced_types(
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
fn is_primitive_type(name: &str) -> bool {
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
    all_functions: &[&ParsedFunction],
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

    // Generate namespace-level free functions
    let function_items = generate_unified_functions(all_functions, &type_ctx);

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

    // Append collection impl blocks
    if !collections.is_empty() {
        let coll_impls = super::collections::generate_unified_rust_impl_collections(collections);
        writeln!(out).unwrap();
        out.push_str(&coll_impls);
    }

    // Add re-export of all FFI types
    writeln!(out).unwrap();
    writeln!(out, "// Re-export all FFI types to crate::ffi namespace").unwrap();
    writeln!(out, "pub use ffi::*;").unwrap();

    out
}

/// Generate free function declarations for unified mode
fn generate_unified_functions(functions: &[&ParsedFunction], ctx: &TypeContext) -> String {
    let mut out = String::new();
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    for func in functions {
        if func.has_unbindable_types() {
            continue;
        }

        let base_rust_name = func.short_name.to_snake_case();
        let is_mut_version = func.params.iter().any(|p| matches!(&p.ty, Type::MutRef(_)));

        let count = seen_names.entry(base_rust_name.clone()).or_insert(0);
        *count += 1;

        let rust_name = if *count > 1 {
            let suffix = if is_mut_version { "_mut" } else { &format!("_{}", count) };
            format!("{}{}", base_rust_name, suffix)
        } else {
            base_rust_name
        };

        generate_unified_function(&mut out, func, &rust_name, ctx);
    }

    out
}

/// Generate a single free function for unified mode
fn generate_unified_function(
    out: &mut String,
    func: &ParsedFunction,
    rust_name: &str,
    ctx: &TypeContext,
) {
    let cpp_wrapper_name = format!("{}_{}", func.namespace, rust_name);

    let params_str: String = func.params.iter().map(|p| {
        let name = safe_param_name(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        format!("{}: {}", name, ty.rust_type)
    }).collect::<Vec<_>>().join(", ");

    let ret_str = func.return_type.as_ref().map(|ty| {
        let mapped = map_return_type_in_context(ty, ctx);
        // Enums map to i32, not UniquePtr — check needs_unique_ptr from the base mapping
        let base_mapped = map_type_in_context(ty, ctx);
        if base_mapped.needs_unique_ptr {
            format!(" -> UniquePtr<{}>", mapped.rust_type)
        } else {
            format!(" -> {}", mapped.rust_type)
        }
    }).unwrap_or_default();

    let source_attr = format_source_attribution(
        &func.source_header,
        None,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    writeln!(out, "        /// {}", source_attr).unwrap();
    if let Some(ref comment) = func.comment {
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
    writeln!(out, "        #[cxx_name = \"{}\"]", cpp_wrapper_name).unwrap();
    writeln!(out, "        fn {}({}){};", rust_name, params_str, ret_str).unwrap();
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
    functions: &[&ParsedFunction],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");

    // Generate re-exports for functions
    let mut seen_func_names: HashMap<String, usize> = HashMap::new();
    for func in functions {
        if func.has_unbindable_types() {
            continue;
        }

        let base_rust_name = func.short_name.to_snake_case();
        let is_mut_version = func.params.iter().any(|p| matches!(&p.ty, Type::MutRef(_)));

        let count = seen_func_names.entry(base_rust_name.clone()).or_insert(0);
        *count += 1;

        let rust_name = if *count > 1 {
            let suffix = if is_mut_version { "_mut" } else { &format!("_{}", count) };
            format!("{}{}", base_rust_name, suffix)
        } else {
            base_rust_name
        };

        output.push_str(&format!("pub use crate::ffi::{};\n", rust_name));
    }

    if !functions.is_empty() {
        output.push('\n');
    }

    // Generate Rust enum definitions for enums in this module
    let rust_module = crate::module_graph::module_to_rust_name(module_name);
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
        if b.has_to_handle {
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

    output
}
