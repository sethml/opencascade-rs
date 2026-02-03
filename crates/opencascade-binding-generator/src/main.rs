//! OCCT Binding Generator CLI
//!
//! A tool using libclang to parse OCCT C++ headers and generate CXX bridge code,
//! organized into per-module Rust files with type aliasing for cross-module references.

use opencascade_binding_generator::{codegen, header_deps, module_graph, parser, resolver};

use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

/// OCCT binding generator - parses OCCT headers and generates CXX bridge code
#[derive(Parser, Debug)]
#[command(name = "occt-bindgen")]
#[command(about = "Parse OCCT C++ headers and generate CXX bridge code")]
struct Args {
    /// OCCT headers to process
    #[arg(required = true)]
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

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("OCCT Binding Generator");
        println!("======================");
        println!("Headers to process: {:?}", args.headers);
        println!("Include directories: {:?}", args.include_dirs);
        println!("Output directory: {:?}", args.output);
        if let Some(ref module) = args.module {
            println!("Filtering to module: {}", module);
        }
    }

    // Resolve header dependencies if requested
    let headers_to_process = if args.resolve_deps && !args.include_dirs.is_empty() {
        // Use first include dir as OCCT include root
        let occt_include_dir = &args.include_dirs[0];
        
        if args.verbose {
            println!("\nResolving header dependencies...");
            println!("  OCCT include dir: {:?}", occt_include_dir);
        }
        
        let resolved = header_deps::resolve_header_dependencies(
            &args.headers,
            occt_include_dir,
            args.verbose,
        )?;
        
        if args.verbose {
            println!("  Explicit headers: {}", args.headers.len());
            println!("  Resolved headers: {}", resolved.len());
            println!("  Added {} dependency headers", resolved.len() - args.headers.len());
        }
        
        resolved
    } else {
        args.headers.clone()
    };

    // TODO: Implement the pipeline:
    // 1. Parse headers with libclang (parser.rs)
    // 2. Build module dependency graph (module_graph.rs)
    // 3. Generate Rust CXX bridge code (codegen/rust.rs)
    // 4. Generate C++ wrapper code (codegen/cpp.rs)

    println!("Parsing {} headers...", headers_to_process.len());
    let parsed = parser::parse_headers(&headers_to_process, &args.include_dirs, args.verbose)?;

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

    // Note: common.hxx is now hand-maintained in opencascade-sys/src/common.hxx
    // The generated wrappers include it via the include path in build.rs

    // Generate code for each module
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    let all_functions: Vec<_> = parsed.iter().flat_map(|h| &h.functions).collect();
    
    // Build symbol table (Pass 1 of two-pass architecture)
    // This resolves all symbols and makes binding decisions ONCE
    let ordered_modules = graph.modules_in_order();
    let symbol_table = resolver::build_symbol_table(
        &ordered_modules,
        &graph,
        &all_classes,
        &all_enums,
        &all_functions,
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
    let known_headers: std::collections::HashSet<String> = if !args.include_dirs.is_empty() {
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
        std::collections::HashSet::new()
    };
    
    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Track generated Rust files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();

    // Track generated modules for lib.rs
    let mut generated_modules: Vec<&module_graph::Module> = Vec::new();

    // Generate modules in dependency order
    let ordered = graph.modules_in_order();
    for module in ordered {
        // Filter to only this module if --module flag specified
        if let Some(ref filter_module) = args.module {
            if module.name != *filter_module {
                continue;
            }
        }

        // Get classes for this module
        let module_classes: Vec<_> = all_classes
            .iter()
            .filter(|c| c.module == module.name)
            .copied()
            .collect();

        // Get enums for this module
        let module_enums: Vec<_> = all_enums
            .iter()
            .filter(|e| e.module == module.name)
            .copied()
            .collect();

        // Get functions for this module
        let module_functions: Vec<_> = all_functions
            .iter()
            .filter(|f| f.module == module.name)
            .copied()
            .collect();

        if module_classes.is_empty() && module_enums.is_empty() && module_functions.is_empty() {
            continue;
        }

        // Track this module for lib.rs
        generated_modules.push(module);

        println!(
            "  Generating module: {} ({} classes, {} enums, {} functions)",
            module.name,
            module_classes.len(),
            module_enums.len(),
            module_functions.len()
        );

        // Get collections for this module
        let module_collections = codegen::collections::collections_for_module(&module.rust_name);

        // Collect unique header names for this module from classes, enums, and functions
        let mut module_headers: HashSet<String> = HashSet::new();
        for class in &module_classes {
            module_headers.insert(class.source_header.clone());
        }
        for enum_decl in &module_enums {
            module_headers.insert(enum_decl.source_header.clone());
        }
        for func in &module_functions {
            module_headers.insert(func.source_header.clone());
        }
        let mut headers_list: Vec<String> = module_headers.into_iter().collect();
        headers_list.sort();

        // Generate Rust code as String
        let rust_code = codegen::rust::generate_module(
            module,
            &module_classes,
            &module_enums,
            &module_functions,
            &headers_list,
            &module_collections,
            &symbol_table,
        );

        // Write to output directory
        let rust_file = args.output.join(format!("{}.rs", module.rust_name));
        std::fs::write(&rust_file, rust_code)?;
        generated_rs_files.push(rust_file.clone());
        println!("    Wrote: {}", rust_file.display());

        // Generate C++ header wrapper
        let cpp_code = codegen::cpp::generate_module_header(module, &module_classes, &module_functions, &module_collections, &known_headers, &symbol_table);
        // Use wrapper_ prefix to avoid collision with OCCT headers (e.g., gp.hxx)
        let cpp_file = args.output.join(format!("wrapper_{}.hxx", module.rust_name));
        std::fs::write(&cpp_file, &cpp_code)?;
        println!("    Wrote: {}", cpp_file.display());
        
        // Report collections for this module
        if !module_collections.is_empty() {
            println!("    Collections: {}", module_collections.iter().map(|c| c.short_name.as_str()).collect::<Vec<_>>().join(", "));
        }
    }

    // Generate lib.rs with module declarations (no separate collections module)
    let lib_rs = generate_lib_rs(&generated_modules);
    let lib_rs_path = args.output.join("lib.rs");
    std::fs::write(&lib_rs_path, &lib_rs)?;
    generated_rs_files.push(lib_rs_path.clone());
    println!("\n  Wrote: {}", lib_rs_path.display());

    // Format generated Rust files with rustfmt (use nightly for unstable options)
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
    }

    println!("\nCode generation complete!");

    Ok(())
}

/// Generate the lib.rs file with module declarations
fn generate_lib_rs(modules: &[&module_graph::Module]) -> String {
    let mut output = String::new();
    // Use outer doc comment since this will be include!()'d
    output.push_str("// Generated OCCT bindings\n\n");

    for module in modules {
        output.push_str(&format!("pub mod {};\n", module.rust_name));
    }

    output
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
            println!("  ENUMS (all excluded - CXX requires enum class):");
            for enum_decl in enums {
                println!("    ✗ {} ({} variants)", enum_decl.cpp_name, enum_decl.variants.len());
            }
            println!();
        }
    }
    
    println!("===== END SYMBOL TABLE DUMP =====");
}