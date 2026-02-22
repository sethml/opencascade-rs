//! Header parser using libclang
//!
//! Extracts class declarations, methods, constructors, enums, and other information
//! from OCCT C++ headers.

use crate::model::{
    Constructor, EnumVariant, Method, Param, ParsedClass, ParsedEnum, ParsedField, ParsedFunction,
    ParsedHeader, StaticMethod, Type,
};
use anyhow::{Context, Result};
use clang::{Accessibility, Availability, Clang, Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

/// Check if a clang TypeKind represents a C/C++ primitive type (int, double, char, etc.).
/// Used to detect when a typedef resolves to a primitive through canonical type analysis.
fn is_c_primitive_type_kind(kind: TypeKind) -> bool {
    matches!(kind,
        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
        TypeKind::SChar | TypeKind::UChar |
        TypeKind::Short | TypeKind::UShort |
        TypeKind::Int | TypeKind::UInt |
        TypeKind::Long | TypeKind::ULong |
        TypeKind::LongLong | TypeKind::ULongLong |
        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
    )
}

thread_local! {
    /// Map from NCollection template spellings to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: whitespace-stripped template spelling, e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: all typedef names that alias this template, e.g. ["TDF_LabelMap"]
    /// Multiple typedefs can alias the same template (e.g. gp_Vec3f and Graphic3d_Vec3
    /// both alias NCollection_Vec3<Standard_ShortReal>).
    ///
    /// Both the display-name form (with OCCT aliases like Standard_ShortReal) and
    /// the canonical form (with C++ primitives like float) are stored as keys,
    /// so lookups work regardless of which spelling clang uses.
    static TYPEDEF_MAP: RefCell<HashMap<String, Vec<String>>> = RefCell::new(HashMap::new());

    /// Map from simple typedef names to their underlying class names.
    /// Populated by `collect_simple_typedefs()` before type parsing begins.
    /// Key: typedef name (e.g., "BinObjMgt_SRelocationTable")
    /// Value: underlying type name (e.g., "TColStd_IndexedMapOfTransient")
    /// Only contains typedefs where the underlying type is another OCCT class/typedef
    /// (not template specializations, primitives, or pointers).
    static SIMPLE_TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    /// Namespace-scoped typedef aliases collected from OCCT namespaces.
    /// Examples: IMeshData::MapOfInteger, IMeshData::VectorOfInteger.
    /// These are tracked separately so callers can selectively add them to
    /// known-type sets without importing every collected template typedef name.
    static NAMESPACE_TYPEDEF_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Strip whitespace from a C++ type spelling for typedef map key/lookup.
fn normalize_template_spelling(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Strip C++ type qualifier prefixes (const, volatile, struct, class, typename, enum)
/// from a type spelling. Call sites used to chain these manually; this centralizes the
/// stripping logic and avoids accidental divergence.
fn strip_type_qualifiers(s: &str) -> &str {
    s.trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim_start_matches("struct ")
        .trim_start_matches("class ")
        .trim_start_matches("typename ")
        .trim_start_matches("enum ")
        .trim()
}

/// Strip type qualifier prefixes AND trailing reference/pointer decorators.
/// Useful when extracting the base type name from a fully decorated C++ type spelling.
fn strip_type_decorators(s: &str) -> &str {
    strip_type_qualifiers(s)
        .trim_end_matches(" &")
        .trim_end_matches(" &&")
        .trim_end_matches(" *")
        .trim_end_matches('&')
        .trim_end_matches('*')
        .trim()
}

/// Returns true if a namespace chain looks like an OCCT namespace path.
/// Guard rails:
/// - excludes std/opencascade namespaces
/// - every segment must start with an uppercase character
fn is_occt_namespace_chain(path: &str) -> bool {
    !path.is_empty()
        && path.split("::").all(|segment| {
            !segment.is_empty()
                && segment != "std"
                && segment != "opencascade"
                && segment.starts_with(|c: char| c.is_ascii_uppercase())
        })
}

/// Build a guarded namespace-scoped typedef alias when a typedef name itself
/// is unqualified (e.g. local `MapOfInteger` in namespace `IMeshData`).
/// Returns `Some("IMeshData::MapOfInteger")` for OCCT namespace paths.
fn namespace_scoped_typedef_alias(entity: &Entity, typedef_name: &str) -> Option<String> {
    if typedef_name.contains('_') || typedef_name.contains("::") {
        return None;
    }

    let mut namespace_parts = Vec::new();
    let mut parent = entity.get_semantic_parent();
    while let Some(p) = parent {
        if p.get_kind() == EntityKind::Namespace {
            if let Some(ns) = p.get_name() {
                if !ns.is_empty() && !ns.starts_with("(anonymous") {
                    namespace_parts.push(ns);
                }
            }
        }
        parent = p.get_semantic_parent();
    }

    if namespace_parts.is_empty() {
        return None;
    }

    namespace_parts.reverse();
    let namespace = namespace_parts.join("::");
    if !is_occt_namespace_chain(&namespace) {
        return None;
    }

    Some(format!("{}::{}", namespace, typedef_name))
}

/// Walk the AST to collect all typedef/using declarations that resolve to
/// template specializations (NCollection, math_VectorBase, etc.).
/// Populates the thread-local TYPEDEF_MAP.
///
/// For each typedef, we insert keys for BOTH the display-name spelling
/// (e.g. NCollection_Vec3<Standard_ShortReal>) and the canonical spelling
/// (e.g. NCollection_Vec3<float>). This handles OCCT headers that use
/// C++ primitives directly in method signatures rather than the OCCT aliases.
///
/// `included_modules` is the set of module prefixes (e.g. "gp", "Geom") that
/// are included in the binding generation. When multiple typedefs alias the
/// same template, we prefer names from included modules.
fn collect_ncollection_typedefs(root: &Entity, included_modules: &HashSet<String>) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut namespace_typedefs: HashSet<String> = HashSet::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                let namespace_alias = namespace_scoped_typedef_alias(&entity, &name);
                // Record OCCT-style typedef names, and additionally allow
                // guarded namespace-scoped aliases for unqualified local names.
                if !name.contains('_') && namespace_alias.is_none() {
                    return EntityVisitResult::Recurse;
                }

                let mut typedef_names = vec![name.clone()];
                if let Some(alias) = namespace_alias {
                    namespace_typedefs.insert(alias.clone());
                    typedef_names.push(alias);
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Record typedefs that resolve to template specializations,
                    // but skip typedefs to std:: types (e.g. std::pair, std::vector)
                    // since those are STL types that can't be wrapped as opaque OCCT classes.
                    if display.contains('<') && !display.starts_with("std::") {
                        let display_key = normalize_template_spelling(&display);
                        for typedef_name in &typedef_names {
                            map.entry(display_key.clone())
                                .or_default()
                                .push(typedef_name.clone());
                        }

                        // Also insert under the canonical spelling so lookups
                        // work when OCCT headers use C++ primitives (e.g. float)
                        // instead of Standard_* aliases.
                        let canonical = underlying.get_canonical_type().get_display_name();
                        let canonical_key = normalize_template_spelling(&canonical);
                        if canonical_key != display_key && canonical.contains('<') {
                            for typedef_name in &typedef_names {
                                map.entry(canonical_key.clone())
                                    .or_default()
                                    .push(typedef_name.clone());
                            }
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    // Deduplicate and sort each Vec for deterministic lookup.
    // Prefer typedefs from included modules (not excluded), then shortest
    // module prefix, then alphabetically. This ensures e.g. gp_Vec3f (included)
    // is preferred over BVH_Vec3f or Graphic3d_Vec3 (excluded).
    for names in map.values_mut() {
        names.sort_by(|a, b| {
            let module_a = a.split('_').next().unwrap_or(a);
            let module_b = b.split('_').next().unwrap_or(b);
            let inc_a = included_modules.contains(module_a);
            let inc_b = included_modules.contains(module_b);
            // Included first (true > false, so reverse)
            inc_b.cmp(&inc_a)
                .then_with(|| module_a.len().cmp(&module_b.len()))
                .then_with(|| a.cmp(b))
        });
        names.dedup();
    }

    let num_typedefs: usize = map.values().map(|v| v.len()).sum();
    eprintln!("  Collected {} NCollection/template typedef entries ({} unique template spellings)", num_typedefs, map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
    NAMESPACE_TYPEDEF_NAMES.with(|m| {
        *m.borrow_mut() = namespace_typedefs;
    });
}

/// Look up a type's display name in the typedef map.
/// Returns one of the typedef names if found (there may be multiple aliases
/// for the same template; any one is valid for type resolution).
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).and_then(|v| v.first()).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() -> HashSet<String> {
    TYPEDEF_MAP.with(|m| m.borrow().values().flat_map(|v| v.iter()).cloned().collect())
}

/// Get namespace-scoped typedef aliases collected during the last parse.
/// These are a guarded subset (OCCT-style namespace paths only).
pub fn get_collected_namespace_typedef_names() -> HashSet<String> {
    NAMESPACE_TYPEDEF_NAMES.with(|m| m.borrow().clone())
}

/// Walk the AST to collect simple (non-template) typedef declarations where
/// the underlying type is another OCCT class name. Populates SIMPLE_TYPEDEF_MAP.
///
/// This handles cases like:
///   typedef TColStd_IndexedMapOfTransient BinObjMgt_SRelocationTable;
///   typedef LDOM_Element XmlObjMgt_Element;
///   typedef NCollection_Utf8String NCollection_String;
///
/// Function pointer typedefs, pointer typedefs (T*), and primitive typedefs
/// are excluded. Template typedefs (containing '<') are handled separately
/// by `collect_ncollection_typedefs()`.
fn collect_simple_typedefs(root: &Entity) {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut namespace_typedefs: HashSet<String> = HashSet::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                let namespace_alias = namespace_scoped_typedef_alias(&entity, &name);
                // Keep existing OCCT-style typedefs and guarded namespace-scoped aliases.
                if !name.contains('_') && namespace_alias.is_none() {
                    return EntityVisitResult::Recurse;
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let underlying_display = underlying.get_display_name();
                    let underlying_kind = underlying.get_kind();

                    // Skip template typedefs (handled by collect_ncollection_typedefs)
                    if underlying_display.contains('<') {
                        return EntityVisitResult::Recurse;
                    }

                    // Only record typedefs to class/struct types (Record, Elaborated wrapping Record)
                    // This excludes pointer typedefs, function pointer typedefs, primitives, etc.
                    let is_record_type = matches!(
                        underlying_kind,
                        TypeKind::Record | TypeKind::Elaborated | TypeKind::Typedef
                    );

                    if is_record_type {
                        // Get the clean underlying type name
                        let clean = underlying_display
                            .trim()
                            .trim_start_matches("const ")
                            .trim_start_matches("struct ")
                            .trim_start_matches("class ")
                            .trim();

                        // Must look like an OCCT class name (starts with uppercase, no special chars)
                        let looks_like_class = !clean.is_empty()
                            && clean.starts_with(|c: char| c.is_ascii_uppercase())
                            && !clean.contains('<')
                            && !clean.contains('*')
                            && !clean.contains('(')
                            && clean != &name; // skip self-referential typedefs

                        if looks_like_class {
                            map.insert(name.clone(), clean.to_string());
                            if let Some(alias) = namespace_alias.clone() {
                                namespace_typedefs.insert(alias.clone());
                                map.insert(alias, clean.to_string());
                            }
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    // Chase typedef chains: if A -> B and B -> C, resolve A -> C
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot: Vec<(String, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for (key, value) in &snapshot {
            if let Some(resolved) = map.get(value) {
                if resolved != value && map.get(key).unwrap() != resolved {
                    map.insert(key.clone(), resolved.clone());
                    changed = true;
                }
            }
        }
    }

    eprintln!("  Collected {} simple typedef entries", map.len());
    SIMPLE_TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
    NAMESPACE_TYPEDEF_NAMES.with(|m| {
        m.borrow_mut().extend(namespace_typedefs);
    });
}

/// Look up a type name in the simple typedef map.
/// Returns the underlying class name if this is a known typedef.
fn lookup_simple_typedef(name: &str) -> Option<String> {
    SIMPLE_TYPEDEF_MAP.with(|m| m.borrow().get(name).cloned())
}



/// Parse a collection of OCCT header files
/// 
/// Uses batch parsing: creates a synthetic source file that includes all headers,
/// parses once, then extracts entities from each target header. This is much faster
/// than parsing each header separately since OCCT headers have deep include chains.
pub fn parse_headers(
    headers: &[impl AsRef<Path>],
    include_dirs: &[impl AsRef<Path>],
    verbose: bool,
) -> Result<Vec<ParsedHeader>> {
    let clang =
        Clang::new().map_err(|e| anyhow::anyhow!("Failed to initialize libclang: {}", e))?;
    let index = Index::new(&clang, false, true);

    // Build canonical path set for target headers
    let header_paths: Vec<std::path::PathBuf> = headers
        .iter()
        .map(|h| {
            let path = h.as_ref();
            // Try to resolve relative paths using include directories
            if path.is_relative() {
                for inc_dir in include_dirs {
                    let full_path = inc_dir.as_ref().join(path);
                    if let Ok(canonical) = full_path.canonicalize() {
                        return canonical;
                    }
                }
            }
            // Fall back to canonicalizing the path as-is
            path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
        })
        .collect();
    let header_set: std::collections::HashSet<&std::path::Path> = 
        header_paths.iter().map(|p| p.as_path()).collect();

    // Create synthetic source that includes all headers
    let mut synthetic_source = String::new();
    for header in headers {
        synthetic_source.push_str(&format!("#include \"{}\"\n", header.as_ref().display()));
    }

    // Build clang arguments
    let mut args: Vec<String> = vec![
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++14".to_string(),
        "-Wno-pragma-once-outside-header".to_string(),
    ];
    add_system_include_paths(&mut args);
    for include_dir in include_dirs {
        args.push(format!("-I{}", include_dir.as_ref().display()));
    }

    if verbose {
        eprintln!("Clang args: {:?}", args);
    }

    // Parse the synthetic source with all includes
    let parse_start = Instant::now();
    let tu = index
        .parser("synthetic.cpp")
        .arguments(&args)
        .unsaved(&[clang::Unsaved::new("synthetic.cpp", &synthetic_source)])
        .detailed_preprocessing_record(true)
        .skip_function_bodies(true)
        .parse()
        .context("Failed to parse translation unit")?;
    let parse_time = parse_start.elapsed();
    eprintln!("  Clang parse time: {:.2}s", parse_time.as_secs_f64());

    // Check for parse errors — fatal errors (e.g. missing #include <windows.h>)
    // corrupt libclang's type resolution for ALL subsequent headers in the batch,
    // causing template types to silently misresolve to `int`. Fail loudly.
    let diagnostics = tu.get_diagnostics();
    let mut fatal_errors = Vec::new();
    for diag in &diagnostics {
        let severity = diag.get_severity();
        if severity == clang::diagnostic::Severity::Fatal {
            fatal_errors.push(diag.get_text());
        }
        if severity >= clang::diagnostic::Severity::Error && verbose {
            eprintln!("  Parse error: {}", diag.get_text());
        }
    }
    if !fatal_errors.is_empty() {
        let mut msg = format!(
            "Clang encountered {} fatal error(s) during batch parsing.\n\
             Fatal errors corrupt type resolution for all subsequent headers.\n\
             Fix: add the offending header(s) to `exclude_headers` in bindings.toml.\n\
             Fatal errors:",
            fatal_errors.len()
        );
        for err in &fatal_errors {
            msg.push_str(&format!("\n  - {}", err));
        }
        anyhow::bail!(msg);
    }

    // Initialize results - one ParsedHeader per input header
    let mut results: Vec<ParsedHeader> = headers
        .iter()
        .map(|h| ParsedHeader {
            path: h.as_ref().to_path_buf(),
            classes: Vec::new(),
            enums: Vec::new(),
            functions: Vec::new(),
        })
        .collect();

    // Build a map from filename to index for fast lookup
    // Use filename matching because wrapper headers include real source files
    let filename_to_index: std::collections::HashMap<&str, usize> = header_paths
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name, i))
        })
        .collect();

    // Walk the AST once, distributing entities to the appropriate header
    let visit_start = Instant::now();
    let root = tu.get_entity();
    

    // Extract included module names from the headers list.
    // Module name is the prefix before the first underscore in the filename
    // (e.g. "gp" from "gp_Vec3f.hxx", "Geom" from "Geom_Curve.hxx").
    let included_modules: HashSet<String> = headers
        .iter()
        .filter_map(|h| {
            let filename = h.as_ref().file_name()?.to_str()?;
            let stem = filename.strip_suffix(".hxx").unwrap_or(filename);
            stem.split('_').next().map(|s| s.to_string())
        })
        .collect();

    // Pre-scan AST to collect NCollection template typedef mappings.
    // This must happen before class/method parsing so parse_type() can
    // resolve template types back to their typedef names.
    collect_ncollection_typedefs(&root, &included_modules);

    // Pre-scan AST to collect simple (non-template) typedefs that alias other classes.
    // This must happen before class/method parsing so parse_type() can resolve
    // typedef names like BinObjMgt_SRelocationTable -> TColStd_IndexedMapOfTransient.
    collect_simple_typedefs(&root);

    root.visit_children(|entity, _parent| {
        visit_top_level_batch(&entity, &header_set, &filename_to_index, &mut results, verbose)
    });
    let visit_time = visit_start.elapsed();

    eprintln!("\nTiming summary:");
    eprintln!("  Total clang parse time: {:.2}s", parse_time.as_secs_f64());
    eprintln!("  Total AST visit time: {:.2}s", visit_time.as_secs_f64());

    Ok(results)
}

/// Get the canonical path of the file an entity is located in
fn get_entity_file(entity: &Entity) -> Option<std::path::PathBuf> {
    let location = entity.get_location()?;
    let file = location.get_file_location().file?;
    let entity_path = file.get_path();
    entity_path.canonicalize().ok().or(Some(entity_path))
}

/// Get the source line number for an entity
fn get_entity_line(entity: &Entity) -> Option<u32> {
    let location = entity.get_location()?;
    Some(location.get_file_location().line)
}

/// Resolve an entity to its header index by matching its source file name.
/// Returns `(index, entity_file_path)` if the entity belongs to a target header,
/// or `None` if the entity is from a non-target file or has no location.
fn resolve_header_index<'a>(
    entity: &Entity,
    filename_to_index: &std::collections::HashMap<&str, usize>,
) -> Option<(usize, std::path::PathBuf)> {
    let entity_file = get_entity_file(entity)?;
    let filename = entity_file.file_name().and_then(|n| n.to_str())?;
    let &index = filename_to_index.get(filename)?;
    Some((index, entity_file))
}
/// Visit top-level entities for batch parsing
/// Distributes entities to the appropriate ParsedHeader based on source file
fn visit_top_level_batch(
    entity: &Entity,
    _header_set: &std::collections::HashSet<&Path>,
    filename_to_index: &std::collections::HashMap<&str, usize>,
    results: &mut [ParsedHeader],
    verbose: bool,
) -> EntityVisitResult {
    let (index, entity_file) = match resolve_header_index(entity, filename_to_index) {
        Some(resolved) => resolved,
        None => {
            // Not from our target headers - but might need to recurse into namespaces
            // because namespace declarations span multiple files
            if entity.get_kind() == EntityKind::Namespace && entity.get_name().as_deref() != Some("std") {
                let namespace_name = entity.get_name().unwrap_or_default();
                entity.visit_children(|child, _| {
                    visit_namespace_member_batch(&child, filename_to_index, &namespace_name, results, verbose)
                });
            }
            return EntityVisitResult::Continue;
        }
    };

    match entity.get_kind() {
        EntityKind::ClassDecl | EntityKind::StructDecl => {
            let parsed_classes = parse_class(entity, &entity_file.file_name().unwrap_or_default().to_string_lossy(), verbose);
            results[index].classes.extend(parsed_classes);
        }
        EntityKind::EnumDecl => {
            if let Some(parsed) = parse_enum(entity, &entity_file.file_name().unwrap_or_default().to_string_lossy(), verbose) {
                results[index].enums.push(parsed);
            }
        }
        EntityKind::Namespace => {
            // Don't recurse into std namespace
            if entity.get_name().as_deref() != Some("std") {
                let namespace_name = entity.get_name().unwrap_or_default();
                entity.visit_children(|child, _| {
                    visit_namespace_member_batch(&child, filename_to_index, &namespace_name, results, verbose)
                });
            }
        }
        _ => {}
    }

    EntityVisitResult::Continue
}

/// Visit members of a namespace for batch parsing
fn visit_namespace_member_batch(
    entity: &Entity,
    filename_to_index: &std::collections::HashMap<&str, usize>,
    namespace: &str,
    results: &mut [ParsedHeader],
    verbose: bool,
) -> EntityVisitResult {
    let (index, entity_file) = match resolve_header_index(entity, filename_to_index) {
        Some(resolved) => resolved,
        None => return EntityVisitResult::Continue,
    };

    if entity.get_kind() == EntityKind::FunctionDecl {
        // Skip deprecated functions
        if entity.get_availability() == Availability::Deprecated {
            return EntityVisitResult::Continue;
        }
        if let Some(parsed) = parse_function(entity, namespace, &entity_file.file_name().unwrap_or_default().to_string_lossy(), verbose) {
            results[index].functions.push(parsed);
        }
    }

    EntityVisitResult::Continue
}

/// Parse a class or struct declaration.
/// Returns a vector because nested classes/structs defined inside the class
/// are also returned (qualified as `Parent::Nested`).
fn parse_class(entity: &Entity, source_header: &str, verbose: bool) -> Vec<ParsedClass> {
    let name = match entity.get_name() {
        Some(n) => n,
        None => return Vec::new(),
    };

    // Skip forward declarations (no definition)
    if !entity.is_definition() {
        return Vec::new();
    }

    // Skip anonymous classes/structs
    if name.is_empty() {
        return Vec::new();
    }

    // Skip internal/private classes (those starting with underscore)
    if name.starts_with('_') {
        return Vec::new();
    }

    // Skip template classes and template specializations
    // Template classes have get_template() returning Some, or get_template_kind() returning Some
    // Also skip if the display name contains angle brackets (indicates template instantiation)
    if entity.get_template().is_some() {
        if verbose {
            println!("    Skipping {} (template class)", name);
        }
        return Vec::new();
    }
    let display_name = entity.get_display_name().unwrap_or_default();
    if display_name.contains('<') {
        if verbose {
            println!("    Skipping {} (template specialization)", display_name);
        }
        return Vec::new();
    }

    // Skip policy/trait classes used as template parameters
    // These are not meant to be instantiated directly
    if name.contains("Inspector") || name.contains("_Hasher") || name.contains("_Traits") {
        if verbose {
            println!("    Skipping {} (policy/trait class)", name);
        }
        return Vec::new();
    }

    // Skip internal node types that use custom allocators (can't be used with std::unique_ptr)
    if name.ends_with("Node") && name.starts_with("NCollection_") {
        if verbose {
            println!("    Skipping {} (internal node type)", name);
        }
        return Vec::new();
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    // Extract direct base classes for upcast generation
    let base_classes = extract_base_classes(entity);
    
    // Check for protected/private destructor (indicates non-instantiable abstract base class)
    let has_protected_destructor = check_protected_destructor(entity);

    if verbose {
        println!("  Parsing class: {}", name);
        if !base_classes.is_empty() {
            println!("    Base classes: {:?}", base_classes);
        }
        if has_protected_destructor {
            println!("    Has protected destructor (non-instantiable)");
        }
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();
    let mut fields: Vec<ParsedField> = Vec::new();
    let mut has_non_public_fields = false;
    let mut has_virtual_methods = false;
    let mut all_method_names = std::collections::HashSet::new();
    let mut is_abstract = false;
    let mut pure_virtual_methods = std::collections::HashSet::new();
    let mut has_explicit_constructors = false;
    // Track copy constructor: None = no explicit copy ctor seen,
    // Some(true) = public non-deleted copy ctor, Some(false) = deleted/non-public copy ctor
    let mut has_copy_constructor: Option<bool> = None;
    let mut has_move_constructor = false;
    let mut nested_classes: Vec<ParsedClass> = Vec::new();

    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);


    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Continue;
        }

        match child.get_kind() {
            EntityKind::Constructor => {
                // Any explicit constructor means C++ won't generate an implicit default
                has_explicit_constructors = true;

                // Detect copy constructors via libclang
                if child.is_copy_constructor() {
                    let is_available = child.get_availability() != Availability::Unavailable;
                    let is_pub = is_public(&child);
                    // Also check that the copy ctor takes a const reference (const T&),
                    // not a mutable reference (T&). Our to_owned wrapper uses
                    // `const T*` so non-const copy ctors won't compile.
                    let takes_const_ref = child.get_arguments()
                        .and_then(|args| args.first().and_then(|arg| arg.get_type()))
                        .and_then(|ty| ty.get_pointee_type())
                        .map(|pointee| pointee.is_const_qualified())
                        .unwrap_or(true); // Default to true if we can't determine
                    if is_available && is_pub && takes_const_ref {
                        has_copy_constructor = Some(true);
                    } else if has_copy_constructor.is_none() {
                        // Deleted, non-public, or non-const copy constructor
                        has_copy_constructor = Some(false);
                    }
                    // Don't add copy constructors to the regular constructors list
                    return EntityVisitResult::Continue;
                }

                // Detect move constructors — these suppress implicit copy constructors
                if child.is_move_constructor() {
                    has_move_constructor = true;
                    // Don't add move constructors to the regular constructors list
                    return EntityVisitResult::Continue;
                }

                // Skip deprecated constructors
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        println!("    Skipping deprecated constructor for {}", name);
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if let Some(ctor) = parse_constructor(&child, verbose) {
                        constructors.push(ctor);
                    }
                }
            }
            EntityKind::Method => {
                // Check for virtual methods (affects POD detection)
                if child.is_virtual_method() {
                    has_virtual_methods = true;
                }
                // Check if this is a pure virtual method (makes the class abstract)
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                    if let Some(ref method_name) = child.get_name() {
                        pure_virtual_methods.insert(method_name.clone());
                    }
                }

                // Always track all method names (even if not public or skipped) - used for abstract class detection
                if let Some(ref method_name) = child.get_name() {
                    all_method_names.insert(method_name.clone());
                }

                // Skip destructors, operators, and conversion functions from binding generation
                if let Some(ref method_name) = child.get_name() {
                    if method_name.starts_with('~')
                        || method_name.starts_with("operator")
                        || method_name == "DumpJson"
                        || method_name == "InitFromJson"
                    {
                        return EntityVisitResult::Continue;
                    }
                }

                // Skip deprecated methods
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        if let Some(ref method_name) = child.get_name() {
                            println!("    Skipping deprecated method {}::{}", name, method_name);
                        }
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if child.is_static_method() {
                        if let Some(method) = parse_static_method(&child, verbose) {
                            static_methods.push(method);
                        }
                    } else if let Some(method) = parse_method(&child, verbose) {
                        methods.push(method);
                    }
                }
            }
            EntityKind::FieldDecl => {
                if is_public(&child) {
                    if let Some(field) = parse_field(&child, verbose) {
                        fields.push(field);
                    }
                } else {
                    has_non_public_fields = true;
                }
            }
            EntityKind::ClassDecl | EntityKind::StructDecl => {
                // Use tracked access level (not get_accessibility, which is unreliable for structs)
                let is_nested_public = current_access.get() == Accessibility::Public;
                // Parse nested classes/structs defined inside this class
                if is_nested_public && child.is_definition() {
                    let mut parsed = parse_class(&child, source_header, verbose);
                    // Collect the original (unqualified) names of sibling nested classes
                    // so we can fix base class references after qualification.
                    let sibling_names: HashSet<String> = parsed.iter().map(|c| c.name.clone()).collect();
                    for nested in &mut parsed {
                        // Qualify the nested class name with parent: Parent::Nested
                        // Always prepend since multi-level nesting (A::B::C) needs all levels.
                        nested.name = format!("{}::{}", name, nested.name);
                        // nested.module is inherited from source_header
                        // Also qualify base class names that reference sibling nested classes.
                        // Without this, the inheritance graph can't connect e.g.
                        // ShapePersistent_BRep::Curve3D -> GCurve (should be
                        // ShapePersistent_BRep::GCurve) for handle-able class computation.
                        for base in &mut nested.base_classes {
                            if sibling_names.contains(base.as_str()) {
                                *base = format!("{}::{}", name, base);
                            }
                        }
                    }
                    nested_classes.extend(parsed);
                }
            }
            EntityKind::Destructor => {
                // A pure virtual destructor (`virtual ~Foo() = 0`) makes the
                // class abstract even though no non-destructor pure virtuals
                // exist. libclang reports this as Destructor (not Method), so
                // we must check it separately.
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                }
            }
            EntityKind::UsingDeclaration => {
                // `using Base::Method;` in a non-public section narrows the
                // inherited method's access, hiding it from derived classes.
                // Record the name so the inheritance logic treats it as overridden.
                if !is_public(&child) {
                    if let Some(ref using_name) = child.get_name() {
                        all_method_names.insert(using_name.clone());
                    }
                }
            }
            EntityKind::EnumDecl => {
                // A public enum with the same name as an inherited method
                // shadows that method in C++ (e.g. AIS_PointCloud::DisplayMode
                // enum hides PrsMgr_PresentableObject::DisplayMode() method).
                if let Some(ref enum_name) = child.get_name() {
                    if !enum_name.is_empty() {
                        all_method_names.insert(enum_name.clone());
                    }
                }
            }
            EntityKind::FunctionTemplate => {
                // Template methods hide base class methods with the same name
                // (C++ name-hiding rule). Record the name so the inheritance
                // logic blocks the parent's non-template overloads.
                if let Some(ref tmpl_name) = child.get_name() {
                    all_method_names.insert(tmpl_name.clone());
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });

    // Only return classes that have something to bind
    if constructors.is_empty() && methods.is_empty() && static_methods.is_empty() && fields.is_empty() && nested_classes.is_empty() {
        if verbose {
            println!("    Skipping {} (no bindable members)", name);
        }
        return Vec::new();
    }

    // Determine if this is a POD struct:
    // - Has public fields
    // - No non-public fields
    // - No virtual methods (no vtable)
    // - No base classes
    // - All field types are POD-compatible primitives (possibly in fixed-size arrays)
    // - Not abstract
    let is_pod_struct = !fields.is_empty()
        && !has_non_public_fields
        && !has_virtual_methods
        && base_classes.is_empty()
        && !is_abstract
        && fields.iter().all(|f| f.ty.is_pod_field_type());

    if verbose && is_pod_struct {
        println!("    POD struct detected: {} ({} fields)", name, fields.len());
    }

    let mut result = vec![
        ParsedClass {
            name: name.clone(),
            module: module.clone(),
            comment,
            source_header: source_header.to_string(),
            source_line: get_entity_line(entity),
            constructors,
            methods,
            static_methods,
            all_method_names,
            base_classes,
            has_protected_destructor,
            is_abstract,
            pure_virtual_methods,
            has_explicit_constructors,
            fields,
            is_pod_struct,
            has_copy_constructor,
            has_move_constructor,
        },
    ];

    // Append nested classes to the result
    for nested in nested_classes {
        result.push(nested);
    }

    result
}
/// Check if a class has a protected or private destructor
/// Classes with non-public destructors cannot be directly instantiated via the FFI
fn check_protected_destructor(entity: &Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::Destructor {
            // Check if the destructor is not public
            if let Some(accessibility) = child.get_accessibility() {
                return accessibility != clang::Accessibility::Public;
            }
        }
    }
    false
}

/// Collect enum variants (EnumConstantDecl children) from an enum entity.
/// Used by both `parse_enum` and `parse_anonymous_enum`.
fn collect_enum_variants(entity: &Entity, verbose: bool) -> Vec<EnumVariant> {
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
    variants
}

/// Parse an enum declaration
fn parse_enum(entity: &Entity, source_header: &str, verbose: bool) -> Option<ParsedEnum> {
    let raw_name = entity.get_name();
    let name = match raw_name {
        Some(ref n) if !n.is_empty() && !n.starts_with("(unnamed") => n.clone(),
        _ => {
            // Anonymous enum - try to derive a name from variant common prefix
            return parse_anonymous_enum(entity, source_header, verbose);
        }
    };

    // Skip internal enums
    if name.starts_with('_') {
        return None;
    }

    // Skip nested enums (enums defined inside a class/struct)
    // These are not accessible at global scope
    if let Some(parent) = entity.get_semantic_parent() {
        let parent_kind = parent.get_kind();
        if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
            if verbose {
                println!("    Skipping {} (nested enum inside class)", name);
            }
            return None;
        }
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    if verbose {
        println!("  Parsing enum: {}", name);
    }

    let variants = collect_enum_variants(entity, verbose);

    if variants.is_empty() {
        return None;
    }

    Some(ParsedEnum {
        name,
        module,
        comment,
        source_header: source_header.to_string(),
        variants,
    })
}

/// Parse an anonymous enum by deriving a name from the common prefix of its variants.
///
/// OCCT uses a pattern where a `typedef Standard_Integer Foo` is followed by an anonymous
/// enum whose variants are all prefixed with `Foo_`. For example:
///
/// ```cpp
/// typedef Standard_Integer Graphic3d_ZLayerId;
/// enum {
///   Graphic3d_ZLayerId_UNKNOWN = -1,
///   Graphic3d_ZLayerId_Default = 0,
///   ...
/// };
/// ```
///
/// We detect this pattern and synthesize a named enum `Graphic3d_ZLayerId` from the
/// anonymous enum's variants.
fn parse_anonymous_enum(entity: &Entity, source_header: &str, verbose: bool) -> Option<ParsedEnum> {
    let variants = collect_enum_variants(entity, verbose);

    if variants.is_empty() {
        return None;
    }

    // Find the longest common prefix of all variant names.
    // The prefix must end with '_' and have at least one '_' (OCCT naming: Module_Name_VARIANT).
    let variant_names: Vec<String> = variants.iter().map(|v| v.name.clone()).collect();
    let common_prefix = longest_common_prefix(&variant_names);

    // The common prefix should end with '_' and contain at least one '_' before the trailing one
    // (i.e., it should look like "Module_Name_" not just "X_")
    let trimmed_prefix = common_prefix.trim_end_matches('_');
    if trimmed_prefix.is_empty() || !trimmed_prefix.contains('_') || !common_prefix.ends_with('_') {
        if verbose {
            println!("    Skipping anonymous enum (no suitable common prefix: {:?})", common_prefix);
        }
        return None;
    }

    // The enum name is the common prefix without the trailing '_'
    let enum_name = trimmed_prefix.to_string();
    let module = extract_module_from_header(source_header);

    if verbose {
        println!("  Parsing anonymous enum as: {} ({} variants)", enum_name, variant_names.len());
    }

    // Extract the doxygen comment from above the enum (if any)
    let comment = extract_doxygen_comment(entity);

    Some(ParsedEnum {
        name: enum_name,
        module,
        comment,
        source_header: source_header.to_string(),
        variants,
    })
}

/// Find the longest common prefix of a slice of strings.
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = &strings[0];
    let mut prefix_len = first.len();
    for s in &strings[1..] {
        prefix_len = prefix_len.min(s.len());
        for (i, (a, b)) in first.chars().zip(s.chars()).enumerate() {
            if a != b {
                prefix_len = prefix_len.min(i);
                break;
            }
        }
    }
    first[..prefix_len].to_string()
}

/// Parse a namespace-level function declaration
fn parse_function(entity: &Entity, namespace: &str, source_header: &str, verbose: bool) -> Option<ParsedFunction> {
    let name = entity.get_name()?;

    // Skip template functions
    if entity.get_template().is_some() {
        return None;
    }

    // Get the function's result type
    let result_type = entity.get_result_type()?;
    let return_type = parse_type(&result_type);

    // Parse parameters
    let mut params = Vec::new();
    for arg in entity.get_arguments().unwrap_or_default() {
        let param_name = arg.get_name().unwrap_or_else(|| format!("arg{}", params.len()));
        if let Some(param_type) = arg.get_type() {
            let has_default = !arg.get_children().is_empty();
            params.push(Param {
                name: param_name,
                ty: parse_type(&param_type),
                has_default,
                default_value: None,
            });
        }
    }

    let comment = extract_doxygen_comment(entity);
    let full_name = format!("{}::{}", namespace, name);
    let module = namespace.to_string();

    if verbose {
        println!("  Parsing function: {}", full_name);
    }

    Some(ParsedFunction {
        name: full_name,
        namespace: namespace.to_string(),
        short_name: name,
        module,
        comment,
        source_header: source_header.to_string(),
        source_line: get_entity_line(entity),
        params,
        return_type: Some(return_type),
    })
}


/// Extract direct base classes from an entity (only public base classes)
fn extract_base_classes(entity: &Entity) -> Vec<String> {
    let mut base_classes = Vec::new();
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::BaseSpecifier {
            // Only include public base classes - protected/private bases can't be upcast to
            let accessibility = child.get_accessibility();
            if accessibility != Some(Accessibility::Public) {
                continue;
            }
            
            if let Some(base_type) = child.get_type() {
                let base_name = base_type.get_display_name();
                // Only include OCCT classes (those with underscore prefix pattern)
                if !base_name.contains('_') {
                    continue;
                }
                // Template base classes (e.g. BVH_PairTraverse<Standard_Real, 3>)
                // can't be used directly as type names. Try to resolve via the
                // typedef map (e.g. BVH_PrimitiveSet<double, 3> -> BVH_PrimitiveSet3d).
                // If no typedef is found, skip the base — the concrete class's own
                // methods are still fully usable, only upcasts to the template base
                // are lost.
                if base_name.contains('<') {
                    if let Some(typedef_name) = lookup_typedef(&base_name) {
                        base_classes.push(typedef_name);
                    }
                    // else: no typedef found, skip this template base
                } else {
                    base_classes.push(base_name);
                }
            }
        }
    }
    base_classes
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
            .collect();

        if cleaned.iter().all(|line| line.is_empty()) {
            return None;
        }

        // Preserve newlines in the comment for proper formatting
        return Some(cleaned.join("\n"));
    }
    None
}

/// Extract module name from OCCT header filename (e.g., "gp_Pnt.hxx" -> "gp")
///
/// Module names are derived from the header file, not the class/type name.
/// This avoids mismatches for types like Fortran common blocks (e.g., `maovpar_1_`
/// in `AdvApp2Var_Data.hxx`) and helper classes that don't follow the standard
/// `Module_Class` naming convention.
fn extract_module_from_header(header: &str) -> String {
    // Strip .hxx extension first
    let name = header.strip_suffix(".hxx").unwrap_or(header);
    // OCCT naming convention: ModuleName_ClassName.hxx
    // Examples: gp_Pnt.hxx -> "gp", TopoDS_Shape.hxx -> "TopoDS"
    if let Some(underscore_pos) = name.find('_') {
        name[..underscore_pos].to_string()
    } else {
        // No underscore - single-word module (e.g., "gp.hxx" -> "gp")
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
    let source_line = get_entity_line(entity);

    if verbose {
        let param_str = params
            .iter()
            .map(|p| {
                let default_str = if p.has_default { " [default]" } else { "" };
                format!("{}: {:?}{}", p.name, p.ty, default_str)
            })
            .collect::<Vec<_>>()
            .join(", ");
        println!("    Constructor({})", param_str);
    }

    Some(Constructor { comment, params, source_line })
}

/// Parse an instance method
fn parse_method(entity: &Entity, verbose: bool) -> Option<Method> {
    let name = entity.get_name()?;
    let comment = extract_doxygen_comment(entity);
    let is_const = entity.is_const_method();
    let params = parse_params(entity);
    let return_type = parse_return_type(entity);
    let source_line = get_entity_line(entity);

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
        source_line,
    })
}

/// Parse a public data member (field) declaration
fn parse_field(entity: &Entity, verbose: bool) -> Option<ParsedField> {
    let name = entity.get_name()?;
    let field_type = entity.get_type()?;
    let comment = extract_doxygen_comment(entity);

    // Check if this is a fixed-size array (e.g., `Standard_Boolean myPeriodic[3]`)
    let (base_type, array_size) = if field_type.get_kind() == TypeKind::ConstantArray {
        let element_type = field_type.get_element_type()
            .expect("ConstantArray should have element type");
        let size = field_type.get_size()
            .expect("ConstantArray should have size");
        (parse_type(&element_type), Some(size))
    } else {
        (parse_type(&field_type), None)
    };

    if verbose {
        if let Some(sz) = array_size {
            println!("    Field: {} : {:?}[{}]", name, base_type, sz);
        } else {
            println!("    Field: {} : {:?}", name, base_type);
        }
    }

    Some(ParsedField {
        name,
        ty: base_type,
        array_size,
        comment,
    })
}


/// Parse a static method
fn parse_static_method(entity: &Entity, verbose: bool) -> Option<StaticMethod> {
    let name = entity.get_name()?;
    let comment = extract_doxygen_comment(entity);
    let params = parse_params(entity);
    let return_type = parse_return_type(entity);
    let source_line = get_entity_line(entity);

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
        source_line,
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
            // Detect default values: a ParmDecl has a default if it has expression
            // children (DeclRefExpr, UnexposedExpr, IntegerLiteral, etc.).
            // TypeRef, NamespaceRef, TemplateRef are just type-related and don't
            // indicate defaults.
            let children = param.get_children();
            let has_default = children.iter().any(|c| {
                !matches!(
                    c.get_kind(),
                    EntityKind::TypeRef | EntityKind::NamespaceRef | EntityKind::TemplateRef
                )
            });
            let default_value = if has_default {
                extract_default_value(&param)
            } else {
                None
            };
            Some(Param {
                name,
                ty: parse_type(&param_type),
                has_default,
                default_value,
            })
        })
        .collect()
}

/// Extract a default value from a parameter's AST children as a Rust literal expression.
/// Recursively walks through wrapper nodes (UnexposedExpr, CStyleCastExpr, etc.)
/// to find the actual literal.
fn extract_default_value(param: &Entity) -> Option<String> {
    for child in param.get_children() {
        if let Some(val) = extract_default_from_expr(&child) {
            return Some(val);
        }
    }
    // Fallback: for macro-expanded literals (e.g., Standard_False → false),
    // the individual expression node may not have usable source ranges.
    // Try tokenizing the entire ParmDecl to find `= <value>` pattern.
    if let Some(range) = param.get_range() {
        let tokens = range.tokenize();
        let spellings: Vec<String> = tokens.iter().map(|t| t.get_spelling()).collect();
        // Look for "=" followed by a value token
        if let Some(eq_pos) = spellings.iter().position(|s| s == "=") {
            if eq_pos + 1 < spellings.len() {
                let val = &spellings[eq_pos + 1];
                match val.as_str() {
                    "true" | "Standard_True" => return Some("true".to_string()),
                    "false" | "Standard_False" => return Some("false".to_string()),
                    _ => {
                        // Could be an integer or float literal
                        if val.parse::<i64>().is_ok() || val.parse::<u64>().is_ok() {
                            return Some(val.clone());
                        }
                        if val.parse::<f64>().is_ok() {
                            return Some(val.clone());
                        }
                        // Check for negative literal: = - <number>
                        if val == "-" && eq_pos + 2 < spellings.len() {
                            let next = &spellings[eq_pos + 2];
                            if next.parse::<i64>().is_ok() || next.parse::<f64>().is_ok() {
                                return Some(format!("-{}", next));
                            }
                        }
                    }
                }
            }
        }
    }
    // Debug: print AST for params where we expected a default but couldn't extract it
    if std::env::var("BINDGEN_DEBUG_DEFAULTS").is_ok() {
        eprintln!("  [default-debug] Could not extract default for param {:?}", param.get_name());
    }
    None
}

/// Recursively extract a literal value from an expression AST node.
fn extract_default_from_expr(expr: &Entity) -> Option<String> {
    use clang::EntityKind::*;
    match expr.get_kind() {
        IntegerLiteral => {
            if let Some(range) = expr.get_range() {
                let tokens = range.tokenize();
                if let Some(tok) = tokens.first() {
                    return Some(tok.get_spelling());
                }
            }
            None
        }
        FloatingLiteral => {
            if let Some(range) = expr.get_range() {
                let tokens = range.tokenize();
                if let Some(tok) = tokens.first() {
                    let text = tok.get_spelling();
                    // Ensure it has a decimal point for Rust
                    if text.contains('.') {
                        return Some(text);
                    } else {
                        return Some(format!("{}.0", text));
                    }
                }
            }
            None
        }
        BoolLiteralExpr => {
            // Try tokenization (works for non-macro-expanded bool literals)
            if let Some(range) = expr.get_range() {
                let tokens = range.tokenize();
                if let Some(tok) = tokens.first() {
                    let text = tok.get_spelling();
                    return match text.as_str() {
                        "true" => Some("true".to_string()),
                        "false" => Some("false".to_string()),
                        _ => None,
                    };
                }
            }
            // For macro-expanded bool literals (Standard_False, Standard_True),
            // tokenization fails. Return None here; the fallback in
            // extract_default_value will handle it by tokenizing the parent ParmDecl.
            None
        }
        NullPtrLiteralExpr => Some("std::ptr::null()".to_string()),
        // Wrapper expressions — look through them to find the actual literal
        UnexposedExpr | ParenExpr | CStyleCastExpr => {
            for child in expr.get_children() {
                if let Some(val) = extract_default_from_expr(&child) {
                    return Some(val);
                }
            }
            None
        }
        UnaryOperator => {
            // Check if it's a negation of a literal (e.g. -1)
            if let Some(range) = expr.get_range() {
                let tokens = range.tokenize();
                let texts: Vec<String> = tokens.iter().map(|t| t.get_spelling()).collect();
                if texts.len() >= 2 && texts[0] == "-" {
                    return Some(format!("-{}", texts[1]));
                }
            }
            None
        }
        _ => None,
    }
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

fn parse_fixed_array_size(type_spelling: &str) -> Option<usize> {
    let lb = type_spelling.rfind('[')?;
    let rb = type_spelling[lb..].find(']')? + lb;
    if rb <= lb + 1 {
        return None;
    }
    type_spelling[lb + 1..rb].trim().parse::<usize>().ok()
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

    // Check if this is a typedef to size_t by examining the declaration
    // This catches cases where get_display_name() returns the canonical type
    if let Some(decl) = clang_type.get_declaration() {
        if let Some(decl_name) = decl.get_name() {
            if decl_name == "size_t" || decl_name.ends_with("_Size") {
                return Type::Usize;
            }
        }
    }

    // Check if this is a known NCollection/template typedef.
    // When clang desugars types (especially through references/pointers),
    // the display name may show the raw template form instead of the typedef.
    // E.g., "NCollection_Map<TDF_Label, NCollection_DefaultHasher<TDF_Label>>"
    // instead of "TDF_LabelMap". Look up the typedef name from our pre-scanned map.
    let clean_for_lookup = strip_type_qualifiers(trimmed_spelling);
    if clean_for_lookup.contains('<') && !clean_for_lookup.starts_with("opencascade::handle<") && !clean_for_lookup.starts_with("Handle(") {
        if let Some(typedef_name) = lookup_typedef(clean_for_lookup) {
            return Type::Class(typedef_name);
        }
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = strip_type_qualifiers(&canonical_spelling);
    // Defense-in-depth: detect when clang's canonical type is a primitive (int, double, etc.)
    // but the display name clearly identifies a class/typedef. This can happen if a template
    // type fails to instantiate. Legitimate typedefs to primitives (e.g.,
    // `typedef unsigned int Poly_MeshPurpose`) use a typedef chain to a builtin type.
    let spelling_looks_like_class = {
        let s = strip_type_qualifiers(trimmed_spelling);
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && map_standard_type(s).is_none()
            && s != "Standard_Boolean"
            && !s.contains('<')
            && !s.contains("::");

        if !looks_like_class {
            false
        } else {
            // Check if this is a typedef whose underlying type is a primitive.
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int).
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            // The underlying type of a typedef chain (e.g., Graphic3d_ZLayerId -> Standard_Integer -> int)
            // may appear as Elaborated rather than Typedef, so we accept both.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| {
                        let uk = u.get_kind();
                        is_c_primitive_type_kind(uk)
                        || matches!(uk, TypeKind::Typedef | TypeKind::Elaborated)  // chain through another typedef
                    })
                    .unwrap_or(false);

            !is_primitive_typedef
        }
    };

    // Handle primitives via canonical type.
    // Skip this if:
    // 1. The spelling clearly identifies a class type (spelling_looks_like_class), OR
    // 2. The spelling contains '<' or '::' — template or namespace-scoped types
    //    whose canonical resolves to int/double/etc. should not be treated as primitives.
    let spelling_is_template_or_namespaced = {
        let s = strip_type_qualifiers(trimmed_spelling);
        s.contains('<') || s.contains("::")
    };
    if !spelling_looks_like_class && !spelling_is_template_or_namespaced {

        if let Some(ty) = map_standard_type(canonical_clean) {
            return ty;
        }
    }

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int", construct the class type directly instead of recursing
    // into the pointee (whose display name might already be "int", losing the
    // typedef info).
    //
    // Exception: genuine primitive typedefs (e.g., MeshVS_DisplayModeFlags = Standard_Integer = int)
    // should NOT be intercepted — let them fall through to normal reference handling
    // which will resolve them to the primitive type through canonical resolution.
    if kind == TypeKind::LValueReference || kind == TypeKind::RValueReference || kind == TypeKind::Pointer

    {
        let canonical_base = strip_type_decorators(canonical_clean);
        if canonical_base == "int" {
            // Strip qualifiers and ref/ptr decorators from the outer display name
            let base = strip_type_decorators(trimmed_spelling);
            let base_looks_like_class = base.starts_with(|c: char| c.is_ascii_uppercase())
                && map_standard_type(base).is_none()
                && base != "Standard_Boolean"
                && !base.contains(' ');
            // Also handle template/namespaced types (e.g. "NCollection_Map<...>" or
            // "IMeshData::IMapOfReal") — these are clearly not primitives.
            let base_looks_like_type = base_looks_like_class
                || base.contains('<')
                || base.contains("::");
            if base_looks_like_type {
                // For simple typedef names (not template/namespaced), check if the
                // pointee's canonical type is actually a primitive. If so, this is a
                // genuine typedef-to-primitive (e.g., MeshVS_DisplayModeFlags = int)
                // and should be resolved normally, not preserved as a class name.
                let is_template_or_ns = base.contains('<') || base.contains("::");
                let pointee_is_primitive_canonical = !is_template_or_ns
                    && clang_type.get_pointee_type().map(|pt| {
                        is_c_primitive_type_kind(pt.get_canonical_type().get_kind())
                    }).unwrap_or(false);

                if !pointee_is_primitive_canonical {
                    let inner = Type::Class(base.to_string());
                    if let Some(pointee) = clang_type.get_pointee_type() {
                        let is_const = pointee.is_const_qualified();
                        return match kind {
                            TypeKind::LValueReference if is_const => Type::ConstRef(Box::new(inner)),
                            TypeKind::LValueReference => Type::MutRef(Box::new(inner)),
                            TypeKind::RValueReference => Type::RValueRef(Box::new(inner)),
                            TypeKind::Pointer if is_const => Type::ConstPtr(Box::new(inner)),
                            TypeKind::Pointer => Type::MutPtr(Box::new(inner)),
                            _ => inner,
                        };
                    }
                    return inner;
                }
                // Genuine primitive typedef — fall through to normal reference handling
            }
        }
    }


    // Handle reference types

    if kind == TypeKind::LValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let pk = pointee.get_kind();
            // Reference-to-fixed-array (e.g., int(&)[3]) is represented as
            // ConstRef/MutRef(FixedArray(elem, N)) and lowered in the wrapper layer
            // to `elem const*` / `elem*` plus a reinterpret_cast to `elem (&)[N]`.
            if pk == TypeKind::ConstantArray {
                let is_const = pointee.is_const_qualified();
                let arr_display = pointee.get_display_name();
                if let Some(elem) = pointee.get_element_type() {
                    let elem_ty = parse_type(&elem);
                    let arr_ty = if let Some(size) = parse_fixed_array_size(&arr_display) {
                        Type::FixedArray(Box::new(elem_ty), size)
                    } else {
                        Type::Class(arr_display)
                    };
                    return if is_const {
                        Type::ConstRef(Box::new(arr_ty))
                    } else {
                        Type::MutRef(Box::new(arr_ty))
                    };
                }
                return if is_const {
                    Type::ConstRef(Box::new(Type::Class(arr_display)))
                } else {
                    Type::MutRef(Box::new(Type::Class(arr_display)))
                };
            }
            // Incomplete array refs are not representable as fixed arrays.
            if pk == TypeKind::IncompleteArray {
                let arr_display = pointee.get_display_name();
                let is_const = pointee.is_const_qualified();
                let inner = Type::Class(arr_display);
                return if is_const {
                    Type::ConstRef(Box::new(inner))
                } else {
                    Type::MutRef(Box::new(inner))
                };
            }
            let is_const = pointee.is_const_qualified();
            let inner = parse_type(&pointee);
            return if is_const {
                Type::ConstRef(Box::new(inner))
            } else {
                Type::MutRef(Box::new(inner))
            };
        }
    }

    // Handle rvalue reference types (T&&) - not bindable but we need to parse them
    if kind == TypeKind::RValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let inner = parse_type(&pointee);
            return Type::RValueRef(Box::new(inner));
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

    // Handle C-style array types: ConstantArray (int[16]) and IncompleteArray (int[]).
    // In function parameters, arrays decay to pointers. Fixed-size array references
    // are handled above in the LValueReference branch.
    if kind == TypeKind::ConstantArray || kind == TypeKind::IncompleteArray {
        if let Some(elem) = clang_type.get_element_type() {
            let inner = parse_type(&elem);
            return Type::MutPtr(Box::new(inner));
        }
    }

    // Handle Handle<T> types (opencascade::handle<T>)
    // Check both the display name AND the canonical type spelling, because
    // namespace-scoped Handle typedefs (e.g., IMeshData::IEdgeHandle) have
    // a display name like "IMeshData::IEdgeHandle" but canonical type
    // "opencascade::handle<IMeshData_Edge>".
    // Be careful NOT to match function pointer typedefs whose return type is a Handle,
    // e.g., StdObjMgt_Persistent::Instantiator = Handle(StdObjMgt_Persistent) (*)()
    // has canonical "opencascade::handle<StdObjMgt_Persistent> (*)()". These are
    // detected by checking that the canonical ends with '>' (a pure handle type).
    let clean_spelling = spelling.trim_start_matches("const ").trim();
    let canonical_is_pure_handle = canonical_clean.starts_with("opencascade::handle<")
        && canonical_clean.ends_with('>');
    if clean_spelling.starts_with("opencascade::handle<") || clean_spelling.starts_with("Handle(")
        || canonical_is_pure_handle
    {
        // Prefer the canonical type spelling for the inner type name, because
        // clang's display name may use unqualified names for nested classes
        // (e.g., "Curve" instead of "ShapePersistent_BRep::Curve") when the
        // Handle appears in a method within the parent class scope.
        let inner = if canonical_is_pure_handle {
            extract_template_arg(canonical_clean)
        } else {
            extract_template_arg(clean_spelling)
        };
        return Type::Handle(inner);
    }

    // For nested types (e.g., TColgp_Array1OfPnt::value_type) or template types,
    // use the canonical type to get the resolved underlying type.
    // clang resolves these for us (e.g., value_type -> gp_Pnt)
    let clean_name = strip_type_decorators(&spelling);
    
    // If the spelling contains :: or < (nested/template type), try typedef map first,
    // then try to use canonical
    if clean_name.contains("::") || clean_name.contains('<') {
        // For template types, check if this is a known typedef
        if clean_name.contains('<') {
            if let Some(typedef_name) = lookup_typedef(clean_name) {
                return Type::Class(typedef_name);
            }
        }
        let canonical_base = strip_type_decorators(&canonical_spelling);
        
        // Only use canonical if it's simpler (no :: or <) AND still looks like a class name.
        // If canonical is a primitive like "int", that would produce Type::Class("int")
        // which is nonsensical. By keeping the template/namespaced spelling,
        // type_uses_unknown_type() will properly filter methods with unresolvable types.
        let canonical_looks_like_class = canonical_base
            .starts_with(|c: char| c.is_ascii_uppercase());
        if !canonical_base.contains("::") && !canonical_base.contains('<') && !canonical_base.is_empty() && canonical_looks_like_class {
            return Type::Class(canonical_base.to_string());
        }
    }
    
    // Check if this type's declaration is nested inside a class
    // This catches types like DESTEP_Parameters::ReadMode_ProductContext that
    // appear as "ReadMode_ProductContext" in method signatures but are actually nested
    // Only apply if clean_name is NOT already qualified (doesn't contain ::)
    if !clean_name.contains("::") {
        if let Some(decl) = clang_type.get_declaration() {
            if let Some(parent) = decl.get_semantic_parent() {
                let parent_kind = parent.get_kind();
                if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
                    // This is a nested type - include the parent class name to mark it as nested
                    if let Some(parent_name) = parent.get_name() {
                        let nested_name = format!("{}::{}", parent_name, clean_name);
                        return Type::Class(nested_name);
                    }
                }
            }
        }
    }

    // Check if this class name is actually a simple typedef for another class.
    // E.g., BinObjMgt_SRelocationTable -> TColStd_IndexedMapOfTransient,
    // XmlObjMgt_Element -> LDOM_Element, NCollection_String -> NCollection_Utf8String.
    // This must be done at the end, after all other type resolution has been attempted,
    // because reference/pointer wrapping strips the typedef TypeKind layer by the time
    // we recurse into the pointee type.
    if let Some(resolved_name) = lookup_simple_typedef(clean_name) {
        return Type::Class(resolved_name);
    }

    // Late-stage canonical resolution for unrecognized typedefs.
    // When a typedef's display name is unrecognized (not in map_standard_type, not a known
    // class, not a simple typedef), try resolving through clang's canonical type.
    // This handles:
    // - Pointer typedefs: Standard_PCharacter = char*, BOPDS_PDS = BOPDS_DS*
    // - Primitive typedefs not caught earlier (fallback for edge cases)
    if matches!(kind, TypeKind::Typedef | TypeKind::Elaborated) {
        let canon_kind = canonical.get_kind();

        // Pointer typedef: canonical is a pointer type.
        // Exclude function pointer typedefs (canonical pointee is a function type)
        // such as StdObjMgt_Persistent::Instantiator = Handle(...) (*)()
        if canon_kind == TypeKind::Pointer {
            if let Some(pointee) = canonical.get_pointee_type() {
                let pointee_kind = pointee.get_kind();
                if !matches!(pointee_kind, TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype) {
                    let is_const = pointee.is_const_qualified();
                    let inner = parse_type(&pointee);
                    return if is_const {
                        Type::ConstPtr(Box::new(inner))
                    } else {
                        Type::MutPtr(Box::new(inner))
                    };
                }
            }
        }

        // Primitive typedef: canonical is a C primitive type
        if is_c_primitive_type_kind(canon_kind) {
            if let Some(ty) = map_standard_type(canonical_clean) {
                return ty;
            }
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
    let clean = strip_type_qualifiers(type_name);

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::CHAR16),
        "Standard_Utf32Char" => Some(Type::U32),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::CHAR16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I16),
        "int16_t" => Some(Type::I16),
        "unsigned short" | "uint16_t" => Some(Type::U16),
        "char16_t" => Some(Type::CHAR16),
        "char32_t" => Some(Type::U32),
        "unsigned char" | "uint8_t" | "Standard_Byte" | "Standard_Utf8UChar" => Some(Type::U8),
        "signed char" | "int8_t" => Some(Type::I8),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - map both OCCT typedef names and bare C++ names to the
        // same Type::Class so they're recognized as known manual_types.
        "Standard_OStream" | "std::ostream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" | "std::istream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" | "std::stringstream" => Some(Type::Class("Standard_SStream".to_string())),
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
    fn test_extract_module_from_header() {
        assert_eq!(extract_module_from_header("gp_Pnt.hxx"), "gp");
        assert_eq!(extract_module_from_header("TopoDS_Shape.hxx"), "TopoDS");
        assert_eq!(extract_module_from_header("BRepPrimAPI_MakeBox.hxx"), "BRepPrimAPI");
        assert_eq!(extract_module_from_header("gp.hxx"), "gp");
        // Fortran common blocks in AdvApp2Var_Data.hxx get module "AdvApp2Var"
        assert_eq!(extract_module_from_header("AdvApp2Var_Data.hxx"), "AdvApp2Var");
        // Helper types without underscore still work
        assert_eq!(extract_module_from_header("Standalone.hxx"), "Standalone");
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
    fn test_is_occt_namespace_chain() {
        assert!(is_occt_namespace_chain("IMeshData"));
        assert!(is_occt_namespace_chain("ShapePersistent_BRep"));
        assert!(!is_occt_namespace_chain("std"));
        assert!(!is_occt_namespace_chain("opencascade"));
        assert!(!is_occt_namespace_chain("lowercase"));
    }

    #[test]
    fn test_map_standard_type() {
        assert!(matches!(map_standard_type("Standard_Real"), Some(Type::F64)));
        assert!(matches!(map_standard_type("Standard_Integer"), Some(Type::I32)));
        assert!(matches!(map_standard_type("Standard_Boolean"), Some(Type::Bool)));
        assert!(matches!(map_standard_type("Standard_Utf32Char"), Some(Type::U32)));
        assert!(matches!(map_standard_type("char32_t"), Some(Type::U32)));
        assert!(map_standard_type("gp_Pnt").is_none());
        // Bare C++ stream types map to Standard_* equivalents
        match map_standard_type("std::istream") {
            Some(Type::Class(name)) => assert_eq!(name, "Standard_IStream"),
            other => panic!("Expected Standard_IStream, got {:?}", other),
        }
        match map_standard_type("std::ostream") {
            Some(Type::Class(name)) => assert_eq!(name, "Standard_OStream"),
            other => panic!("Expected Standard_OStream, got {:?}", other),
        }
    }
}

