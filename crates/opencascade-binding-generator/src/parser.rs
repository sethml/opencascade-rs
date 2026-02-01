//! Header parser using libclang
//!
//! Extracts class declarations, methods, constructors, enums, and other information
//! from OCCT C++ headers.

use crate::model::{
    Constructor, EnumVariant, Method, Param, ParsedClass, ParsedEnum, ParsedHeader, StaticMethod,
    Type,
};
use anyhow::{Context, Result};
use clang::{Clang, Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::path::Path;

/// Parse a collection of OCCT header files
pub fn parse_headers(
    headers: &[impl AsRef<Path>],
    include_dirs: &[impl AsRef<Path>],
    verbose: bool,
) -> Result<Vec<ParsedHeader>> {
    let clang =
        Clang::new().map_err(|e| anyhow::anyhow!("Failed to initialize libclang: {}", e))?;
    let index = Index::new(&clang, false, true);

    let mut results = Vec::new();

    for header in headers {
        let header = header.as_ref();
        if verbose {
            println!("Parsing: {}", header.display());
        }
        let parsed = parse_header(&index, header, include_dirs, verbose)
            .with_context(|| format!("Failed to parse header: {}", header.display()))?;

        if verbose {
            println!(
                "  Found {} classes in {}",
                parsed.classes.len(),
                header.display()
            );
            for class in &parsed.classes {
                println!(
                    "    - {} ({} ctors, {} methods, {} static methods)",
                    class.name,
                    class.constructors.len(),
                    class.methods.len(),
                    class.static_methods.len()
                );
            }
        }

        results.push(parsed);
    }

    Ok(results)
}

/// Parse a single OCCT header file
fn parse_header(
    index: &Index,
    header: &Path,
    include_dirs: &[impl AsRef<Path>],
    verbose: bool,
) -> Result<ParsedHeader> {
    // Build clang arguments
    let mut args: Vec<String> = vec![
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++17".to_string(),
        // Suppress some warnings that aren't relevant for parsing
        "-Wno-pragma-once-outside-header".to_string(),
    ];

    // Add system C++ standard library paths
    // This is needed because libclang doesn't automatically include them
    add_system_include_paths(&mut args);

    for include_dir in include_dirs {
        args.push(format!("-I{}", include_dir.as_ref().display()));
    }

    if verbose {
        println!("  Clang args: {:?}", args);
    }

    let tu = index
        .parser(header)
        .arguments(&args)
        .detailed_preprocessing_record(true)
        .skip_function_bodies(true)
        .parse()
        .context("Failed to parse translation unit")?;

    // Check for parse errors
    let diagnostics = tu.get_diagnostics();
    for diag in &diagnostics {
        if diag.get_severity() >= clang::diagnostic::Severity::Error {
            if verbose {
                println!("  Parse error: {}", diag.get_text());
            }
        }
    }

    let header_canonical = header.canonicalize().unwrap_or_else(|_| header.to_path_buf());

    // Walk the AST looking for class/struct declarations
    let root = tu.get_entity();
    let mut classes = Vec::new();
    let mut enums = Vec::new();
    
    root.visit_children(|entity, _parent| {
        visit_top_level(&entity, &header_canonical, &mut classes, &mut enums, verbose)
    });

    Ok(ParsedHeader {
        path: header.to_path_buf(),
        classes,
        enums,
    })
}

/// Visit top-level entities in the translation unit
fn visit_top_level(
    entity: &Entity,
    source_file: &Path,
    classes: &mut Vec<ParsedClass>,
    enums: &mut Vec<ParsedEnum>,
    verbose: bool,
) -> EntityVisitResult {
    // Only process entities from our source file (not included headers)
    if !is_from_file(entity, source_file) {
        return EntityVisitResult::Continue;
    }

    match entity.get_kind() {
        EntityKind::ClassDecl | EntityKind::StructDecl => {
            if let Some(parsed) = parse_class(entity, verbose) {
                classes.push(parsed);
            }
        }
        EntityKind::EnumDecl => {
            if let Some(parsed) = parse_enum(entity, verbose) {
                enums.push(parsed);
            }
        }
        EntityKind::Namespace => {
            // Don't recurse into std namespace
            if entity.get_name().as_deref() != Some("std") {
                return EntityVisitResult::Recurse;
            }
        }
        _ => {}
    }

    EntityVisitResult::Continue
}

/// Check if an entity is from the specified source file
fn is_from_file(entity: &Entity, source_file: &Path) -> bool {
    if let Some(location) = entity.get_location() {
        if let Some(file) = location.get_file_location().file {
            let entity_path = file.get_path();
            // Compare canonical paths
            if let Ok(entity_canonical) = entity_path.canonicalize() {
                return entity_canonical == source_file;
            }
            return entity_path == source_file;
        }
    }
    false
}

/// Parse a class or struct declaration
fn parse_class(entity: &Entity, verbose: bool) -> Option<ParsedClass> {
    let name = entity.get_name()?;

    // Skip forward declarations (no definition)
    if !entity.is_definition() {
        return None;
    }

    // Skip anonymous classes/structs
    if name.is_empty() {
        return None;
    }

    // Skip internal/private classes (those starting with underscore)
    if name.starts_with('_') {
        return None;
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_name(&name);

    if verbose {
        println!("  Parsing class: {}", name);
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();

    // Check if there's a DEFINE_STANDARD_HANDLE for this class
    // This is typically done outside the class, so we check the name pattern
    // and look for inheritance from Standard_Transient
    let is_handle_type = check_is_handle_type(entity);

    entity.visit_children(|child, _| {
        match child.get_kind() {
            EntityKind::Constructor => {
                if is_public(&child) {
                    if let Some(ctor) = parse_constructor(&child, verbose) {
                        constructors.push(ctor);
                    }
                }
            }
            EntityKind::Method => {
                if is_public(&child) {
                    // Skip destructors, operators, and conversion functions
                    if let Some(ref method_name) = child.get_name() {
                        if method_name.starts_with('~')
                            || method_name.starts_with("operator")
                            || method_name == "DumpJson"
                            || method_name == "InitFromJson"
                        {
                            return EntityVisitResult::Continue;
                        }
                    }

                    if child.is_static_method() {
                        if let Some(method) = parse_static_method(&child, verbose) {
                            static_methods.push(method);
                        }
                    } else if let Some(method) = parse_method(&child, verbose) {
                        methods.push(method);
                    }
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });

    // Only return classes that have something to bind
    if constructors.is_empty() && methods.is_empty() && static_methods.is_empty() {
        if verbose {
            println!("    Skipping {} (no bindable members)", name);
        }
        return None;
    }

    Some(ParsedClass {
        name,
        module,
        comment,
        constructors,
        methods,
        static_methods,
        is_handle_type,
    })
}

/// Parse an enum declaration
fn parse_enum(entity: &Entity, verbose: bool) -> Option<ParsedEnum> {
    let name = entity.get_name()?;

    // Skip anonymous enums (empty name or compiler-generated "(unnamed enum at ...)")
    if name.is_empty() || name.starts_with("(unnamed") {
        return None;
    }

    // Skip internal enums
    if name.starts_with('_') {
        return None;
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_name(&name);

    if verbose {
        println!("  Parsing enum: {}", name);
    }

    let mut variants = Vec::new();

    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::EnumConstantDecl {
            if let Some(variant_name) = child.get_name() {
                let value = child.get_enum_constant_value().map(|(signed, _unsigned)| signed);
                let comment = extract_doxygen_comment(&child);

                if verbose {
                    if let Some(v) = value {
                        println!("    Variant: {} = {}", variant_name, v);
                    } else {
                        println!("    Variant: {}", variant_name);
                    }
                }

                variants.push(EnumVariant {
                    name: variant_name,
                    value,
                    comment,
                });
            }
        }
        EntityVisitResult::Continue
    });

    if variants.is_empty() {
        return None;
    }

    Some(ParsedEnum {
        name,
        module,
        comment,
        variants,
    })
}

/// Check if a class is a Handle type (inherits from Standard_Transient)
fn check_is_handle_type(entity: &Entity) -> bool {
    // Check base classes
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::BaseSpecifier {
            if let Some(base_type) = child.get_type() {
                let base_name = base_type.get_display_name();
                // If it inherits from Standard_Transient or any Geom_* class, it's likely a Handle type
                if base_name.contains("Standard_Transient")
                    || base_name.starts_with("Geom_")
                    || base_name.starts_with("TopoDS_")
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Extract Doxygen comment from an entity
fn extract_doxygen_comment(entity: &Entity) -> Option<String> {
    // Try to get the raw comment
    if let Some(comment) = entity.get_comment() {
        // Clean up the comment - remove //! or /// prefixes and leading/trailing whitespace
        let cleaned: Vec<&str> = comment
            .lines()
            .map(|line| {
                line.trim()
                    .trim_start_matches("//!")
                    .trim_start_matches("///")
                    .trim_start_matches("/**")
                    .trim_start_matches("/*!")
                    .trim_end_matches("*/")
                    .trim_start_matches('*')
                    .trim()
            })
            .filter(|line| !line.is_empty())
            .collect();

        if cleaned.is_empty() {
            return None;
        }

        return Some(cleaned.join(" "));
    }
    None
}

/// Extract module name from OCCT class name (e.g., "gp_Pnt" -> "gp")
fn extract_module_from_name(name: &str) -> String {
    // OCCT naming convention: ModuleName_ClassName
    // Examples: gp_Pnt, TopoDS_Shape, BRepPrimAPI_MakeBox
    if let Some(underscore_pos) = name.find('_') {
        name[..underscore_pos].to_string()
    } else {
        // No underscore - might be a single-word class name
        name.to_string()
    }
}

/// Check if a method/constructor is in the public section
fn is_public(entity: &Entity) -> bool {
    entity.get_accessibility() == Some(clang::Accessibility::Public)
}

/// Check if a method should be bound based on OCCT conventions
/// Binds: Standard_EXPORT methods OR public methods with doc comments
#[allow(dead_code)]
fn should_bind_method(entity: &Entity) -> bool {
    // Check for Standard_EXPORT by looking at the display name or attributes
    // Standard_EXPORT methods are always bindable
    if has_standard_export(entity) {
        return true;
    }

    // Public methods with documentation comments are also bindable
    // (these are typically inline getters/setters)
    if entity.get_comment().is_some() {
        return true;
    }

    // For now, bind all public methods - can be refined later
    true
}

/// Check if a method has Standard_EXPORT annotation
fn has_standard_export(entity: &Entity) -> bool {
    // Standard_EXPORT is a macro that expands to __declspec(dllexport) on Windows
    // or __attribute__((visibility("default"))) on other platforms.
    // We can detect it by checking if the method is not inline-only.

    // Methods that are defined inline (have a body in the header) but don't have
    // Standard_EXPORT are typically simple getters/setters
    let has_definition = entity.get_definition().is_some() || entity.is_definition();

    // If there's no definition in this TU, it's likely Standard_EXPORT
    // (defined in a .cxx file)
    !has_definition || entity.get_comment().is_some()
}

/// Parse a constructor
fn parse_constructor(entity: &Entity, verbose: bool) -> Option<Constructor> {
    let comment = extract_doxygen_comment(entity);
    let params = parse_params(entity);

    if verbose {
        let param_str = params
            .iter()
            .map(|p| format!("{}: {:?}", p.name, p.ty))
            .collect::<Vec<_>>()
            .join(", ");
        println!("    Constructor({})", param_str);
    }

    Some(Constructor { comment, params })
}

/// Parse an instance method
fn parse_method(entity: &Entity, verbose: bool) -> Option<Method> {
    let name = entity.get_name()?;
    let comment = extract_doxygen_comment(entity);
    let is_const = entity.is_const_method();
    let params = parse_params(entity);
    let return_type = parse_return_type(entity);

    if verbose {
        let const_str = if is_const { " const" } else { "" };
        let ret_str = return_type
            .as_ref()
            .map(|t| format!(" -> {:?}", t))
            .unwrap_or_default();
        println!("    Method: {}{}{}", name, const_str, ret_str);
    }

    Some(Method {
        name,
        comment,
        is_const,
        params,
        return_type,
    })
}

/// Parse a static method
fn parse_static_method(entity: &Entity, verbose: bool) -> Option<StaticMethod> {
    let name = entity.get_name()?;
    let comment = extract_doxygen_comment(entity);
    let params = parse_params(entity);
    let return_type = parse_return_type(entity);

    if verbose {
        let ret_str = return_type
            .as_ref()
            .map(|t| format!(" -> {:?}", t))
            .unwrap_or_default();
        println!("    Static: {}{}", name, ret_str);
    }

    Some(StaticMethod {
        name,
        comment,
        params,
        return_type,
    })
}

/// Parse function parameters
fn parse_params(entity: &Entity) -> Vec<Param> {
    entity
        .get_arguments()
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .filter_map(|(i, param)| {
            let name = param.get_name().unwrap_or_else(|| format!("arg{}", i));
            let param_type = param.get_type()?;
            Some(Param {
                name,
                ty: parse_type(&param_type),
            })
        })
        .collect()
}

/// Parse the return type of a function
fn parse_return_type(entity: &Entity) -> Option<Type> {
    let result_type = entity.get_result_type()?;

    // void return type
    if result_type.get_kind() == TypeKind::Void {
        return None;
    }

    Some(parse_type(&result_type))
}

/// Parse a clang type into our Type representation
fn parse_type(clang_type: &clang::Type) -> Type {
    let kind = clang_type.get_kind();
    let spelling = clang_type.get_display_name();

    // Handle known OCCT typedefs FIRST (before canonical resolution)
    // This handles cases where clang can't fully resolve types due to missing includes
    // Check both original and trimmed versions
    let trimmed_spelling = spelling.trim();
    if let Some(primitive) = map_standard_type(trimmed_spelling) {
        return primitive;
    }

    // Check for size_t BEFORE canonical resolution, since size_t and unsigned long
    // are the same canonical type on some platforms but we want to preserve size_t semantics
    if trimmed_spelling == "size_t" || trimmed_spelling == "std::size_t" {
        return Type::Usize;
    }

    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();

    // Handle primitives via canonical type
    match canonical_spelling.as_str() {
        "bool" => return Type::Bool,
        "int" => return Type::I32,
        "unsigned int" => return Type::U32,
        "long" => return Type::I64,
        "unsigned long" => return Type::U64,
        "long long" => return Type::I64,
        "unsigned long long" => return Type::U64,
        "float" => return Type::F32,
        "double" => return Type::F64,
        _ => {}
    }

    // Handle reference types
    if kind == TypeKind::LValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let is_const = pointee.is_const_qualified();
            let inner = parse_type(&pointee);
            return if is_const {
                Type::ConstRef(Box::new(inner))
            } else {
                Type::MutRef(Box::new(inner))
            };
        }
    }

    // Handle pointer types
    if kind == TypeKind::Pointer {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let is_const = pointee.is_const_qualified();
            let inner = parse_type(&pointee);
            return if is_const {
                Type::ConstPtr(Box::new(inner))
            } else {
                Type::MutPtr(Box::new(inner))
            };
        }
    }

    // Handle Handle<T> types (opencascade::handle<T>)
    // Strip const prefix before checking
    let clean_spelling = spelling.trim_start_matches("const ").trim();
    if clean_spelling.starts_with("opencascade::handle<") || clean_spelling.starts_with("Handle(") {
        let inner = extract_template_arg(clean_spelling);
        return Type::Handle(inner);
    }

    // For nested types (e.g., TColgp_Array1OfPnt::value_type) or template types,
    // use the canonical type to get the resolved underlying type.
    // clang resolves these for us (e.g., value_type -> gp_Pnt)
    let clean_name = spelling
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim_start_matches("typename ")
        .trim_end_matches(" &")
        .trim_end_matches(" *")
        .trim();
    
    // If the spelling contains :: or < (nested/template type), try to use canonical
    if clean_name.contains("::") || clean_name.contains('<') {
        let canonical_clean = canonical_spelling
            .trim_start_matches("const ")
            .trim_start_matches("class ")
            .trim_start_matches("struct ")
            .trim_end_matches(" &")
            .trim_end_matches(" *")
            .trim();
        
        // Only use canonical if it's simpler (no :: or <)
        if !canonical_clean.contains("::") && !canonical_clean.contains('<') && !canonical_clean.is_empty() {
            return Type::Class(canonical_clean.to_string());
        }
    }

    Type::Class(clean_name.to_string())
}

/// Extract template argument from Handle<T> or similar
fn extract_template_arg(type_name: &str) -> String {
    if type_name.starts_with("Handle(") {
        // Handle(Foo) format
        type_name
            .trim_start_matches("Handle(")
            .trim_end_matches(')')
            .trim()
            .to_string()
    } else if let Some(start) = type_name.find('<') {
        // Template<Foo> format
        let end = type_name.rfind('>').unwrap_or(type_name.len());
        type_name[start + 1..end].trim().to_string()
    } else {
        type_name.to_string()
    }
}

/// Map OCCT Standard_* typedefs to Rust primitive types
fn map_standard_type(type_name: &str) -> Option<Type> {
    // Strip any const/class/struct prefixes
    let clean = type_name
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim();

    match clean {
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        // Standard_Address is void* - can't be bound through CXX, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through CXX
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Add system C++ standard library include paths to clang arguments
/// 
/// libclang doesn't automatically include these paths, so we need to detect
/// and add them manually. This is platform-specific.
fn add_system_include_paths(args: &mut Vec<String>) {
    #[cfg(target_os = "macos")]
    {
        // Try to get SDK path from xcrun
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
        {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                // Add C++ standard library headers
                let cxx_include = format!("{}/usr/include/c++/v1", sdk_path);
                if std::path::Path::new(&cxx_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(cxx_include);
                }
                
                // Add general system headers
                let sys_include = format!("{}/usr/include", sdk_path);
                if std::path::Path::new(&sys_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(sys_include);
                }
            }
        }
        
        // Try to find clang's resource directory for built-in headers
        if let Ok(output) = std::process::Command::new("clang")
            .args(["--print-resource-dir"])
            .output()
        {
            if output.status.success() {
                let resource_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let builtin_include = format!("{}/include", resource_dir);
                if std::path::Path::new(&builtin_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(builtin_include);
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Common Linux C++ standard library paths
        let paths = [
            "/usr/include/c++/13",
            "/usr/include/c++/12", 
            "/usr/include/c++/11",
            "/usr/include/c++/10",
            "/usr/include/x86_64-linux-gnu/c++/13",
            "/usr/include/x86_64-linux-gnu/c++/12",
            "/usr/include/x86_64-linux-gnu/c++/11",
            "/usr/include/x86_64-linux-gnu/c++/10",
            "/usr/include",
        ];
        
        for path in paths {
            if std::path::Path::new(path).exists() {
                args.push("-isystem".to_string());
                args.push(path.to_string());
            }
        }
    }
    
    // Windows would need different handling with MSVC paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_module_from_name() {
        assert_eq!(extract_module_from_name("gp_Pnt"), "gp");
        assert_eq!(extract_module_from_name("TopoDS_Shape"), "TopoDS");
        assert_eq!(extract_module_from_name("BRepPrimAPI_MakeBox"), "BRepPrimAPI");
        assert_eq!(extract_module_from_name("Standalone"), "Standalone");
    }

    #[test]
    fn test_extract_template_arg() {
        assert_eq!(extract_template_arg("Handle(Geom_Curve)"), "Geom_Curve");
        assert_eq!(
            extract_template_arg("opencascade::handle<Geom_Curve>"),
            "Geom_Curve"
        );
    }

    #[test]
    fn test_map_standard_type() {
        assert!(matches!(map_standard_type("Standard_Real"), Some(Type::F64)));
        assert!(matches!(map_standard_type("Standard_Integer"), Some(Type::I32)));
        assert!(matches!(map_standard_type("Standard_Boolean"), Some(Type::Bool)));
        assert!(map_standard_type("gp_Pnt").is_none());
    }
}

