//! Rust CXX bridge code generation
//!
//! Generates a unified #[cxx::bridge] module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, ParsedFunction, Type};

use crate::resolver;
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
    

    code
        .replace(" # ! [", "\n#![")
        .replace(" # [cxx :: bridge]", "\n#[cxx::bridge]")
        .replace(" pub (crate) mod ffi {", "\npub(crate) mod ffi {")
        .replace(" unsafe extern \"C++\" {", "\n    unsafe extern \"C++\" {")
        .replace(" } }", "\n    }\n}")
        .replace(" pub use ffi ::", "\npub use ffi::")
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
///
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

fn function_uses_enum(func: &ParsedFunction, all_enums: &HashSet<String>) -> bool {
    resolver::function_uses_enum(func, all_enums)
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

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: Vec<TokenStream> = all_bindings
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
        tokens_string.push('\n');
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
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    functions: &[&ParsedFunction],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
) -> String {
    let all_enum_names = &symbol_table.all_enum_names;

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


