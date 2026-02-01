//! Rust CXX bridge code generation
//!
//! Generates #[cxx::bridge] modules for each OCCT module with proper
//! type aliasing for cross-module references.

use crate::model::{Constructor, Method, ParsedClass, ParsedEnum, StaticMethod, Type};
use crate::module_graph::{CrossModuleType, Module};
use crate::type_mapping::{map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

/// Rust keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Convert a method name to a safe Rust identifier, adding underscore suffix for keywords
/// Note: CXX doesn't support raw identifiers (r#keyword), so we use a suffix instead
fn safe_method_name(name: &str) -> String {
    let snake_name = name.to_snake_case();
    if RUST_KEYWORDS.contains(&snake_name.as_str()) {
        format!("{}_", snake_name)  // Add trailing underscore for keywords
    } else {
        snake_name
    }
}

/// Generate Rust CXX bridge code for a module
///
/// Returns a TokenStream that can be converted to a string when writing to disk.
pub fn generate_module(
    module: &Module,
    classes: &[&ParsedClass],
    enums: &[&ParsedEnum],
    cross_module_types: &[CrossModuleType],
    all_enum_names: &HashSet<String>,
) -> TokenStream {
    let _module_name = format_ident!("{}", module.rust_name);
    // Use wrapper_ prefix to avoid collision with OCCT headers (e.g., gp.hxx)
    let include_file = format!("wrapper_{}.hxx", module.rust_name);

    // Build the set of classes defined in this module
    let module_classes: HashSet<String> = classes.iter().map(|c| c.name.clone()).collect();

    // Create type context for this module
    let type_ctx = TypeContext {
        current_module: &module.rust_name,
        module_classes: &module_classes,
        all_enums: all_enum_names,
    };

    // Generate cross-module type aliases
    let type_aliases = generate_type_aliases(cross_module_types);

    // Collect all referenced types that need opaque declarations
    let collected_types = collect_referenced_types(classes);
    let opaque_type_decls =
        generate_opaque_type_declarations(&collected_types, classes, cross_module_types);

    // Generate enum definitions (CXX shared enums)
    let enum_items = enums.iter().map(|e| generate_enum(e)).collect::<Vec<_>>();

    // Generate type declarations and methods for each class
    let class_items = classes
        .iter()
        .map(|class| generate_class(class, &type_ctx))
        .collect::<Vec<_>>();

    // Generate impl UniquePtr blocks for cross-module types
    let unique_ptr_impls = generate_unique_ptr_impls(classes);

    // Generate re-exports
    let re_exports = generate_re_exports(classes);

    // Assemble the module
    let tokens = quote! {
        //! Generated CXX bridge for OCCT module
        #![allow(dead_code)]
        #![allow(non_snake_case)]
        #![allow(clippy::missing_safety_doc)]

        #[cxx::bridge]
        pub(crate) mod ffi {
            // ========================
            // Shared enums
            // ========================
            #(#enum_items)*

            unsafe extern "C++" {
                include!(#include_file);

                // ========================
                // Cross-module type aliases
                // ========================
                #(#type_aliases)*

                // ========================
                // Referenced types (opaque)
                // ========================
                #(#opaque_type_decls)*

                // ========================
                // Module types and methods
                // ========================
                #(#class_items)*
            }

            #(#unique_ptr_impls)*
        }

        // Re-exports
        #(#re_exports)*
    };

    tokens
}

/// Types collected from class interfaces
struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve")
    classes: HashSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>)
    handles: HashSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
fn collect_referenced_types(classes: &[&ParsedClass]) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: HashSet::new(),
        handles: HashSet::new(),
    };

    for class in classes {
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
    match ty {
        Type::Class(name) => {
            collected.classes.insert(name.clone());
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

/// Generate opaque type declarations for referenced types not defined in this module
fn generate_opaque_type_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    cross_module_types: &[CrossModuleType],
) -> Vec<TokenStream> {
    // Get names of classes we're generating
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();

    // Get C++ names of cross-module type aliases (they're already declared)
    let aliased_types: HashSet<_> = cross_module_types.iter().map(|t| &t.cpp_name).collect();

    let mut declarations = Vec::new();

    // Generate declarations for class types
    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) {
            continue;
        }
        if aliased_types.contains(type_name) {
            continue;
        }
        if is_primitive_type(type_name) {
            continue;
        }

        let ident = format_ident!("{}", type_name);
        declarations.push(quote! {
            /// Referenced type from C++
            #[cxx_name = #type_name]
            type #ident;
        });
    }

    // Generate declarations for Handle types
    // Use a HashSet to track which Handle types we've already declared
    let mut declared_handles = std::collections::HashSet::new();
    for inner_class in &collected_types.handles {
        // Use the full class name to avoid collisions (e.g., Geom_Curve vs Geom2d_Curve)
        let handle_name = format!("Handle{}", inner_class.replace("_", ""));
        if declared_handles.contains(&handle_name) {
            continue;
        }
        declared_handles.insert(handle_name.clone());
        let ident = format_ident!("{}", handle_name);
        declarations.push(quote! {
            /// Handle to OCCT object
            #[cxx_name = #handle_name]
            type #ident;
        });
    }

    declarations
}

/// Extract short class name without module prefix
fn extract_short_class_name(name: &str) -> String {
    if let Some(underscore_pos) = name.find('_') {
        name[underscore_pos + 1..].to_string()
    } else {
        name.to_string()
    }
}

/// Check if a type name is a primitive (not an OCCT class)
fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        "bool" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "char"
    )
}

/// Generate type alias declarations for cross-module types
fn generate_type_aliases(cross_module_types: &[CrossModuleType]) -> Vec<TokenStream> {
    use crate::type_mapping::safe_short_name;

    cross_module_types
        .iter()
        .map(|cmt| {
            let cpp_name = &cmt.cpp_name;
            let source_module = format_ident!("{}", cmt.source_module);
            let doc_comment = format!("{} from {} module", cmt.rust_name, cmt.source_module);

            // For enums, use the full C++ name (enums are declared with full names)
            // For classes, use the short name (possibly with trailing underscore for reserved names)
            let rust_name_in_source = if cmt.is_enum {
                format_ident!("{}", cpp_name)
            } else {
                let internal_name = safe_short_name(&cmt.rust_name);
                format_ident!("{}", internal_name)
            };

            // Generate: type gp_Vec = crate::gp::ffi::Vec_;
            let alias_path = quote! { crate::#source_module::ffi::#rust_name_in_source };
            let cpp_name_ident = format_ident!("{}", cpp_name);

            quote! {
                #[doc = #doc_comment]
                type #cpp_name_ident = #alias_path;
            }
        })
        .collect()
}

/// Generate CXX shared enum declaration
fn generate_enum(enum_decl: &ParsedEnum) -> TokenStream {
    let enum_name = format_ident!("{}", enum_decl.name);

    // Doc comment
    let doc = enum_decl
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    // Generate variants
    let variants = enum_decl.variants.iter().map(|v| {
        let variant_name = format_ident!("{}", v.name);
        let variant_doc = v
            .comment
            .as_ref()
            .map(|c| quote! { #[doc = #c] })
            .unwrap_or_default();

        quote! {
            #variant_doc
            #variant_name
        }
    });

    quote! {
        #doc
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum #enum_name {
            #(#variants),*
        }
    }
}

/// Generate CXX declarations for a single class
fn generate_class(class: &ParsedClass, ctx: &TypeContext) -> TokenStream {
    let cpp_name = &class.name;
    // Use safe short name (e.g., "Pnt" instead of "gp_Pnt", or "GpVec" for "Vec")
    let short_name = class.safe_short_name();
    let rust_name = format_ident!("{}", short_name);

    // Doc comment
    let doc = class
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    // Type declaration - use short Rust name, map to full C++ name
    let type_decl = quote! {
        #doc
        #[cxx_name = #cpp_name]
        type #rust_name;
    };

    // Constructor functions
    let ctors = generate_constructors(class, ctx);

    // Instance methods (that can be called directly)
    let methods = generate_methods(class, ctx);

    // Wrapper methods (free functions for methods returning by value)
    let wrapper_methods = generate_wrapper_methods(class, ctx);

    // Static methods
    let static_methods = generate_static_methods(class, ctx);

    // Section header as doc comment (will render as /// in output)
    let section_line = format!(" ======================== {} ========================", cpp_name);

    quote! {
        #[doc = #section_line]
        #type_decl

        #(#ctors)*
        #(#methods)*
        #(#wrapper_methods)*
        #(#static_methods)*
    }
}

/// Generate constructor function declarations
fn generate_constructors(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    // Group constructors by overload suffix to handle naming conflicts
    let mut ctor_names: HashMap<String, usize> = HashMap::new();

    class
        .constructors
        .iter()
        .filter(|ctor| {
            // Skip constructors with class/handle parameters passed by value (not supported by CXX)
            !ctor.params.iter().any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_)))
        })
        .map(|ctor| {
            let base_suffix = ctor.overload_suffix();
            let suffix = if base_suffix.is_empty() {
                "ctor".to_string()
            } else {
                format!("ctor{}", base_suffix)
            };

            // Handle duplicate suffixes by adding a number
            let count = ctor_names.entry(suffix.clone()).or_insert(0);
            *count += 1;
            let final_suffix = if *count > 1 {
                format!("{}_{}", suffix, count)
            } else {
                suffix
            };

            generate_constructor(class, ctor, &final_suffix, ctx)
        })
        .collect()
}

/// Generate a single constructor function
fn generate_constructor(class: &ParsedClass, ctor: &Constructor, suffix: &str, ctx: &TypeContext) -> TokenStream {
    let cpp_name = &class.name;
    let short_name = class.safe_short_name();
    let rust_type = format_ident!("{}", short_name);
    let cpp_wrapper_name = format!("{}_{}", cpp_name, suffix);
    let rust_fn_name = format_ident!(
        "{}_{}",
        short_name,
        suffix.to_snake_case()
    );

    // Parameters - use original C++ names
    let params = ctor.params.iter().map(|p| {
        let name = format_ident!("{}", &p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    // Doc comment
    let doc = ctor
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    quote! {
        #doc
        #[cxx_name = #cpp_wrapper_name]
        fn #rust_fn_name(#(#params),*) -> UniquePtr<#rust_type>;
    }
}

/// Check if a method returns a class type by value (needs wrapper function)
fn needs_wrapper_function(method: &Method) -> bool {
    method
        .return_type
        .as_ref()
        .map(|ty| ty.is_class() || ty.is_handle())
        .unwrap_or(false)
}

/// Check if a method has parameters that would be passed by value (not supported by CXX for opaque types)
fn has_unsupported_by_value_params(method: &Method) -> bool {
    method.params.iter().any(|p| {
        // Class types passed directly (not by reference) are not supported
        matches!(&p.ty, Type::Class(_) | Type::Handle(_))
    })
}

/// Generate instance method declarations
fn generate_methods(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    // Group methods by name to handle overloads
    let mut method_counts: HashMap<String, usize> = HashMap::new();

    class
        .methods
        .iter()
        .filter_map(|method| {
            // Skip methods that need wrapper functions - they're generated separately
            if needs_wrapper_function(method) {
                return None;
            }
            
            // Skip methods with class/handle parameters passed by value (not supported by CXX)
            if has_unsupported_by_value_params(method) {
                return None;
            }

            // Skip methods with unbindable types (streams, void pointers, etc.)
            if method.has_unbindable_types() {
                return None;
            }

            let count = method_counts.entry(method.name.clone()).or_insert(0);
            *count += 1;

            // Generate a suffix based on parameters for overloaded methods
            let overload_suffix = generate_overload_suffix(method, *count, class);

            generate_method_with_suffix(class, method, &overload_suffix, ctx)
        })
        .collect()
}

/// Generate free function declarations for methods returning class types by value
fn generate_wrapper_methods(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    // Group methods by name to handle overloads
    let mut method_counts: HashMap<String, usize> = HashMap::new();

    class
        .methods
        .iter()
        .filter_map(|method| {
            // Only process methods that need wrapper functions
            if !needs_wrapper_function(method) {
                return None;
            }
            
            // Skip methods with class/handle parameters passed by value (not supported by CXX)
            if has_unsupported_by_value_params(method) {
                return None;
            }

            // Skip methods with unbindable types (streams, void pointers, etc.)
            if method.has_unbindable_types() {
                return None;
            }

            let count = method_counts.entry(method.name.clone()).or_insert(0);
            *count += 1;

            // Generate a suffix based on parameters for overloaded methods
            let overload_suffix = generate_overload_suffix_for_wrappers(method, *count, class);

            generate_wrapper_method(class, method, &overload_suffix, ctx)
        })
        .collect()
}

/// Generate a suffix for overloaded wrapper methods
fn generate_overload_suffix_for_wrappers(method: &Method, count: usize, class: &ParsedClass) -> String {
    // Count how many wrapper methods have this name
    let same_name_count = class
        .methods
        .iter()
        .filter(|m| m.name == method.name && needs_wrapper_function(m))
        .count();

    if same_name_count <= 1 {
        return String::new();
    }

    // Generate suffix from first distinguishing parameter type
    if let Some(param) = method.params.first() {
        let suffix = param.ty.short_name();
        if count > 1 {
            format!("_{}_{}", suffix, count)
        } else {
            format!("_{}", suffix)
        }
    } else if count > 1 {
        format!("_{}", count)
    } else {
        String::new()
    }
}

/// Generate a wrapper method (free function that calls C++ wrapper)
fn generate_wrapper_method(
    class: &ParsedClass,
    method: &Method,
    suffix: &str,
    ctx: &TypeContext,
) -> Option<TokenStream> {
    let short_name = class.safe_short_name();
    let rust_type = format_ident!("{}", short_name);

    // C++ wrapper function name: ClassName_MethodName
    let cpp_wrapper_name = format!("{}_{}", class.name, method.name);

    // Rust function name: ClassName_method_name with optional suffix
    let rust_name = if suffix.is_empty() {
        format_ident!("{}_{}", short_name, method.name.to_snake_case())
    } else {
        format_ident!(
            "{}_{}{}",
            short_name,
            method.name.to_snake_case(),
            suffix.to_snake_case()
        )
    };

    // Self parameter (passed by reference as first param)
    // Use "self_" not "this" because "this" is a C++ keyword
    let self_param = if method.is_const {
        quote! { self_: &#rust_type }
    } else {
        quote! { self_: Pin<&mut #rust_type> }
    };

    // Other parameters - use original C++ names
    let params = method.params.iter().map(|p| {
        let name = format_ident!("{}", &p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    // Return type
    let return_type = method.return_type.as_ref().map(|ty| {
        let mapping = map_return_type_in_context(ty, ctx);
        let ty_tokens: TokenStream = mapping.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    // Doc comment
    let doc = method
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    Some(quote! {
        #doc
        #[cxx_name = #cpp_wrapper_name]
        fn #rust_name(#self_param, #(#params),*) #return_type;
    })
}

/// Generate a suffix for overloaded methods based on their parameter types
fn generate_overload_suffix(method: &Method, count: usize, class: &ParsedClass) -> String {
    // Count how many methods have this name
    let same_name_count = class.methods.iter().filter(|m| m.name == method.name).count();

    if same_name_count <= 1 {
        // No overloads, no suffix needed
        return String::new();
    }

    // Generate suffix from first distinguishing parameter type
    if let Some(param) = method.params.first() {
        let suffix = param.ty.short_name();
        // For multiple overloads with same first param, add count
        if count > 1 {
            format!("_{}_{}", suffix, count)
        } else {
            format!("_{}", suffix)
        }
    } else if count > 1 {
        format!("_{}", count)
    } else {
        String::new()
    }
}

/// Generate a single instance method with optional suffix for overloads
fn generate_method_with_suffix(
    class: &ParsedClass,
    method: &Method,
    suffix: &str,
    ctx: &TypeContext,
) -> Option<TokenStream> {
    let cpp_name = &method.name;
    let base_name = safe_method_name(&method.name);
    let rust_name_str = if suffix.is_empty() {
        base_name
    } else {
        format!("{}{}", base_name, suffix.to_snake_case())
    };
    let rust_name = format_ident!("{}", rust_name_str);
    let short_name = class.safe_short_name();
    let rust_type = format_ident!("{}", short_name);

    // Self parameter
    let self_param = if method.is_const {
        quote! { self: &#rust_type }
    } else {
        quote! { self: Pin<&mut #rust_type> }
    };

    // Other parameters - use original C++ names
    let params = method.params.iter().map(|p| {
        let name = format_ident!("{}", &p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    // Return type
    let return_type = method.return_type.as_ref().map(|ty| {
        let mapping = map_return_type_in_context(ty, ctx);
        let ty_tokens: TokenStream = mapping.rust_type.parse().unwrap_or_else(|_| quote! { () });

        // Methods that return by value need wrappers
        if ty.is_class() || ty.is_handle() {
            // This will require a C++ wrapper - mark it
            quote! { -> #ty_tokens }
        } else {
            quote! { -> #ty_tokens }
        }
    });

    // Doc comment
    let doc = method
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    Some(quote! {
        #doc
        #[cxx_name = #cpp_name]
        fn #rust_name(#self_param, #(#params),*) #return_type;
    })
}

/// Generate a single instance method
#[allow(dead_code)]
fn generate_method(class: &ParsedClass, method: &Method, ctx: &TypeContext) -> Option<TokenStream> {
    generate_method_with_suffix(class, method, "", ctx)
}

/// Generate static method declarations
fn generate_static_methods(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    // Track method counts for overload disambiguation
    let mut method_counts: HashMap<String, usize> = HashMap::new();
    
    class
        .static_methods
        .iter()
        .filter_map(|method| {
            let count = method_counts.entry(method.name.clone()).or_insert(0);
            *count += 1;
            generate_static_method(class, method, *count, ctx)
        })
        .collect()
}

/// Generate a single static method
fn generate_static_method(
    class: &ParsedClass,
    method: &StaticMethod,
    _occurrence: usize,
    ctx: &TypeContext,
) -> Option<TokenStream> {
    // Skip methods with stream parameters - CXX can't bind C++ streams
    if method.has_unbindable_types() {
        return None;
    }

    // Skip static methods that return bare references - these need lifetimes
    // (e.g., get_type_descriptor() -> &Handle<Standard_Type>)
    if method.return_type.as_ref().map(|t| t.is_reference()).unwrap_or(false) {
        return None;
    }

    let _cpp_name = &method.name;
    let short_name = class.safe_short_name();
    
    // Count how many static methods have this name to determine if suffix is needed
    let same_name_count = class
        .static_methods
        .iter()
        .filter(|m| m.name == method.name)
        .count();
    
    // Generate overload suffix based on parameter types
    let overload_suffix = if same_name_count > 1 {
        method.overload_suffix()
    } else {
        String::new()
    };
    
    let cpp_wrapper_name = format!("{}_{}{}", class.name, method.name, overload_suffix);
    let rust_name = format_ident!(
        "{}_{}{}", 
        short_name,
        method.name.to_snake_case(),
        overload_suffix
    );

    // Parameters - use original C++ names
    let params = method.params.iter().map(|p| {
        let name = format_ident!("{}", &p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    // Return type
    let return_type = method.return_type.as_ref().map(|ty| {
        let mapping = map_return_type_in_context(ty, ctx);
        let ty_tokens: TokenStream = mapping.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    // Doc comment
    let doc = method
        .comment
        .as_ref()
        .map(|c| quote! { #[doc = #c] })
        .unwrap_or_default();

    Some(quote! {
        #doc
        #[cxx_name = #cpp_wrapper_name]
        fn #rust_name(#(#params),*) #return_type;
    })
}

/// Generate impl UniquePtr blocks for types defined in this module
fn generate_unique_ptr_impls(classes: &[&ParsedClass]) -> Vec<TokenStream> {
    classes
        .iter()
        .map(|class| {
            let rust_type = format_ident!("{}", class.safe_short_name());
            quote! {
                impl UniquePtr<#rust_type> {}
            }
        })
        .collect()
}

/// Generate re-export statements and impl blocks with constructor methods
fn generate_re_exports(classes: &[&ParsedClass]) -> Vec<TokenStream> {
    use crate::type_mapping::is_reserved_name;
    
    // Build set of class names owned by this module (use short names in ffi)
    let owned_classes: HashSet<String> = classes.iter().map(|c| c.name.clone()).collect();
    
    let mut exports = Vec::new();

    for class in classes {
        let short_name = class.short_name();
        let safe_name = class.safe_short_name();
        
        if is_reserved_name(short_name) {
            // For reserved names, re-export with alias: pub use ffi::Vec_ as Vec;
            let ffi_type = format_ident!("{}", safe_name);  // Vec_
            let pub_type = format_ident!("{}", short_name); // Vec
            exports.push(quote! {
                pub use ffi::#ffi_type as #pub_type;
            });
        } else {
            let rust_type = format_ident!("{}", short_name);
            exports.push(quote! {
                pub use ffi::#rust_type;
            });
        }

        // Generate impl block with constructor methods if there are any constructors
        if !class.constructors.is_empty() {
            let rust_type = format_ident!("{}", short_name);
            
            // Track constructor names to handle duplicates
            let mut ctor_names: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            
            // Filter constructors the same way as generate_constructors does
            let methods: Vec<TokenStream> = class.constructors.iter()
                .filter(|ctor| {
                    // Skip constructors with class/handle parameters passed by value (not supported by CXX)
                    !ctor.params.iter().any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_)))
                })
                .map(|ctor| {
                let base_suffix = ctor.overload_suffix();
                let method_name = if base_suffix.is_empty() {
                    "new".to_string()
                } else {
                    format!("new{}", base_suffix)
                };
                
                // Handle duplicate method names by adding a number
                let count = ctor_names.entry(method_name.clone()).or_insert(0);
                *count += 1;
                let final_method_name = if *count > 1 {
                    format!("{}_{}", method_name, count)
                } else {
                    method_name
                };
                let method_ident = format_ident!("{}", final_method_name.to_snake_case());
                
                // Generate the ffi function name (must match what generate_constructors produces)
                let ffi_suffix = if base_suffix.is_empty() {
                    "ctor".to_string()
                } else {
                    format!("ctor{}", base_suffix)
                };
                let ffi_fn_name = format_ident!(
                    "{}_{}",
                    safe_name,
                    ffi_suffix.to_snake_case()
                );
                
                // Generate parameter list for method signature - keep original C++ names
                // For types in the ffi module, we need to use the correct name:
                // - Owned classes use short names (Pnt, Vec_, etc.)
                // - Opaque/external types use full C++ names (gp_Trsf2d, etc.)
                let params: Vec<TokenStream> = ctor.params.iter().map(|p| {
                    let name = format_ident!("{}", &p.name);
                    let ty_str = type_to_ffi_string(&p.ty, &owned_classes);
                    let ty_tokens: TokenStream = ty_str.parse().unwrap_or_else(|_| quote! { () });
                    quote! { #name: #ty_tokens }
                }).collect();
                
                // Generate argument list for calling ffi function
                let args: Vec<TokenStream> = ctor.params.iter().map(|p| {
                    let name = format_ident!("{}", &p.name);
                    quote! { #name }
                }).collect();
                
                // Doc comment
                let doc = ctor
                    .comment
                    .as_ref()
                    .map(|c| quote! { #[doc = #c] })
                    .unwrap_or_default();
                
                quote! {
                    #doc
                    pub fn #method_ident(#(#params),*) -> cxx::UniquePtr<Self> {
                        ffi::#ffi_fn_name(#(#args),*)
                    }
                }
            }).collect();
            
            exports.push(quote! {
                impl #rust_type {
                    #(#methods)*
                }
            });
        }
    }

    exports
}

/// Convert a Type to a string suitable for use in impl blocks outside ffi.
/// owned_classes contains the full C++ names of classes defined in this module.
fn type_to_ffi_string(ty: &Type, owned_classes: &HashSet<String>) -> String {
    use crate::type_mapping::is_reserved_name;
    
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::ConstRef(inner) => {
            let inner_str = type_to_ffi_string(inner, owned_classes);
            format!("&{}", inner_str)
        }
        Type::MutRef(inner) => {
            let inner_str = type_to_ffi_string(inner, owned_classes);
            format!("&mut {}", inner_str)
        }
        Type::ConstPtr(inner) => {
            let inner_str = type_to_ffi_string(inner, owned_classes);
            format!("*const {}", inner_str)
        }
        Type::MutPtr(inner) => {
            let inner_str = type_to_ffi_string(inner, owned_classes);
            format!("*mut {}", inner_str)
        }
        Type::Handle(name) => {
            // Use full class name (without underscores) to match declaration and avoid collisions
            // e.g., "Geom_Surface" -> "HandleGeomSurface"
            format!("ffi::Handle{}", name.replace("_", ""))
        }
        Type::Class(name) => {
            if owned_classes.contains(name) {
                // Owned class - use short name (with reserved name handling)
                let short_name = if let Some(underscore_pos) = name.find('_') {
                    &name[underscore_pos + 1..]
                } else {
                    name.as_str()
                };
                let safe_name = if is_reserved_name(short_name) {
                    format!("{}_", short_name)
                } else {
                    short_name.to_string()
                };
                format!("ffi::{}", safe_name)
            } else {
                // External/opaque type - use full C++ name as declared in ffi
                format!("ffi::{}", name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Constructor, Method, Param};

    #[test]
    fn test_generate_simple_class() {
        let class = ParsedClass {
            name: "gp_Pnt".to_string(),
            module: "gp".to_string(),
            comment: Some("A 3D point".to_string()),
            constructors: vec![Constructor {
                comment: Some("Default constructor".to_string()),
                params: vec![],
            }],
            methods: vec![Method {
                name: "X".to_string(),
                comment: Some("Returns the X coordinate".to_string()),
                is_const: true,
                params: vec![],
                return_type: Some(Type::F64),
            }],
            static_methods: vec![],
            is_handle_type: false,
        };

        let module = Module::new("gp");
        let all_enums = HashSet::new();
        let output = generate_module(&module, &[&class], &[], &[], &all_enums);
        assert!(output.contains("gp_Pnt"));
        assert!(output.contains("Pnt"));
    }
}
