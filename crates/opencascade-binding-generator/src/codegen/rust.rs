//! Rust extern "C" FFI code generation
//!
//! Generates the extern "C" FFI module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, Type};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
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
        "bool" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// =============================================================================
// FFI MODULE GENERATION
// =============================================================================
//
// These functions generate the FFI module containing ALL types,
// plus per-module re-export files. This avoids cross-module type filtering
// issues and simplifies the architecture.

/// Generate the ffi.rs file containing ALL types from all modules
///
/// This generates extern "C" declarations with all types using full C++ names
/// (e.g., gp_Pnt, TopoDS_Shape) to avoid collisions and make the mapping obvious.
///
/// Returns the generated Rust code as a String.
pub fn generate_ffi(
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors
    let protected_destructor_class_names = symbol_table.protected_destructor_class_names();

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

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

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, &symbol_table.handle_able_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes);
    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
    );

    // Generate nested type destructor declarations for ffi extern block
    let nested_destructor_decls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s, "    // Nested type destructors").unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s).unwrap();
        for nt in &nested_types {
            writeln!(s, "    pub fn {}_destructor(self_: *mut {});", nt.ffi_name, nt.ffi_name).unwrap();
        }
        s
    };

    // Generate CppDeletable impls for nested types
    let nested_deletable_impls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "// CppDeletable impls for nested types").unwrap();
        for nt in &nested_types {
            writeln!(s, "unsafe impl crate::CppDeletable for {} {{", nt.ffi_name).unwrap();
            writeln!(s, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(s, "        {}_destructor(ptr);", nt.ffi_name).unwrap();
            writeln!(s, "    }}").unwrap();
            writeln!(s, "}}").unwrap();
        }
        s
    };

    // Build the output
    let mut out = String::new();

    // File header
    let header_count = all_headers.len();
    writeln!(out, "//! extern \"C\" FFI for OpenCASCADE").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! This file was automatically generated by opencascade-binding-generator").unwrap();
    writeln!(out, "//! from {} OCCT headers.", header_count).unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! Do not edit this file directly.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_snake_case)]").unwrap();
    writeln!(out, "#![allow(clippy::missing_safety_doc)]").unwrap();
    writeln!(out).unwrap();

    // Handle types section (opaque structs outside extern "C")
    if !handle_decls.is_empty() {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Handle types").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&handle_decls);
        writeln!(out).unwrap();
    }

    // Class types (opaque structs outside extern "C")
    {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Class types (opaque)").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        for b in all_bindings.iter().filter(|b| !b.is_pod_struct).filter(|b| !collection_type_names.contains(&b.cpp_name)) {
            writeln!(out, "#[repr(C)]").unwrap();
            writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", b.cpp_name).unwrap();
        }
        writeln!(out).unwrap();
    }

    // POD struct types (transparent repr(C) with real fields)
    {
        let pod_structs: Vec<_> = all_bindings.iter().filter(|b| b.is_pod_struct).collect();
        if !pod_structs.is_empty() {
            writeln!(out, "// ========================").unwrap();
            writeln!(out, "// POD struct types").unwrap();
            writeln!(out, "// ========================").unwrap();
            writeln!(out).unwrap();
            for b in &pod_structs {
                writeln!(out, "#[repr(C)]").unwrap();
                writeln!(out, "#[derive(Debug, Clone, Copy)]").unwrap();
                writeln!(out, "pub struct {} {{", b.cpp_name).unwrap();
                for field in &b.pod_fields {
                    if let Some(ref comment) = field.doc_comment {
                        for line in comment.lines() {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                writeln!(out, "    ///").unwrap();
                            } else {
                                writeln!(out, "    /// {}", trimmed).unwrap();
                            }
                        }
                    }
                    if let Some(size) = field.array_size {
                        writeln!(out, "    pub {}: [{}; {}],", field.rust_name, field.rust_type, size).unwrap();
                    } else {
                        writeln!(out, "    pub {}: {},", field.rust_name, field.rust_type).unwrap();
                    }
                }
                writeln!(out, "}}").unwrap();
                writeln!(out).unwrap();
            }
        }
    }

    // Referenced types (opaque structs outside extern "C")
    if !opaque_type_decls.is_empty() {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Referenced types (opaque)").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&opaque_type_decls);
        writeln!(out).unwrap();
    }

    // Insert collection opaque type declarations outside extern "C"
    if !collections.is_empty() {
        let (coll_type_aliases, coll_ffi_decls) =
            super::collections::generate_rust_ffi_collections(collections);
        out.push_str(&coll_type_aliases);

        // Open extern "C" block
        writeln!(out, "extern \"C\" {{").unwrap();

        // All types and methods section
        if !class_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // All types and methods").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&class_items);
        }

        // Free functions section
        if !function_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // Free functions").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&function_items);
        }

        out.push_str(&coll_ffi_decls);

        // Nested type destructor declarations
        out.push_str(&nested_destructor_decls);

        // Close extern "C" block
        writeln!(out, "}}").unwrap();

        // CppDeletable impls for nested types (must be after extern block)
        out.push_str(&nested_deletable_impls);
    } else {
        // Open extern "C" block
        writeln!(out, "extern \"C\" {{").unwrap();

        // All types and methods section
        if !class_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // All types and methods").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&class_items);
        }

        // Free functions section
        if !function_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // Free functions").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&function_items);
        }

        // Nested type destructor declarations
        out.push_str(&nested_destructor_decls);

        // Close extern "C" block
        writeln!(out, "}}").unwrap();

        // CppDeletable impls for nested types (must be after extern block)
        out.push_str(&nested_deletable_impls);
    }

    (out, nested_types)
}

/// Generate free function declarations from pre-computed FunctionBindings.
fn generate_functions_from_bindings(
    function_bindings: &[super::bindings::FunctionBinding],
) -> String {
    let mut out = String::new();
    for func in function_bindings {
        let params_str: String = func.params.iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_ffi_type))
            .collect::<Vec<_>>()
            .join(", ");

        let ret_str = func.return_type.as_ref()
            .map(|rt| format!(" -> {}", rt.rust_ffi_type))
            .unwrap_or_default();

        let source_attr = format_source_attribution(
            &func.source_header,
            func.source_line,
            &format!("{}::{}", func.namespace, func.short_name),
        );
        writeln!(out, "    /// {}", source_attr).unwrap();
        writeln!(out, "    pub fn {}({}){};\n", func.cpp_wrapper_name, params_str, ret_str).unwrap();
    }
    out
}

/// Generate Handle type declarations
fn generate_handle_declarations(classes: &[&ParsedClass], extra_handle_able: &HashSet<String>) -> String {
    let mut handles = BTreeSet::new();

    // Classes parsed from non-excluded headers
    let mut defined_handles = BTreeSet::new();
    for class in classes {
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
            defined_handles.insert(class.name.clone());
        }
    }

    // Also generate Handle declarations for types that appear in Handle(...)
    // in method signatures, even if their own headers are excluded.
    // This ensures that methods like GeomAPI_Interpolate(Handle(TColgp_HArray1OfPnt))
    // can be generated even when TColgp_HArray1OfPnt.hxx is excluded.
    for name in extra_handle_able {
        handles.insert(name.clone());
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
        writeln!(out, "/// Handle to {}", class_name).unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", handle_type_name).unwrap();
    }

    // For extra handle types (not from parsed classes), generate standalone
    // CppDeletable impls and destructor FFI declarations.
    // Parsed classes get these in their module files instead.
    let extra_handles: Vec<_> = handles.iter()
        .filter(|name| !defined_handles.contains(*name))
        .collect();
    if !extra_handles.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "extern \"C\" {{").unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "    pub fn {}_destructor(ptr: *mut {});", handle_type_name, handle_type_name).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "unsafe impl crate::CppDeletable for {} {{", handle_type_name).unwrap();
            writeln!(out, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(out, "        {}_destructor(ptr);", handle_type_name).unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
    }

    out
}

/// Generate opaque type declarations
/// Nested type info for destructor generation.
/// (cpp_name with ::, ffi_name with _)
pub struct NestedTypeInfo {
    pub cpp_name: String,
    pub ffi_name: String,
}

fn generate_opaque_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    all_enum_names: &HashSet<String>,
    protected_destructor_classes: &HashSet<String>,
    collection_type_names: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();
    let mut out = String::new();
    let mut emitted: HashSet<String> = HashSet::new();
    let mut nested_types: Vec<NestedTypeInfo> = Vec::new();

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) {
            continue;
        }
        if all_enum_names.contains(type_name) {
            continue;
        }
        // Protected destructor classes still need opaque declarations when referenced
        // in method signatures; they just won't get CppDeletable.
        let has_protected_dtor = protected_destructor_classes.contains(type_name);
        if is_primitive_type(type_name) {
            continue;
        }
        // Skip collection types - they're generated separately
        if collection_type_names.contains(type_name) {
            continue;
        }
        // Nested C++ types (e.g., "Poly_CoherentTriangulation::TwoIntegers") get
        // flattened to valid Rust identifiers ("Poly_CoherentTriangulation_TwoIntegers")
        let is_nested = type_name.contains("::");
        let safe_name = if is_nested {
            type_name.replace("::", "_")
        } else {
            type_name.clone()
        };
        // Skip types with pointer/ref qualifiers leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution)
        if safe_name.contains('*') || safe_name.contains('&') {
            continue;
        }
        // Avoid duplicate opaque declarations (flattened nested name might collide
        // with an existing class name or another nested type)
        if defined_classes.contains(&safe_name) || !emitted.insert(safe_name.clone()) {
            continue;
        }

        writeln!(out, "/// Referenced type from C++").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", safe_name).unwrap();

        // Track nested types for destructor generation (skip protected destructor types)
        if is_nested && !has_protected_dtor {
            nested_types.push(NestedTypeInfo {
                cpp_name: type_name.clone(),
                ffi_name: safe_name,
            });
        }
    }

    (out, nested_types)
}

// UniquePtr impl blocks are no longer needed with extern "C" FFI

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
    writeln!(output, "    fn try_from(value: i32) -> ::core::result::Result<Self, i32> {{").unwrap();
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

/// Emit a wrapper function for a free function in the public module.
/// All free functions are real functions (not pub use re-exports) so that
/// IDE "go to definition" lands in the public module, not ffi::.
/// Includes source attribution and doc comments.
fn emit_free_function_wrapper(
    output: &mut String,
    func: &super::bindings::FunctionBinding,
) {
    use std::fmt::Write;

    // Source attribution + doc comment
    let source_attr = format_source_attribution(
        &func.source_header,
        func.source_line,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    writeln!(output, "/// {}", source_attr).unwrap();
    if let Some(ref comment) = func.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }

    // Build parameter list using pre-computed re-export types
    let params: Vec<String> = func.params.iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
        .collect();

    // Build args with .into() for enum params, CString conversion for &str params
    let args: Vec<String> = func.params.iter()
        .map(|p| {
            if p.rust_reexport_type == "&str" {
                format!("c_{}.as_ptr()", p.rust_name)
            } else if p.enum_rust_type.is_some() {
                format!("{}.into()", p.rust_name)
            } else {
                p.rust_name.clone()
            }
        })
        .collect();

    // Generate CString prelude for &str params
    let prelude: String = func.params.iter()
        .filter(|p| p.rust_reexport_type == "&str")
        .map(|p| format!("    let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name))
        .collect();

    // Build return type string
    let return_type_str = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", rt.rust_reexport_type))
        .unwrap_or_default();

    // Build call expression
    let call_expr = format!("crate::ffi::{}({})", func.cpp_wrapper_name, args.join(", "));

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
            call_expr
        }
    } else {
        call_expr
    };

    writeln!(output, "pub fn {}({}){} {{", func.rust_ffi_name, params.join(", "), return_type_str).unwrap();
    write!(output, "{}", prelude).unwrap();
    writeln!(output, "    unsafe {{ {} }}", body).unwrap();
    writeln!(output, "}}").unwrap();
}

/// Generate a module re-export file
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
    module_fn_bindings: &[&super::bindings::FunctionBinding],
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

    // Generate re-exports for free functions from pre-computed FunctionBindings.
    for func in module_fn_bindings {
        // All free functions become real wrapper functions (not pub use re-exports)
        // so IDE "go to definition" lands in the public module.
        emit_free_function_wrapper(&mut output, func);
    }

    if !module_fn_bindings.is_empty() {
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

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
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
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
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
                "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                short, coll_name
            ));
            // size
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_size(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // clear
            out.push_str(&format!(
                "    /// Remove all elements\n    pub fn clear(&mut self) {{\n        unsafe {{ crate::ffi::{}_clear(self as *mut Self) }}\n    }}\n\n",
                coll_name
            ));
            // append
            out.push_str(&format!(
                "    /// Append an element\n    pub fn append(&mut self, item: &crate::ffi::{}) {{\n        unsafe {{ crate::ffi::{}_append(self as *mut Self, item as *const crate::ffi::{}) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
            if coll.kind == CollectionKind::List {
                // prepend
                out.push_str(&format!(
                    "    /// Prepend an element\n    pub fn prepend(&mut self, item: &crate::ffi::{}) {{\n        unsafe {{ crate::ffi::{}_prepend(self as *mut Self, item as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, coll_name, elem
                ));
            }
            if coll.kind == CollectionKind::Sequence {
                // value (1-based index)
                out.push_str(&format!(
                    "    /// Get element at 1-based index\n    pub fn value(&self, index: i32) -> &crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_value(self as *const Self, index) }}\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                short, coll_name
            ));
            // size
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_size(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // clear
            out.push_str(&format!(
                "    /// Remove all elements\n    pub fn clear(&mut self) {{\n        unsafe {{ crate::ffi::{}_clear(self as *mut Self) }}\n    }}\n\n",
                coll_name
            ));
            // add
            out.push_str(&format!(
                "    /// Add an element, returns index\n    pub fn add(&mut self, item: &crate::ffi::{}) -> i32 {{\n        unsafe {{ crate::ffi::{}_add(self as *mut Self, item as *const crate::ffi::{}) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
            if coll.kind == CollectionKind::IndexedMap {
                // find_key (1-based)
                out.push_str(&format!(
                    "    /// Get element at 1-based index\n    pub fn find_key(&self, index: i32) -> &crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_find_key(self as *const Self, index) }}\n    }}\n\n",
                    elem, coll_name
                ));
            }
        }
        CollectionKind::DataMap => {
            if let Some(ref value_type) = coll.value_type {
                // new
                out.push_str(&format!(
                    "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                    short, coll_name
                ));
                // size
                out.push_str(&format!(
                    "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_size(self as *const Self) }}\n    }}\n\n",
                    coll_name
                ));
                // clear
                out.push_str(&format!(
                    "    /// Remove all elements\n    pub fn clear(&mut self) {{\n        unsafe {{ crate::ffi::{}_clear(self as *mut Self) }}\n    }}\n\n",
                    coll_name
                ));
                // bind
                out.push_str(&format!(
                    "    /// Bind a key to a value\n    pub fn bind(&mut self, key: &crate::ffi::{}, value: &crate::ffi::{}) -> bool {{\n        unsafe {{ crate::ffi::{}_bind(self as *mut Self, key as *const crate::ffi::{}, value as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, value_type, coll_name, elem, value_type
                ));
                // find
                out.push_str(&format!(
                    "    /// Find a value by key (returns nullptr if not found)\n    pub fn find(&self, key: &crate::ffi::{}) -> crate::OwnedPtr<crate::ffi::{}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_find(self as *const Self, key as *const crate::ffi::{})) }}\n    }}\n\n",
                    elem, value_type, coll_name, elem
                ));
                // contains
                out.push_str(&format!(
                    "    /// Check if key exists\n    pub fn contains(&self, key: &crate::ffi::{}) -> bool {{\n        unsafe {{ crate::ffi::{}_contains(self as *const Self, key as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, coll_name, elem
                ));
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(ref value_type) = coll.value_type {
                // new
                out.push_str(&format!(
                    "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                    short, coll_name
                ));
                // size
                out.push_str(&format!(
                    "    /// Get number of elements\n    pub fn size(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_size(self as *const Self) }}\n    }}\n\n",
                    coll_name
                ));
                // clear
                out.push_str(&format!(
                    "    /// Remove all elements\n    pub fn clear(&mut self) {{\n        unsafe {{ crate::ffi::{}_clear(self as *mut Self) }}\n    }}\n\n",
                    coll_name
                ));
                // add
                out.push_str(&format!(
                    "    /// Add a key-value pair, returns index\n    pub fn add(&mut self, key: &crate::ffi::{}, value: &crate::ffi::{}) -> i32 {{\n        unsafe {{ crate::ffi::{}_add(self as *mut Self, key as *const crate::ffi::{}, value as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, value_type, coll_name, elem, value_type
                ));
                // find_from_key
                out.push_str(&format!(
                    "    /// Find value by key\n    pub fn find_from_key<'a>(&'a self, key: &crate::ffi::{}) -> &'a crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_find_from_key(self as *const Self, key as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, value_type, coll_name, elem
                ));
                // find_from_index
                out.push_str(&format!(
                    "    /// Find value by 1-based index\n    pub fn find_from_index<'a>(&'a self, index: i32) -> &'a crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_find_from_index(self as *const Self, index) }}\n    }}\n\n",
                    value_type, coll_name
                ));
                // find_key
                out.push_str(&format!(
                    "    /// Find key by 1-based index\n    pub fn find_key(&self, index: i32) -> crate::OwnedPtr<crate::ffi::{}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_find_key(self as *const Self, index)) }}\n    }}\n\n",
                    elem, coll_name
                ));
                // find_index
                out.push_str(&format!(
                    "    /// Find index by key (returns 0 if not found)\n    pub fn find_index(&self, key: &crate::ffi::{}) -> i32 {{\n        unsafe {{ crate::ffi::{}_find_index(self as *const Self, key as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, coll_name, elem
                ));
                // contains
                out.push_str(&format!(
                    "    /// Check if key exists\n    pub fn contains(&self, key: &crate::ffi::{}) -> bool {{\n        unsafe {{ crate::ffi::{}_contains(self as *const Self, key as *const crate::ffi::{}) }}\n    }}\n\n",
                    elem, coll_name, elem
                ));
            }
        }
        CollectionKind::Array1 => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                short, coll_name
            ));
            // ctor with bounds
            out.push_str(&format!(
                "    /// Create with lower and upper bounds\n    pub fn new_int2(lower: i32, upper: i32) -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_ctor_int2(lower, upper)) }}\n    }}\n\n",
                coll_name
            ));
            // ctor with bounds and init value
            out.push_str(&format!(
                "    /// Create with bounds, all elements initialized\n    pub fn new_int2_value(lower: i32, upper: i32, value: &crate::ffi::{}) -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_ctor_int2_value(lower, upper, value as *const crate::ffi::{})) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
            // length
            out.push_str(&format!(
                "    /// Get number of elements\n    pub fn length(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_length(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // lower
            out.push_str(&format!(
                "    /// Get lower bound index\n    pub fn lower(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_lower(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // upper
            out.push_str(&format!(
                "    /// Get upper bound index\n    pub fn upper(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_upper(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // value
            out.push_str(&format!(
                "    /// Get element at index\n    pub fn value(&self, index: i32) -> &crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_value(self as *const Self, index) }}\n    }}\n\n",
                elem, coll_name
            ));
            // set_value
            out.push_str(&format!(
                "    /// Set element at index\n    pub fn set_value(&mut self, index: i32, item: &crate::ffi::{}) {{\n        unsafe {{ crate::ffi::{}_set_value(self as *mut Self, index, item as *const crate::ffi::{}) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
        }
        CollectionKind::Array2 => {
            // new
            out.push_str(&format!(
                "    /// Create a new empty {}\n    pub fn new() -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_new()) }}\n    }}\n\n",
                short, coll_name
            ));
            // ctor with bounds
            out.push_str(&format!(
                "    /// Create with row and column bounds\n    pub fn new_int4(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_ctor_int4(row_lower, row_upper, col_lower, col_upper)) }}\n    }}\n\n",
                coll_name
            ));
            // ctor with bounds and init value
            out.push_str(&format!(
                "    /// Create with bounds, all elements initialized\n    pub fn new_int4_value(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32, value: &crate::ffi::{}) -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_ctor_int4_value(row_lower, row_upper, col_lower, col_upper, value as *const crate::ffi::{})) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
            // nb_rows / nb_columns
            out.push_str(&format!(
                "    /// Get number of rows\n    pub fn nb_rows(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_nb_rows(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            out.push_str(&format!(
                "    /// Get number of columns\n    pub fn nb_columns(&self) -> i32 {{\n        unsafe {{ crate::ffi::{}_nb_columns(self as *const Self) }}\n    }}\n\n",
                coll_name
            ));
            // value
            out.push_str(&format!(
                "    /// Get element at (row, col)\n    pub fn value(&self, row: i32, col: i32) -> &crate::ffi::{} {{\n        unsafe {{ &*crate::ffi::{}_value(self as *const Self, row, col) }}\n    }}\n\n",
                elem, coll_name
            ));
            // set_value
            out.push_str(&format!(
                "    /// Set element at (row, col)\n    pub fn set_value(&mut self, row: i32, col: i32, item: &crate::ffi::{}) {{\n        unsafe {{ crate::ffi::{}_set_value(self as *mut Self, row, col, item as *const crate::ffi::{}) }}\n    }}\n\n",
                elem, coll_name, elem
            ));
        }
    }

    // Add iter() method for non-array collection types
    match coll.kind {
        CollectionKind::Array1 | CollectionKind::Array2 => {}
        _ => {
            let iter_type = format!("{}Iterator", coll.short_name);
            out.push_str(&format!(
                "    /// Create an iterator over the collection\n    pub fn iter(&self) -> crate::OwnedPtr<crate::ffi::{}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}_iter(self as *const Self)) }}\n    }}\n\n",
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
                "    /// Get next element (returns None when done)\n    pub fn next(&mut self) -> Option<crate::OwnedPtr<crate::ffi::{}>> {{\n        let ptr = unsafe {{ crate::ffi::{}(self as *mut Self) }};\n        if ptr.is_null() {{\n            None\n        }} else {{\n            Some(unsafe {{ crate::OwnedPtr::from_raw(ptr) }})\n        }}\n    }}\n",
                elem, next_fn
            ));
            out.push_str("}\n\n");

            // CppDeletable for iterator
            out.push_str(&format!(
                "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
                iter_type, iter_type
            ));
        }
    }

    // CppDeletable for collection
    out.push_str(&format!(
        "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
        short, coll_name
    ));

    out
}