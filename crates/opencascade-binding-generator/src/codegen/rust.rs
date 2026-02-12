//! Rust CXX bridge code generation
//!
//! Generates a unified #[cxx::bridge] module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{Constructor, Method, ParsedClass, ParsedFunction, StaticMethod, Type};

use crate::resolver;
use crate::type_mapping;
use crate::type_mapping::{map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeSet, HashMap, HashSet};

/// Rust keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Check if a type uses an unknown class or handle given the TypeContext.
/// For unified mode (when handle_able_classes is set), uses stricter checking for Handle types.
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a parameter type uses an unknown Handle (but allow opaque Class types).
/// This is used for constructor filtering where opaque classes are acceptable
/// (they'll be declared as opaque types in the FFI), but undeclared Handles are not.
fn param_uses_unknown_handle(ty: &Type, handle_able_classes: &HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => param_uses_unknown_handle(inner, handle_able_classes),
        _ => false,
    }
}

/// Check if a type is or contains `const char*` which requires special wrapper handling
/// and can't be directly bound in CXX static methods
fn type_is_cstring(ty: &Type) -> bool {
    match ty {
        Type::ConstPtr(inner) => matches!(inner.as_ref(), Type::Class(name) if name == "char"),
        Type::ConstRef(inner) | Type::MutRef(inner) => type_is_cstring(inner),
        _ => false,
    }
}

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Convert TokenStream to formatted string with basic newlines
fn format_tokenstream(tokens: &TokenStream) -> String {
    let code = tokens.to_string();

    // Add basic formatting by inserting newlines in key places to make it more readable
    // Note: Don't add newlines before "impl " - it would break impl blocks inside extern "C++"
    let formatted = code
        .replace(" # ! [", "\n#![")
        .replace(" # [cxx :: bridge]", "\n#[cxx::bridge]")
        .replace(" pub (crate) mod ffi {", "\npub(crate) mod ffi {")
        .replace(" unsafe extern \"C++\" {", "\n    unsafe extern \"C++\" {")
        .replace(" } }", "\n    }\n}")
        .replace(" pub use ffi ::", "\npub use ffi::");

    formatted
}

/// Post-process generated code after rustfmt formatting:
/// 1. Convert #[doc = "..."] attributes to /// ... comments for better readability
/// 2. Convert SECTION: markers to regular // comments
/// 3. Convert BLANK_LINE markers to actual blank lines
pub fn postprocess_generated_code(code: &str) -> String {
    use regex::Regex;
    
    // First, convert all #[doc = "..."] and #[doc = r"..."] to /// ... format
    // This regex captures leading whitespace and the content
    // We use (?s) to make . match newlines
    // The r? makes the raw string prefix optional
    // The content pattern matches: non-quote chars OR escaped quotes (backslash followed by quote)
    let doc_attr_re = Regex::new(r#"(?s)([ \t]*)#\[doc = r?"((?:[^"\\]|\\.)*)"\]"#).unwrap();
    let result = doc_attr_re.replace_all(code, |caps: &regex::Captures| {
        let indent = &caps[1];
        let content = &caps[2];
        // Handle empty doc comments
        if content.is_empty() {
            format!("{}///", indent)
        } else {
            // Handle escape sequences from quote! macro:
            // - \n becomes actual newline
            // - \" becomes "
            // - \\ becomes \
            let unescaped = content
                .replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            unescaped
                .lines()
                .map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        format!("{}///", indent)
                    } else {
                        format!("{}/// {}", indent, trimmed)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    });
    
    // Convert BLANK_LINE markers to actual blank lines
    // Pattern: /// BLANK_LINE (possibly with trailing whitespace) -> empty line
    let blank_re = Regex::new(r#"[ \t]*/// BLANK_LINE[ \t]*\n"#).unwrap();
    let result = blank_re.replace_all(&result, "\n");
    
    // Now convert SECTION: markers to regular comments (not doc comments)
    // Pattern: /// SECTION: ... -> // ...
    let section_re = Regex::new(r#"/// SECTION: ([^\n]*)"#).unwrap();
    let result = section_re.replace_all(&result, "// $1");
    
    result.into_owned()
}

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

/// Convert a parameter name to a safe Rust identifier
/// Rust keywords are renamed with trailing underscore (e.g., "where" -> "where_")
/// because CXX doesn't support r#keyword syntax in FFI declarations
fn safe_param_ident(name: &str) -> proc_macro2::Ident {
    if RUST_KEYWORDS.contains(&name) {
        format_ident!("{}_", name)
    } else {
        format_ident!("{}", name)
    }
}

/// Generate Rust CXX bridge code for a module
///
/// Returns a String with properly formatted documentation.

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
        
        // NOTE: Inherited method type collection disabled - inherited methods are not generated
        // due to CXX method pointer verification limitations. See generate_inherited_methods().
        // The code below is kept for reference in case we implement wrapper functions for
        // inherited methods in the future.
        /*
        // From inherited methods (methods from base classes that will be generated)
        // We need to collect types from these too since they'll appear in the generated code
        let (ancestor_methods, _overridden_in_hierarchy) = collect_ancestor_methods(class, all_classes);
        let existing_methods: HashSet<String> = class.methods.iter().map(|m| m.name.clone()).collect();
        let mut seen_methods: HashSet<String> = HashSet::new();
        
        for method in ancestor_methods {
            // Skip if this class already declares this method directly (override)
            if existing_methods.contains(&method.name) {
                continue;
            }
            
            // Skip if any class in the hierarchy has overridden this method (could be protected)
            // We only want to inherit from the direct ancestor that first declares the method
            if class.all_method_names.contains(&method.name) {
                continue;
            }
            
            // Skip if we've already seen this method
            if seen_methods.contains(&method.name) {
                continue;
            }
            
            // Skip methods with unbindable types
            if method.has_unbindable_types() {
                continue;
            }
            
            // Skip methods with class/handle params by value (not supported by CXX)
            if method.params.iter().any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_))) {
                continue;
            }
            
            // Skip methods returning by value (would need wrapper - TODO)
            if let Some(ref ret_ty) = method.return_type {
                if matches!(ret_ty, Type::Class(_) | Type::Handle(_)) {
                    continue;
                }
            }
            
            // Skip const methods that return mutable references (CXX doesn't allow this)
            // This must match the filter in generate_inherited_methods
            if method.is_const {
                if let Some(ref ret_ty) = method.return_type {
                    if matches!(ret_ty, Type::MutRef(_)) {
                        continue;
                    }
                }
            }
            
            seen_methods.insert(method.name.clone());
            
            // Collect types from this inherited method
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
        */
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

// Filter functions - delegate to centralized implementations in resolver module
// Note: We keep these wrappers for API compatibility within this module

fn method_uses_enum(method: &Method, all_enums: &HashSet<String>) -> bool {
    resolver::method_uses_enum(method, all_enums)
}

fn constructor_uses_enum(ctor: &Constructor, all_enums: &HashSet<String>) -> bool {
    resolver::constructor_uses_enum(ctor, all_enums)
}

fn static_method_uses_enum(method: &StaticMethod, all_enums: &HashSet<String>) -> bool {
    resolver::static_method_uses_enum(method, all_enums)
}

fn function_uses_enum(func: &ParsedFunction, all_enums: &HashSet<String>) -> bool {
    resolver::function_uses_enum(func, all_enums)
}

fn needs_wrapper_function(method: &Method, all_enums: &std::collections::HashSet<String>) -> bool {
    // Methods with const char* parameters need wrappers to convert from rust::Str
    if method.params.iter().any(|p| p.ty.is_c_string()) {
        return true;
    }
    
    // Methods returning const char* need wrappers to convert to rust::String
    if method.return_type.as_ref().map(|t| t.is_c_string()).unwrap_or(false) {
        return true;
    }
    
    method
        .return_type
        .as_ref()
        .map(|ty| {
            // Check if it's a class or handle
            let is_class_or_handle = ty.is_class() || ty.is_handle();
            // But exclude enums - they don't need wrappers
            let is_enum = match ty {
                Type::Class(name) => all_enums.contains(name),
                _ => false,
            };
            is_class_or_handle && !is_enum
        })
        .unwrap_or(false)
}

// Additional filter functions - delegate to centralized implementations in resolver module

/// Check if a method has parameters that would be passed by value (not supported by CXX for opaque types)
fn has_unsupported_by_value_params(method: &Method) -> bool {
    resolver::method_has_unsupported_by_value_params(method).is_some()
}

/// Check if a const method returns a mutable reference (not allowed by CXX)
/// CXX requires &mut self when returning &mut, but C++ allows const methods to return non-const refs
fn has_const_mut_return_mismatch(method: &Method) -> bool {
    resolver::has_const_mut_return_mismatch(method)
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
) -> String {
    // Build sets for type context
    let all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
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

    // Generate type declarations and methods for each class
    // Skip classes with protected destructors - CXX can't handle them
    // Skip collection types - they're generated separately
    let class_items: Vec<TokenStream> = all_classes
        .iter()
        .filter(|class| !class.has_protected_destructor)
        .filter(|class| !collection_type_names.contains(&class.name))
        .map(|class| generate_unified_class(class, &type_ctx, symbol_table))
        .collect();

    // Generate namespace-level free functions
    let function_items = generate_unified_functions(all_functions, &type_ctx);

    // Generate Handle type declarations
    let handle_decls = generate_unified_handle_declarations(all_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(&all_classes.iter().cloned().collect::<Vec<_>>());
    let opaque_type_decls = generate_unified_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
    );

    // Generate UniquePtr impl blocks
    let unique_ptr_impls = generate_unified_unique_ptr_impls(all_classes);

    // Generate the file header
    let file_header = generate_unified_file_header(all_headers);

    // Build section headers
    let class_section = if !class_items.is_empty() {
        quote! {
            #[doc = " BLANK_LINE"]
            #[doc = " SECTION: ========================"]
            #[doc = " SECTION: All types and methods"]
            #[doc = " SECTION: ========================"]
            #[doc = " BLANK_LINE"]
            #(#class_items)*
        }
    } else {
        quote! {}
    };

    let function_section = if !function_items.is_empty() {
        quote! {
            #[doc = " BLANK_LINE"]
            #[doc = " SECTION: ========================"]
            #[doc = " SECTION: Free functions"]
            #[doc = " SECTION: ========================"]
            #[doc = " BLANK_LINE"]
            #(#function_items)*
        }
    } else {
        quote! {}
    };

    let handle_section = if !handle_decls.is_empty() {
        quote! {
            #[doc = " BLANK_LINE"]
            #[doc = " SECTION: ========================"]
            #[doc = " SECTION: Handle types"]
            #[doc = " SECTION: ========================"]
            #[doc = " BLANK_LINE"]
            #(#handle_decls)*
        }
    } else {
        quote! {}
    };

    let opaque_section = if !opaque_type_decls.is_empty() {
        quote! {
            #[doc = " BLANK_LINE"]
            #[doc = " SECTION: ========================"]
            #[doc = " SECTION: Referenced types (opaque)"]
            #[doc = " SECTION: ========================"]
            #[doc = " BLANK_LINE"]
            #(#opaque_type_decls)*
        }
    } else {
        quote! {}
    };

    // Assemble the module
    let tokens = quote! {
        #![allow(dead_code)]
        #![allow(non_snake_case)]
        #![allow(clippy::missing_safety_doc)]

        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!("wrappers.hxx");

                #handle_section
                #class_section
                #function_section
                #opaque_section
            }

            #(#unique_ptr_impls)*
        }
    };

    let mut tokens_string = format_tokenstream(&tokens);

    // Insert collection FFI code if there are collections
    if !collections.is_empty() {
        let (coll_type_aliases, coll_ffi_decls) =
            super::collections::generate_unified_rust_ffi_collections(collections);

        // Insert collection code into the extern C++ block
        if let Some(impl_unique_pos) = tokens_string.find("impl UniquePtr") {
            let search_region = &tokens_string[..impl_unique_pos];
            if let Some(closing_brace) = search_region.rfind('}') {
                let collection_code = format!("{}{}\n", coll_type_aliases, coll_ffi_decls);
                tokens_string.insert_str(closing_brace, &collection_code);
            }
        }

        // Append collection impl blocks
        let coll_impls = super::collections::generate_unified_rust_impl_collections(collections);
        tokens_string.push_str("\n");
        tokens_string.push_str(&coll_impls);
    }

    // Add re-export of all FFI types
    tokens_string.push_str("\n// Re-export all FFI types to crate::ffi namespace\n");
    tokens_string.push_str("pub use ffi::*;\n");

    format!("{}\n\n{}", file_header, tokens_string)
}

/// Generate file header for unified ffi.rs
fn generate_unified_file_header(headers: &[String]) -> String {
    let header_count = headers.len();
    format!(
        "//! Unified CXX bridge for OpenCASCADE\n//!\n//! This file was automatically generated by opencascade-binding-generator\n//! from {} OCCT headers.\n//!\n//! Do not edit this file directly.",
        header_count
    )
}

/// Generate CXX declarations for a single class in unified mode
/// Uses full C++ names as Rust identifiers (e.g., gp_Pnt, TopoDS_Shape)
fn generate_unified_class(
    class: &ParsedClass,
    ctx: &TypeContext,
    symbol_table: &crate::resolver::SymbolTable,
) -> TokenStream {
    let cpp_name = &class.name;
    let rust_name = format_ident!("{}", cpp_name);  // Full C++ name as Rust ident

    // Doc comment with source attribution
    let source_attr = format_source_attribution(&class.source_header, class.source_line, &class.name);
    let doc = if let Some(ref comment) = class.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    // Type declaration
    let type_decl = quote! {
        #doc
        type #rust_name;
    };

    // Constructor functions
    let ctors = generate_unified_constructors(class, ctx);

    // Instance methods (CXX method declarations - no function name tracking needed)
    let methods = generate_unified_methods(class, ctx);

    // Wrapper methods (free functions for methods returning by value)
    // Returns the set of function names used, to avoid conflicts with static methods
    let (wrapper_methods, wrapper_fn_names) = generate_unified_wrapper_methods(class, ctx);

    // Static methods (takes reserved names to avoid conflicts)
    let static_methods = generate_unified_static_methods(class, ctx, &wrapper_fn_names);

    // Upcast functions (for inheritance)
    let upcast_methods = generate_unified_upcast_methods(class, ctx, symbol_table);

    // to_owned function (copy constructor)
    let to_owned = generate_unified_to_owned(class);

    // to_handle function (for transient classes)
    let to_handle = generate_unified_to_handle_ffi(class);

    // Handle upcast functions
    let handle_upcasts = generate_unified_handle_upcast_ffi(class, ctx, symbol_table);

    // Section header
    let section_line = format!(" ======================== {} ========================", cpp_name);

    // Methods are declared as flat functions with self receiver (CXX method syntax)
    // CXX doesn't support impl blocks inside extern "C++"
    quote! {
        #[doc = #section_line]
        #type_decl

        #(#ctors)*
        #(#methods)*
        #(#wrapper_methods)*
        #(#static_methods)*
        #(#upcast_methods)*
        #to_owned
        #to_handle
        #(#handle_upcasts)*
    }
}

/// Generate constructor function declarations for unified mode
fn generate_unified_constructors(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    if class.has_protected_destructor || class.is_abstract {
        return Vec::new();
    }

    let mut ctor_names: HashMap<String, usize> = HashMap::new();

    class
        .constructors
        .iter()
        .filter(|ctor| {
            if ctor.params.iter().any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_))) {
                return false;
            }
            if ctor.has_unbindable_types() {
                return false;
            }
            if constructor_uses_enum(ctor, ctx.all_enums) {
                return false;
            }
            // Filter constructors using unknown Handle types (but allow opaque Class types)
            if let Some(handle_classes) = ctx.handle_able_classes {
                if ctor.params.iter().any(|p| param_uses_unknown_handle(&p.ty, handle_classes)) {
                    return false;
                }
            }
            true
        })
        .map(|ctor| {
            let base_suffix = ctor.overload_suffix();
            let suffix = if base_suffix.is_empty() {
                "ctor".to_string()
            } else {
                format!("ctor{}", base_suffix)
            };

            let count = ctor_names.entry(suffix.clone()).or_insert(0);
            *count += 1;
            let final_suffix = if *count > 1 {
                format!("{}_{}", suffix, count)
            } else {
                suffix
            };

            generate_unified_constructor(class, ctor, &final_suffix, ctx)
        })
        .collect()
}

/// Generate a single constructor function for unified mode
fn generate_unified_constructor(
    class: &ParsedClass,
    ctor: &Constructor,
    suffix: &str,
    ctx: &TypeContext,
) -> TokenStream {
    let cpp_name = &class.name;
    let rust_type = format_ident!("{}", cpp_name);
    let cpp_wrapper_name = format!("{}_{}", cpp_name, suffix);
    let rust_fn_name = format_ident!("{}", cpp_wrapper_name);

    let params = ctor.params.iter().map(|p| {
        let name = safe_param_ident(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    let source_attr = format_source_attribution(
        &class.source_header,
        ctor.source_line,
        &format!("{}::{}()", cpp_name, cpp_name),
    );
    let doc = if let Some(ref comment) = ctor.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    quote! {
        #doc
        fn #rust_fn_name(#(#params),*) -> UniquePtr<#rust_type>;
    }
}

/// Generate instance method declarations for unified mode
fn generate_unified_methods(class: &ParsedClass, ctx: &TypeContext) -> Vec<TokenStream> {
    // Filter bindable methods - only include methods that CXX can bind directly
    // Methods that need wrappers (return by value, c_string params) are handled separately
    let bindable_methods: Vec<&Method> = class
        .methods
        .iter()
        .filter(|method| {
            // Skip methods with types that can't be bound at all
            if method.has_unbindable_types() {
                return false;
            }
            // Skip methods that need C++ wrapper functions (return by value, c_string params)
            if needs_wrapper_function(method, ctx.all_enums) {
                return false;
            }
            // Skip methods with by-value class/handle parameters (not supported by CXX)
            if has_unsupported_by_value_params(method) {
                return false;
            }
            // Skip const methods returning mutable references (CXX can't represent this)
            if has_const_mut_return_mismatch(method) {
                return false;
            }
            // Skip methods using enums (enums need special handling)
            if method_uses_enum(method, ctx.all_enums) {
                return false;
            }
            // Skip methods that need explicit lifetime annotations
            if resolver::method_needs_explicit_lifetimes(method) {
                return false;
            }
            // Filter methods using unknown classes/handles (not declared in FFI)
            if method.params.iter().any(|p| type_uses_unknown_type(&p.ty, ctx)) {
                return false;
            }
            if let Some(ref ret) = method.return_type {
                if type_uses_unknown_type(ret, ctx) {
                    return false;
                }
            }
            true
        })
        .collect();

    // Count methods by name to determine which need overload suffixes
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in &bindable_methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    // Track seen method names to ensure uniqueness
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    bindable_methods
        .iter()
        .map(|method| {
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            let base_suffix = if needs_suffix {
                // Use overload_suffix based on parameter types, with const suffix if needed
                let base_suffix = method.overload_suffix();
                // Check if there's another method with same base suffix but different constness
                let same_suffix_diff_const = bindable_methods.iter()
                    .any(|m| m.name == method.name && m.overload_suffix() == base_suffix && m.is_const != method.is_const);
                if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                }
            } else {
                String::new()
            };

            // Build method name and ensure uniqueness
            let base_rust_name = safe_method_name(&method.name);
            let candidate_name = if base_suffix.is_empty() {
                base_rust_name.clone()
            } else {
                format!("{}{}", base_rust_name, base_suffix)
            };

            let count = seen_names.entry(candidate_name.clone()).or_insert(0);
            *count += 1;
            let suffix = if *count > 1 {
                format!("{}_{}", base_suffix, count)
            } else {
                base_suffix
            };

            generate_unified_method_with_suffix(class, method, &suffix, ctx)
        })
        .collect()
}

/// Generate a single method for unified mode with an overload suffix
/// Methods are generated as CXX method declarations with self receiver.
/// CXX will bind these directly to the C++ class methods.
fn generate_unified_method_with_suffix(class: &ParsedClass, method: &Method, suffix: &str, ctx: &TypeContext) -> TokenStream {
    let cpp_name = &class.name;
    let rust_type = format_ident!("{}", cpp_name);

    // Build the Rust method name with suffix
    let base_rust_name = safe_method_name(&method.name);
    let rust_method_name_str = if suffix.is_empty() {
        base_rust_name
    } else {
        format!("{}{}", base_rust_name, suffix)
    };
    let rust_method_name = format_ident!("{}", rust_method_name_str);

    // CXX method syntax: use self: &Type for methods
    // CXX will bind these as methods on the C++ class
    let receiver = if method.is_const {
        quote! { self: &#rust_type }
    } else {
        quote! { self: Pin<&mut #rust_type> }
    };

    let params = method.params.iter().map(|p| {
        let name = safe_param_ident(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    let ret_type = method.return_type.as_ref().map(|ty| {
        let mapped = map_return_type_in_context(ty, ctx);
        let ty_tokens: TokenStream = mapped.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    let source_attr = format_source_attribution(
        &class.source_header,
        method.source_line,
        &format!("{}::{}()", cpp_name, &method.name),
    );
    let doc = if let Some(ref comment) = method.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    let cxx_name = &method.name;

    quote! {
        #doc
        #[cxx_name = #cxx_name]
        fn #rust_method_name(#receiver, #(#params),*) #ret_type;
    }
}

/// Generate wrapper method declarations for unified mode
/// Generate wrapper methods for unified mode, returning both the declarations and the used function names
fn generate_unified_wrapper_methods(class: &ParsedClass, ctx: &TypeContext) -> (Vec<TokenStream>, HashSet<String>) {
    // Filter bindable wrapper methods
    let wrapper_methods: Vec<&Method> = class
        .methods
        .iter()
        .filter(|method| {
            if method.has_unbindable_types() {
                return false;
            }
            if !needs_wrapper_function(method, ctx.all_enums) {
                return false;
            }
            if has_unsupported_by_value_params(method) {
                return false;
            }
            if has_const_mut_return_mismatch(method) {
                return false;
            }
            if method_uses_enum(method, ctx.all_enums) {
                return false;
            }
            if resolver::method_needs_explicit_lifetimes(method) {
                return false;
            }
            // Filter methods using unknown classes/handles
            if method.params.iter().any(|p| type_uses_unknown_type(&p.ty, ctx)) {
                return false;
            }
            if let Some(ref ret) = method.return_type {
                if type_uses_unknown_type(ret, ctx) {
                    return false;
                }
            }
            true
        })
        .collect();

    // Count methods by name to determine which need overload suffixes
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in &wrapper_methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    let cpp_name = &class.name;
    let mut used_names = HashSet::new();
    let methods: Vec<TokenStream> = wrapper_methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            let fn_name = if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = wrapper_methods.iter()
                    .any(|m| m.name == method.name && m.overload_suffix() == base_suffix && m.is_const != method.is_const);
                let suffix = if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                };
                format!("{}{}", base_name, suffix)
            } else {
                base_name
            };
            // Track the full function name used
            used_names.insert(format!("{}_{}", cpp_name, fn_name));
            generate_unified_wrapper_method(class, method, &fn_name, ctx)
        })
        .collect();
    (methods, used_names)
}

/// Generate a single wrapper method for unified mode
fn generate_unified_wrapper_method(
    class: &ParsedClass,
    method: &Method,
    rust_fn_name: &str,
    ctx: &TypeContext,
) -> TokenStream {
    let cpp_name = &class.name;
    let rust_type = format_ident!("{}", cpp_name);
    let wrapper_fn_name = format!("{}_{}", cpp_name, rust_fn_name);
    let rust_fn_ident = format_ident!("{}", wrapper_fn_name);

    let self_param = if method.is_const {
        quote! { self_: &#rust_type }
    } else {
        quote! { self_: Pin<&mut #rust_type> }
    };

    let params = method.params.iter().map(|p| {
        let name = safe_param_ident(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    let ret_type = method.return_type.as_ref().map(|ty| {
        let mapped = map_return_type_in_context(ty, ctx);
        // map_return_type_in_context already wraps class/handle types in UniquePtr
        let ty_tokens: TokenStream = mapped.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    let source_attr = format_source_attribution(
        &class.source_header,
        method.source_line,
        &format!("{}::{}()", cpp_name, &method.name),
    );
    let doc = if let Some(ref comment) = method.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    quote! {
        #doc
        fn #rust_fn_ident(#self_param, #(#params),*) #ret_type;
    }
}

/// Generate static method declarations for unified mode
/// Generate static method declarations for unified mode
/// `reserved_names` contains function names already used by wrapper methods
fn generate_unified_static_methods(
    class: &ParsedClass,
    ctx: &TypeContext,
    reserved_names: &HashSet<String>,
) -> Vec<TokenStream> {
    let cpp_name = &class.name;
    
    // Filter bindable static methods
    let static_methods: Vec<&StaticMethod> = class
        .static_methods
        .iter()
        .filter(|method| {
            if method.has_unbindable_types() {
                return false;
            }
            if static_method_uses_enum(method, ctx.all_enums) {
                return false;
            }
            // Filter methods using unknown classes/handles
            if method.params.iter().any(|p| type_uses_unknown_type(&p.ty, ctx)) {
                return false;
            }
            if let Some(ref ret) = method.return_type {
                if type_uses_unknown_type(ret, ctx) {
                    return false;
                }
                // Filter methods returning const char* - CXX can't handle direct binding
                if type_is_cstring(ret) {
                    return false;
                }
            }
            true
        })
        .collect();

    // Count methods by name (for internal static method conflicts)
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in &static_methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    static_methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let has_internal_conflict = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            
            // Generate candidate function name
            let fn_name = if has_internal_conflict {
                // Multiple static methods with same name - use suffix
                let suffix = method.overload_suffix();
                format!("{}{}", base_name, suffix)
            } else {
                base_name.clone()
            };
            
            // Check if this conflicts with reserved names (from wrapper methods)
            let full_fn_name = format!("{}_{}", cpp_name, fn_name);
            let final_fn_name = if reserved_names.contains(&full_fn_name) {
                // Conflict with wrapper method - add suffix to distinguish
                let suffix = method.overload_suffix();
                if suffix.is_empty() {
                    // No params, add "_static" to distinguish
                    format!("{}_static", base_name)
                } else {
                    format!("{}{}", base_name, suffix)
                }
            } else {
                fn_name
            };
            
            generate_unified_static_method(class, method, &final_fn_name, ctx)
        })
        .collect()
}

/// Generate a single static method for unified mode
fn generate_unified_static_method(
    class: &ParsedClass,
    method: &StaticMethod,
    rust_fn_name: &str,
    ctx: &TypeContext,
) -> TokenStream {
    let cpp_name = &class.name;
    let wrapper_fn_name = format!("{}_{}", cpp_name, rust_fn_name);
    let rust_fn_ident = format_ident!("{}", wrapper_fn_name);

    let params = method.params.iter().map(|p| {
        let name = safe_param_ident(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    let ret_type = method.return_type.as_ref().map(|ty| {
        let mapped = map_return_type_in_context(ty, ctx);
        // map_return_type_in_context already wraps class/handle types in UniquePtr
        let mut ty_str = mapped.rust_type;
        // Static methods returning references need 'static lifetime
        if ty.is_reference() && ty_str.starts_with('&') {
            ty_str = ty_str.replace("&", "&'static ");
        }
        let ty_tokens: TokenStream = ty_str.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    let source_attr = format_source_attribution(
        &class.source_header,
        method.source_line,
        &format!("{}::{}()", cpp_name, &method.name),
    );
    let doc = if let Some(ref comment) = method.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    quote! {
        #doc
        fn #rust_fn_ident(#(#params),*) #ret_type;
    }
}

/// Generate upcast method declarations for unified mode
fn generate_unified_upcast_methods(
    class: &ParsedClass,
    ctx: &TypeContext,
    symbol_table: &crate::resolver::SymbolTable,
) -> Vec<TokenStream> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    let mut methods = Vec::new();

    for base_class in all_ancestors {
        // Skip protected destructor classes
        if protected_destructor_classes.contains(&base_class) {
            continue;
        }

        // Skip base classes not in our binding set
        if !ctx.all_classes.contains(&base_class) {
            continue;
        }

        let cpp_name = &class.name;
        let derived_type = format_ident!("{}", cpp_name);
        let base_type = format_ident!("{}", base_class);

        // Function names
        let fn_name = format!("{}_as_{}", cpp_name, base_class);
        let fn_name_ident = format_ident!("{}", fn_name);
        let fn_name_mut = format!("{}_mut", fn_name);
        let fn_name_mut_ident = format_ident!("{}", fn_name_mut);

        let doc = format!("Upcast {} to {}", cpp_name, base_class);
        let doc_mut = format!("Upcast {} to {} (mutable)", cpp_name, base_class);

        methods.push(quote! {
            #[doc = #doc]
            fn #fn_name_ident(self_: &#derived_type) -> &#base_type;
        });

        methods.push(quote! {
            #[doc = #doc_mut]
            fn #fn_name_mut_ident(self_: Pin<&mut #derived_type>) -> Pin<&mut #base_type>;
        });
    }

    methods
}

/// Generate to_owned function for unified mode
fn generate_unified_to_owned(class: &ParsedClass) -> Option<TokenStream> {
    let copyable_modules = ["TopoDS", "gp", "TopLoc", "Bnd", "GProp"];
    if !copyable_modules.contains(&class.module.as_str())
        || class.has_protected_destructor
        || class.is_abstract
    {
        return None;
    }

    let cpp_name = &class.name;
    let rust_type = format_ident!("{}", cpp_name);
    let fn_name = format!("{}_to_owned", cpp_name);
    let fn_name_ident = format_ident!("{}", fn_name);

    Some(quote! {
        #[doc = "Clone into a new UniquePtr via copy constructor"]
        fn #fn_name_ident(self_: &#rust_type) -> UniquePtr<#rust_type>;
    })
}

/// Generate to_handle function for unified mode
fn generate_unified_to_handle_ffi(class: &ParsedClass) -> Option<TokenStream> {
    if !class.is_handle_type || class.has_protected_destructor || class.is_abstract {
        return None;
    }

    let cpp_name = &class.name;
    let rust_type = format_ident!("{}", cpp_name);
    let fn_name = format!("{}_to_handle", cpp_name);
    let fn_name_ident = format_ident!("{}", fn_name);
    let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));
    let handle_type = format_ident!("{}", handle_type_name);

    let doc = format!("Wrap {} in a Handle", cpp_name);

    Some(quote! {
        #[doc = #doc]
        fn #fn_name_ident(obj: UniquePtr<#rust_type>) -> UniquePtr<#handle_type>;
    })
}

/// Generate Handle upcast functions for unified mode
fn generate_unified_handle_upcast_ffi(
    class: &ParsedClass,
    ctx: &TypeContext,
    symbol_table: &crate::resolver::SymbolTable,
) -> Vec<TokenStream> {
    if !class.is_handle_type || class.has_protected_destructor || class.is_abstract {
        return Vec::new();
    }

    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    // Get handle_able_classes from context (or fall back to all_classes)
    let handle_able = ctx.handle_able_classes.unwrap_or(ctx.all_classes);

    // Filter to transient base classes that are in our binding set
    let transient_bases: Vec<_> = all_ancestors
        .iter()
        .filter(|base| {
            if protected_destructor_classes.contains(*base) {
                return false;
            }
            // Only include bases that will have Handle declarations
            if !handle_able.contains(*base) {
                return false;
            }
            if let Some(base_class) = symbol_table.class_by_name(base) {
                base_class.is_handle_type
            } else {
                false
            }
        })
        .collect();

    let mut methods = Vec::new();

    for base_class in transient_bases {
        let cpp_name = &class.name;
        let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));
        let handle_type = format_ident!("{}", handle_type_name);
        let base_handle_name = format!("Handle{}", base_class.replace("_", ""));
        let base_handle_type = format_ident!("{}", base_handle_name);

        let fn_name = format!("{}_to_{}", handle_type_name, base_handle_name);
        let fn_name_ident = format_ident!("{}", fn_name);

        let doc = format!("Upcast Handle<{}> to Handle<{}>", cpp_name, base_class);

        methods.push(quote! {
            #[doc = #doc]
            fn #fn_name_ident(self_: &#handle_type) -> UniquePtr<#base_handle_type>;
        });
    }

    methods
}

/// Generate free function declarations for unified mode
fn generate_unified_functions(functions: &[&ParsedFunction], ctx: &TypeContext) -> Vec<TokenStream> {
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    functions
        .iter()
        .filter(|func| {
            if func.has_unbindable_types() {
                return false;
            }
            if function_uses_enum(func, ctx.all_enums) {
                return false;
            }
            true
        })
        .map(|func| {
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

            generate_unified_function(func, &rust_name, ctx)
        })
        .collect()
}

/// Generate a single free function for unified mode
fn generate_unified_function(
    func: &ParsedFunction,
    rust_name: &str,
    ctx: &TypeContext,
) -> TokenStream {
    let cpp_wrapper_name = format!("{}_{}", func.namespace, rust_name);
    let rust_fn_ident = format_ident!("{}", rust_name);

    let params = func.params.iter().map(|p| {
        let name = safe_param_ident(&p.name);
        let ty = map_type_in_context(&p.ty, ctx);
        let ty_tokens: TokenStream = ty.rust_type.parse().unwrap_or_else(|_| quote! { () });
        quote! { #name: #ty_tokens }
    });

    let ret_type = func.return_type.as_ref().map(|ty| {
        let mapped = map_return_type_in_context(ty, ctx);
        let ty_str = if ty.is_class() || ty.is_handle() {
            format!("UniquePtr<{}>", mapped.rust_type)
        } else {
            mapped.rust_type
        };
        let ty_tokens: TokenStream = ty_str.parse().unwrap_or_else(|_| quote! { () });
        quote! { -> #ty_tokens }
    });

    let source_attr = format_source_attribution(
        &func.source_header,
        None,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    let doc = if let Some(ref comment) = func.comment {
        quote! {
            #[doc = #source_attr]
            #[doc = ""]
            #[doc = #comment]
        }
    } else {
        quote! { #[doc = #source_attr] }
    };

    quote! {
        #doc
        #[cxx_name = #cpp_wrapper_name]
        fn #rust_fn_ident(#(#params),*) #ret_type;
    }
}

/// Generate Handle type declarations for unified mode
fn generate_unified_handle_declarations(classes: &[&ParsedClass]) -> Vec<TokenStream> {
    let mut handles = BTreeSet::new();

    for class in classes {
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
        }
    }

    handles
        .iter()
        .map(|class_name| {
            let handle_type_name = format!("Handle{}", class_name.replace("_", ""));
            let handle_type = format_ident!("{}", handle_type_name);
            let doc = format!("Handle to {}", class_name);

            quote! {
                #[doc = #doc]
                type #handle_type;
            }
        })
        .collect()
}

/// Generate opaque type declarations for unified mode
fn generate_unified_opaque_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    all_enum_names: &HashSet<String>,
    protected_destructor_classes: &HashSet<String>,
    collection_type_names: &HashSet<String>,
) -> Vec<TokenStream> {
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();
    let mut declarations = Vec::new();

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

        let ident = format_ident!("{}", type_name);
        declarations.push(quote! {
            #[doc = "Referenced type from C++"]
            type #ident;
        });
    }

    declarations
}

/// Generate UniquePtr impl blocks for unified mode
fn generate_unified_unique_ptr_impls(classes: &[&ParsedClass]) -> Vec<TokenStream> {
    classes
        .iter()
        .filter(|class| !class.has_protected_destructor)
        .map(|class| {
            let cpp_name = &class.name;
            let rust_type = format_ident!("{}", cpp_name);

            quote! {
                impl UniquePtr<#rust_type> {}
            }
        })
        .collect()
}

/// Generate a module re-export file for the unified architecture
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    rust_module_name: &str,
    classes: &[&ParsedClass],
    functions: &[&ParsedFunction],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
) -> String {
    let all_enum_names = &symbol_table.all_enum_names;
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_class_names = &symbol_table.all_class_names;

    // Build set of ALL classes that can have Handle<T> declarations (across all modules)
    // This must match generate_unified_ffi's handle_able_classes to ensure consistent filtering
    let handle_able_classes: HashSet<String> = symbol_table.classes.values()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.cpp_name.clone())
        .collect();

    // Build set of class names owned by this module
    let owned_classes: HashSet<String> = classes.iter().map(|c| c.name.clone()).collect();
    
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
        if function_uses_enum(func, all_enum_names) {
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

    // Group classes by source header for organized output
    use std::collections::BTreeMap;
    let mut classes_by_header: BTreeMap<String, Vec<&ParsedClass>> = BTreeMap::new();
    for class in classes {
        if class.has_protected_destructor {
            continue;
        }
        classes_by_header
            .entry(class.source_header.clone())
            .or_insert_with(Vec::new)
            .push(class);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    for (header, header_classes) in classes_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");

        for class in header_classes {
            let cpp_name = &class.name;
        let short_name = class.short_name();
        let safe_name = class.safe_short_name();
        
        // Re-export with short name
        let doc_comment = if let Some(comment) = &class.comment {
            // Format multiline comment properly
            comment.lines()
                .map(|line| format!("/// {}", line.trim()))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            String::new()
        };
        
        if !doc_comment.is_empty() {
            output.push_str(&doc_comment);
            output.push('\n');
        }
        output.push_str(&format!("pub use crate::ffi::{} as {};\n\n", cpp_name, short_name));
        
        // Generate impl block
        let mut impl_methods = Vec::new();
        
        // Constructors
        if !class.is_abstract && !class.has_protected_destructor {
            let mut ctor_names: HashMap<String, usize> = HashMap::new();
            
            for ctor in &class.constructors {
                if ctor.params.iter().any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_))) {
                    continue;
                }
                if ctor.has_unbindable_types() {
                    continue;
                }
                if constructor_uses_enum(ctor, all_enum_names) {
                    continue;
                }
                // Filter constructors using unknown Handle types (but allow opaque Class types)
                if ctor.params.iter().any(|p| param_uses_unknown_handle(&p.ty, &handle_able_classes)) {
                    continue;
                }

                let base_suffix = ctor.overload_suffix();
                let method_name = if base_suffix.is_empty() {
                    "new".to_string()
                } else {
                    format!("new{}", base_suffix)
                };
                
                let count = ctor_names.entry(method_name.clone()).or_insert(0);
                *count += 1;
                let final_method_name = if *count > 1 {
                    format!("{}_{}", method_name, count)
                } else {
                    method_name
                };
                let method_ident = final_method_name.to_snake_case();
                
                let ffi_suffix = if base_suffix.is_empty() {
                    "ctor".to_string()
                } else {
                    format!("ctor{}", base_suffix)
                };
                let ffi_fn_name = format!("{}_{}", cpp_name, ffi_suffix);
                
                // Parameters
                let params: Vec<String> = ctor.params.iter().map(|p| {
                    let name = if RUST_KEYWORDS.contains(&p.name.as_str()) {
                        format!("{}_", p.name)
                    } else {
                        p.name.clone()
                    };
                    let ty_str = unified_type_to_string(&p.ty, &owned_classes, all_enum_names);
                    format!("{}: {}", name, ty_str)
                }).collect();
                
                let args: Vec<String> = ctor.params.iter().map(|p| {
                    if RUST_KEYWORDS.contains(&p.name.as_str()) {
                        format!("{}_", p.name)
                    } else {
                        p.name.clone()
                    }
                }).collect();
                
                let doc = if let Some(comment) = &ctor.comment {
                    comment.lines()
                        .map(|line| format!("    /// {}", line.trim()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                };
                
                let doc_prefix = if doc.is_empty() { String::new() } else { format!("{}\n", doc) };
                impl_methods.push(format!(
                    "{}    pub fn {}({}) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}({})\n    }}\n",
                    doc_prefix,
                    method_ident,
                    params.join(", "),
                    ffi_fn_name,
                    args.join(", ")
                ));
            }
        }

        // Collect CXX method names (non-wrapper methods bound directly by CXX)
        // Used to detect naming conflicts between wrappers/statics and CXX methods
        let cxx_method_names: HashSet<String> = class.methods.iter()
            .filter(|m| !m.has_unbindable_types() && !needs_wrapper_function(m, all_enum_names))
            .map(|m| safe_method_name(&m.name))
            .collect();

        // Track all instance method names (CXX + wrapper) for static method conflict detection
        let mut all_instance_method_names: HashSet<String> = cxx_method_names.clone();
        // Track full FFI function names (ClassName_method) reserved by wrapper methods
        // This must match the reserved_names set used in generate_unified_static_methods
        let mut wrapper_reserved_names: HashSet<String> = HashSet::new();

        // Instance method wrappers - ONLY for methods that are generated as free functions
        // in ffi.rs (i.e., needs_wrapper_function returns true). CXX methods (with self
        // receiver) are automatically available on the type via the type alias.
        {
            let type_ctx = TypeContext {
                current_module: module_name,
                module_classes: all_class_names,
                all_enums: all_enum_names,
                all_classes: all_class_names,
                handle_able_classes: Some(&handle_able_classes),
            };

            // First pass: collect all bindable wrapper methods (same filter as generate_unified_wrapper_methods)
            let wrapper_methods: Vec<&Method> = class.methods.iter().filter(|method| {
                if method.has_unbindable_types() { return false; }
                if !needs_wrapper_function(method, all_enum_names) { return false; }
                if method_uses_enum(method, all_enum_names) { return false; }
                if has_unsupported_by_value_params(method) { return false; }
                if has_const_mut_return_mismatch(method) { return false; }
                if method.params.iter().any(|p| type_uses_unknown_type(&p.ty, &type_ctx)) { return false; }
                if let Some(ref ret) = method.return_type {
                    if type_uses_unknown_type(ret, &type_ctx) { return false; }
                }
                if resolver::method_needs_explicit_lifetimes(method) { return false; }
                true
            }).collect();

            // Count wrapper methods by name - must match generate_unified_wrapper_methods logic
            let mut name_counts: HashMap<String, usize> = HashMap::new();
            for method in &wrapper_methods {
                *name_counts.entry(method.name.clone()).or_insert(0) += 1;
            }

            for method in &wrapper_methods {
                // Determine method name - must match generate_unified_wrapper_methods exactly
                let base_name = safe_method_name(&method.name);
                let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
                let ffi_method_ident = if needs_suffix {
                    let base_suffix = method.overload_suffix();
                    let same_suffix_diff_const = wrapper_methods.iter()
                        .any(|m| m.name == method.name && m.overload_suffix() == base_suffix && m.is_const != method.is_const);
                    let suffix = if same_suffix_diff_const && !method.is_const {
                        format!("{}_mut", base_suffix)
                    } else {
                        base_suffix
                    };
                    format!("{}{}", base_name, suffix)
                } else {
                    base_name.clone()
                };

                // For the impl method name in the re-export, add overload suffix if it
                // would conflict with a CXX method of the same name
                let method_ident = if cxx_method_names.contains(&ffi_method_ident) {
                    let suffix = method.overload_suffix();
                    if suffix.is_empty() {
                        format!("{}_wrapper", ffi_method_ident)
                    } else {
                        format!("{}{}", base_name, suffix)
                    }
                } else {
                    ffi_method_ident.clone()
                };

                // Track this wrapper method name for static method conflict detection
                all_instance_method_names.insert(method_ident.clone());

                // FFI function name matches generate_unified_wrapper_method: ClassName_ffi_method_ident
                let ffi_fn_name = format!("{}_{}", cpp_name, ffi_method_ident);
                wrapper_reserved_names.insert(ffi_fn_name.clone());

                // Build parameter list for the wrapper
                let self_param = if method.is_const {
                    "&self".to_string()
                } else {
                    "self: std::pin::Pin<&mut Self>".to_string()
                };

                let params: Vec<String> = std::iter::once(self_param)
                    .chain(method.params.iter().map(|p| {
                        let name = if RUST_KEYWORDS.contains(&p.name.as_str()) {
                            format!("{}_", p.name)
                        } else {
                            p.name.clone()
                        };
                        let ty_str = unified_type_to_string(&p.ty, &owned_classes, all_enum_names);
                        format!("{}: {}", name, ty_str)
                    }))
                    .collect();

                let args: Vec<String> = std::iter::once("self".to_string())
                    .chain(method.params.iter().map(|p| {
                        if RUST_KEYWORDS.contains(&p.name.as_str()) {
                            format!("{}_", p.name)
                        } else {
                            p.name.clone()
                        }
                    }))
                    .collect();

                let return_type = if let Some(ref ret) = method.return_type {
                    let ty_str = unified_return_type_to_string(ret, &owned_classes, all_enum_names);
                    format!(" -> {}", ty_str)
                } else {
                    String::new()
                };

                let doc = if let Some(comment) = &method.comment {
                    comment.lines()
                        .map(|line| format!("    /// {}", line.trim()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                };

                let doc_prefix = if doc.is_empty() { String::new() } else { format!("{}\n", doc) };
                impl_methods.push(format!(
                    "{}    pub fn {}({}){} {{\n        crate::ffi::{}({})\n    }}\n",
                    doc_prefix,
                    method_ident,
                    params.join(", "),
                    return_type,
                    ffi_fn_name,
                    args.join(", ")
                ));
            }
        }

        // Static method wrappers
        {
            let type_ctx = TypeContext {
                current_module: module_name,
                module_classes: all_class_names,
                all_enums: all_enum_names,
                all_classes: all_class_names,
                handle_able_classes: Some(&handle_able_classes),
            };

            // First pass: collect all bindable static methods (same filter as generate_unified_static_methods)
            let static_methods: Vec<&StaticMethod> = class.static_methods.iter().filter(|method| {
                if method.has_unbindable_types() { return false; }
                if static_method_uses_enum(method, all_enum_names) { return false; }
                if method.params.iter().any(|p| type_uses_unknown_type(&p.ty, &type_ctx)) { return false; }
                if let Some(ref ret) = method.return_type {
                    if type_uses_unknown_type(ret, &type_ctx) { return false; }
                    if type_is_cstring(ret) { return false; }
                }
                true
            }).collect();

            // Count by name - must match generate_unified_static_methods
            let mut name_counts: HashMap<String, usize> = HashMap::new();
            for method in &static_methods {
                *name_counts.entry(method.name.clone()).or_insert(0) += 1;
            }

            for method in &static_methods {
                // Determine method name - must match generate_unified_static_methods exactly
                let base_name = safe_method_name(&method.name);
                let has_internal_conflict = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
                let candidate_fn_name = if has_internal_conflict {
                    let suffix = method.overload_suffix();
                    format!("{}{}", base_name, suffix)
                } else {
                    base_name.clone()
                };

                // Check if candidate conflicts with wrapper method reserved names
                // (matches generate_unified_static_methods reserved_names logic)
                let candidate_full = format!("{}_{}", cpp_name, candidate_fn_name);
                let ffi_method_ident = if wrapper_reserved_names.contains(&candidate_full) {
                    let suffix = method.overload_suffix();
                    if suffix.is_empty() {
                        format!("{}_static", base_name)
                    } else {
                        format!("{}{}", base_name, suffix)
                    }
                } else {
                    candidate_fn_name
                };

                // For the impl method name, add suffix if it conflicts with any instance method
                // (CXX methods or wrapper methods)
                let method_ident = if all_instance_method_names.contains(&ffi_method_ident) {
                    let suffix = method.overload_suffix();
                    if suffix.is_empty() {
                        format!("{}_static", ffi_method_ident)
                    } else {
                        format!("{}{}", base_name, suffix)
                    }
                } else {
                    ffi_method_ident.clone()
                };

                // FFI function name matches generate_unified_static_method: ClassName_ffi_method_ident
                let ffi_fn_name = format!("{}_{}", cpp_name, ffi_method_ident);

                // Build parameter list (no self for static methods)
                let params: Vec<String> = method.params.iter().map(|p| {
                    let name = if RUST_KEYWORDS.contains(&p.name.as_str()) {
                        format!("{}_", p.name)
                    } else {
                        p.name.clone()
                    };
                    let ty_str = unified_type_to_string(&p.ty, &owned_classes, all_enum_names);
                    format!("{}: {}", name, ty_str)
                }).collect();

                let args: Vec<String> = method.params.iter().map(|p| {
                    if RUST_KEYWORDS.contains(&p.name.as_str()) {
                        format!("{}_", p.name)
                    } else {
                        p.name.clone()
                    }
                }).collect();

                let return_type = if let Some(ref ret) = method.return_type {
                    let mut ty_str = unified_return_type_to_string(ret, &owned_classes, all_enum_names);
                    // Static methods returning references/pointers need 'static lifetime
                    if ty_str.starts_with('&') && !ty_str.contains("'static") {
                        ty_str = ty_str.replacen("&", "&'static ", 1);
                    }
                    format!(" -> {}", ty_str)
                } else {
                    String::new()
                };

                let doc = if let Some(comment) = &method.comment {
                    comment.lines()
                        .map(|line| format!("    /// {}", line.trim()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                };

                let doc_prefix = if doc.is_empty() { String::new() } else { format!("{}\n", doc) };
                impl_methods.push(format!(
                    "{}    pub fn {}({}){} {{\n        crate::ffi::{}({})\n    }}\n",
                    doc_prefix,
                    method_ident,
                    params.join(", "),
                    return_type,
                    ffi_fn_name,
                    args.join(", ")
                ));
            }
        }

        // Upcast methods
        let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
        for base_class in &all_ancestors {
            if protected_destructor_classes.contains(base_class) {
                continue;
            }
            if !all_class_names.contains(base_class) {
                continue;
            }
            
            let base_short = if let Some(underscore_pos) = base_class.find('_') {
                crate::type_mapping::safe_short_name(&base_class[underscore_pos + 1..])
            } else {
                crate::type_mapping::safe_short_name(base_class)
            };
            
            let base_module = if let Some(underscore_pos) = base_class.find('_') {
                &base_class[..underscore_pos]
            } else {
                base_class.as_str()
            };
            
            let ffi_fn_name = format!("{}_as_{}", cpp_name, base_class);
            let ffi_fn_name_mut = format!("{}_mut", ffi_fn_name);
            
            let method_name = if base_module == module_name {
                format!("as_{}", heck::AsSnakeCase(&base_short))
            } else {
                format!("as_{}", heck::AsSnakeCase(base_class.as_str()))
            };
            
            let ret_type = if base_module == module_name {
                base_short.to_string()
            } else {
                let rust_mod = crate::module_graph::module_to_rust_name(base_module);
                format!("crate::{}::{}", rust_mod, base_short)
            };
            
            impl_methods.push(format!(
                "    /// Upcast to {}\n    pub fn {}(&self) -> &{} {{\n        crate::ffi::{}(self)\n    }}\n",
                base_class, method_name, ret_type, ffi_fn_name
            ));
            
            impl_methods.push(format!(
                "    /// Upcast to {} (mutable)\n    pub fn {}_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut {}> {{\n        crate::ffi::{}(self)\n    }}\n",
                base_class, method_name, ret_type, ffi_fn_name_mut
            ));
        }
        
        // to_owned method
        let copyable_modules = ["TopoDS", "gp", "TopLoc", "Bnd", "GProp"];
        if copyable_modules.contains(&class.module.as_str()) && !class.has_protected_destructor && !class.is_abstract {
            let ffi_fn_name = format!("{}_to_owned", cpp_name);
            impl_methods.push(format!(
                "    /// Clone into a new UniquePtr via copy constructor\n    pub fn to_owned(&self) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}(self)\n    }}\n",
                ffi_fn_name
            ));
        }
        
        // to_handle method
        if class.is_handle_type && !class.has_protected_destructor && !class.is_abstract {
            let ffi_fn_name = format!("{}_to_handle", cpp_name);
            let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));
            impl_methods.push(format!(
                "    /// Wrap in a Handle (reference-counted smart pointer)\n    pub fn to_handle(obj: cxx::UniquePtr<Self>) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}(obj)\n    }}\n",
                handle_type_name, ffi_fn_name
            ));
        }
        
        // Generate the impl block
        if !impl_methods.is_empty() {
            output.push_str(&format!("impl {} {{\n", short_name));
            for method in impl_methods {
                output.push_str(&method);
            }
            output.push_str("}\n\n");
        }
        }
    }

    output
}

/// Convert a Type to a string for unified FFI
/// Like `unified_type_to_string` but for return types from wrapper functions.
/// Classes and Handles returned by value are wrapped in UniquePtr by CXX.
fn unified_return_type_to_string(ty: &Type, owned_classes: &HashSet<String>, all_enums: &HashSet<String>) -> String {
    match ty {
        // Classes returned by value are wrapped in UniquePtr
        Type::Class(name) if name != "char" => {
            format!("cxx::UniquePtr<crate::ffi::{}>", name)
        }
        // Handles returned by value are wrapped in UniquePtr
        Type::Handle(name) => {
            format!("cxx::UniquePtr<crate::ffi::Handle{}>", name.replace("_", ""))
        }
        // const char* return -> String (CXX wrapper converts to rust::String)
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // For references, static methods returning refs need 'static lifetime
        // Everything else delegates to the normal type_to_string
        _ => unified_type_to_string(ty, owned_classes, all_enums),
    }
}

fn unified_type_to_string(ty: &Type, _owned_classes: &HashSet<String>, _all_enums: &HashSet<String>) -> String {
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
        Type::Class(name) => {
            // Special case for char - map to Rust's c_char
            if name == "char" {
                "std::os::raw::c_char".to_string()
            } else {
                format!("crate::ffi::{}", name)
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
        Type::ConstRef(inner) => format!("&{}", unified_type_to_string(inner, _owned_classes, _all_enums)),
        Type::MutRef(inner) => {
            if inner.is_primitive() {
                format!("&mut {}", unified_type_to_string(inner, _owned_classes, _all_enums))
            } else {
                format!("std::pin::Pin<&mut {}>", unified_type_to_string(inner, _owned_classes, _all_enums))
            }
        }
        Type::RValueRef(_inner) => "()".to_string(), // RValue refs are unbindable
        Type::ConstPtr(inner) => {
            // Special case for const char* - map to &str for C strings
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "&str".to_string()
            } else {
                format!("*const {}", unified_type_to_string(inner, _owned_classes, _all_enums))
            }
        }
        Type::MutPtr(inner) => format!("*mut {}", unified_type_to_string(inner, _owned_classes, _all_enums)),
    }
}

