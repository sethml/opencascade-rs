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
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address"
                ) {
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

/// Generate wrappers for all namespace-level free functions from pre-computed FunctionBindings
fn generate_function_wrappers(
    output: &mut String,
    function_bindings: &[super::bindings::FunctionBinding],
    known_headers: &HashSet<String>,
) {
    if function_bindings.is_empty() {
        return;
    }

    // Group functions by namespace
    let mut by_namespace: std::collections::HashMap<&str, Vec<&super::bindings::FunctionBinding>> =
        std::collections::HashMap::new();
    for func in function_bindings {
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

        // Collect unique headers for this namespace
        let mut extra_headers: HashSet<String> = HashSet::new();
        let ns_header = format!("{}.hxx", namespace);
        if known_headers.is_empty() || known_headers.contains(&ns_header) {
            extra_headers.insert(ns_header);
        }
        for func in namespace_functions {
            for h in &func.cpp_headers {
                extra_headers.insert(h.clone());
            }
        }

        let mut sorted_headers: Vec<_> = extra_headers.into_iter().collect();
        sorted_headers.sort();
        for header in &sorted_headers {
            writeln!(output, "#include <{}>", header).unwrap();
        }

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
                if rt.enum_cpp_name.is_some() {
                    writeln!(
                        output,
                        "extern \"C\" {} {}({}) {{ return static_cast<int32_t>({}); }}",
                        rt.cpp_type, wrapper_name, params_str, call
                    ).unwrap();
                } else if rt.needs_unique_ptr {
                    // Return type is the base C++ type; wrapper returns pointer
                    // cpp_type for unique_ptr returns is the base type (e.g. "gp_Pnt")
                    // but the FFI returns a pointer to it
                    let base_type = &rt.cpp_type;
                    writeln!(
                        output,
                        "extern \"C\" {0}* {1}({2}) {{ return new {0}({3}); }}",
                        base_type, wrapper_name, params_str, call
                    ).unwrap();
                } else {
                    writeln!(
                        output,
                        "extern \"C\" {} {}({}) {{ return {}; }}",
                        rt.cpp_type, wrapper_name, params_str, call
                    ).unwrap();
                }
            } else {
                writeln!(
                    output,
                    "extern \"C\" void {}({}) {{ {}; }}",
                    wrapper_name, params_str, call
                ).unwrap();
            }
        }
        writeln!(output).unwrap();
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
    writeln!(output, "#include <cstdint>").unwrap();
    writeln!(output, "#include <new>").unwrap();
    writeln!(output).unwrap();

    // Collect ALL headers needed
    let mut headers = collect_all_required_headers(all_classes, known_headers);
    // Add headers needed for template instantiations
    for inst in template_instantiations.values() {
        // OCCT headers (.hxx) must be in known_headers; standard library headers
        // (no extension, e.g., "utility", "memory") are always available.
        let is_std_header = !inst.header.contains('.');
        if (is_std_header || known_headers.contains(&inst.header)) && !headers.contains(&inst.header) {
            headers.push(inst.header.clone());
        }
    }
    
    for header in &headers {
        writeln!(output, "#include <{}>", header).unwrap();
    }
    writeln!(output).unwrap();

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
    generate_function_wrappers(&mut output, function_bindings, known_headers);

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
