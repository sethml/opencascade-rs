//! OCCT Binding Generator CLI
//!
//! A tool using libclang to parse OCCT C++ headers and generate extern "C" FFI bindings
//! Generates FFI bindings with a single ffi.rs module and per-module re-exports.

use opencascade_binding_generator::{codegen, config, header_deps, model, module_graph, parser, resolver};

use anyhow::Result;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;

/// OCCT binding generator - parses OCCT headers and generates extern "C" FFI bindings
#[derive(Parser, Debug)]
#[command(name = "occt-bindgen")]
#[command(about = "Parse OCCT C++ headers and generate extern 'C' FFI bindings")]
struct Args {
    /// TOML configuration file specifying which headers to process
    #[arg(long)]
    config: Option<PathBuf>,

    /// OCCT headers to process (alternative to --config)
    headers: Vec<PathBuf>,

    /// OCCT include directory (can be specified multiple times)
    #[arg(short = 'I', long = "include")]
    include_dirs: Vec<PathBuf>,

    /// Output directory for generated code
    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    /// Only generate for specific module (e.g., "gp", "TopoDS")
    #[arg(long)]
    module: Option<String>,

    /// Print parsed information without generating code
    #[arg(long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Automatically include header dependencies (recursively)
    #[arg(long, default_value = "true")]
    resolve_deps: bool,

    /// Dump the symbol table for debugging (shows all resolved symbols and their binding status)
    #[arg(long)]
    dump_symbols: bool,
}

/// Check if a manual binding file exists for this module and return the
/// `include!()` directive to append to the generated module file.
fn manual_include_directive(output_dir: &std::path::Path, rust_module_name: &str) -> Option<String> {
    let manual_dir = output_dir.parent()?.join("manual");
    let manual_rs = manual_dir.join(format!("{}.rs", rust_module_name));
    if manual_rs.exists() {
        // Read the first few comment lines to extract the description
        let content = std::fs::read_to_string(&manual_rs).ok()?;
        let description_lines: Vec<&str> = content
            .lines()
            .take_while(|l| l.starts_with("//"))
            .collect();
        let description = if description_lines.is_empty() {
            format!("Manual bindings for {}", rust_module_name)
        } else {
            description_lines.join("\n")
        };
        Some(format!(
            "\n// Manual bindings:\n{}\ninclude!(\"../manual/{}.rs\");\n",
            description, rust_module_name
        ))
    } else {
        None
    }
}


fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("OCCT Binding Generator");
        println!("======================");
        println!("Include directories: {:?}", args.include_dirs);
        println!("Output directory: {:?}", args.output);
        if let Some(ref module) = args.module {
            println!("Filtering to module: {}", module);
        }
    }

    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods, manual_type_names) = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;
        let resolve = cfg.general.resolve_deps;

        if args.include_dirs.is_empty() {
            anyhow::bail!("--config requires at least one -I <include_dir>");
        }
        let occt_include_dir = &args.include_dirs[0];

        let headers = config::expand_headers(&cfg, occt_include_dir)?;

        if args.verbose {
            println!("Config file: {:?}", config_path);
            println!("  Modules: {:?}", cfg.modules);
            println!("  Exclude modules: {:?}", cfg.exclude_modules);
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            println!("  Exclude methods: {} entries", cfg.exclude_methods.len());
            println!("  Expanded to {} headers", headers.len());
        }

        println!("Loaded config: {} module patterns, {} exclude module patterns, {} individual headers, {} header exclusions -> {} headers",
            cfg.modules.len(), cfg.exclude_modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());

        // Parse exclude_methods into (ClassName, MethodName) pairs.
        // Uses rsplit to support nested classes: "Outer::Inner::Method" splits
        // as class="Outer::Inner", method="Method".
        let method_exclusions: HashSet<(String, String)> = cfg.exclude_methods
            .iter()
            .filter_map(|s| {
                if let Some(pos) = s.rfind("::") {
                    let class_name = s[..pos].to_string();
                    let method_name = s[pos + 2..].to_string();
                    Some((class_name, method_name))
                } else {
                    eprintln!("Warning: invalid exclude_methods entry (expected ClassName::MethodName): {}", s);
                    None
                }
            })
            .collect();

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        (headers, resolve, excludes, exclude_mods, method_exclusions, manual_names)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new(), HashSet::new())
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    // Resolve header dependencies if requested
    let headers_to_process = if resolve_deps && !args.include_dirs.is_empty() {
        // Use first include dir as OCCT include root
        let occt_include_dir = &args.include_dirs[0];

        if args.verbose {
            println!("\nResolving header dependencies...");
            println!("  OCCT include dir: {:?}", occt_include_dir);
        }

        let resolved = header_deps::resolve_header_dependencies(
            &explicit_headers,
            occt_include_dir,
            args.verbose,
        )?;

        if args.verbose {
            println!("  Explicit headers: {}", explicit_headers.len());
            println!("  Resolved headers: {}", resolved.len());
            println!("  Added {} dependency headers", resolved.len() - explicit_headers.len());
        }

        resolved
    } else {
        explicit_headers
    };

    // Apply exclusions to resolved headers (exclusions apply after dep resolution too)
    let headers_to_process = {
        let before = headers_to_process.len();
        let filtered: Vec<_> = headers_to_process
            .into_iter()
            .filter(|path| {
                let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
                // Check specific header exclusions
                if !exclude_set.is_empty() && exclude_set.contains(filename) {
                    return false;
                }
                // Check module-level exclusions: extract module prefix from filename
                if !exclude_modules.is_empty() {
                    let stem = filename.trim_end_matches(".hxx");
                    let module = if let Some(pos) = stem.find('_') {
                        &stem[..pos]
                    } else {
                        stem
                    };
                    if exclude_modules.iter().any(|pattern| config::module_matches_pattern(module, pattern)) {
                        return false;
                    }
                }
                true
            })
            .collect();
        if filtered.len() < before {
            println!("  Excluded {} headers after dependency resolution", before - filtered.len());
        }
        filtered
    };

    println!("Parsing {} headers...", headers_to_process.len());
    let mut parsed = parser::parse_headers(&headers_to_process, &args.include_dirs, args.verbose)?;

    // Detect "utility namespace classes" — classes with no underscore in the name
    // (class name == module name), only static methods, and no instance methods/constructors.
    // These are OCCT's namespace-like patterns (e.g., `gp` with `gp::OX()`, `gp::Origin()`).
    // Convert their static methods to free functions so they appear as module-level
    // functions (e.g., `gp::ox()`) instead of awkward `gp::gp::ox()`.
    convert_utility_classes_to_functions(&mut parsed, args.verbose);

    if args.verbose {
        println!("\nParsing complete. Summary:");
        let total_classes: usize = parsed.iter().map(|h| h.classes.len()).sum();
        let total_methods: usize = parsed
            .iter()
            .flat_map(|h| &h.classes)
            .map(|c| c.methods.len())
            .sum();
        let total_ctors: usize = parsed
            .iter()
            .flat_map(|h| &h.classes)
            .map(|c| c.constructors.len())
            .sum();
        println!("  {} headers parsed", parsed.len());
        println!("  {} classes found", total_classes);
        println!("  {} constructors found", total_ctors);
        println!("  {} methods found", total_methods);
    }

    // Build module dependency graph
    println!("\nBuilding module dependency graph...");
    let graph = module_graph::ModuleGraph::from_headers(&parsed);

    if args.verbose {
        println!("\nModule graph analysis:");
        println!("  {} modules found", graph.modules.len());

        // Show modules in dependency order
        let ordered = graph.modules_in_order();
        println!("\nModules in dependency order:");
        for module in &ordered {
            if module.dependencies.is_empty() {
                println!("  {} ({} types)", module.name, module.types.len());
            } else {
                let deps: Vec<_> = module.dependencies.iter().collect();
                println!(
                    "  {} ({} types) -> depends on: {:?}",
                    module.name,
                    module.types.len(),
                    deps
                );
            }
        }

        // Show cross-module type references for each module
        println!("\nCross-module type references:");
        for module in &ordered {
            let cross_types = graph.get_cross_module_types(&module.name);
            if !cross_types.is_empty() {
                println!("  {} needs types from other modules:", module.name);
                for ct in &cross_types {
                    println!("    - {}::{} (C++: {})", ct.source_module, ct.rust_name, ct.cpp_name);
                }
            }
        }
    }

    if args.dry_run {
        println!("\nDry run - skipping code generation");
        return Ok(());
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Generate code
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    let all_functions: Vec<_> = parsed.iter().flat_map(|h| &h.functions).collect();

    // Get collection type names (needed for symbol resolution filtering)
    let all_collections = codegen::collections::all_known_collections();
    let collection_type_names: HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();

    // Compute handle-able classes via transitive closure of inheritance graph.
    // This is done before the symbol table so the resolver can use it instead of
    // relying on the old per-class parser heuristic.
    let handle_able_classes = codegen::bindings::compute_handle_able_classes(&all_classes);

    // Build symbol table (Pass 1 of two-pass architecture)
    // This resolves all symbols and makes binding decisions ONCE
    let ordered_modules = graph.modules_in_order();
    let symbol_table = resolver::build_symbol_table(
        &ordered_modules,
        &graph,
        &all_classes,
        &all_enums,
        &all_functions,
        &collection_type_names,
        &handle_able_classes,
        &manual_type_names,
    );

    if args.verbose {
        println!("\nSymbol table built:");
        println!("  {} classes", symbol_table.classes.len());
        println!("  {} constructors", symbol_table.constructors.len());
        println!("  {} methods", symbol_table.methods.len());
        println!("  {} static methods", symbol_table.static_methods.len());
        println!("  {} functions", symbol_table.functions.len());
        println!("  {} enums", symbol_table.enums.len());

        // Count included vs excluded
        let included_classes = symbol_table.classes.values().filter(|c| c.status.is_included()).count();
        let included_ctors = symbol_table.constructors.values().filter(|c| c.status.is_included()).count();
        let included_methods = symbol_table.methods.values().filter(|m| m.status.is_included()).count();
        let included_static = symbol_table.static_methods.values().filter(|m| m.status.is_included()).count();
        let included_funcs = symbol_table.functions.values().filter(|f| f.status.is_included()).count();

        println!("\n  Included for binding:");
        println!("    {} classes (of {})", included_classes, symbol_table.classes.len());
        println!("    {} constructors (of {})", included_ctors, symbol_table.constructors.len());
        println!("    {} methods (of {})", included_methods, symbol_table.methods.len());
        println!("    {} static methods (of {})", included_static, symbol_table.static_methods.len());
        println!("    {} functions (of {})", included_funcs, symbol_table.functions.len());
    }

    // Dump symbol table if requested
    if args.dump_symbols {
        dump_symbol_table(&symbol_table);
        return Ok(());
    }

    // Collect set of known header filenames that actually exist
    // This is used to filter out headers for types that don't have their own header files
    let known_headers: HashSet<String> = if !args.include_dirs.is_empty() {
        let occt_include_dir = &args.include_dirs[0];
        std::fs::read_dir(occt_include_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("hxx") {
                            path.file_name().and_then(|s| s.to_str()).map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        HashSet::new()
    };
    // Remove excluded headers from the known set so they won't be included
    // in wrappers.cpp (e.g., RWGltf_GltfOStreamWriter.hxx depends on rapidjson
    // which is not bundled)
    let known_headers: HashSet<String> = known_headers.difference(&exclude_set).cloned().collect();

    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &handle_able_classes, &manual_type_names)
}

/// Detect "utility namespace classes" and convert their static methods to free functions.
///
/// OCCT has a pattern where some packages use a class with only static methods instead of
/// a C++ namespace (e.g., `class gp { static const gp_Pnt& Origin(); ... }`). These are
/// conceptually namespaces, not instantiable types.
///
/// Detection criteria:
/// - Class name has no underscore (name == module, e.g., "gp")
/// - Has ONLY static methods (no instance methods)
/// - Has no constructors (or only a default constructor with no params)
///
/// Conversion: static methods → ParsedFunction entries in the same header,
/// and the utility class is removed from the header's class list.
fn convert_utility_classes_to_functions(
    parsed: &mut [model::ParsedHeader],
    verbose: bool,
) {
    for header in parsed.iter_mut() {
        let mut functions_to_add = Vec::new();
        let mut classes_to_remove = Vec::new();

        for (idx, class) in header.classes.iter().enumerate() {
            // Must have no underscore in the name (class name == module name pattern)
            if class.name.contains('_') {
                continue;
            }

            // Must have static methods
            if class.static_methods.is_empty() {
                continue;
            }

            // Must have NO instance methods
            if !class.methods.is_empty() {
                continue;
            }

            // Must have no meaningful constructors (allow synthetic/empty default)
            let has_meaningful_ctors = class.constructors.iter().any(|c| !c.params.is_empty());
            if has_meaningful_ctors {
                continue;
            }

            // This is a utility class — convert static methods to functions
            if verbose {
                println!(
                    "  Detected utility class '{}' with {} static methods → converting to module-level functions",
                    class.name,
                    class.static_methods.len()
                );
            }

            for sm in &class.static_methods {
                let mut return_type = sm.return_type.clone();

                // If return type is ConstRef and there are no ref params,
                // strip the ConstRef wrapper (return by-value copy). The FFI can't
                // express references from free functions with no borrowable
                // params, so we copy instead.
                let has_ref_params = sm.params.iter().any(|p| matches!(&p.ty, model::Type::ConstRef(_) | model::Type::MutRef(_)));
                if !has_ref_params {
                    if let Some(model::Type::ConstRef(inner)) = &return_type {
                        return_type = Some(*inner.clone());
                    }
                }

                functions_to_add.push(model::ParsedFunction {
                    name: format!("{}::{}", class.name, sm.name),
                    namespace: class.name.clone(),
                    short_name: sm.name.clone(),
                    module: class.module.clone(),
                    comment: sm.comment.clone(),
                    source_header: class.source_header.clone(),
                    source_line: sm.source_line,
                    params: sm.params.clone(),
                    return_type,
                });
            }

            classes_to_remove.push(idx);
        }

        // Remove utility classes (in reverse order to preserve indices)
        for idx in classes_to_remove.into_iter().rev() {
            header.classes.remove(idx);
        }

        // Add converted functions
        header.functions.extend(functions_to_add);
    }
}

/// Dump the symbol table for debugging purposes
fn dump_symbol_table(table: &resolver::SymbolTable) {
    println!("\n===== SYMBOL TABLE DUMP =====\n");

    // Group classes by module for organized output
    let mut modules: Vec<_> = table.classes_by_module.keys().collect();
    modules.sort();

    for module in modules {
        println!("=== Module: {} ===\n", module);

        // Dump classes in this module
        let classes = table.classes_for_module(module);
        for class in classes {
            let status_str = match &class.status {
                resolver::BindingStatus::Included => "✓ INCLUDED".to_string(),
                resolver::BindingStatus::Excluded(reason) => format!("✗ EXCLUDED: {:?}", reason),
            };
            println!("  CLASS {} ({}) [{}]", class.cpp_name, class.rust_ffi_name, status_str);

            if class.is_abstract {
                println!("    [abstract]");
            }
            if class.has_protected_destructor {
                println!("    [protected destructor]");
            }
            if !class.base_classes.is_empty() {
                println!("    bases: {:?}", class.base_classes);
            }

            // Constructors
            let ctors = table.included_constructors(class);
            let all_ctors: Vec<_> = class.constructors.iter()
                .filter_map(|id| table.constructors.get(id))
                .collect();
            println!("    Constructors: {}/{} included", ctors.len(), all_ctors.len());
            for ctor in all_ctors {
                let ctor_status = match &ctor.status {
                    resolver::BindingStatus::Included => "✓".to_string(),
                    resolver::BindingStatus::Excluded(reason) => format!("✗ {:?}", reason),
                };
                let params: Vec<_> = ctor.params.iter().map(|p| format!("{}: {}", p.rust_name, p.ty.rust_ffi_type)).collect();
                println!("      {} {}({}) [{}]", ctor_status, ctor.rust_name, params.join(", "),
                    if ctor.status.is_included() { "included" } else { "excluded" });
            }

            // Methods
            let methods = table.included_methods(class);
            let all_methods: Vec<_> = class.methods.iter()
                .filter_map(|id| table.methods.get(id))
                .collect();
            println!("    Methods: {}/{} included", methods.len(), all_methods.len());

            // Show excluded methods with reasons
            for method in all_methods.iter().filter(|m| m.status.is_excluded()) {
                if let resolver::BindingStatus::Excluded(reason) = &method.status {
                    println!("      ✗ {} - {:?}", method.cpp_name, reason);
                }
            }

            // Static methods
            let statics = table.included_static_methods(class);
            let all_statics: Vec<_> = class.static_methods.iter()
                .filter_map(|id| table.static_methods.get(id))
                .collect();
            if !all_statics.is_empty() {
                println!("    Static methods: {}/{} included", statics.len(), all_statics.len());
            }

            println!();
        }

        // Dump functions in this module
        let functions = table.functions_for_module(module);
        if !functions.is_empty() {
            println!("  FUNCTIONS:");
            for func in functions {
                let status_str = match &func.status {
                    resolver::BindingStatus::Included => "✓".to_string(),
                    resolver::BindingStatus::Excluded(reason) => format!("✗ {:?}", reason),
                };
                println!("    {} {} [{}]", status_str, func.cpp_name,
                    if func.status.is_included() { "included" } else { "excluded" });
            }
            println!();
        }

        // Dump enums in this module
        let enums = table.enums_for_module(module);
        if !enums.is_empty() {
            println!("  ENUMS (all excluded - externs require integer conversion):");
            for enum_decl in enums {
                println!("    ✗ {} ({} variants)", enum_decl.cpp_name, enum_decl.variants.len());
            }
            println!();
        }
    }

    println!("===== END SYMBOL TABLE DUMP =====");
}

/// Generate FFI module output files
///
/// This generates:
/// - ffi.rs: Single extern "C" FFI module with ALL types using full C++ names
/// - wrappers.hxx: Single C++ header with all wrappers
/// - MODULE.rs: Per-module re-exports with impl blocks
/// - lib.rs: Module declarations
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
) -> Result<()> {
    use model::ParsedClass;

    println!("\n=== Generating FFI output ===\n");

    // Collect all headers
    let mut all_headers: HashSet<String> = HashSet::new();
    for class in all_classes {
        all_headers.insert(class.source_header.clone());
    }
    for func in all_functions {
        all_headers.insert(func.source_header.clone());
    }
    let all_headers_list: Vec<String> = all_headers.into_iter().collect();

    // Get all collections
    let all_collections = codegen::collections::all_known_collections();

    // Compute ClassBindings once for ALL classes — shared by all three generators
    let collection_type_names: std::collections::HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();
    let extra_typedef_names = parser::get_collected_typedef_names();
    let all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,
    );

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();

    // 1. Generate ffi.rs
    println!("Generating ffi.rs...");
    let (ffi_code, nested_types) = codegen::rust::generate_ffi(
        all_classes,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
    );
    let ffi_path = args.output.join("ffi.rs");
    std::fs::write(&ffi_path, ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} ({} classes, {} functions)",
        ffi_path.display(), all_classes.len(), all_functions.len());

    // 2. Generate wrappers.cpp
    println!("Generating wrappers.cpp...");
    let cpp_code = codegen::cpp::generate_wrappers(
        all_classes,
        &all_collections,
        known_headers,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &nested_types,
        &handle_able_classes,
    );
    let cpp_path = args.output.join("wrappers.cpp");
    std::fs::write(&cpp_path, &cpp_code)?;
    println!("  Wrote: {}", cpp_path.display());

    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
    let mut bindings_by_module: HashMap<String, Vec<&codegen::bindings::ClassBindings>> =
        HashMap::new();
    for b in &all_bindings {
        bindings_by_module
            .entry(b.module.clone())
            .or_default()
            .push(b);
    }

    // Index function bindings by module for quick lookup
    let mut fn_bindings_by_module: HashMap<String, Vec<&codegen::bindings::FunctionBinding>> =
        HashMap::new();
    for fb in &all_function_bindings {
        fn_bindings_by_module
            .entry(fb.module.clone())
            .or_default()
            .push(fb);
    }

    // Index skipped function symbols by module
    let mut skipped_fns_by_module: HashMap<String, Vec<&codegen::bindings::SkippedSymbol>> =
        HashMap::new();
    for sf in &all_skipped_functions {
        skipped_fns_by_module
            .entry(sf.module.clone())
            .or_default()
            .push(sf);
    }

    // Compute ALL types that appear in ffi.rs so we can find unreexported ones
    // 1. Class types from ClassBindings (already re-exported via emit_reexport_class)
    let mut already_reexported: HashSet<String> = HashSet::new();
    for b in &all_bindings {
        already_reexported.insert(b.cpp_name.clone());
        // Handle types generated for this class
        if b.has_to_handle || b.has_handle_get {
            let handle_name = format!("Handle{}", b.cpp_name.replace('_', ""));
            already_reexported.insert(handle_name);
        }
        // Handle upcasts reference base handle types
        for hu in &b.handle_upcasts {
            already_reexported.insert(hu.base_handle_name.clone());
        }
    }

    // 2. Collection types (re-exported via collections loop)
    for coll in &all_collections {
        already_reexported.insert(coll.typedef_name.clone());
    }

    // Now compute ALL types in ffi.rs and find unreexported ones:
    // A. Handle types for all transient classes
    let mut all_ffi_types: Vec<(String, String)> = Vec::new(); // (ffi_name, module_prefix)
    for class in all_classes {
        if handle_able_classes.contains(&class.name) {
            let handle_name = format!("Handle{}", class.name.replace('_', ""));
            if !already_reexported.contains(&handle_name) {
                // Use the class's actual module (not derived from handle name)
                all_ffi_types.push((handle_name, class.module.clone()));
            }
        }
    }

    // B. Opaque referenced types (types referenced in method signatures but not defined)
    let collected_types = codegen::rust::collect_referenced_types(all_classes, &handle_able_classes);
    let defined_classes: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    let all_enum_names = &symbol_table.all_enum_names;

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) { continue; }
        if all_enum_names.contains(type_name) { continue; }
        // Protected destructor classes are still needed as type references in module reexports
        if codegen::rust::is_primitive_type(type_name) { continue; }
        if collection_type_names.contains(type_name) { continue; }
        if already_reexported.contains(type_name) { continue; }
        // Skip namespace-scoped types and types with pointer/ref qualifiers
        // leaked into the name (e.g., "IMeshData::ListOfPnt2d", "IMeshData_Edge *const")
        if type_name.contains("::") || type_name.contains('*') || type_name.contains('&') { continue; }

        // Determine module from type_to_module map, falling back to name-based
        if let Some(module) = symbol_table.type_to_module.get(type_name) {
            all_ffi_types.push((type_name.clone(), module.clone()));
        } else if let Some(underscore_pos) = type_name.find('_') {
            let module_prefix = &type_name[..underscore_pos];
            all_ffi_types.push((type_name.clone(), module_prefix.to_string()));
        }
    }

    // C. Collection iterator types (not currently re-exported)
    for coll in &all_collections {
        match coll.kind {
            codegen::collections::CollectionKind::Array1 | codegen::collections::CollectionKind::Array2 => {
                // Array types don't have iterator types
            }
            _ => {
                let iter_name = format!("{}Iterator", coll.short_name);
                if !already_reexported.contains(&iter_name) {
                    // Determine module from collection module
                    // coll.module is already a rust module name; we need the C++ module name
                    // Extract it from the typedef_name
                    if let Some(underscore_pos) = coll.typedef_name.find('_') {
                        let module_prefix = &coll.typedef_name[..underscore_pos];
                        all_ffi_types.push((iter_name, module_prefix.to_string()));
                    }
                }
            }
        }
    }

    // Group extra types by module (C++ module name)
    let mut extra_types_by_module: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for (ffi_name, module_prefix) in &all_ffi_types {
        // Compute short name based on type category
        let short_name = if ffi_name.starts_with("Handle") && !ffi_name.contains('_') {
            // Handle types like "HandleGeomEvaluatorCurve" — keep as-is (no short alias)
            ffi_name.clone()
        } else if ffi_name.ends_with("Iterator") && !ffi_name.contains('_') {
            // Collection iterator types like "ListOfShapeIterator" — keep as-is
            ffi_name.clone()
        } else {
            // Use module-relative short name derivation
            opencascade_binding_generator::type_mapping::safe_short_name(&opencascade_binding_generator::type_mapping::short_name_for_module(ffi_name, module_prefix))
        };
        extra_types_by_module
            .entry(module_prefix.clone())
            .or_default()
            .push((ffi_name.clone(), short_name));
    }

    // Sort each module's extra types for deterministic output
    for types in extra_types_by_module.values_mut() {
        types.sort();
    }

    let ordered = graph.modules_in_order();
    let mut generated_modules: Vec<&module_graph::Module> = Vec::new();

    for module in &ordered {
        // Get classes for this module
        let module_classes: Vec<&ParsedClass> = all_classes
            .iter()
            .filter(|c| c.module == module.name)
            .copied()
            .collect();

        // Check if this module has any functions in the symbol table
        let has_module_functions = !symbol_table.included_functions_for_module(&module.rust_name).is_empty();

        let has_module_enums = symbol_table.enums_by_module.contains_key(&module.rust_name);
        let has_extra_types = extra_types_by_module.contains_key(&module.name);
        if module_classes.is_empty() && !has_module_functions && !has_module_enums && !has_extra_types {
            continue;
        }

        generated_modules.push(module);

        // Get collections for this module
        let module_collections: Vec<_> = all_collections
            .iter()
            .filter(|c| c.module == module.rust_name)
            .collect();

        // Get pre-computed bindings for this module
        let empty_bindings = Vec::new();
        let module_bindings = bindings_by_module
            .get(&module.name)
            .unwrap_or(&empty_bindings);

        // Get extra types for this module
        let empty_extra = Vec::new();
        let module_extra_types = extra_types_by_module
            .get(&module.name)
            .unwrap_or(&empty_extra);

        let empty_fn_bindings: Vec<&codegen::bindings::FunctionBinding> = Vec::new();
        let module_fn_bindings = fn_bindings_by_module
            .get(&module.rust_name)
            .unwrap_or(&empty_fn_bindings);
        let empty_skipped_fns: Vec<&codegen::bindings::SkippedSymbol> = Vec::new();
        let module_skipped_fns = skipped_fns_by_module
            .get(&module.rust_name)
            .unwrap_or(&empty_skipped_fns);

        let reexport_code = codegen::rust::generate_module_reexports(
            &module.name,
            &module.rust_name,
            &module_classes,
            &module_collections,
            symbol_table,
            module_bindings,
            module_fn_bindings,
            module_skipped_fns,
            module_extra_types,
        );

        let module_path = args.output.join(format!("{}.rs", module.rust_name));
        let mut reexport_code = reexport_code;
        if let Some(include) = manual_include_directive(&args.output, &module.rust_name) {
            reexport_code.push_str(&include);
        }
        std::fs::write(&module_path, reexport_code)?;
        generated_rs_files.push(module_path.clone());
        println!("  Wrote: {} ({} types, {} extra)",
            module_path.display(), module_classes.len(), module_extra_types.len());
    }

    // Generate module files for extra types whose modules aren't in the graph
    // (e.g., handle types, opaque references from dependency headers)
    let graph_module_names: HashSet<&String> = ordered.iter().map(|m| &m.name).collect();
    let graph_rust_names: HashSet<String> = ordered.iter().map(|m| m.rust_name.clone()).collect();
    let mut extra_only_modules: Vec<(String, String)> = Vec::new(); // (cpp_name, rust_name)
    for (module_name, types) in &extra_types_by_module {
        if !graph_module_names.contains(module_name) && !types.is_empty() {
            let rust_name = module_graph::module_to_rust_name(module_name);
            // Get collections for this module (needed for typedef'd NCollection types)
            let module_collections: Vec<_> = all_collections
                .iter()
                .filter(|c| c.module == rust_name)
                .collect();
            let empty_fn_bindings: Vec<&codegen::bindings::FunctionBinding> = Vec::new();
            let module_fn_bindings = fn_bindings_by_module
                .get(&rust_name)
                .unwrap_or(&empty_fn_bindings);
            let empty_skipped_fns: Vec<&codegen::bindings::SkippedSymbol> = Vec::new();
            let module_skipped_fns = skipped_fns_by_module
                .get(&rust_name)
                .unwrap_or(&empty_skipped_fns);
            let reexport_code = codegen::rust::generate_module_reexports(
                module_name,
                &rust_name,
                &[],
                &module_collections,
                symbol_table,
                &[],
                module_fn_bindings,
                module_skipped_fns,
                types,
            );
            let module_path = args.output.join(format!("{}.rs", rust_name));
            let mut reexport_code = reexport_code;
            if let Some(include) = manual_include_directive(&args.output, &rust_name) {
                reexport_code.push_str(&include);
            }
            std::fs::write(&module_path, &reexport_code)?;
            generated_rs_files.push(module_path.clone());
            extra_only_modules.push((module_name.clone(), rust_name.clone()));
            println!("  Wrote: {} (extra types only, {} types)", module_path.display(), types.len());
        }
    }

    // Generate module files for function-only modules (utility classes converted
    // to free functions that left no classes behind in the module)
    let extra_type_modules: HashSet<String> = extra_only_modules.iter().map(|(_, r)| r.clone()).collect();
    let _empty_fn_bindings_vec: Vec<&codegen::bindings::FunctionBinding> = Vec::new();
    for (rust_module, fn_bindings) in &fn_bindings_by_module {
        if graph_rust_names.contains(rust_module) || extra_type_modules.contains(rust_module) {
            continue;
        }
        if fn_bindings.is_empty() {
            continue;
        }
        // Derive the C++ module name from the namespace of the first function
        let cpp_name = fn_bindings[0].namespace.clone();
        let empty_skipped_fns: Vec<&codegen::bindings::SkippedSymbol> = Vec::new();
        let module_skipped_fns = skipped_fns_by_module
            .get(rust_module)
            .unwrap_or(&empty_skipped_fns);
        let reexport_code = codegen::rust::generate_module_reexports(
            &cpp_name,
            rust_module,
            &[],
            &[],
            symbol_table,
            &[],
            fn_bindings,
            module_skipped_fns,
            &[],
        );
        let module_path = args.output.join(format!("{}.rs", rust_module));
        let mut reexport_code = reexport_code;
        if let Some(include) = manual_include_directive(&args.output, rust_module) {
            reexport_code.push_str(&include);
        }
        std::fs::write(&module_path, &reexport_code)?;
        generated_rs_files.push(module_path.clone());
        extra_only_modules.push((cpp_name.clone(), rust_module.clone()));
        println!("  Wrote: {} (function-only module, {} functions)", module_path.display(), fn_bindings.len());
    }

    // 4. Generate lib.rs with module declarations
    let lib_rs = generate_lib_rs(&generated_modules, &extra_only_modules);
    let lib_rs_path = args.output.join("lib.rs");
    std::fs::write(&lib_rs_path, &lib_rs)?;
    generated_rs_files.push(lib_rs_path.clone());
    println!("  Wrote: {}", lib_rs_path.display());

    // Format generated Rust files
    if !generated_rs_files.is_empty() {
        println!("\nFormatting generated Rust code with rustfmt...");
        let status = Command::new("rustfmt")
            .arg("+nightly")
            .args(&generated_rs_files)
            .status();

        match status {
            Ok(s) if s.success() => println!("  Formatting complete."),
            Ok(s) => eprintln!("  Warning: rustfmt exited with status: {}", s),
            Err(e) => eprintln!("  Warning: Failed to run rustfmt: {}", e),
        }

        // No post-processing needed - string-based generation emits correct output directly
    }

    println!("\nCode generation complete!");
    println!("  {} modules generated", generated_modules.len());

    Ok(())
}

/// Generate lib.rs with module declarations
fn generate_lib_rs(modules: &[&module_graph::Module], extra_modules: &[(String, String)]) -> String {
    let mut output = String::new();
    output.push_str("// Generated OCCT bindings\n");
    output.push_str("// Nested C++ types use Parent_Child naming, which is intentional\n");
    output.push_str("#![allow(non_camel_case_types)]\n\n");

    output.push_str("// Core FFI module with all types (pub(crate) to prevent direct access, use module re-exports instead)\n");
    output.push_str("pub(crate) mod ffi;\n\n");
    output.push_str("// Per-module re-exports\n");

    // Collect all module rust names and sort for deterministic output
    let mut all_rust_names: Vec<&str> = modules.iter().map(|m| m.rust_name.as_str()).collect();
    for (_, rust_name) in extra_modules {
        all_rust_names.push(rust_name);
    }
    all_rust_names.sort();
    all_rust_names.dedup();

    for rust_name in all_rust_names {
        output.push_str(&format!("pub mod {};\n", rust_name));
    }

    output
}
