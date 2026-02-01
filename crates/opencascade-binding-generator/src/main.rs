//! OCCT Binding Generator CLI
//!
//! A tool using libclang to parse OCCT C++ headers and generate CXX bridge code,
//! organized into per-module Rust files with type aliasing for cross-module references.

use opencascade_binding_generator::{codegen, module_graph, parser};

use anyhow::Result;
use clap::Parser;
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

    // TODO: Implement the pipeline:
    // 1. Parse headers with libclang (parser.rs)
    // 2. Build module dependency graph (module_graph.rs)
    // 3. Generate Rust CXX bridge code (codegen/rust.rs)
    // 4. Generate C++ wrapper code (codegen/cpp.rs)

    println!("Parsing headers...");
    let parsed = parser::parse_headers(&args.headers, &args.include_dirs, args.verbose)?;

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

    // Generate common.hxx header with shared utilities
    let common_hxx = codegen::cpp::generate_common_header();
    let common_file = args.output.join("common.hxx");
    std::fs::write(&common_file, &common_hxx)?;
    println!("  Wrote: {}", common_file.display());

    // Generate code for each module
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    
    // Collect all enum names for cross-module enum type resolution
    let all_enum_names: std::collections::HashSet<String> = 
        all_enums.iter().map(|e| e.name.clone()).collect();

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

        if module_classes.is_empty() && module_enums.is_empty() {
            continue;
        }

        // Track this module for lib.rs
        generated_modules.push(module);

        println!(
            "  Generating module: {} ({} classes, {} enums)",
            module.name,
            module_classes.len(),
            module_enums.len()
        );

        // Get cross-module types
        let cross_types = graph.get_cross_module_types(&module.name);

        // Generate Rust code as TokenStream and convert to string
        let rust_code =
            codegen::rust::generate_module(module, &module_classes, &module_enums, &cross_types, &all_enum_names);

        // Write to output directory
        let rust_file = args.output.join(format!("{}.rs", module.rust_name));
        std::fs::write(&rust_file, rust_code.to_string())?;
        generated_rs_files.push(rust_file.clone());
        println!("    Wrote: {}", rust_file.display());

        // Generate C++ header wrapper
        let cpp_code = codegen::cpp::generate_module_header(module, &module_classes, &cross_types, &all_enum_names);
        // Use wrapper_ prefix to avoid collision with OCCT headers (e.g., gp.hxx)
        let cpp_file = args.output.join(format!("wrapper_{}.hxx", module.rust_name));
        std::fs::write(&cpp_file, &cpp_code)?;
        println!("    Wrote: {}", cpp_file.display());
    }

    // Generate lib.rs with module declarations
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
