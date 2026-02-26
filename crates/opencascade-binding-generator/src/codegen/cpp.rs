//! C++ wrapper code generation
//!
//! Generates extern "C" wrapper functions for all OCCT methods:
//! - Constructors (using placement new or heap allocation)
//! - Return-by-value methods
//! - Static methods
//! - Overloaded methods
//!
//! All wrapper functions use extern "C" linkage for direct FFI access.

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;
use std::path::Path;

/// Generate the shared C++ exception handling boilerplate.
/// This includes the OcctResult<T> template, occt_make_exception helper,
/// and OCCT_CATCH_RETURN / OCCT_CATCH_RETURN_VOID macros.
fn generate_exception_handling_boilerplate() -> &'static str {
    r#"
// ========================
// Exception handling
// ========================

#include <cxxabi.h>

template<typename T>
struct OcctResult {
    T ret;
    const char* exc;
};

template<>
struct OcctResult<void> {
    const char* exc;
};

extern "C" const char* occt_alloc_exception(const char* ptr, size_t len);

static const char* occt_make_exception(const char* type_name, const char* message) {
    std::string combined;
    if (type_name) {
        int status = 0;
        char* demangled = abi::__cxa_demangle(type_name, nullptr, nullptr, &status);
        if (status == 0 && demangled) {
            combined = demangled;
            std::free(demangled);
        } else {
            combined = type_name;
            std::free(demangled);
        }
    } else {
        combined = "<unknown>";
    }
    if (message && message[0] != '\0') {
        combined += ": ";
        combined += message;
    }
    return occt_alloc_exception(combined.data(), combined.size());
}

#define OCCT_CATCH_RETURN \
    catch (const Standard_Failure& e) { return {{}, occt_make_exception(typeid(e).name(), e.GetMessageString())}; } \
    catch (const std::exception& e) { return {{}, occt_make_exception(typeid(e).name(), e.what())}; } \
    catch (...) { return {{}, occt_make_exception(nullptr, "unknown C++ exception")}; }

#define OCCT_CATCH_RETURN_VOID \
    catch (const Standard_Failure& e) { return occt_make_exception(typeid(e).name(), e.GetMessageString()); } \
    catch (const std::exception& e) { return occt_make_exception(typeid(e).name(), e.what()); } \
    catch (...) { return occt_make_exception(nullptr, "unknown C++ exception"); }

"#
}
fn collect_handle_types(classes: &[&ParsedClass], handle_able_classes: &HashSet<String>) -> Vec<(String, String)> {
    let mut handles = HashSet::new();

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // Handle types with protected destructors are included because Handle<T>
        // manages lifetime via reference counting, not direct delete.
        if handle_able_classes.contains(&class.name) {
            handles.insert(class.name.clone());
        }

        for method in &class.methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for method in &class.static_methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }
    }

    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|inner_class| {
            // Skip pointer/reference types leaked into names, and template types
            // whose instantiated names aren't valid C++ identifiers
            !inner_class.contains('*') && !inner_class.contains('&') && !inner_class.contains('<')
        })
        .map(|inner_class| {
            // Use full class name to match Rust side (e.g., HandleGeom2dCurve not HandleCurve)
            let handle_name = crate::type_mapping::handle_type_name(&inner_class);
            (inner_class, handle_name)
        })
        .collect();
    result.sort();
    result
}

/// Collect all Handle type names referenced by ClassBindings that may not appear
/// in ParsedClass methods (e.g., from handle upcasts/downcasts, to_handle).
/// Returns (inner_class_name, handle_typedef_name) pairs.
fn collect_handle_types_from_bindings(
    bindings: &[&super::bindings::ClassBindings],
    handle_able_classes: &HashSet<String>,
) -> Vec<(String, String)> {
    let mut handles = HashSet::new();
    for b in bindings {
        // Handle type for the class itself (used by has_to_handle, has_handle_get)
        if b.has_to_handle || b.has_handle_get {
            if handle_able_classes.contains(&b.cpp_name) {
                handles.insert(b.cpp_name.clone());
            }
        }
        // Handle upcasts reference base class handle types
        for hup in &b.handle_upcasts {
            handles.insert(hup.base_class.clone());
            // Derived class too (the class itself)
            if handle_able_classes.contains(&b.cpp_name) {
                handles.insert(b.cpp_name.clone());
            }
        }
        // Handle downcasts reference derived class handle types
        for hdc in &b.handle_downcasts {
            handles.insert(hdc.derived_class.clone());
            // Base class too (the class itself)
            if handle_able_classes.contains(&b.cpp_name) {
                handles.insert(b.cpp_name.clone());
            }
        }
    }
    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|name| !name.contains('*') && !name.contains('&') && !name.contains('<'))
        .map(|name| {
            let handle_name = crate::type_mapping::handle_type_name(&name);
            (name, handle_name)
        })
        .collect();
    result.sort();
    result
}


/// Collect Handle type inner classes from a type
fn collect_type_handles(ty: &Option<Type>, handles: &mut HashSet<String>) {
    if let Some(ty) = ty {
        match ty {
            Type::Handle(name) => {
                handles.insert(name.clone());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_handles(&Some(inner.as_ref().clone()), handles);
            }
            _ => {}
        }
    }
}

/// Extract a potential C++ type name from a cpp_type string like "const gp_Pnt&", "TopoDS_Shape*"
/// and add its header if it exists in known_headers. Returns the bare type name if unresolved.
fn collect_headers_from_cpp_type_str(
    cpp_type: &str,
    headers: &mut HashSet<String>,
    known_headers: &HashSet<String>,
    class_header_map: &std::collections::HashMap<String, String>,
    unresolved: &mut HashSet<String>,
) {
    // Strip const, &, * to get the bare type name
    let name = cpp_type
        .trim()
        .trim_start_matches("const ")
        .trim_end_matches('&')
        .trim_end_matches('*')
        .trim()
        .trim_end_matches("const")
        .trim();
    // Skip empty, primitives, and types without underscore (likely nested/builtin)
    if name.is_empty() || !name.contains('_') {
        return;
    }
    // Skip void-like types
    if matches!(name, "bool" | "char" | "int" | "unsigned" | "float" | "double" | "size_t") {
        return;
    }
    // Check class_header_map first (authoritative)
    if let Some(header) = class_header_map.get(name) {
        headers.insert(header.clone());
        return;
    }
    // Try the convention: TypeName.hxx
    let header = format!("{}.hxx", name);
    if known_headers.is_empty() || known_headers.contains(&header) {
        headers.insert(header);
        return;
    }
    // Track unresolved type for batch search later
    unresolved.insert(name.to_string());
}

/// Collect headers for types referenced in ClassBindings methods (especially inherited methods)
/// that may come from other toolkits. If include_dir is provided, does a single batch search
/// for any types that don't have their own .hxx file.
pub fn collect_headers_from_bindings(
    bindings: &[&super::bindings::ClassBindings],
    known_headers: &HashSet<String>,
    class_header_map: &std::collections::HashMap<String, String>,
    include_dir: Option<&Path>,
) -> Vec<String> {
    let mut headers = HashSet::new();
    let mut unresolved = HashSet::new();
    for b in bindings {
        // Inherited methods are the main source of cross-toolkit type references
        for im in &b.inherited_methods {
            for p in &im.params {
                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
            if let Some(rt) = &im.return_type {
                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
        }
        // Also check wrapper methods (may reference cross-toolkit types)
        for wm in &b.wrapper_methods {
            for p in &wm.params {
                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
            if let Some(rt) = &wm.return_type {
                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
        }
        // Static methods
        for sm in &b.static_methods {
            for p in &sm.params {
                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
            if let Some(rt) = &sm.return_type {
                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
        }
        // Constructors
        for ctor in &b.constructors {
            for p in &ctor.params {
                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);
            }
        }
    }
    // Batch-resolve unresolved types by scanning OCCT headers once
    if !unresolved.is_empty() {
        if let Some(inc_dir) = include_dir {
            batch_find_defining_headers(&unresolved, inc_dir, known_headers, &mut headers);
        }
    }
    let mut result: Vec<_> = headers.into_iter().collect();
    result.sort();
    result
}

/// Extract type names from template arguments and add their headers.
/// Handles nested templates like NCollection_Shared<NCollection_DynamicArray<BRepMesh_Vertex>>
/// If include_dir is provided, searches OCCT headers for types without their own .hxx file.
fn collect_template_arg_headers(
    spelling: &str,
    known_headers: &HashSet<String>,
    headers: &mut HashSet<String>,
    include_dir: Option<&std::path::Path>,
) {
    // Find all identifiers that look like OCCT type names (contain underscore)
    // by splitting on template delimiters and whitespace
    let chars: Vec<char> = spelling.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        // Skip non-alphabetic chars
        if !chars[i].is_alphanumeric() && chars[i] != '_' {
            i += 1;
            continue;
        }
        // Collect an identifier
        let start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        let ident: String = chars[start..i].iter().collect();
        // Skip template class names (NCollection_*), primitives, and keywords
        if ident.starts_with("NCollection_") || ident.starts_with("opencascade") {
            continue;
        }
        if matches!(ident.as_str(), "handle" | "const" | "Standard_Real" | "Standard_Integer"
            | "Standard_Boolean" | "Standard_ShortReal" | "Standard_Character" | "bool"
            | "int" | "double" | "float" | "void" | "char" | "unsigned") {
            continue;
        }
        if ident.contains('_') || ident.starts_with("Standard") {
            let header = format!("{}.hxx", ident);
            if known_headers.contains(&header) {
                headers.insert(header);
            } else if let Some(dir) = include_dir {
                // Type doesn't have its own header - search OCCT headers for its definition
                if let Some(defining_header) = find_defining_header(&ident, dir, known_headers) {
                    headers.insert(defining_header);
                }
            }
        }
    }
}

/// Batch-search OCCT headers to find which .hxx files define the given type names.
/// More efficient than calling find_defining_header per type since it scans each header only once.
fn batch_find_defining_headers(
    type_names: &HashSet<String>,
    include_dir: &Path,
    known_headers: &HashSet<String>,
    headers: &mut HashSet<String>,
) {
    use std::io::BufRead;
    if type_names.is_empty() {
        return;
    }
    let mut remaining: HashSet<&str> = type_names.iter().map(|s| s.as_str()).collect();
    // Group type names by module prefix for prioritized search
    let mut prefix_types: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for name in &remaining {
        let prefix = name.split('_').next().unwrap_or("");
        prefix_types.entry(prefix).or_default().push(name);
    }
    // Sort headers for deterministic iteration order
    let mut sorted_known: Vec<&String> = known_headers.iter().collect();
    sorted_known.sort();
    // Pass 1: Search headers matching module prefix
    for header_name in &sorted_known {
        if remaining.is_empty() {
            break;
        }
        let header_prefix = header_name.split('_').next().unwrap_or("");
        // Only check if any remaining type has this prefix
        let relevant_types: Vec<&str> = prefix_types
            .get(header_prefix)
            .map(|v| v.iter().copied().filter(|t| remaining.contains(t)).collect())
            .unwrap_or_default();
        if relevant_types.is_empty() {
            continue;
        }
        let path = include_dir.join(header_name.as_str());
        if let Ok(file) = std::fs::File::open(&path) {
            let reader = std::io::BufReader::new(file);
            let mut prev_had_typedef = false;
            for line in reader.lines() {
                if let Ok(line) = line {
                    for &type_name in &relevant_types {
                        if remaining.contains(type_name) && line.contains(type_name) {
                            if line.contains("typedef") || line.contains("enum ") || prev_had_typedef {
                                headers.insert((*header_name).clone());
                                remaining.remove(type_name);
                            }
                        }
                    }
                    prev_had_typedef = line.contains("typedef") && !line.contains(';');
                }
            }
        }
    }
    // Pass 2: Search ALL headers for any still-unresolved types
    if !remaining.is_empty() {
        for header_name in &sorted_known {
            if remaining.is_empty() {
                break;
            }
            let path = include_dir.join(header_name.as_str());
            if let Ok(file) = std::fs::File::open(&path) {
                let reader = std::io::BufReader::new(file);
                let mut prev_had_typedef = false;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        for type_name in remaining.iter().copied().collect::<Vec<_>>() {
                            if line.contains(type_name) {
                                if line.contains("typedef") || line.contains("enum ") || prev_had_typedef {
                                    headers.insert((*header_name).clone());
                                    remaining.remove(type_name);
                                }
                            }
                        }
                        prev_had_typedef = line.contains("typedef") && !line.contains(';');
                    }
                }
            }
        }
    }
}

/// Search OCCT headers to find which .hxx file defines a given type name.
/// Looks for typedef/enum declarations containing the type name.
/// Prefers file-scope definitions (non-indented) over class-scope ones.
fn find_defining_header(
    type_name: &str,
    include_dir: &std::path::Path,
    known_headers: &HashSet<String>,
) -> Option<String> {
    use std::io::BufRead;
    let module_prefix = type_name.split('_').next().unwrap_or("");
    let mut class_scope_match: Option<String> = None;
    // Sort headers for deterministic iteration order
    let mut sorted_known: Vec<&String> = known_headers.iter().collect();
    sorted_known.sort();

    for pass in 0..2 {
        for header_name in &sorted_known {
            if pass == 0 && !header_name.starts_with(module_prefix) {
                continue;
            }
            if pass == 1 && header_name.starts_with(module_prefix) {
                continue;
            }
            let path = include_dir.join(header_name.as_str());
            if let Ok(file) = std::fs::File::open(&path) {
                let reader = std::io::BufReader::new(file);
                let mut prev_had_typedef = false;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let has_type_name = line.contains(type_name);
                        if has_type_name {
                            if line.contains("typedef") || line.contains("enum ") || prev_had_typedef {
                                // Prefer file-scope definitions (non-indented)
                                if !line.starts_with(' ') && !line.starts_with('\t') {
                                    return Some((*header_name).clone());
                                }
                                if class_scope_match.is_none() {
                                    class_scope_match = Some((*header_name).clone());
                                }
                            }
                        }
                        prev_had_typedef = line.contains("typedef") && !line.contains(';');
                    }
                }
            }
        }
    }
    class_scope_match
}

/// Collect headers needed for a type
fn collect_type_headers(ty: &Option<Type>, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if let Some(ty) = ty {
        // Skip unbindable types (arrays, streams, void pointers, etc.)
        // But allow class raw pointers — they're bindable as &T / &mut T
        if ty.is_unbindable() && ty.class_ptr_inner_name().is_none() {
            return;
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Skip primitive types that don't have headers
                // Also skip void-pointer types (Standard_Address, void_pointer_types from config)
                // which are typedefs in Standard_TypeDef.hxx, not their own files.
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | "size_t"
                ) || crate::model::is_void_type_name(name.as_str()) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
                if let Some(parent) = name.split("::").next() {
                    if name.contains("::") {
                        // Nested type — include the parent's header
                        if parent.contains('_') || parent.starts_with("Standard") {
                            let header = format!("{}.hxx", parent);
                            if known_headers.is_empty() || known_headers.contains(&header) {
                                headers.insert(header);
                            }
                        }
                        return;
                    }
                }
                // Skip types without underscore that aren't Standard* — likely nested types
                // whose qualified name was resolved by clang to just the leaf name
                if !name.contains('_') && !name.starts_with("Standard") {
                    return;
                }
                // Only include headers that actually exist in the OCCT include directory
                let header = format!("{}.hxx", name);
                if known_headers.is_empty() || known_headers.contains(&header) {
                    headers.insert(header);
                }
            }
            Type::Handle(name) => {
                let header = format!("{}.hxx", name);
                if known_headers.is_empty() || known_headers.contains(&header) {
                    headers.insert(header);
                }
                headers.insert("Standard_Handle.hxx".to_string());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_headers(&Some(inner.as_ref().clone()), headers, known_headers);
            }
            _ => {}
        }
    }
}

/// Generate wrappers for namespace-level free functions from pre-computed FunctionBindings.
/// Accepts both `&[FunctionBinding]` and `&[&FunctionBinding]` via `Borrow`.
fn generate_function_wrappers<T: std::borrow::Borrow<super::bindings::FunctionBinding>>(
    output: &mut String,
    function_bindings: &[T],
) {
    if function_bindings.is_empty() {
        return;
    }

    // Group functions by namespace
    let mut by_namespace: std::collections::HashMap<&str, Vec<&super::bindings::FunctionBinding>> =
        std::collections::HashMap::new();
    for func in function_bindings {
        let func = func.borrow();
        by_namespace
            .entry(&func.namespace)
            .or_default()
            .push(func);
    }

    let mut namespaces: Vec<&&str> = by_namespace.keys().collect();
    namespaces.sort();

    for namespace in namespaces {
        let namespace_functions = &by_namespace[namespace];

        writeln!(output, "// ========================").unwrap();
        writeln!(output, "// {} namespace functions", namespace).unwrap();
        writeln!(output, "// ========================").unwrap();

        for func in namespace_functions {
            let wrapper_name = &func.cpp_wrapper_name;

            // Build param declarations from pre-computed cpp_type
            let params_cpp: Vec<String> = func.params.iter()
                .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
                .collect();
            let params_str = params_cpp.join(", ");

            // Build argument expressions from pre-computed cpp_arg_expr
            let args: Vec<String> = func.params.iter()
                .map(|p| p.cpp_arg_expr.clone())
                .collect();
            let args_str = args.join(", ");

            let call = format!("{}::{}({})", namespace, func.short_name, args_str);

            // Determine return pattern from pre-computed return type binding
            if let Some(ref rt) = func.return_type {
                let ret_type_cpp = rt.ffi_cpp_return_type();
                let expr = rt.format_cpp_return_expr(&call);
                writeln!(
                    output,
                    "extern \"C\" OcctResult<{}> {}({}) {{",
                    ret_type_cpp, wrapper_name, params_str
                ).unwrap();
                writeln!(output, "    try {{ return {{{}, nullptr}}; }}", expr).unwrap();
                writeln!(output, "    OCCT_CATCH_RETURN").unwrap();
                writeln!(output, "}}").unwrap();
            } else {
                writeln!(
                    output,
                    "extern \"C\" const char* {}({}) {{",
                    wrapper_name, params_str
                ).unwrap();
                writeln!(output, "    try {{ {}; return nullptr; }}", call).unwrap();
                writeln!(output, "    OCCT_CATCH_RETURN_VOID").unwrap();
                writeln!(output, "}}").unwrap();
            }

        }
        writeln!(output).unwrap();
    }
}

fn collect_function_required_headers<T: std::borrow::Borrow<super::bindings::FunctionBinding>>(
    function_bindings: &[T],
    known_headers: &HashSet<String>,
) -> Vec<String> {
    let mut headers = HashSet::new();

    for func in function_bindings {
        let func = func.borrow();
        let ns_header = format!("{}.hxx", func.namespace);
        if known_headers.is_empty() || known_headers.contains(&ns_header) {
            headers.insert(ns_header);
        }

        for header in &func.cpp_headers {
            headers.insert(header.clone());
        }
    }

    let mut result: Vec<_> = headers.into_iter().collect();
    result.sort();
    result
}

fn extend_unique_headers(headers: &mut Vec<String>, additional_headers: impl IntoIterator<Item = String>) {
    for header in additional_headers {
        if !headers.contains(&header) {
            headers.push(header);
        }
    }
}

pub fn generate_wrappers(
    all_classes: &[&ParsedClass],
    collections: &[super::collections::CollectionInfo],
    known_headers: &HashSet<String>,
    _symbol_table: &SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    nested_types: &[super::rust::NestedTypeInfo],
    handle_able_classes: &HashSet<String>,
    template_instantiations: &std::collections::HashMap<String, crate::config::TemplateInstantiation>,
) -> String {
    let mut output = String::new();

    // Header guard and includes
    writeln!(output, "// Generated by opencascade-binding-generator").unwrap();
    writeln!(output, "// C++ wrappers for all OCCT modules").unwrap();
    writeln!(output).unwrap();

    // Collect ALL headers needed
    let mut headers = collect_all_required_headers(all_classes, known_headers);
    extend_unique_headers(
        &mut headers,
        collect_function_required_headers(function_bindings, known_headers),
    );
    extend_unique_headers(
        &mut headers,
        super::collections::collect_collection_headers(collections),
    );
    extend_unique_headers(&mut headers, ["cstdint".to_string(), "new".to_string(), "typeinfo".to_string(), "cstring".to_string()]);

    // Add headers needed for template instantiations
    for inst in template_instantiations.values() {
        // OCCT headers (.hxx) must be in known_headers; standard library headers
        // (no extension, e.g., "utility", "memory") are always available.
        let is_std_header = !inst.header.contains('.');
        if is_std_header || known_headers.contains(&inst.header) {
            extend_unique_headers(&mut headers, [inst.header.clone()]);
        }
    }

    headers.sort();
    headers.dedup();

    for header in &headers {
        writeln!(output, "#include <{}>", header).unwrap();
    }
    writeln!(output).unwrap();

    // Exception handling: OcctResult<T> template with null-terminated exc string.
    // Non-void wrappers return OcctResult<T>, void wrappers return const char*.
    output.push_str(&generate_exception_handling_boilerplate());

    // Generate typedefs for template instantiation aliases.
    // These MUST come before Handle typedefs since handles reference the alias names.
    // Only class typedefs are emitted here; Handle typedefs and destructors are
    // handled by the existing collect_handle_types / nested_types mechanisms.
    if !template_instantiations.is_empty() {
        writeln!(output, "// Template instantiation aliases").unwrap();
        let mut sorted_tmpls: Vec<_> = template_instantiations.iter().collect();
        sorted_tmpls.sort_by_key(|(k, _)| (*k).clone());
        for (spelling, _inst) in &sorted_tmpls {
            let alias = crate::config::template_alias_name(spelling);
            writeln!(output, "typedef {} {};", spelling, alias).unwrap();
        }
        writeln!(output).unwrap();
    }

    // Generate Handle typedefs for ALL classes
    let handle_types = collect_handle_types(all_classes, handle_able_classes);
    if !handle_types.is_empty() {
        writeln!(output, "// Handle type aliases").unwrap();
        for (inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "typedef opencascade::handle<{}> {};",
                inner_class, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();

        // Handle type destructors
        writeln!(output, "// Handle type destructors").unwrap();
        for (_inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}",
                handle_name, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Generate wrapper functions for ALL classes from pre-computed ClassBindings
    for bindings in all_bindings {
        output.push_str(&super::bindings::emit_cpp_class(bindings));
    }

    // Generate wrappers for ALL namespace-level free functions
    generate_function_wrappers(&mut output, function_bindings);

    // Generate destructors for nested types and extra typedef types (e.g., gp_Vec3f)
    if !nested_types.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// Nested type and typedef type destructors").unwrap();
        for nt in nested_types {
            writeln!(
                output,
                "extern \"C\" void {ffi}_destructor({cpp}* self_) {{ delete self_; }}",
                ffi = nt.ffi_name,
                cpp = nt.cpp_name
            )
            .unwrap();
        }
    }

    // Generate collection wrappers
    if !collections.is_empty() {
        output.push_str(&super::collections::generate_cpp_collections(collections));
    }

    output
}

/// Generate the common C++ preamble header.
///
/// Contains exception handling boilerplate (OcctResult template, catch macros,
/// occt_make_exception) shared across all wrapper files.
pub fn generate_preamble(
    template_instantiations: &std::collections::HashMap<String, crate::config::TemplateInstantiation>,
    known_headers: &HashSet<String>,
    include_dir: Option<&std::path::Path>,
) -> String {
    let mut output = String::new();
    writeln!(output, "// Generated by opencascade-binding-generator").unwrap();
    writeln!(output, "// Common preamble for split C++ wrapper files").unwrap();
    writeln!(output, "#pragma once").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "#include <cstdint>").unwrap();
    writeln!(output, "#include <cstring>").unwrap();
    writeln!(output, "#include <new>").unwrap();
    writeln!(output, "#include <string>").unwrap();
    writeln!(output, "#include <typeinfo>").unwrap();
    writeln!(output).unwrap();
    output.push_str(&generate_exception_handling_boilerplate());

    // Template instantiation headers and typedefs (available to all split files)
    if !template_instantiations.is_empty() {
        let mut sorted_tmpls: Vec<_> = template_instantiations.iter().collect();
        sorted_tmpls.sort_by_key(|(k, _)| (*k).clone());

        // Collect ALL headers needed: template class headers + element type headers
        let mut tmpl_headers: HashSet<String> = HashSet::new();
        for (spelling, inst) in template_instantiations {
            // Template class header
            let is_std_header = !inst.header.contains('.');
            if is_std_header || known_headers.contains(&inst.header) {
                tmpl_headers.insert(inst.header.clone());
            }
            // Extract element/value type headers from template arguments
            collect_template_arg_headers(spelling, known_headers, &mut tmpl_headers, include_dir);
        }
        let mut sorted_headers: Vec<_> = tmpl_headers.into_iter().collect();
        sorted_headers.sort();
        for header in &sorted_headers {
            writeln!(output, "#include <{}>", header).unwrap();
        }
        writeln!(output).unwrap();

        writeln!(output, "// Template instantiation aliases").unwrap();
        for (spelling, _inst) in &sorted_tmpls {
            let alias = crate::config::template_alias_name(spelling);
            writeln!(output, "typedef {} {};", spelling, alias).unwrap();
        }
        writeln!(output).unwrap();
    }

    output
}

/// `own_class_names` controls which Handle destructors are emitted in this file
/// to avoid duplicate symbols across split files. Only handles whose inner class
/// is in `own_class_names` get destructors here; all referenced handles get typedefs.
pub fn generate_wrappers_for_group(
    group_name: &str,
    classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    known_headers: &HashSet<String>,
    bindings: &[&super::bindings::ClassBindings],
    function_bindings: &[&super::bindings::FunctionBinding],
    nested_types: &[&super::rust::NestedTypeInfo],
    handle_able_classes: &HashSet<String>,
    template_instantiations: &std::collections::HashMap<String, crate::config::TemplateInstantiation>,
    preamble_filename: &str,
    own_class_names: &HashSet<String>,
    class_header_map: &std::collections::HashMap<String, String>,
    include_dir: Option<&Path>,
) -> String {
    let mut output = String::new();

    writeln!(output, "// Generated by opencascade-binding-generator").unwrap();
    writeln!(output, "// C++ wrappers for toolkit: {}", group_name).unwrap();
    writeln!(output).unwrap();

    // Include the common preamble
    writeln!(output, "#include \"{}\"", preamble_filename).unwrap();
    writeln!(output).unwrap();


    // Compute handle types early (needed for header collection)
    let mut handle_types = collect_handle_types(classes, handle_able_classes);
    let binding_handles = collect_handle_types_from_bindings(bindings, handle_able_classes);
    for (inner, hname) in binding_handles {
        if !handle_types.iter().any(|(i, _)| *i == inner) {
            handle_types.push((inner, hname));
        }
    }
    handle_types.sort();

    // Collect headers needed for this group's classes
    let mut headers = collect_all_required_headers(classes, known_headers);
    extend_unique_headers(
        &mut headers,
        collect_function_required_headers(function_bindings, known_headers),
    );
    let coll_vec: Vec<_> = collections.iter().copied().cloned().collect();
    extend_unique_headers(
        &mut headers,
        super::collections::collect_collection_headers(&coll_vec),
    );

    // Add headers for nested types (TypeName.hxx convention)
    for nt in nested_types {
        let header = format!("{}.hxx", nt.cpp_name);
        if known_headers.is_empty() || known_headers.contains(&header) {
            extend_unique_headers(&mut headers, [header]);
        }
    }

    // Add headers for cross-toolkit types referenced in bindings (especially inherited methods)
    extend_unique_headers(
        &mut headers,
        collect_headers_from_bindings(bindings, known_headers, class_header_map, include_dir),
    );


    // Add headers for ALL handle types (including cross-toolkit ones)
    // Use class_header_map to find correct headers for classes whose header
    // doesn't match ClassName.hxx (e.g., BOPAlgo_Alerts.hxx)
    for (inner_class, _) in &handle_types {
        if let Some(header) = class_header_map.get(inner_class) {
            extend_unique_headers(&mut headers, [header.clone()]);
        } else {
            let header = format!("{}.hxx", inner_class);
            if known_headers.is_empty() || known_headers.contains(&header) {
                extend_unique_headers(&mut headers, [header]);
            }
        }
    }
    if !handle_types.is_empty() {
        extend_unique_headers(&mut headers, ["Standard_Handle.hxx".to_string()]);
    }

    // Don't include headers already in preamble (including template instantiation headers)
    let preamble_headers = ["cstdint", "new", "typeinfo", "cstring", "string"];
    headers.retain(|h| !preamble_headers.contains(&h.as_str()));
    // Remove template instantiation headers (now in preamble)
    let mut tmpl_headers_to_skip: HashSet<String> = HashSet::new();
    for (spelling, inst) in template_instantiations {
        tmpl_headers_to_skip.insert(inst.header.clone());
        collect_template_arg_headers(spelling, known_headers, &mut tmpl_headers_to_skip, None);
    }
    headers.retain(|h| !tmpl_headers_to_skip.contains(h));

    headers.sort();
    headers.dedup();

    for header in &headers {
        writeln!(output, "#include <{}>", header).unwrap();
    }
    writeln!(output).unwrap();



    // Handle typedefs and destructors
    if !handle_types.is_empty() {
        writeln!(output, "// Handle type aliases").unwrap();
        for (inner_class, handle_name) in &handle_types {
            writeln!(output, "typedef opencascade::handle<{}> {};", inner_class, handle_name).unwrap();
        }
        writeln!(output).unwrap();

        writeln!(output, "// Handle type destructors").unwrap();
        for (inner_class, handle_name) in &handle_types {
            if own_class_names.contains(inner_class) {
                writeln!(output, "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}", handle_name, handle_name).unwrap();
            }
        }
        writeln!(output).unwrap();
    }

    // Wrapper functions for this group's classes
    for b in bindings {
        output.push_str(&super::bindings::emit_cpp_class(b));
    }

    // Free function wrappers
    generate_function_wrappers(&mut output, function_bindings);

    // Nested type destructors
    if !nested_types.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// Nested type and typedef type destructors").unwrap();
        for nt in nested_types {
            writeln!(output, "extern \"C\" void {ffi}_destructor({cpp}* self_) {{ delete self_; }}",
                ffi = nt.ffi_name, cpp = nt.cpp_name).unwrap();
        }
    }

    // Collection wrappers
    if !collections.is_empty() {
        let coll_vec: Vec<_> = collections.iter().copied().cloned().collect();
        output.push_str(&super::collections::generate_cpp_collections(&coll_vec));
    }

    output
}

/// Collect ALL OCCT headers needed for all classes
fn collect_all_required_headers(
    classes: &[&ParsedClass],
    known_headers: &HashSet<String>,
) -> Vec<String> {
    let mut headers = HashSet::new();

    for class in classes {
        // Add header for the class itself - use the actual source header
        // (class name doesn't always match header name, e.g. Extrema_GlobOptFuncCCC0 is in Extrema_GlobOptFuncCC.hxx)
        let source_header = &class.source_header;
        if known_headers.is_empty() || known_headers.contains(source_header) {
            headers.insert(source_header.clone());
        } else {
            // Fallback: try class_name.hxx 
            let class_header = format!("{}.hxx", class.name);
            if known_headers.contains(&class_header) {
                headers.insert(class_header);
            }
        }

        // Add headers for types used in methods
        for method in &class.methods {
            collect_type_headers(&method.return_type, &mut headers, known_headers);
            for param in &method.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }

        for method in &class.static_methods {
            collect_type_headers(&method.return_type, &mut headers, known_headers);
            for param in &method.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }
    }

    let mut result: Vec<_> = headers.into_iter().collect();
    result.sort();
    result
}
