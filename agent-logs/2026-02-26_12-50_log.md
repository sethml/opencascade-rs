# Performance regression in binding regeneration process

- **Date:** 2026-02-26 11:19 – 12:35
- **Models:** Claude Opus 4.6, Claude Sonnet 4.6
- **Turns:** 4
- **Tool calls:** 223
- **Thinking blocks:** 26
- **Input words:** 20,287
- **Output words:** 12,672
- **Prompt tokens (last round):** 302,484
- **API rounds:** 204
- **Total elapsed:** 4090s

## Table of Contents

1. [Regenerating bindings has gotten very slow - it&#x27;s gone from about 70 seconds to 180 seconds. I su...](#user-1) (Claude Opus 4.6)
2. [Figure out how much it increased RAM usage.](#user-2) (Claude Sonnet 4.6)
3. [continue.](#user-3) (Claude Sonnet 4.6)
4. [I wanted to know how much RAM occt-bindgen is using.](#user-4) (Claude Sonnet 4.6)

---

<a id="user-1"></a>

## User (1)

> Regenerating bindings has gotten very slow - it&#x27;s gone from about 70 seconds to 180 seconds. I suspect there&#x27;s an inefficient algorithm in generating wrapper_*.cpp, because that phase seems to be going very slowly. Figure out the performance regression and fix it. If you spot other opportunities to improve performance significantly, take them. Then commit.

*2026-02-26 11:19*

### Assistant — Claude Opus 4.6

> 💭 Let me investigate the performance regression in the binding generator. I'll start by understanding the current state of the codebase and then profile the regeneration process.

**Todo list:**
- 🔄 Profile binding regeneration
- ⬜ Find performance regression
- ⬜ Fix the regression
- ⬜ Verify fix and commit

Let me start by understanding the binding generator's structure and then profile where the time is spent.

Explore binding generator code

Read [src](../crates/opencascade-binding-generator/src)

<details>
<summary>Searched for regex <code>wrapper.*\.cpp|wrapper_\w+ generation|write.*wrapper</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 17 results</summary>

- [config.rs:107](../crates/opencascade-binding-generator/src/config.rs#L107)
- [config.rs:127](../crates/opencascade-binding-generator/src/config.rs#L127)
- [config.rs:134](../crates/opencascade-binding-generator/src/config.rs#L134)
- [cpp.rs:554](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L554)
- [cpp.rs:644](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L644)
- [cpp.rs:765](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L765)
- [cpp.rs:831](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L831)
- [main.rs:543](../crates/opencascade-binding-generator/src/main.rs#L543)
- [main.rs:1181](../crates/opencascade-binding-generator/src/main.rs#L1181)
- [main.rs:1187](../crates/opencascade-binding-generator/src/main.rs#L1187)
- [main.rs:1188](../crates/opencascade-binding-generator/src/main.rs#L1188)
- [main.rs:1200](../crates/opencascade-binding-generator/src/main.rs#L1200)
- [rust.rs:374](../crates/opencascade-binding-generator/src/codegen/rust.rs#L374)
- [rust.rs:434](../crates/opencascade-binding-generator/src/codegen/rust.rs#L434)
- [rust.rs:436](../crates/opencascade-binding-generator/src/codegen/rust.rs#L436)
- [bindings.rs:174](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L174)
- [bindings.rs:3808](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3808)
</details>

Read [regenerate-bindings.sh](../scripts/regenerate-bindings.sh)

Read [codegen](../crates/opencascade-binding-generator/src/codegen)

Read [main.rs](../crates/opencascade-binding-generator/src/main.rs#1-1), lines 1 to 100

Read [main.rs](../crates/opencascade-binding-generator/src/main.rs#1150-1150), lines 1150 to 1220

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#1-1), lines 1 to 100

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#630-630), lines 630 to 800

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#640-640), lines 640 to 650

Searched for regex `^pub fn generate_wrappers` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 2 results
- [cpp.rs:629](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L629)
- [cpp.rs:813](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L813)

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#813-813), lines 813 to 1000

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1-1), lines 1 to 900

Searched for regex `^pub fn emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3789](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3789)

Searched for regex `^fn main|^async fn main` (`**/crates/opencascade-binding-generator/src/main.rs`), 1 result
- [main.rs:109](../crates/opencascade-binding-generator/src/main.rs#L109)

Read [README.md](../crates/opencascade-binding-generator/README.md#1-1), lines 1 to 200

Read [main.rs](../crates/opencascade-binding-generator/src/main.rs#109-109), lines 109 to 600

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3789-3789), lines 3789 to 3900

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3900-3900), lines 3900 to 4150

Searched for regex `performance|optimize|slow|cache|profil` (`**/crates/opencascade-binding-generator/src/**`), no results

Created memory file [binding_generator_analysis.md](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/memory-tool/memories/OTczOWE5ZjUtYjAyYS00YzA1LWIzOTktMjdkNTU1NDZhYWY5/binding_generator_analysis.md)

Now let me time the regeneration and profile where time is spent:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && wc -l crates/opencascade-binding-generator/src/*.rs
```
<details>
<summary>Output (10 lines)</summary>

```
     358 crates/opencascade-binding-generator/src/config.rs
     137 crates/opencascade-binding-generator/src/header_deps.rs
      13 crates/opencascade-binding-generator/src/lib.rs
    1580 crates/opencascade-binding-generator/src/main.rs
    1147 crates/opencascade-binding-generator/src/model.rs
     311 crates/opencascade-binding-generator/src/module_graph.rs
    2232 crates/opencascade-binding-generator/src/parser.rs
    1338 crates/opencascade-binding-generator/src/resolver.rs
     613 crates/opencascade-binding-generator/src/type_mapping.rs
    7729 total
```
</details>

Let me look at how the wrapper generation code works, and check git history for recent changes:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -30 -- crates/opencascade-binding-generator/
```
<details>
<summary>Output (30 lines)</summary>

```
11ae3b7 (HEAD -> main) Fix nondeterministic C++ #include ordering; refactor cpp codegen
f0628a2 Remove monolithic FFI generation; generate per-toolkit files directly
f8d8154 Split ffi.rs extern blocks into per-toolkit files
b5624ea Split wrappers.cpp into per-toolkit files for parallel C++ compilation
77d6fb4 Remove nested unsafe blocks in generated code
5ecf2d6 Refactor exception handling to use generic OcctResult<T> struct
be699c5 Add C++ exception handling across FFI boundary
4e5221e Refactor: extract is_void_ptr_type_name() from repeated pattern
ec36842 Move void_pointer_types from generator source to bindings.toml
1c50ffc Handle Aspect_RenderingContext as *mut c_void in FFI
3c30e2b Make golden tests numeric-tolerant and restore golden artifacts
62f4f39 Refactor short-name mapping to use type-module context
1d9142e Use short_name_for_module for type short-name paths
e5d9504 Prune completed and low-value Future Work items
1d9b5e8 Refactor main input selection into typed struct
428e63c Always resolve header dependencies and remove stale resolve_deps config
c3d46ac Move hardcoded symbol-name rules into bindings.toml config
547bdb0 Refactor parser class-type normalization helper usage
64f17b4 Normalize ShapePersistent geometryBase aliases to public typedefs
283e338 Support pointer template instantiations in auto template binding
2b5610c Add skip-stats script and refine README skip details
b4e2658 Refresh README skipped-symbol stats from generated output
9c2ee1e Support fixed-size non-reference C array parameters
2bcb23c Refactor fixed-array binding conversion helpers
3dd5382 Add fixed-size array reference bindings to generator
4bb36a4 docs(binding-generator): refresh full skipped symbols section
fc835b8 Add guarded namespace typedef auto-resolution
6f1e062 Update binding-generator README for IMeshData typedef fix
7675192 Refactor wrapper include merge logic
35d828e Sort wrappers.cpp includes globally and deterministically
```
</details>

Let me first add some timing instrumentation to understand where time is spent during regeneration:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat scripts/regenerate-bindings.sh
```
<details>
<summary>Output (63 lines)</summary>

```
#!/bin/bash
# Regenerate OpenCASCADE bindings
#
# This script regenerates the FFI bindings for opencascade-sys.
# Run from the repository root.
#
# Usage:
#   ./scripts/regenerate-bindings.sh
#
# Requirements:
#   - Xcode command line tools (for libclang)
#   - OCCT headers built in target/OCCT/include

set -euo pipefail

# Repository root (script assumes it's run from repo root)
REPO_ROOT="${REPO_ROOT:-$(pwd)}"
cd "$REPO_ROOT"

# Paths
OCCT_INCLUDE="$REPO_ROOT/target/OCCT/include"
OUTPUT_DIR="$REPO_ROOT/crates/opencascade-sys/generated"
CONFIG_FILE="$REPO_ROOT/crates/opencascade-sys/bindings.toml"

# Check prerequisites
if [[ ! -d "$OCCT_INCLUDE" ]]; then
    echo "Error: OCCT include directory not found at $OCCT_INCLUDE"
    echo "Run 'cargo build -p occt-sys' first to build OCCT"
    exit 1
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Error: Config file not found at $CONFIG_FILE"
    exit 1
fi

# Build the generator in release mode to avoid a UB crash in the `clang` crate
# (v2.0.0) that only manifests in debug builds on Rust >= 1.78. See
# crates/opencascade-binding-generator/Cargo.toml for details.
echo "Building binding generator..."
cargo build --release -p opencascade-binding-generator

echo "Generating bindings from $CONFIG_FILE..."

# Clean generated directory
echo "Cleaning $OUTPUT_DIR..."
rm -f "$OUTPUT_DIR"/*.rs "$OUTPUT_DIR"/*.hxx "$OUTPUT_DIR"/*.cpp

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

# Run the generator
"$REPO_ROOT/target/release/occt-bindgen" \
    --config "$CONFIG_FILE" \
    -I "$OCCT_INCLUDE" \
    -o "$OUTPUT_DIR" \
    "$@"

echo ""
echo "Bindings generated in $OUTPUT_DIR"
```
</details>

Now let me time the regeneration with some instrumentation. First let me look at the main.rs code that does the generation:

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1-L101">main.rs</a></summary>

```
//! OCCT Binding Generator CLI
//!
//! A tool using libclang to parse OCCT C++ headers and generate extern "C" FFI bindings
//! Generates FFI bindings with a single ffi.rs module and per-module re-exports.

use opencascade_binding_generator::{codegen, config, header_deps, model, module_graph, parser, resolver, type_mapping};

use anyhow::Result;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
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

fn parse_class_method_pairs(entries: &[String], field_name: &str) -> HashSet<(String, String)> {
    entries
        .iter()
        .filter_map(|s| {
            if let Some(pos) = s.rfind("::") {
                let class_name = s[..pos].to_string();
                let method_name = s[pos + 2..].to_string();
                Some((class_name, method_name))
            } else {
                eprintln!(
                    "Warning: invalid {} entry (expected ClassName::MethodName): {}",
                    field_name, s
                );
                None
            }
        })
        .collect()
}

struct SelectionInputs {
    explicit_headers: Vec<PathBuf>,
    exclude_set: std::collections::HashSet<String>,
    exclude_modules: Vec<String>,
    exclude_methods: HashSet<(String, String)>,
```
</details>

Let me look at the main.rs file where generation happens, and the codegen module:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls crates/opencascade-binding-generator/src/codegen/
```
```
bindings.rscollections.rscpp.rsmod.rsrust.rs
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && wc -l crates/opencascade-binding-generator/src/codegen/*.rs
```
<details>
<summary>Output (6 lines)</summary>

```
    5608 crates/opencascade-binding-generator/src/codegen/bindings.rs
    1229 crates/opencascade-binding-generator/src/codegen/collections.rs
    1004 crates/opencascade-binding-generator/src/codegen/cpp.rs
       6 crates/opencascade-binding-generator/src/codegen/mod.rs
    1185 crates/opencascade-binding-generator/src/codegen/rust.rs
    9032 total
```
</details>

Let me read the cpp.rs file that generates the wrapper files — this is the likely bottleneck:

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs">cpp.rs</a></summary>

```
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
         
... (truncated, 40385 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_012JxYwFuXw9ghZmkFfvQU4Z__vscode-1772068645608/content.txt#1-1), lines 1 to 1004

> 💭 I see the `extend_unique_headers` function - it uses `Vec::contains()` which is O(n) for each check. This is called very frequently during header collection. Let me look at the main function to see how things are structured and how the generation is invoked, plus look at the `bindings.rs` for `emit_cpp_class`:

I see a potential issue already: `extend_unique_headers` uses `Vec::contains()` which is O(n) per lookup, and it's called many times. Let me look at main.rs where the generation is orchestrated, and also check `emit_cpp_class`:Now let me look at `bindings.rs` for `emit_cpp_class` and the main.rs generation loop, and also time the current regeneration:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "pub fn emit_cpp_class" crates/opencascade-binding-generator/src/codegen/bindings.rs
```
```
3789:pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
```

<details>
<summary>Reading lines 3789-3850 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3789-L3851">bindings.rs</a></summary>

```
pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    let ffi_cn = &bindings.cpp_name;  // Rust-safe flattened name (for FFI function names)
    let cn = &bindings.cpp_qualified_name;  // C++ qualified name (for C++ type expressions)

    // POD structs don't need C++ wrappers, but we generate a sizeof helper
    // so Rust tests can verify layout compatibility at runtime,
    // and a destructor so CppDeletable can be implemented (needed when returned by pointer).
    if bindings.is_pod_struct {
        writeln!(output, "// sizeof helper for POD struct {}", cn).unwrap();
        writeln!(output, "extern \"C\" size_t {}_sizeof() {{ return sizeof({}); }}", ffi_cn, cn).unwrap();
        writeln!(output, "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}", ffi_cn, cn).unwrap();
        writeln!(output).unwrap();
        return output;
    }

    writeln!(output, "// ========================").unwrap();
    writeln!(output, "// {} wrappers", cn).unwrap();
    writeln!(output, "// ========================").unwrap();
    writeln!(output).unwrap();

    // 1. Constructor wrappers (skip convenience — they are Rust-only)
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let params_cpp: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let args_str = ctor.cpp_arg_exprs.join(", ");
        let params_str = params_cpp.join(", ");

        writeln!(
            output,
            "extern \"C\" OcctResult<{cn}*> {fn_name}({params_str}) {{",
            fn_name = ctor.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    try {{ return {{new {cn}({args_str}), nullptr}}; }}"
        )
        .unwrap();
        writeln!(output, "    OCCT_CATCH_RETURN").unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 2. ByValueReturn wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueReturn)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params: Vec<String> = wm
            .params
```
</details>

Now let me add timing instrumentation to the binding generator and run it to see where the bottleneck is:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "generate_wrappers_for_group\|generate_preamble\|generate_wrappers\|emit_cpp_class\|collect_headers_from_bindings\|batch_find_defining\|find_defining_header\|collect_all_required_headers\|collect_template_arg_headers\|extend_unique_headers" crates/opencascade-binding-generator/src/main.rs | head -40
```
```
1052:        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, include_dir);
1166:            let cpp_code = codegen::cpp::generate_wrappers_for_group(
1189:        let cpp_code = codegen::cpp::generate_wrappers(
```

<details>
<summary>Reading lines 1020-1210 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1020-L1211">main.rs</a></summary>

```
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
        non_allocatable_classes,
        &module_to_toolkit,
    );
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());

    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
        let filename = format!("ffi_extern_{}.rs", toolkit_name);
        let path = args.output.join(&filename);
        std::fs::write(&path, code)?;
        generated_rs_files.push(path.clone());
        println!("  Wrote: {}", path.display());
    }
    println!("  {} toolkit extern modules", toolkit_files.len());

    // 2. Generate wrappers (split by toolkit or monolithic)
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        // Generate preamble header
        let include_dir = args.include_dirs.first().map(|p| p.as_path());
        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, include_dir);
        let preamble_path = args.output.join("occt_preamble.hxx");
        std::fs::write(&preamble_path, &preamble)?;
        println!("  Wrote: {}", preamble_path.display());

        // Helper to extract C++ module prefix from a name
        fn cpp_module(name: &str) -> &str {
            let base = name.split("::").next().unwrap_or(name);
            base.split('_').next().unwrap_or(base)
        }

        // Group ClassBindings by toolkit
        let mut tk_bindings: HashMap<String, Vec<&codegen::bindings::ClassBindings>> = HashMap::new();
        for b in &all_bindings {
            let tk = module_to_toolkit.get(b.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_bindings.entry(tk).or_default().push(b);
        }

        // Group ParsedClass by toolkit (for header collection)
        let mut tk_classes: HashMap<String, Vec<&&ParsedClass>> = HashMap::new();
        for c in all_classes {
            let tk = module_to_toolkit.get(c.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_classes.entry(tk).or_default().push(c);
        }

        // Group FunctionBindings by toolkit (use namespace = C++ module)
        let mut tk_functions: HashMap<String, Vec<&codegen::bindings::FunctionBinding>> = HashMap::new();
        for f in &all_function_bindings {
            let tk = module_to_toolkit.get(f.namespace.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_functions.entry(tk).or_default().push(f);
        }

        // Group collections by toolkit (derive C++ module from typedef prefix)
        let mut tk_collections: HashMap<String, Vec<&codegen::collections::CollectionInfo>> = HashMap::new();
        for c in &all_collections {
            let module = cpp_module(&c.typedef_name);
            let tk = module_to_toolkit.get(module).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_collections.entry(tk).or_default().push(c);
        }

        // Group nested types by toolkit (derive C++ module from cpp_name prefix)
        let mut tk_nested: HashMap<String, Vec<&codegen::rust::NestedTypeInfo>> = HashMap::new();
        for nt in &nested_types {
            let module = cpp_module(&nt.cpp_name);
            let tk = module_to_toolkit.get(module).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_nested.entry(tk).or_default().push(nt);
        }

        // Assign template instantiations to toolkits by their module
        let mut tk_templates: HashMap<String, HashMap<String, config::TemplateInstantiation>> = HashMap::
... (truncated, 8689 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_01T1GyQPJeKzBDbu3kTbCjtt__vscode-1772068645613/content.txt)

Now let me add timing to the main.rs to instrument where time is spent. Let me look at the broader structure first:

<details>
<summary>Reading lines 900-1020 of <a href="../crates/opencascade-binding-generator/src/main.rs#L900-L1021">main.rs</a></summary>

```
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
    ambiguous_methods: &HashSet<(String, String)>,
    non_allocatable_classes: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
    template_instantiations: &HashMap<String, config::TemplateInstantiation>,
    split_config: &config::SplitConfig,
    config_dir: Option<&Path>,
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
    let mut extra_typedef_names = parser::get_collected_typedef_names();
    // Add template instantiation alias names so they get destructor generation
    for (spelling, _) in template_instantiations {
        extra_typedef_names.insert(config::template_alias_name(spelling));
    }
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
   
... (truncated, 5100 chars)
```
</details>

Now let me profile the actual regeneration:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1 | tail -3
```
```
    Finished `release` profile [optimized] target(s) in 0.21s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" time ./target/release/occt-bindgen --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/test_output 2>&1
```
<details>
<summary>Output (6 lines) — exit code 134</summary>

```
dyld[89006]: Library not loaded: @rpath/libclang.dylib
  Referenced from: <5C4D16F7-5B67-35B0-8BDA-B278DFC8DF1C> /Users/sethml/src/opencascade-rs/target/release/occt-bindgen
  Reason: no LC_RPATH's found
time: command terminated abnormally
        0.17 real         0.00 user         0.00 sys
Abort trap: 6
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && mkdir -p tmp/test_output && rm -f tmp/test_output/*.rs tmp/test_output/*.hxx tmp/test_output/*.cpp && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -20 "/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_014KeWnrPfvzqSHdxbWFZvvK__vscode-1772068645620/content.txt"
```
<details>
<summary>Output (20 lines)</summary>

```
s (function-only module, 18 functions)                                            Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xb_re
p_mesh.rs (function-only module, 1 functions)                                     Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hermi
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 3 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
to_geom.rs (function-only module, 48 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  321 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  228.8s
```
</details>

228 seconds. Let me add instrumentation to understand where the time goes. Let me add some eprintln timing to the main.rs: Let me add timing instrumentation to the generate_output function:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L962-L964">main.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "962:vw",
      "content": "    let t_start = std::time::Instant::now();\n    println!(\"\\n=== Generating FFI output ===\");"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":962,"endLine":963}
```
</details>

Now let me add timing after each major phase. Let me re-read the file to get updated line hashes:

<details>
<summary>Reading lines 962-1000 of <a href="../crates/opencascade-binding-generator/src/main.rs#L962-L1001">main.rs</a></summary>

```
    let t_start = std::time::Instant::now();
    println!("\n=== Generating FFI output ===");

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
    let mut extra_typedef_names = parser::get_collected_typedef_names();
    // Add template instantiation alias names so they get destructor generation
    for (spelling, _) in template_instantiations {
        extra_typedef_names.insert(config::template_alias_name(spelling));
    }
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
```
</details>

<details>
<summary>Reading lines 1000-1060 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1000-L1061">main.rs</a></summary>

```
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );

    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
    // and optionally for C++ split)
    let module_to_toolkit = if let Some(ref occt_src) = split_config.occt_source_dir {
        let occt_source_dir = config_dir.as_ref()
            .expect("config_dir required for toolkit split")
            .join(occt_src);
        config::discover_toolkit_modules(&occt_source_dir)?
    } else {
        HashMap::new()
    };

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();
    // 1. Generate ffi type definitions and per-toolkit extern blocks
    println!("Generating ffi_types.rs + per-toolkit extern modules...");
    let (ffi_code, nested_types, toolkit_files) = codegen::rust::generate_ffi_split(
        all_classes,
        &all_functions,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
        non_allocatable_classes,
        &module_to_toolkit,
    );
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());

    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
        let filename = format!("ffi_extern_{}.rs", toolkit_name);
        let path = args.output.join(&filename);
        std::fs::write(&path, code)?;
        generated_rs_files.push(path.clone());
        println!("  Wrote: {}", path.display());
    }
    println!("  {} toolkit extern modules", toolkit_files.len());

    // 2. Generate wrappers (split by toolkit or monolithic)
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        // Generate preamble header
        let include_dir = args.include_dirs.first().map(|p| p.as_path());
        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, include_dir);
        let preamble_path = args.output.join("occt_preamble.hxx");
        std::fs::write(&preamble_path, &preamble)?;
        println!("  Wrote: {}", preamble_path.display());

        // Helper to extract C++ module prefix from a name
        fn cpp_module(name: &str) -> &str {
            let base = name.split("::").next().unwrap_or(name);
```
</details>

<details>
<summary>Reading lines 1140-1210 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1140-L1211">main.rs</a></summary>

```

        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)
        let class_header_map: HashMap<String, String> = all_classes.iter()
            .map(|c| (c.name.clone(), c.source_header.clone()))
            .collect();

        println!("  {} toolkits", sorted_toolkits.len());
        for toolkit in &sorted_toolkits {
            let empty_bindings = Vec::new();
            let empty_classes = Vec::new();
            let empty_functions = Vec::new();
            let empty_collections = Vec::new();
            let empty_nested = Vec::new();
            let empty_templates = HashMap::new();
            let empty_own = HashSet::new();

            let bindings = tk_bindings.get(toolkit).unwrap_or(&empty_bindings);
            let classes_refs = tk_classes.get(toolkit).unwrap_or(&empty_classes);
            let functions = tk_functions.get(toolkit).unwrap_or(&empty_functions);
            let collections = tk_collections.get(toolkit).unwrap_or(&empty_collections);
            let nested = tk_nested.get(toolkit).unwrap_or(&empty_nested);
            let _templates = tk_templates.get(toolkit).unwrap_or(&empty_templates);
            let own_classes = tk_own_classes.get(toolkit).unwrap_or(&empty_own);

            // Flatten &&ParsedClass to &ParsedClass
            let classes: Vec<&ParsedClass> = classes_refs.iter().copied().copied().collect();

            let cpp_code = codegen::cpp::generate_wrappers_for_group(
                toolkit,
                &classes,
                collections,
                known_headers,
                bindings,
                functions,
                nested,
                &handle_able_classes,
                template_instantiations,
                "occt_preamble.hxx",
                own_classes,
                &class_header_map,
                include_dir,
            );
            let cpp_path = args.output.join(format!("wrappers_{}.cpp", toolkit));
            std::fs::write(&cpp_path, &cpp_code)?;
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
        }
    } else {
        // Monolithic wrappers.cpp (original behavior)
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
            template_instantiations,
        );
        let cpp_path = args.output.join("wrappers.cpp");
        std::fs::write(&cpp_path, &cpp_code)?;
        println!("  Wrote: {}", cpp_path.display());
    }

    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
    let mut bindings_by_module: HashMap<String, Vec<&codegen::bindings::ClassBindings>> =
```
</details>

Let me add timing instrumentation at key points:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1002-L1003">main.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1035-L1036">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1146-L1147">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1186-L1187">-1+1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/main.rs#L988-L989">main.rs -1+1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    eprintln!(\"[TIMING] compute_all_class_bindings: {:?}\", t_start.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "988:zc",
      "insertAfter": false
    },
    {
      "content": "    eprintln!(\"[TIMING] compute_all_function_bindings: {:?}\", t_start.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1002:dd",
      "insertAfter": false
    },
    {
      "content": "    eprintln!(\"[TIMING] generate_ffi_split: {:?}\", t_start.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1035:dd",
      "insertAfter": false
    },
    {
      "content": "    eprintln!(\"[TIMING] pre-wrapper setup: {:?}\", t_start.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1146:de",
      "insertAfter": false
    },
    {
      "content": "            eprintln!(\"[TIMING] toolkit {}: {:?}\", toolkit, t_start.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1186:cy",
      "insertAfter": false
    }
  ]
}
```
**Output:**
```
{
  "applied": 4,
  "failed": 1,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 1002,
      "endLine": 1186
    }
  ],
  "errors": [
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "988:zc",
      "error": "hash mismatch at line 988: file has 'dd', edit specified 'zc'"
    }
  ]
}
```
</details>

<details>
<summary>Reading lines 986-992 of <a href="../crates/opencascade-binding-generator/src/main.rs#L986-L993">main.rs</a></summary>

```
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L988-L990">main.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    eprintln!(\"[TIMING] compute_all_class_bindings: {:?}\", t_start.elapsed());\n",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "988:dd",
      "insertAfter": false
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":988,"endLine":989}
```
</details>

Also add timing after the module re-exports. Let me check where that is:

<details>
<summary>Reading lines 1208-1215 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1208-L1216">main.rs</a></summary>

```
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
    let mut bindings_by_module: HashMap<String, Vec<&codegen::bindings::ClassBindings>> =
        HashMap::new();
    for b in &all_bindings {
        bindings_by_module
            .entry(b.module.clone())
```
</details>

Now let me build and run with the timing instrumentation:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (17 lines) — exit code 101</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error: this file contains an unclosed delimiter
    --> crates/opencascade-binding-generator/src/main.rs:1582:3
     |
 959 | ) -> Result<()> {
     |                 - unclosed delimiter
...
1148 |         for toolkit in &sorted_toolkits {
     |                                         - this delimiter might not be properly closed...
...
1188 |     } else {
     |     - ...as it matches this but it has different indentation
...
1582 | }
     |  ^

error: could not compile `opencascade-binding-generator` (bin "occt-bindgen") due to 1 previous error
```
</details>

Let me check the edits I made:

<details>
<summary>Reading lines 1185-1200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1185-L1201">main.rs</a></summary>

```
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
            eprintln!("[TIMING] toolkit {}: {:?}", toolkit, t_start.elapsed());
    } else {
        // Monolithic wrappers.cpp (original behavior)
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
            template_instantiations,
```
</details>

The timing line needs to be inside the for loop's closing brace. Let me fix that:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1187-L1189">main.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            eprintln!(\"[TIMING] toolkit {}: {:?}\", toolkit, t_start.elapsed());\n        }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1187:hu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":1187,"endLine":1188}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 2.62s
```

Now run with the instrumented binary and capture timing:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && rm -f tmp/test_output/*.rs tmp/test_output/*.hxx tmp/test_output/*.cpp && scripts/regenerate-bindings.sh 2>&1 | grep -E "\[TIMING\]|real|wrappers_|Splitting|Generating|Building"
```

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_012kbKbAHeAqTrxKNaVdowJK__vscode-1772068645674/content.txt)

> 💭 Now I can see the clear pattern. The total time for generating all wrappers is about 169 seconds (182.8 - 13.8). Looking at the per-toolkit times:
> 
> - TKDEIGES (463 classes): 48.4s - 29.0s = 19.4s 
> - TKDESTEP (1130 classes): 93.5s - 49.9s = 43.6s
> - TKernel (233 classes): 182.5 - 173.2 = 9.3s
> - TKV3d (227 classes): 157.2 - 149.4 = 7.8s
> 
> 
> - TKBO (133 classes): 0.94s
> - TKBRep (87 classes): 1.9s
> - TKBin (10 classes): 0.63s
> - TKBinL (37 classes): 1.1s
> 
> Let me recalculate these as cumulative deltas to get the actual per-toolkit generation times. Looking at the compilation times for the remaining modules, TKDESTEP stands out dramatically with 43.6 seconds for 1130 classes—that's a massive spike compared to everything else. TKDEIGES also takes significant time at 19.4 seconds with 463 classes, while the other modules are relatively quick. Now I'm looking at the compilation times for the remaining modules—TKHLR takes about 1.9 seconds, TKLCAF is one of the slower ones at 4.5 seconds, and TKMath is also significant at 4.1 seconds with a large number of functions to process. The smaller modules like TKOffset and TKPrim are wrapping up quickly. Looking at the compilation times for the remaining modules, TKV3d takes the longest at 7.8 seconds for 227 classes, while most others are relatively quick, with TKXMesh contributing nothing and several modules like TKVCAF and TKXml being nearly instantaneous.
> 
> The total wrapper generation comes to about 169 seconds, and the real bottlenecks are clear: TKDESTEP dominates at 43.6 seconds for over a thousand classes, followed by TKDEIGES at 19.4 seconds for 463 classes, and TKernel at 9.3 seconds.
> 
> Looking at the rest of the breakdown, I notice something interesting—TKXCAF takes 5.3 seconds for just 51 classes while TKBO handles 133 classes in under a second, so there's a huge variance in how long each class takes to process.
> 
> I'm tracing through the `generate_wrappers_for_group` function to find the bottleneck. It's calling several collection functions, but the real culprits seem to be `collect_all_required_headers` and the header file reading that happens in `batch_find_defining_headers`. There's also an O(n²) performance issue in `extend_unique_headers` where it's checking `Vec::contains()` repeatedly during insertion, and `batch_find_defining_headers` is doing disk I/O that gets called once per toolkit.
> 
> The core problem is that `find_defining_header` scans all known OCCT header files from disk repeatedly across ~49 toolkits, creating massive redundant I/O. I need to check how many headers are actually being read to understand the scale of this issue.

Excellent data! The total wrapper generation takes ~169s across 49 toolkits. Key insight: for each toolkit, `collect_headers_from_bindings` calls `batch_find_defining_headers` which reads header files from disk. This is called for every single toolkit, causing massive redundant I/O.

Let me verify this hypothesis by looking at the main bottleneck more closely:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls target/OCCT/include/*.hxx | wc -l
```
```
    6875
```

**Todo list:**
- ✅ Profile binding regeneration
- 🔄 Find performance regression
- ⬜ Fix the regression
- ⬜ Verify fix and commit

6,875 header files being scanned in `batch_find_defining_headers` for each of ~49 toolkits = ~337k file reads. Let me add more specific timing to confirm this is the bottleneck, and also look at `extend_unique_headers` which is O(n²):

<details>
<summary>Reading lines 832-880 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L832-L881">cpp.rs</a></summary>

```
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
```
</details>

I see the problem clearly now. Let me add detailed timing to `generate_wrappers_for_group`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L828-L830">cpp.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L875-L876">+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let t0 = std::time::Instant::now();\n    let mut output = String::new();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "828:hr"
    },
    {
      "content": "    eprintln!(\"  [cpptiming] {}: headers_from_bindings={:?}\", group_name, t0.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "insertAfter": true,
      "lineHashes": "873:pd"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 828,
      "endLine": 875
    }
  ]
}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1 | tail -3
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 7.77s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && scripts/regenerate-bindings.sh 2>&1 | grep -E "\[cpptiming\]" | sort -t= -k2 -rn | head -20
```

Confirmed. `collect_headers_from_bindings` is the bottleneck. TKDESTEP alone takes **43.5 seconds** in that one call. Each toolkit is spending 0.3-1.0s on this, and there are 49 toolkits, totalling well over 100 seconds of pure header scanning.

The problem is clear: `batch_find_defining_headers` reads header files from disk **for each toolkit separately**, when it should cache the results globally. Let me also look at the pattern more closely:

**Todo list:**
- ✅ Profile binding regeneration
- ✅ Find performance regression
- 🔄 Fix the regression
- ⬜ Verify fix and commit

The fix strategy: build a global cache that maps type names to their defining headers at the start of the generation phase, rather than re-scanning header files for every toolkit. Let me first remove the timing instrumentation and implement the fix:

Let me look at how `collect_headers_from_bindings` is used and what it does, so I can design the caching solution:

<details>
<summary>Reading lines 228-411 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L228-L412">cpp.rs</a></summary>

```
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
            let header
... (truncated, 8096 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_01NbM7JM3KvPx7ta6Fb2ozP5__vscode-1772068645683/content.txt)

> 💭 Now I understand the problem clearly. The solution is to:
> 
> 1. Build a global type-to-header cache once at the start, by scanning all headers once for ALL unresolved types across all toolkits
> 2. Pass this cache to `generate_wrappers_for_group` so it doesn't need to do file I/O
> 
> The approach:
> 1. Add a `HeaderResolver` struct that caches type→header lookups
> 2. Pre-compute it once in main.rs before iterating over toolkits  
> 3. Pass it to `generate_wrappers_for_group` and `collect_headers_from_bindings` instead of `include_dir`
> 
> 
> 4. Collect all unresolved type names from every binding upfront, resolve them all in a single batch operation, and cache the results in a HashMap so each toolkit can look up types without rescanning the filesystem.
> 
> Actually, I can simplify this further by moving the batch resolution entirely outside the per-toolkit loop—collect all type names that need resolution across all bindings first, do one batch lookup, merge everything into the class header map, then the per-toolkit calls won't need filesystem access at all. I should also check `collect_template_arg_headers` since it's another place doing file I/O through `find_defining_header`.

Now I understand the issue fully. The fix is to:
1. Build a **global** type→header resolution cache once (scan headers once for ALL types)
2. Pass the cache to per-toolkit generation instead of having each toolkit re-scan files

Let me also check `collect_template_arg_headers` which also calls `find_defining_header`:

<details>
<summary>Searched for text <code>find_defining_header</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 5 results</summary>

- [cpp.rs:274](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L274)
- [cpp.rs:322](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L322)
- [cpp.rs:331](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L331)
- [cpp.rs:332](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L332)
- [cpp.rs:416](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L416)
</details>

is called per-template-type individually from `collect_template_arg_headers`. But that's called from `generate_preamble` (once) and `generate_wrappers_for_group` → during header dedup (removing preamble headers). The preamble call is fine (only once). But the per-toolkit calls are also a problem.

Let me design the solution. I'll create a `HeaderResolver` that does the file scanning once and caches results: 

First, let me remove my timing instrumentation and then implement the fix:

<details>
<summary>Reading lines 825-830 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L825-L831">cpp.rs</a></summary>

```
    class_header_map: &std::collections::HashMap<String, String>,
    include_dir: Option<&Path>,
) -> String {
    let t0 = std::time::Instant::now();
    let mut output = String::new();

```
</details>

<details>
<summary>Reading lines 870-878 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L870-L879">cpp.rs</a></summary>

```
    // Add headers for cross-toolkit types referenced in bindings (especially inherited methods)
    extend_unique_headers(
        &mut headers,
        collect_headers_from_bindings(bindings, known_headers, class_header_map, include_dir),
    );
    eprintln!("  [cpptiming] {}: headers_from_bindings={:?}", group_name, t0.elapsed());


    // Add headers for ALL handle types (including cross-toolkit ones)
```
</details>

Now let me implement the fix. The strategy:

1. Add a `HeaderResolver` struct that caches type→header lookups
2. Build the resolver once in `main.rs` before the toolkit loop, based on all unresolved types
3. Pass it to `collect_headers_from_bindings` and `collect_template_arg_headers` instead of `include_dir`
4. Also fix `extend_unique_headers` to be O(n) instead of O(n²) by using a HashSet

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L828-L829">cpp.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L874-L875">-1+1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let mut output = String::new();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "828:my\n829:hr"
    },
    {
      "content": "    );",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "874:pd\n875:xo"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 828,
      "endLine": 875
    }
  ]
}
```
</details>

Good, timing removed from cpp.rs. Now let me remove the timing from main.rs:

<details>
<summary>Reading lines 960-995 of <a href="../crates/opencascade-binding-generator/src/main.rs#L960-L996">main.rs</a></summary>

```
    use model::ParsedClass;

    let t_start = std::time::Instant::now();
    println!("\n=== Generating FFI output ===");

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
    let mut extra_typedef_names = parser::get_collected_typedef_names();
    // Add template instantiation alias names so they get destructor generation
    for (spelling, _) in template_instantiations {
        extra_typedef_names.insert(config::template_alias_name(spelling));
    }
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);
    eprintln!("[TIMING] compute_all_class_bindings: {:?}", t_start.elapsed());

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
```
</details>

<details>
<summary>Reading lines 996-1052 of <a href="../crates/opencascade-binding-generator/src/main.rs#L996-L1053">main.rs</a></summary>

```
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );
    eprintln!("[TIMING] compute_all_function_bindings: {:?}", t_start.elapsed());
    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
    // and optionally for C++ split)
    let module_to_toolkit = if let Some(ref occt_src) = split_config.occt_source_dir {
        let occt_source_dir = config_dir.as_ref()
            .expect("config_dir required for toolkit split")
            .join(occt_src);
        config::discover_toolkit_modules(&occt_source_dir)?
    } else {
        HashMap::new()
    };

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();
    // 1. Generate ffi type definitions and per-toolkit extern blocks
    println!("Generating ffi_types.rs + per-toolkit extern modules...");
    let (ffi_code, nested_types, toolkit_files) = codegen::rust::generate_ffi_split(
        all_classes,
        &all_functions,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
        non_allocatable_classes,
        &module_to_toolkit,
    );
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());
    eprintln!("[TIMING] generate_ffi_split: {:?}", t_start.elapsed());
    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
        let filename = format!("ffi_extern_{}.rs", toolkit_name);
        let path = args.output.join(&filename);
        std::fs::write(&path, code)?;
        generated_rs_files.push(path.clone());
        println!("  Wrote: {}", path.display());
    }
    println!("  {} toolkit extern modules", toolkit_files.len());

    // 2. Generate wrappers (split by toolkit or monolithic)
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        // Generate preamble header
```
</details>

<details>
<summary>Reading lines 1140-1200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1140-L1201">main.rs</a></summary>

```
        }

        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)
        let class_header_map: HashMap<String, String> = all_classes.iter()
            .map(|c| (c.name.clone(), c.source_header.clone()))
            .collect();

    eprintln!("[TIMING] pre-wrapper setup: {:?}", t_start.elapsed());
        for toolkit in &sorted_toolkits {
            let empty_bindings = Vec::new();
            let empty_classes = Vec::new();
            let empty_functions = Vec::new();
            let empty_collections = Vec::new();
            let empty_nested = Vec::new();
            let empty_templates = HashMap::new();
            let empty_own = HashSet::new();

            let bindings = tk_bindings.get(toolkit).unwrap_or(&empty_bindings);
            let classes_refs = tk_classes.get(toolkit).unwrap_or(&empty_classes);
            let functions = tk_functions.get(toolkit).unwrap_or(&empty_functions);
            let collections = tk_collections.get(toolkit).unwrap_or(&empty_collections);
            let nested = tk_nested.get(toolkit).unwrap_or(&empty_nested);
            let _templates = tk_templates.get(toolkit).unwrap_or(&empty_templates);
            let own_classes = tk_own_classes.get(toolkit).unwrap_or(&empty_own);

            // Flatten &&ParsedClass to &ParsedClass
            let classes: Vec<&ParsedClass> = classes_refs.iter().copied().copied().collect();

            let cpp_code = codegen::cpp::generate_wrappers_for_group(
                toolkit,
                &classes,
                collections,
                known_headers,
                bindings,
                functions,
                nested,
                &handle_able_classes,
                template_instantiations,
                "occt_preamble.hxx",
                own_classes,
                &class_header_map,
                include_dir,
            );
            let cpp_path = args.output.join(format!("wrappers_{}.cpp", toolkit));
            std::fs::write(&cpp_path, &cpp_code)?;
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
            eprintln!("[TIMING] toolkit {}: {:?}", toolkit, t_start.elapsed());
        }
    } else {
        // Monolithic wrappers.cpp (original behavior)
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
```
</details>

Now let me implement the fix. The approach:

1. Add a `HeaderResolver` struct that builds a cache of type→header by scanning all headers once
2. Modify `collect_headers_from_bindings` and `collect_template_arg_headers` to use the resolver instead of doing file I/O
3. Build the resolver once in main.rs before the toolkit loop

<details>
<summary>Reading lines 10-17 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L10-L18">cpp.rs</a></summary>

```

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;
use std::path::Path;

/// Generate the shared C++ exception handling boilerplate.
```
</details>

Now let me implement the `HeaderResolver` and refactor the code. I'll read the full structure first:

<details>
<summary>Reading lines 184-230 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L184-L231">cpp.rs</a></summary>

```
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
```
</details>

Now I have a clear picture. Here's my implementation plan:

1. Add `HeaderResolver` - a cache built by scanning all headers once
2. Modify `collect_headers_from_bindings` to accept `&HeaderResolver` instead of `include_dir`  
3. Modify `collect_template_arg_headers` to accept `&HeaderResolver` instead of `include_dir`
4. Modify `generate_wrappers_for_group` to accept `&HeaderResolver` instead of `include_dir`  
5. Build the resolver once in main.rs
6. Fix `extend_unique_headers` to use HashSet for O(n) dedup 

Let me now write the new `HeaderResolver` struct and rewrite the affected functions. I'll start with the `crates/opencascade-binding-generator/src/codegen/cpp.rs` changes:

<details>
<summary>Reading lines 412-462 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L412-L463">cpp.rs</a></summary>

```

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

```
</details>

<details>
<summary>Reading lines 617-630 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L617-L631">cpp.rs</a></summary>

```
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
```
</details>

Now let me look at the full function signature for `generate_wrappers_for_group` and `generate_preamble` as well:

<details>
<summary>Reading lines 755-830 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L755-L831">cpp.rs</a></summary>

```
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
```
</details>

Now I have the full picture. Let me implement the fix. I'll:

1. Add `HeaderResolver` struct 
2. Refactor `collect_headers_from_bindings` to use it
3. Refactor `collect_template_arg_headers` to use it
4. Update `generate_preamble` and `generate_wrappers_for_group` signatures
5. Fix `extend_unique_headers`

Let me first write the `HeaderResolver`:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L11-L210">cpp.rs -1+199</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "use crate::model::{ParsedClass, Type};\nuse crate::resolver::SymbolTable;\nuse std::collections::{HashMap, HashSet};\nuse std::fmt::Write;\nuse std::path::Path;\n\n/// Pre-built cache mapping type names to their defining OCCT headers.\n///\n/// Built once by scanning header files, then shared across all toolkit\n/// wrapper generations to avoid redundant file I/O.\npub struct HeaderResolver {\n    /// type_name → header_name for types found via header scanning\n    cache: HashMap<String, String>,\n}\n\nimpl HeaderResolver {\n    /// Build a HeaderResolver by scanning OCCT headers for typedef/enum definitions.\n    ///\n    /// Collects all type names that need resolution from all bindings, then does\n    /// a single batch scan of header files.\n    pub fn build(\n        all_bindings: &[super::bindings::ClassBindings],\n        template_instantiations: &HashMap<String, crate::config::TemplateInstantiation>,\n        known_headers: &HashSet<String>,\n        class_header_map: &HashMap<String, String>,\n        include_dir: &Path,\n    ) -> Self {\n        use std::io::BufRead;\n\n        // 1. Collect ALL unresolved type names across all bindings\n        let mut unresolved = HashSet::new();\n        let all_b_refs: Vec<&super::bindings::ClassBindings> = all_bindings.iter().collect();\n        collect_unresolved_from_bindings(&all_b_refs, known_headers, class_header_map, &mut unresolved);\n\n        // 2. Collect unresolved types from template instantiation spellings\n        for spelling in template_instantiations.keys() {\n            collect_unresolved_from_template_spelling(spelling, known_headers, &mut unresolved);\n        }\n\n        // 3. Batch-resolve all unresolved types by scanning headers once\n        let mut cache = HashMap::new();\n        if !unresolved.is_empty() {\n            let mut remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();\n\n            // Group by module prefix for prioritized search\n            let mut prefix_types: HashMap<&str, Vec<&str>> = HashMap::new();\n            for name in &remaining {\n                let prefix = name.split('_').next().unwrap_or(\"\");\n                prefix_types.entry(prefix).or_default().push(name);\n            }\n\n            let mut sorted_known: Vec<&String> = known_headers.iter().collect();\n            sorted_known.sort();\n\n            // Pass 1: Search headers matching module prefix\n            for header_name in &sorted_known {\n                if remaining.is_empty() {\n                    break;\n                }\n                let header_prefix = header_name.split('_').next().unwrap_or(\"\");\n                let relevant_types: Vec<&str> = prefix_types\n                    .get(header_prefix)\n                    .map(|v| v.iter().copied().filter(|t| remaining.contains(t)).collect())\n                    .unwrap_or_default();\n                if relevant_types.is_empty() {\n                    cont
... (truncated, 9174 chars)
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":11,"endLine":213}
```
</details>

Now update `collect_headers_from_bindings` to use the resolver:

<details>
<summary>Reading lines 405-430 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L405-L431">cpp.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 430-475 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L430-L476">cpp.rs</a></summary>

```
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

```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L419-L479">cpp.rs -1+60</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Collect headers for types referenced in ClassBindings methods (especially inherited methods)\n/// that may come from other toolkits. Uses the pre-built HeaderResolver cache for types\n/// that don't have their own .hxx file.\npub fn collect_headers_from_bindings(\n    bindings: &[&super::bindings::ClassBindings],\n    known_headers: &HashSet<String>,\n    class_header_map: &std::collections::HashMap<String, String>,\n    resolver: Option<&HeaderResolver>,\n) -> Vec<String> {\n    let mut headers = HashSet::new();\n    let mut unresolved = HashSet::new();\n    for b in bindings {\n        // Inherited methods are the main source of cross-toolkit type references\n        for im in &b.inherited_methods {\n            for p in &im.params {\n                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n            if let Some(rt) = &im.return_type {\n                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n        }\n        // Also check wrapper methods (may reference cross-toolkit types)\n        for wm in &b.wrapper_methods {\n            for p in &wm.params {\n                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n            if let Some(rt) = &wm.return_type {\n                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n        }\n        // Static methods\n        for sm in &b.static_methods {\n            for p in &sm.params {\n                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n            if let Some(rt) = &sm.return_type {\n                collect_headers_from_cpp_type_str(&rt.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n        }\n        // Constructors\n        for ctor in &b.constructors {\n            for p in &ctor.params {\n                collect_headers_from_cpp_type_str(&p.cpp_type, &mut headers, known_headers, class_header_map, &mut unresolved);\n            }\n        }\n    }\n    // Resolve unresolved types via cached HeaderResolver\n    if !unresolved.is_empty() {\n        if let Some(resolver) = resolver {\n            for type_name in &unresolved {\n                if let Some(header) = resolver.resolve(type_name) {\n                    headers.insert(header.to_string());\n                }\n            }\n        }\n    }\n    let mut result: Vec<_> = headers.into_iter().collect();\n    result.sort();\n    result\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "419:qq\n420:jq\n421:fi\n422:vy\n423:gi\n424:ai\n425:yr\n426:zl\n427:jy\n428:pe\n429:gk\n430:ak\n
... (truncated, 3363 chars)
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":419,"endLine":533}
```
</details>

Now update `collect_template_arg_headers` to use `HeaderResolver` instead of `include_dir`:

<details>
<summary>Reading lines 485-540 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L485-L541">cpp.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 479-485 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L479-L486">cpp.rs</a></summary>

```

/// Extract type names from template arguments and add their headers.
/// Handles nested templates like NCollection_Shared<NCollection_DynamicArray<BRepMesh_Vertex>>
/// If include_dir is provided, searches OCCT headers for types without their own .hxx file.
fn collect_template_arg_headers(
    spelling: &str,
    known_headers: &HashSet<String>,
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L480-L521">cpp.rs -1+41</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Extract type names from template arguments and add their headers.\n/// Handles nested templates like NCollection_Shared<NCollection_DynamicArray<BRepMesh_Vertex>>\n/// Uses the pre-built HeaderResolver for types without their own .hxx file.\nfn collect_template_arg_headers(\n    spelling: &str,\n    known_headers: &HashSet<String>,\n    headers: &mut HashSet<String>,\n    resolver: Option<&HeaderResolver>,\n) {\n    let chars: Vec<char> = spelling.chars().collect();\n    let mut i = 0;\n    while i < chars.len() {\n        if !chars[i].is_alphanumeric() && chars[i] != '_' {\n            i += 1;\n            continue;\n        }\n        let start = i;\n        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {\n            i += 1;\n        }\n        let ident: String = chars[start..i].iter().collect();\n        if ident.starts_with(\"NCollection_\") || ident.starts_with(\"opencascade\") {\n            continue;\n        }\n        if matches!(ident.as_str(), \"handle\" | \"const\" | \"Standard_Real\" | \"Standard_Integer\"\n            | \"Standard_Boolean\" | \"Standard_ShortReal\" | \"Standard_Character\" | \"bool\"\n            | \"int\" | \"double\" | \"float\" | \"void\" | \"char\" | \"unsigned\") {\n            continue;\n        }\n        if ident.contains('_') || ident.starts_with(\"Standard\") {\n            let header = format!(\"{}.hxx\", ident);\n            if known_headers.contains(&header) {\n                headers.insert(header);\n            } else if let Some(resolver) = resolver {\n                if let Some(defining_header) = resolver.resolve(&ident) {\n                    headers.insert(defining_header.to_string());\n                }\n            }\n        }\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "480:jr\n481:si\n482:di\n483:gq\n484:is\n485:ai\n486:uo\n487:pz\n488:ho\n489:qa\n490:jr\n491:ux\n492:pf\n493:ly\n494:ga\n495:qh\n496:gb\n497:zr\n498:cy\n499:xx\n500:ti\n501:ih\n502:gb\n503:cy\n504:te\n505:hi\n506:ep\n507:zr\n508:cy\n509:il\n510:ez\n511:kc\n512:zr\n513:cy\n514:on\n515:uj\n516:ik\n517:tw\n518:xz\n519:mt\n520:sy\n521:mi\n522:gl\n523:uo\n524:cy\n525:ax\n526:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":480,"endLine":566}
```
</details>

Good. Now let me also remove the old `batch_find_defining_headers` and `find_defining_header` functions since they're no longer needed:

<details>
<summary>Reading lines 524-660 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L524-L661">cpp.rs</a></summary>

```
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
    // Sort headers for deterministic iteration or
... (truncated, 6019 chars)
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L524">cpp.rs -1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "524:tj\n525:va\n526:as\n527:ai\n528:uo\n529:ho\n530:pd\n531:tu\n532:ah\n533:ax\n534:qk\n535:fr\n536:da\n537:bk\n538:pa\n539:lb\n540:ax\n541:dm\n542:uu\n543:cb\n544:ca\n545:gt\n546:gv\n547:zx\n548:cy\n549:xh\n550:wr\n551:zu\n552:aa\n553:ih\n554:mt\n555:ho\n556:zr\n557:cy\n558:gu\n559:sj\n560:pb\n561:iw\n562:li\n563:mb\n564:to\n565:fz\n566:ys\n567:un\n568:dp\n569:qf\n570:mf\n571:or\n572:pa\n573:gl\n574:uo\n575:cy\n576:ax\n577:vv\n578:xh\n579:ke\n580:ea\n581:xp\n582:uo\n583:mf\n584:kr\n585:dc\n586:mo\n587:bd\n588:uf\n589:sq\n590:fs\n591:kt\n592:iz\n593:fj\n594:ew\n595:qf\n596:mf\n597:pj\n598:or\n599:gl\n600:uo\n601:cy\n602:ax\n603:io\n604:dd\n605:pn\n606:zo\n607:wp\n608:jm\n609:ma\n610:ix\n611:ai\n612:an\n613:pd\n614:je\n615:dr\n616:dm\n617:uu\n618:cb\n619:dd\n620:lz\n621:ke\n622:ub\n623:fq\n624:uo\n625:kp\n626:fq\n627:uo\n628:mf\n629:kr\n630:dc\n631:mo\n632:bd\n633:uf\n634:hs\n635:em\n636:ys\n637:gj\n638:rk\n639:vc\n640:ew\n641:un\n642:hh\n643:ew\n644:qf\n645:mf\n646:pj\n647:or\n648:gl\n649:uo\n650:cy\n651:ax\n652:mm\n653:io\n654:dd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":524,"endLine":653}
```
</details>

Now update `generate_preamble` to accept `HeaderResolver` instead of `include_dir`:

<details>
<summary>Reading lines 630-690 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L630-L691">cpp.rs</a></summary>

```

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
```
</details>

Now let me update `generate_preamble` and `generate_wrappers_for_group`:

<details>
<summary>Reading lines 815-830 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L815-L831">cpp.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 830-870 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L830-L871">cpp.rs</a></summary>

```
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

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L851-L852">cpp.rs -1+1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L819-L824">cpp.rs -1+5</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "pub fn generate_preamble(\n    template_instantiations: &std::collections::HashMap<String, crate::config::TemplateInstantiation>,\n    known_headers: &HashSet<String>,\n    resolver: Option<&HeaderResolver>,\n) -> String {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "819:uc\n820:rn\n821:ai\n822:pz\n823:vp"
    },
    {
      "content": "            collect_template_arg_headers(spelling, known_headers, &mut tmpl_headers, resolver);",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "851:em"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 819,
      "endLine": 855
    }
  ]
}
```
</details>

Now update `generate_wrappers_for_group` to accept `HeaderResolver` instead of `include_dir`:

<details>
<summary>Reading lines 870-900 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L870-L901">cpp.rs</a></summary>

```

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
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L874-L889">cpp.rs -1+15</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "pub fn generate_wrappers_for_group(\n    group_name: &str,\n    classes: &[&ParsedClass],\n    collections: &[&super::collections::CollectionInfo],\n    known_headers: &HashSet<String>,\n    bindings: &[&super::bindings::ClassBindings],\n    function_bindings: &[&super::bindings::FunctionBinding],\n    nested_types: &[&super::rust::NestedTypeInfo],\n    handle_able_classes: &HashSet<String>,\n    template_instantiations: &std::collections::HashMap<String, crate::config::TemplateInstantiation>,\n    preamble_filename: &str,\n    own_class_names: &HashSet<String>,\n    class_header_map: &std::collections::HashMap<String, String>,\n    resolver: Option<&HeaderResolver>,\n) -> String {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "874:og\n875:dr\n876:ih\n877:is\n878:ai\n879:gi\n880:gf\n881:oh\n882:kf\n883:rn\n884:wk\n885:gh\n886:yr\n887:zl\n888:vp"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":874,"endLine":902}
```
</details>

Now update the internal call to `collect_headers_from_bindings` in `generate_wrappers_for_group`:

<details>
<summary>Reading lines 925-945 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L925-L946">cpp.rs</a></summary>

```
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L933-L934">cpp.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        collect_headers_from_bindings(bindings, known_headers, class_header_map, resolver),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "933:mn"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":933,"endLine":933}
```
</details>

Now update the template headers dedup section:

<details>
<summary>Reading lines 954-970 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L954-L971">cpp.rs</a></summary>

```
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
```
</details>

Good, line 961 calls `collect_template_arg_headers` with `None` for the resolver. That won't do file I/O, so it's fine.

Now let me also fix the `extend_unique_headers` function to be O(n) instead of O(n²):

<details>
<summary>Reading lines 682-690 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L682-L691">cpp.rs</a></summary>

```
fn extend_unique_headers(headers: &mut Vec<String>, additional_headers: impl IntoIterator<Item = String>) {
    for header in additional_headers {
        if !headers.contains(&header) {
            headers.push(header);
        }
    }
}

pub fn generate_wrappers(
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L682-L690">cpp.rs -1+8</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn extend_unique_headers(headers: &mut Vec<String>, additional_headers: impl IntoIterator<Item = String>) {\n    let existing: HashSet<&str> = headers.iter().map(|s| s.as_str()).collect();\n    for header in additional_headers {\n        if !existing.contains(header.as_str()) {\n            headers.push(header);\n        }\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "682:mx\n683:vr\n684:bi\n685:os\n686:cy\n687:ax\n688:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":682,"endLine":695}
```
</details>

Now update main.rs to build the `HeaderResolver` once and pass it to all generators. Let me first remove timing instrumentation and then add the resolver logic:

<details>
<summary>Reading lines 960-1004 of <a href="../crates/opencascade-binding-generator/src/main.rs#L960-L1005">main.rs</a></summary>

```
    use model::ParsedClass;

    let t_start = std::time::Instant::now();
    println!("\n=== Generating FFI output ===");

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
    let mut extra_typedef_names = parser::get_collected_typedef_names();
    // Add template instantiation alias names so they get destructor generation
    for (spelling, _) in template_instantiations {
        extra_typedef_names.insert(config::template_alias_name(spelling));
    }
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);
    eprintln!("[TIMING] compute_all_class_bindings: {:?}", t_start.elapsed());

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );
    eprintln!("[TIMING] compute_all_function_bindings: {:?}", t_start.elapsed());
    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
```
</details>

Remove timing and add resolver building:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L988">main.rs -1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/main.rs#L962-L963">main.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1002-L1003">-1+1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    println!(\"\\n=== Generating FFI output ===\\n\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "962:no\n963:vd"
    },
    {
      "content": "",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "988:iu"
    },
    {
      "content": "    );",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1002:pd\n1003:wf"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 962,
      "endLine": 1002
    }
  ]
}
```
</details>

Now let me find and update more references. Let me check where `generate_preamble` and `generate_wrappers_for_group` are called:

<details>
<summary>Reading lines 1030-1060 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1030-L1061">main.rs</a></summary>

```
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());
    eprintln!("[TIMING] generate_ffi_split: {:?}", t_start.elapsed());
    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
        let filename = format!("ffi_extern_{}.rs", toolkit_name);
        let path = args.output.join(&filename);
        std::fs::write(&path, code)?;
        generated_rs_files.push(path.clone());
        println!("  Wrote: {}", path.display());
    }
    println!("  {} toolkit extern modules", toolkit_files.len());

    // 2. Generate wrappers (split by toolkit or monolithic)
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        // Generate preamble header
        let include_dir = args.include_dirs.first().map(|p| p.as_path());
        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, include_dir);
        let preamble_path = args.output.join("occt_preamble.hxx");
        std::fs::write(&preamble_path, &preamble)?;
        println!("  Wrote: {}", preamble_path.display());

        // Helper to extract C++ module prefix from a name
        fn cpp_module(name: &str) -> &str {
            let base = name.split("::").next().unwrap_or(name);
            base.split('_').next().unwrap_or(base)
        }
```
</details>

<details>
<summary>Reading lines 1138-1200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1138-L1201">main.rs</a></summary>

```

        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)
        let class_header_map: HashMap<String, String> = all_classes.iter()
            .map(|c| (c.name.clone(), c.source_header.clone()))
            .collect();

    eprintln!("[TIMING] pre-wrapper setup: {:?}", t_start.elapsed());
        for toolkit in &sorted_toolkits {
            let empty_bindings = Vec::new();
            let empty_classes = Vec::new();
            let empty_functions = Vec::new();
            let empty_collections = Vec::new();
            let empty_nested = Vec::new();
            let empty_templates = HashMap::new();
            let empty_own = HashSet::new();

            let bindings = tk_bindings.get(toolkit).unwrap_or(&empty_bindings);
            let classes_refs = tk_classes.get(toolkit).unwrap_or(&empty_classes);
            let functions = tk_functions.get(toolkit).unwrap_or(&empty_functions);
            let collections = tk_collections.get(toolkit).unwrap_or(&empty_collections);
            let nested = tk_nested.get(toolkit).unwrap_or(&empty_nested);
            let _templates = tk_templates.get(toolkit).unwrap_or(&empty_templates);
            let own_classes = tk_own_classes.get(toolkit).unwrap_or(&empty_own);

            // Flatten &&ParsedClass to &ParsedClass
            let classes: Vec<&ParsedClass> = classes_refs.iter().copied().copied().collect();

            let cpp_code = codegen::cpp::generate_wrappers_for_group(
                toolkit,
                &classes,
                collections,
                known_headers,
                bindings,
                functions,
                nested,
                &handle_able_classes,
                template_instantiations,
                "occt_preamble.hxx",
                own_classes,
                &class_header_map,
                include_dir,
            );
            let cpp_path = args.output.join(format!("wrappers_{}.cpp", toolkit));
            std::fs::write(&cpp_path, &cpp_code)?;
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
            eprintln!("[TIMING] toolkit {}: {:?}", toolkit, t_start.elapsed());
        }
    } else {
        // Monolithic wrappers.cpp (original behavior)
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
            template_instantiations,
        );
        let cpp_path = args.output.join("wrappers.cpp");
```
</details>

Now I need to:
1. Remove remaining timing lines
2. Build the HeaderResolver before the toolkit loop
3. Pass it to `generate_preamble` and `generate_wrappers_for_group`

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1033">main.rs -1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/main.rs#L1049-L1069">main.rs -1+20</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1158-L1160">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1197-L1200">-1+3</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1206-L1207">-1+1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1033:hx"
    },
    {
      "content": "        // Build HeaderResolver once for all toolkits (avoids redundant file I/O)\n        let include_dir = args.include_dirs.first().map(|p| p.as_path());\n        let class_header_map: HashMap<String, String> = all_classes.iter()\n            .map(|c| (c.name.clone(), c.source_header.clone()))\n            .collect();\n        let resolver = include_dir.map(|dir| {\n            codegen::cpp::HeaderResolver::build(\n                &all_bindings,\n                template_instantiations,\n                known_headers,\n                &class_header_map,\n                dir,\n            )\n        });\n\n        // Generate preamble header\n        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, resolver.as_ref());\n        let preamble_path = args.output.join(\"occt_preamble.hxx\");\n        std::fs::write(&preamble_path, &preamble)?;\n        println!(\"  Wrote: {}\", preamble_path.display());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1049:pt\n1050:ll\n1051:ij\n1052:nk\n1053:hg\n1054:td"
    },
    {
      "content": "        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)\n        // (class_header_map and resolver were built above)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1139:cv\n1140:bv\n1141:ee\n1142:ny\n1143:dd\n1144:vw"
    },
    {
      "content": "                &class_header_map,\n                resolver.as_ref(),\n            );",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1177:ul\n1178:bc\n1179:ns"
    },
    {
      "content": "        }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1184:hu\n1185:cy"
    }
  ]
}
```
**Output:**
```
{
  "applied": 5,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 1033,
      "endLine": 1206
    }
  ]
}
```
</details>

Now let me try to build and see if there are any compilation errors:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (14 lines) — exit code 101</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error[E0502]: cannot borrow `*headers` as mutable because it is also borrowed as immutable
   --> crates/opencascade-binding-generator/src/codegen/cpp.rs:686:13
    |
683 | ... existing: HashSet<&str> = headers.iter().map(|s| s.as_str()).collec...
    |                               ------- immutable borrow occurs here
684 | ... header in additional_headers {
685 | ... if !existing.contains(header.as_str()) {
    |         -------- immutable borrow later used here
686 | ...     headers.push(header);
    |         ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
```
</details>

Need to fix the borrow issue in `extend_unique_headers`:

<details>
<summary>Reading lines 682-696 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L682-L697">cpp.rs</a></summary>

```
fn extend_unique_headers(headers: &mut Vec<String>, additional_headers: impl IntoIterator<Item = String>) {
    let existing: HashSet<&str> = headers.iter().map(|s| s.as_str()).collect();
    for header in additional_headers {
        if !existing.contains(header.as_str()) {
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
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L682-L690">cpp.rs -1+8</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn extend_unique_headers(headers: &mut Vec<String>, additional_headers: impl IntoIterator<Item = String>) {\n    let existing: HashSet<String> = headers.iter().cloned().collect();\n    for header in additional_headers {\n        if !existing.contains(&header) {\n            headers.push(header);\n        }\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "682:mx\n683:ih\n684:vr\n685:db\n686:os\n687:cy\n688:ax\n689:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":682,"endLine":696}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 6.63s
```

Compiles. Now let me test it:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | tail -15
```

> 💭 158 seconds, down from 228. But that's still a lot. I wonder if there are other big time sinks. The profiling earlier showed:
> - compute_all_class_bindings: 12.5s
> - compute_all_function_bindings: 12.5s (negligible extra) 
> - generate_ffi_split: 12.7s
> - pre-wrapper setup: 13.8s (including toolkit grouping)
> - wrapper generation: 13.8 → 182.8s = 169s
> 
> So with the fix, wrapper generation should now be much faster since the file I/O was ~100+ seconds of the 169s toolkit loop. 
> 
> Let me reconcile the numbers—we went from 228s down to 158s, which is only a 70-second improvement, but the batch header lookups were accounting for around 105 seconds of overhead, so something doesn't quite add up there.
> 
> Now I need to verify the output is actually correct and check if there are other optimization opportunities I'm missing.

Down from 228s to 158s — a 70s improvement. But let me check the output is correct first, then look for more optimization opportunities:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/ 2>&1 | tail -5
```
```
 crates/opencascade-sys/generated/occt_preamble.hxx | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
```

Only 1 file changed — let me check what changed:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/occt_preamble.hxx
```
<details>
<summary>Output (22 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/occt_preamble.hxx b/crates/opencascade-sys/generated/occt_preamble.hxx
index 7d60509..8ad24b2 100644
--- a/crates/opencascade-sys/generated/occt_preamble.hxx
+++ b/crates/opencascade-sys/generated/occt_preamble.hxx
@@ -64,7 +64,7 @@ static const char* occt_make_exception(const char* type_name, const char* messag
 #include <AIS_InteractiveObject.hxx>
 #include <Adaptor3d_Surface.hxx>
 #include <Aspect_FrustumLRBT.hxx>
-#include <BRepExtrema_ProximityValueTool.hxx>
+#include <BRepExtrema_ProximityDistTool.hxx>
 #include <BRepMesh_Vertex.hxx>
 #include <BVH_Box.hxx>
 #include <BVH_Builder.hxx>
@@ -76,7 +76,7 @@ static const char* occt_make_exception(const char* type_name, const char* messag
 #include <Graphic3d_Layer.hxx>
 #include <Graphic3d_ShaderVariable.hxx>
 #include <Graphic3d_Structure.hxx>
-#include <Graphic3d_Vec3.hxx>
+#include <Graphic3d_TransformUtils.hxx>
 #include <HLRAlgo_PolyData.hxx>
 #include <HLRAlgo_PolyShellData.hxx>
 #include <Image_PixMap.hxx>
```
</details>

> 💭 The preamble has slight differences in which headers were resolved for a few types. The old code used `find_defining_header` which had a preference for non-indented (file-scope) definitions, while the new batch resolver doesn't have that preference. Let me check if this matters - these are both correct OCCT headers that define the needed types, just different headers that happen to contain the typedef.
> 
> Actually, this could cause C++ compilation failures if the wrong header is included. Let me check if the original `find_defining_header` logic had a preference for file-scope definitions that my new code is missing.
> 
> Looking at the original `find_defining_header`:
> 
> 
> The logic checks if a line is non-indented and returns that header immediately, otherwise it falls back to a class-scoped match. So it definitely prioritizes file-scope definitions. But now I'm realizing `batch_find_defining_headers` - the old batch version - didn't have this preference either, it just took the first match. So the preference was only in `find_defining_header`, not in the batch version I'm using.
> 
> The real difference is that the old `collect_template_arg_headers` called the individual `find_defining_header` per type, which had the file-scope preference, while my new batch resolver doesn't have that same logic. I should verify whether this actually causes problems by testing the full build.
> 
> Looking at the preamble changes, I'm seeing different headers being selected for the same types - like `BRepExtrema_ProximityValueTool.hxx` swapping for `BRepExtrema_ProximityDistTool.hxx` - but both should legitimately define those types, so C++ compilation should still work. Still, to match the original behavior and avoid potential issues, I should add the file-scope preference logic to the batch resolver.
> 
> Actually, for an optimization task, let me just verify the build works first rather than over-engineering this. Now I'm looking at where the remaining time is going - we're still at 158 seconds when the target is around 70. The parsing phase, binding computation, and module re-exports are probably consuming most of that, but I need to understand what the baseline 70 seconds actually included to know where the real bottlenecks are.

The header resolution differences are minor (different but equivalent headers). Let me verify the build still works and also profile more to find additional bottlenecks since we're at 158s vs the 70s target:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1 | tail -5
```
<details>
<summary>Output (6 lines)</summary>

```


  error occurred in cc-rs: command did not execute successfully (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/74271c7d28a70e01-wrappers_TKBO.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers_TKBO.cpp"


real  2.2s
```
</details>

There's a C++ compilation error with the `BRepExtrema_ProximityDistTool.hxx` header. Let me check:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build -p opencascade-sys 2>&1 | grep "error:" | head -10
```
```
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/occt_preamble.hxx:176:28: error: unknown type name 'ProxPnt_Status'; did you mean 'BRepExtrema_ProximityDistTool::ProxPnt_Status'?
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)`
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/occt_preamble.hxx:176:28: error: unknown type name 'ProxPnt_Status'; did you mean 'BRepExtrema_ProximityDistTool::ProxPnt_Status'?
```

> 💭 The header resolution is wrong for some types. The `ProxPnt_Status` type is a nested enum inside `BRepExtrema_ProximityDistTool`, so the template instantiation for `BVH_PrimitiveSet3d` (or whatever uses `ProxPnt_Status`) actually needs `BRepExtrema_ProximityValueTool.hxx` which re-exports or defines it at file scope with a typedef.
> 
> The issue is that my batch resolver doesn't have the file-scope preference that the old `find_defining_header` had. Let me fix the resolver to prefer file-scope (non-indented) definitions.
> 
> 
> Actually, I see the real problem now—the old code used `find_defining_header` for template arguments with file-scope preference, but `batch_find_defining_headers` for batch bindings without it. My new HeaderResolver merged both into one pass, which means template argument resolution lost that file-scope preference. I need to restore that preference in the HeaderResolver.

The resolver lost the file-scope preference that `find_defining_header` had. I need to make the resolver prefer non-indented (file-scope) typedef/enum definitions. Let me fix this:

<details>
<summary>Reading lines 45-130 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L45-L131">cpp.rs</a></summary>

```
        // 2. Collect unresolved types from template instantiation spellings
        for spelling in template_instantiations.keys() {
            collect_unresolved_from_template_spelling(spelling, known_headers, &mut unresolved);
        }

        // 3. Batch-resolve all unresolved types by scanning headers once
        let mut cache = HashMap::new();
        if !unresolved.is_empty() {
            let mut remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();

            // Group by module prefix for prioritized search
            let mut prefix_types: HashMap<&str, Vec<&str>> = HashMap::new();
            for name in &remaining {
                let prefix = name.split('_').next().unwrap_or("");
                prefix_types.entry(prefix).or_default().push(name);
            }

            let mut sorted_known: Vec<&String> = known_headers.iter().collect();
            sorted_known.sort();

            // Pass 1: Search headers matching module prefix
            for header_name in &sorted_known {
                if remaining.is_empty() {
                    break;
                }
                let header_prefix = header_name.split('_').next().unwrap_or("");
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
                                        cache.insert(type_name.to_string(), (*header_name).clone());
                                        remaining.remove(type_name);
                                    }
                                }
                            }
                            prev_had_typedef = line.contains("typedef") && !line.contains(';');
                        }
                    }
                }
            }

            // Pass 2: Search ALL headers for still-unresolved types
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
                                            cache.insert(type_name.to_string(), (*header_name).clone());
                                            remaining.remove(type_name);
                                        }
                                    }
                                }
                                prev_had_typedef = line.contains("typedef") && !line.contains(';');
                            }
                        }
                    }
                }
    
... (truncated, 4165 chars)
```
</details>

I need to implement the same logic as `find_defining_header` - prefer file-scope (non-indented) definitions. Let me restructure the resolver to track class-scope matches separately and only use them as fallbacks:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L50-L141">cpp.rs -1+91</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        // 3. Batch-resolve all unresolved types by scanning headers once\n        let mut cache = HashMap::new();\n        if !unresolved.is_empty() {\n            let mut remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();\n            // Track class-scope matches as fallbacks (indented typedef/enum)\n            let mut class_scope: HashMap<String, String> = HashMap::new();\n\n            // Group by module prefix for prioritized search\n            let mut prefix_types: HashMap<&str, Vec<&str>> = HashMap::new();\n            for name in &remaining {\n                let prefix = name.split('_').next().unwrap_or(\"\");\n                prefix_types.entry(prefix).or_default().push(name);\n            }\n\n            let mut sorted_known: Vec<&String> = known_headers.iter().collect();\n            sorted_known.sort();\n\n            // Pass 1: Search headers matching module prefix\n            for header_name in &sorted_known {\n                if remaining.is_empty() {\n                    break;\n                }\n                let header_prefix = header_name.split('_').next().unwrap_or(\"\");\n                let relevant_types: Vec<&str> = prefix_types\n                    .get(header_prefix)\n                    .map(|v| v.iter().copied().filter(|t| remaining.contains(t)).collect())\n                    .unwrap_or_default();\n                if relevant_types.is_empty() {\n                    continue;\n                }\n                let path = include_dir.join(header_name.as_str());\n                if let Ok(file) = std::fs::File::open(&path) {\n                    let reader = std::io::BufReader::new(file);\n                    let mut prev_had_typedef = false;\n                    for line in reader.lines() {\n                        if let Ok(line) = line {\n                            for &type_name in &relevant_types {\n                                if remaining.contains(type_name) && line.contains(type_name) {\n                                    if line.contains(\"typedef\") || line.contains(\"enum \") || prev_had_typedef {\n                                        // Prefer file-scope (non-indented) definitions\n                                        if !line.starts_with(' ') && !line.starts_with('\\t') {\n                                            cache.insert(type_name.to_string(), (*header_name).clone());\n                                            remaining.remove(type_name);\n                                        } else if !class_scope.contains_key(type_name) {\n                                            class_scope.insert(type_name.to_string(), (*header_name).clone());\n                                        }\n                                    }\n                                }\n                            }\n                            prev_had_typedef = line.contains(\"typedef\") && !line.contains(';');\n                        }\n         
... (truncated, 5727 chars)
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":50,"endLine":214}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | tail -10
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/ 2>&1
```
<details>
<summary>Output (14 lines)</summary>

```
 crates/opencascade-sys/generated/wrappers_TKBool.cpp     | 1 -
 crates/opencascade-sys/generated/wrappers_TKCDF.cpp      | 1 -
 crates/opencascade-sys/generated/wrappers_TKG3d.cpp      | 1 -
 crates/opencascade-sys/generated/wrappers_TKGeomAlgo.cpp | 1 -
 crates/opencascade-sys/generated/wrappers_TKGeomBase.cpp | 1 -
 crates/opencascade-sys/generated/wrappers_TKHLR.cpp      | 1 -
 crates/opencascade-sys/generated/wrappers_TKLCAF.cpp     | 1 -
 crates/opencascade-sys/generated/wrappers_TKMath.cpp     | 1 -
 crates/opencascade-sys/generated/wrappers_TKMeshVS.cpp   | 3 +--
 crates/opencascade-sys/generated/wrappers_TKPrim.cpp     | 1 -
 crates/opencascade-sys/generated/wrappers_TKService.cpp  | 2 --
 crates/opencascade-sys/generated/wrappers_TKTopAlgo.cpp  | 1 -
 crates/opencascade-sys/generated/wrappers_TKV3d.cpp      | 3 +--
 13 files changed, 2 insertions(+), 16 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/ 2>&1 | head -80
```
<details>
<summary>Output (80 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/wrappers_TKBool.cpp b/crates/opencascade-sys/generated/wrappers_TKBool.cpp
index c48218f..09e591b 100644
--- a/crates/opencascade-sys/generated/wrappers_TKBool.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKBool.cpp
@@ -95,7 +95,6 @@
 #include <Law_Function.hxx>
 #include <Law_Laws.hxx>
 #include <Standard_Handle.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_Type.hxx>
 #include <Standard_TypeDef.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKCDF.cpp b/crates/opencascade-sys/generated/wrappers_TKCDF.cpp
index 3247149..07604ba 100644
--- a/crates/opencascade-sys/generated/wrappers_TKCDF.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKCDF.cpp
@@ -80,7 +80,6 @@
 #include <Standard_GUID.hxx>
 #include <Standard_Handle.hxx>
 #include <Standard_IStream.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_Persistent.hxx>
 #include <Standard_SStream.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKG3d.cpp b/crates/opencascade-sys/generated/wrappers_TKG3d.cpp
index b5b96a0..314f546 100644
--- a/crates/opencascade-sys/generated/wrappers_TKG3d.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKG3d.cpp
@@ -117,7 +117,6 @@
 #include <Standard_DomainError.hxx>
 #include <Standard_Failure.hxx>
 #include <Standard_Handle.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_SStream.hxx>
 #include <Standard_Type.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKGeomAlgo.cpp b/crates/opencascade-sys/generated/wrappers_TKGeomAlgo.cpp
index 8784c38..0aa9404 100644
--- a/crates/opencascade-sys/generated/wrappers_TKGeomAlgo.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKGeomAlgo.cpp
@@ -467,7 +467,6 @@
 #include <Standard_DomainError.hxx>
 #include <Standard_Failure.hxx>
 #include <Standard_Handle.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_SStream.hxx>
 #include <Standard_Type.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKGeomBase.cpp b/crates/opencascade-sys/generated/wrappers_TKGeomBase.cpp
index 862b674..1b1f519 100644
--- a/crates/opencascade-sys/generated/wrappers_TKGeomBase.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKGeomBase.cpp
@@ -358,7 +358,6 @@
 #include <ShapeUpgrade_UnifySameDomain.hxx>
 #include <Standard_Handle.hxx>
 #include <Standard_IStream.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_Type.hxx>
 #include <Standard_TypeDef.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKHLR.cpp b/crates/opencascade-sys/generated/wrappers_TKHLR.cpp
index 9c3518b..4c9911e 100644
--- a/crates/opencascade-sys/generated/wrappers_TKHLR.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKHLR.cpp
@@ -171,7 +171,6 @@
 #include <Intrv_Position.hxx>
 #inclu
... (truncated, 3605 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/ 2>&1 | tail -40
```
<details>
<summary>Output (40 lines)</summary>

```
 #include <Standard_IStream.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_Mutex.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_OutOfRange.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKTopAlgo.cpp b/crates/opencascade-sys/generated/wrappers_TKTopAlgo.cpp
index bfda8f0..2397e26 100644
--- a/crates/opencascade-sys/generated/wrappers_TKTopAlgo.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKTopAlgo.cpp
@@ -252,7 +252,6 @@
 #include <Standard_DomainError.hxx>
 #include <Standard_Failure.hxx>
 #include <Standard_Handle.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_SStream.hxx>
 #include <Standard_Type.hxx>
diff --git a/crates/opencascade-sys/generated/wrappers_TKV3d.cpp b/crates/opencascade-sys/generated/wrappers_TKV3d.cpp
index 4f600e8..700583a 100644
--- a/crates/opencascade-sys/generated/wrappers_TKV3d.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKV3d.cpp
@@ -183,9 +183,9 @@
 #include <Graphic3d_Texture2D.hxx>
 #include <Graphic3d_TextureEnv.hxx>
 #include <Graphic3d_TransformPers.hxx>
-#include <Graphic3d_TransformUtils.hxx>
 #include <Graphic3d_TypeOfBackfacingModel.hxx>
 #include <Graphic3d_TypeOfShadingModel.hxx>
+#include <Graphic3d_Vec2.hxx>
 #include <Graphic3d_Vec3.hxx>
 #include <Graphic3d_Vec4.hxx>
 #include <Graphic3d_Vertex.hxx>
@@ -358,7 +358,6 @@
 #include <Standard_DomainError.hxx>
 #include <Standard_Failure.hxx>
 #include <Standard_Handle.hxx>
-#include <Standard_MMgrOpt.hxx>
 #include <Standard_Mutex.hxx>
 #include <Standard_OStream.hxx>
 #include <Standard_OutOfRange.hxx>
```
</details>

> 💭 The differences fall into two categories:
> 
> 1. `Standard_MMgrOpt.hxx` is no longer being included in several files - this was likely being found by the old code's Pass 2 (search ALL headers) but the new code with file-scope preference skipped it. This seems like the type `Standard_MMgrOpt` was found in `Standard_MMgrOpt.hxx` by the old code's `batch_find_defining_headers` which didn't have the file-scope preference, but now with the file-scope preference it's resolved differently. Wait - actually, the old code for `collect_headers_from_bindings` used `batch_find_defining_headers` (no file-scope preference) while the new code uses the resolver which HAS file-scope preference. So the resolver is now resolving some types to different headers.
>    
> The diff shows fewer headers being included, not different ones - `Standard_MMgrOpt.hxx` is being removed entirely. This suggests that some type previously resolved to that header by the old `batch_find_defining_headers` is either no longer being resolved or isn't even being collected as unresolved. The core issue is that the old `collect_headers_from_bindings` used `batch_find_defining_headers` without file-scope preference, whereas my new resolver applies file-scope preference, which is changing how certain types get resolved.
> 
> The old algorithm would immediately use a class-scope match it found, but the new resolver defers class-scope matches and only uses them if no file-scope match exists. So when a type has both a file-scope definition in one header and a class-scope typedef in `Standard_MMgrOpt.hxx`, the old code would pick the class-scope match first (depending on iteration order), while the new code correctly prefers the file-scope definition instead - which is actually the right behavior.
> 
> I'm seeing similar resolution changes in other headers like `Graphic3d_TransformUtils.hxx` switching to `Graphic3d_Vec2.hxx`, but these should be functionally harmless since the removed headers are just redundant includes that don't affect what types are available. Let me verify the build still works.

The changes are mostly removing unnecessary `Standard_MMgrOpt.hxx` includes, which is actually slightly better. Let me verify the build works:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1 | tail -5
```

Build succeeds. Now let me look for more optimization opportunities. The regeneration is at 160s. Let me add timing to understand where the remaining time goes:

<details>
<summary>Reading lines 770-800 of <a href="../crates/opencascade-binding-generator/src/main.rs#L770-L801">main.rs</a></summary>

```

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
                // Check exclude_methods for this static method
                if exclude_methods.contains(&(class.name.clone(), sm.name.clone())) {
                    if verbose {
                        println!("    Skipping excluded method {}::{}", class.name, sm.name);
                    }
                    continue;
                }

                let mut return_type = sm.return_type.clone();

                // If return type is ConstRef and there are no ref params,
                // strip the ConstRef wrapper (return by-value copy). The FFI can't
                // express references from free functions with no borrowable
                // params, so we copy instead.
```
</details>

Let me find the main flow — where parsing time is:

Searched for regex `fn main\(\)|fn run\(\)` (`**/crates/opencascade-binding-generator/src/main.rs`), 1 result
- [main.rs:109](../crates/opencascade-binding-generator/src/main.rs#L109)

<details>
<summary>Reading lines 109-200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L109-L201">main.rs</a></summary>

```
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
    let mut split_config = config::SplitConfig::default();
    let mut config_dir: Option<PathBuf> = None;
    let inputs = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;
        split_config = cfg.split.clone();
        config_dir = Some(config_path.parent().unwrap_or(Path::new(".")).to_path_buf());

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

        let method_exclusions = parse_class_method_pairs(&cfg.exclude_methods, "exclude_methods");
        let ambiguous_method_exclusions =
            parse_class_method_pairs(&cfg.ambiguous_methods, "ambiguous_methods");

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let non_alloc_cls: HashSet<String> = cfg.non_allocatable_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        let tmpl_inst = cfg.template_instantiations;
        let occt_alias_overrides = cfg.occt_alias_type_overrides;
        let void_ptr_types: HashSet<String> = cfg.void_pointer_types.into_iter().collect();
        SelectionInputs {
            explicit_headers: headers,
            exclude_set: excludes,
            exclude_modules: exclude_mods,
            exclude_methods: method_exclusions,
            ambiguous_methods: ambiguous_method_exclusions,
            non_allocatable_classes: non_alloc_cls,
            manual_type_names: manual_names,
            template_instantiations: tmpl_inst,
            occt_alias_type_overrides: occt_alias_overrides,
            void_pointer_types: void_ptr_types,
        }
    } else if !args.headers.is_empty() {
        SelectionInputs {
            explicit_headers: args.headers.clone(),
            exclude_set: std::collections::HashSet::new(),
            exclude_modules: Vec::new(),
            exclude_methods: HashSet::new(),
            ambiguous_methods: HashSet::new(),
            non_allocatable_classes: HashSet::new(),
            manual_type_names: HashSet::new(),
            template_instantiations: HashMap::new(),
            occt_alias_type_overrides: HashMap::new(),
            void_pointer_types: HashSet::new(),
        }
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    let SelectionInputs {
        explicit_headers,
        exclude_set,
        exclude_modules,
        exclude_meth
... (truncated, 4187 chars)
```
</details>

<details>
<summary>Reading lines 200-280 of <a href="../crates/opencascade-binding-generator/src/main.rs#L200-L281">main.rs</a></summary>

```
        void_pointer_types,
    } = inputs;

    // Resolve header dependencies when include directories are available.
    let headers_to_process = if !args.include_dirs.is_empty() {
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
    crate::model::set_void_pointer_type_names(void_pointer_types);
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
    // Type::Handle("BVH_Builder_double_3") so downstream code sees clean alias names.
    let template_alias_map: HashMap<String, String> = template_instantiations
        .iter()
        .map(|(spelling, _)| (spelling.clone(), config::template_alias_name(spelling)))
        .collect();
    if !template_alias_map.is_empty() {
        let rewritten = rewrite_template_types(&mut parsed, &template_alias_map);
        if rewritten > 0 {
```
</details>

<details>
<summary>Reading lines 280-400 of <a href="../crates/opencascade-binding-generator/src/main.rs#L280-L401">main.rs</a></summary>

```
        if rewritten > 0 {
            println!("  Rewrote {} template Handle type references to alias names", rewritten);
        }
    }

    // Auto-collect remaining template type spellings from parsed data.
    // These are Type::Class(name) entries where name still contains '<' after the
    // explicit template_instantiations rewrite above. We auto-infer headers and
    // generate typedefs so these template types become bindable.
    let known_headers_for_templates: HashSet<String> = if !args.include_dirs.is_empty() {
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
    let auto_template_types = collect_template_class_types(&parsed);
    let mut template_instantiations = template_instantiations;
    if !auto_template_types.is_empty() {
        let mut auto_count = 0;
        for spelling in &auto_template_types {
            // Skip types already in template_instantiations
            if template_instantiations.contains_key(spelling) {
                continue;
            }
            // Try to infer the header for this template type
            if let Some(header) = infer_template_header(spelling, &known_headers_for_templates) {
                // Determine the OCCT module from the base type name
                let base = spelling.split('<').next().unwrap_or(spelling).trim();
                let module = if base.contains('_') {
                    base.split('_').next().unwrap_or(base).to_string()
                } else if base.contains("::") {
                    base.split("::").next().unwrap_or(base).to_string()
                } else {
                    base.to_string()
                };
                template_instantiations.insert(
                    spelling.clone(),
                    config::TemplateInstantiation {
                        header,
                        module,
                        handle: false,
                    },
                );
                auto_count += 1;
            }
        }
        if auto_count > 0 {
            println!("  Auto-detected {} template type instantiations", auto_count);
            // Build extended alias map and rewrite the newly discovered template types
            let extended_alias_map: HashMap<String, String> = template_instantiations
                .iter()
                .map(|(spelling, _)| (spelling.clone(), config::template_alias_name(spelling)))
                .collect();
            let rewritten = rewrite_template_types(&mut parsed, &extended_alias_map);
            if rewritten > 0 {
                println!("  Rewrote {} template Class type references to alias names", rewritten);
            }
        }
    }

    // Detect "utility namespace classes" — classes with no underscore in the name
    // (class name == module name), only static methods, and no instance methods/constructors.
    // These are OCCT's namespace-like patterns (e.g., `gp` with `gp::OX()`, `gp::Origin()`).
    // Convert their static methods to free functions so they appear as module-level
    // functions (e.g., `gp::ox()`) instead of awkward `gp::gp::ox()`.
    let utility_class_names = convert_utility_classes_to_functions(&mut parsed, &exclude_methods, args.verbose);

    if args.verbose {
        println!("\nParsing complete. Summary:");
        let total_classes: usize = parsed.iter().map(|h| h.classes.len()).sum();
    
... (truncated, 5413 chars)
```
</details>

Let me look at where parsing happens, and the resolver:

<details>
<summary>Reading lines 400-470 of <a href="../crates/opencascade-binding-generator/src/main.rs#L400-L471">main.rs</a></summary>

```
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
    let all_enums_raw: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    // Deduplicate enums with the same name (anonymous enums synthesized from
    // constant prefixes may collide with named enums; merge their variants).
    let all_enums: Vec<_> = {
        let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut merged: Vec<model::ParsedEnum> = Vec::new();
        for e in &all_enums_raw {
            if let Some(&idx) = seen.get(e.name.as_str()) {
                // Merge variants from duplicate into existing
                let existing_variant_names: HashSet<String> = merged[idx].variants.iter().map(|v| v.name.clone()).collect();
                for v in &e.variants {
                    if !existing_variant_names.contains(&v.name) {
                        merged[idx].variants.push(v.clone());
                    }
                }
            } else {
                seen.insert(&e.name, merged.len());
                merged.push((*e).clone());
            }
        }
        merged
    };
    let all_enums: Vec<_> = all_enums.iter().collect();
    let all_functions: Vec<_> = parsed.iter().flat_map(|h| &h.functions).collect();

    // Get collection type names (needed for symbol resolution filtering)
    let all_collections = codegen::collections::all_known_collections();
    let collection_type_names: HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();

    // Compute handle-able classes via transitive closure of inheritance graph.
    // This is done before the symbol table so the resolver can use it instead of
    // relying on the old per-class parser heuristic.
    let mut handle_able_classes = codegen::bindings::compute_handle_able_classes(&all_classes);

    // Add template instantiation alias names to the known type sets.
    // These were already rewritten in the parsed data (Handle inner names),
    // so we just need to register the alias names as known handle-able classes.
    let mut manual_type_names = manual_type_names;
    for (spelling, inst) in &template_instantiations {
```
</details>

<details>
<summary>Reading lines 470-530 of <a href="../crates/opencascade-binding-generator/src/main.rs#L470-L531">main.rs</a></summary>

```
    for (spelling, inst) in &template_instantiations {
        let alias = config::template_alias_name(spelling);
        manual_type_names.insert(alias.clone());
        if inst.handle {
            handle_able_classes.insert(alias);
        }
    }

    // Add utility class names (classes converted to module-level functions) to the
    // known type set.  Other classes may reference nested types qualified with
    // the utility class name (e.g. ShapeProcess::OperationsFlags).  Without
    // this, the parent class check in type_uses_unknown_class() fails because
    // the utility class was removed from the parsed class list.
    for name in &utility_class_names {
        manual_type_names.insert(name.clone());
    }

    // Add guarded namespace-scoped typedef aliases discovered by parser
    // (e.g., IMeshData::MapOfInteger) so resolver treats them as known types
    // during bindability filtering.
    for name in parser::get_collected_namespace_typedef_names() {
        manual_type_names.insert(name);
    }

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

    // Use the enriched handle_able_classes from the resolver, which discovers
    // additional Handle types by scanning method signatures (including static methods)
    let handle_able_classes = symbol_table.handle_able_classes.clone();

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
```
</details>

I see the flow now:
1. Config loading (~instant)
2. Header dep resolution 
3. Parsing (`parse_headers`) — likely 10-15s (uses libclang)
4. Module graph
5. Symbol table building
6. Code generation

Let me add quick timing to understand the broader breakdown, then focus on finding more bottlenecks:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L262-L264">main.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L272-L274">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L429-L431">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let t0 = std::time::Instant::now();\n    println!(\"Parsing {} headers...\", headers_to_process.len());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "262:tu"
    },
    {
      "content": "    eprintln!(\"[TIMING] parsing: {:?}\", t0.elapsed());\n    // Rewrite template Handle types to alias names based on template_instantiations config.",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "271:ke"
    },
    {
      "content": "    eprintln!(\"[TIMING] pre-generate: {:?}\", t0.elapsed());\n    println!(\"\\nGenerating code...\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "427:gi"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 262,
      "endLine": 430
    }
  ]
}
```
</details>

Also add timing to the generate_output function to understand where time goes there. Let me find the right lines:

<details>
<summary>Reading lines 960-970 of <a href="../crates/opencascade-binding-generator/src/main.rs#L960-L971">main.rs</a></summary>

```
    split_config: &config::SplitConfig,
    config_dir: Option<&Path>,
) -> Result<()> {
    use model::ParsedClass;

    println!("\n=== Generating FFI output ===\n");

    // Collect all headers
    let mut all_headers: HashSet<String> = HashSet::new();
    for class in all_classes {
        all_headers.insert(class.source_header.clone());
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L965-L967">main.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let t_gen = std::time::Instant::now();\n    println!(\"\\n=== Generating FFI output ===\\n\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "965:vw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":965,"endLine":966}
```
</details>

Now add timing after key steps in generate_output. Let me find where things happen:

<details>
<summary>Reading lines 990-1010 of <a href="../crates/opencascade-binding-generator/src/main.rs#L990-L1011">main.rs</a></summary>

```
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );
    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
    // and optionally for C++ split)
    let module_to_toolkit = if let Some(ref occt_src) = split_config.occt_source_dir {
        let occt_source_dir = config_dir.as_ref()
            .expect("config_dir required for toolkit split")
            .join(occt_src);
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1005-L1006">main.rs +1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    eprintln!(\"[TIMING] compute_bindings: {:?}\", t_gen.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "insertAfter": true,
      "lineHashes": "1004:pd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":1004,"endLine":1005}
```
</details>

Now let me find where the formatter/module re-exports happen and add timing there too:

Searched for text `rustfmt` (`**/crates/opencascade-binding-generator/src/main.rs`), 4 results
- [main.rs:1541](../crates/opencascade-binding-generator/src/main.rs#L1541)
- [main.rs:1542](../crates/opencascade-binding-generator/src/main.rs#L1542)
- [main.rs:1549](../crates/opencascade-binding-generator/src/main.rs#L1549)
- [main.rs:1550](../crates/opencascade-binding-generator/src/main.rs#L1550)

<details>
<summary>Reading lines 1530-1575 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1530-L1576">main.rs</a></summary>

```
    }

    // 4. Generate lib.rs with module declarations
    let lib_rs = generate_lib_rs(&generated_modules, &extra_only_modules, &split_toolkit_names);
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
fn generate_lib_rs(modules: &[&module_graph::Module], extra_modules: &[(String, String)], split_toolkits: &[String]) -> String {
    let mut output = String::new();
    output.push_str("// Generated OCCT bindings\n");
    output.push_str("// Nested C++ types use Parent_Child naming, which is intentional\n");
    output.push_str("#![allow(non_camel_case_types)]\n\n");

    // FFI type definitions + per-toolkit extern modules
    output.push_str("// FFI type definitions (opaque structs, POD structs, handles)\n");
    output.push_str("pub(crate) mod ffi_types;\n\n");
    output.push_str("// Per-toolkit FFI extern blocks\n");
    for toolkit in split_toolkits {
        output.push_str(&format!("pub(crate) mod ffi_extern_{};\n", toolkit));
    }
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1539-L1541">main.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    eprintln!(\"[TIMING] pre-format: {:?}\", t_gen.elapsed());\n    // Format generated Rust files",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1539:et"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":1539,"endLine":1540}
```
</details>

Let me first check what timing instrumentation is currently in the code, then build and run to profile.

<details>
<summary>Searched for regex <code>TIMING|t_gen|t0\.elapsed|t_start</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 12 results</summary>

- [parser.rs:516](../crates/opencascade-binding-generator/src/parser.rs#L516)
- [parser.rs:545](../crates/opencascade-binding-generator/src/parser.rs#L545)
- [parser.rs:547](../crates/opencascade-binding-generator/src/parser.rs#L547)
- [main.rs:272](../crates/opencascade-binding-generator/src/main.rs#L272)
- [main.rs:429](../crates/opencascade-binding-generator/src/main.rs#L429)
- [main.rs:965](../crates/opencascade-binding-generator/src/main.rs#L965)
- [main.rs:1005](../crates/opencascade-binding-generator/src/main.rs#L1005)
- [main.rs:1539](../crates/opencascade-binding-generator/src/main.rs#L1539)
</details>

Let me read the key timing areas in `crates/opencascade-binding-generator/src/main.rs` to understand the structure better and add more timing points:

<details>
<summary>Reading lines 260-275 of <a href="../crates/opencascade-binding-generator/src/main.rs#L260-L276">main.rs</a></summary>

```
    };

    let t0 = std::time::Instant::now();
    println!("Parsing {} headers...", headers_to_process.len());
    crate::model::set_void_pointer_type_names(void_pointer_types);
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    eprintln!("[TIMING] parsing: {:?}", t0.elapsed());
    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
    // Type::Handle("BVH_Builder_double_3") so downstream code sees clean alias names.
```
</details>

<details>
<summary>Reading lines 425-435 of <a href="../crates/opencascade-binding-generator/src/main.rs#L425-L436">main.rs</a></summary>

```
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Generate code
    eprintln!("[TIMING] pre-generate: {:?}", t0.elapsed());
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums_raw: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    // Deduplicate enums with the same name (anonymous enums synthesized from
```
</details>

<details>
<summary>Reading lines 960-1010 of <a href="../crates/opencascade-binding-generator/src/main.rs#L960-L1011">main.rs</a></summary>

```
    split_config: &config::SplitConfig,
    config_dir: Option<&Path>,
) -> Result<()> {
    use model::ParsedClass;

    let t_gen = std::time::Instant::now();
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
    let mut extra_typedef_names = parser::get_collected_typedef_names();
    // Add template instantiation alias names so they get destructor generation
    for (spelling, _) in template_instantiations {
        extra_typedef_names.insert(config::template_alias_name(spelling));
    }
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, ambiguous_methods, manual_type_names, &handle_able_classes);

    // Mark non-allocatable classes as having protected destructors so both the
    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side
    // skip constructors, destructors, CppDeletable, and ToOwned.
    for b in &mut all_bindings {
        if non_allocatable_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );
    eprintln!("[TIMING] compute_bindings: {:?}", t_gen.elapsed());
    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
    // and optionally for C++ split)
    let module_to_toolkit = if let Some(ref occt_src) = split_config.occt_source_dir {
        let occt_source_dir = config_dir.as_ref()
            .expect("config_dir required for toolkit split")
```
</details>

<details>
<summary>Reading lines 1535-1585 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1535-L1586">main.rs</a></summary>

```
    std::fs::write(&lib_rs_path, &lib_rs)?;
    generated_rs_files.push(lib_rs_path.clone());
    println!("  Wrote: {}", lib_rs_path.display());

    eprintln!("[TIMING] pre-format: {:?}", t_gen.elapsed());
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
fn generate_lib_rs(modules: &[&module_graph::Module], extra_modules: &[(String, String)], split_toolkits: &[String]) -> String {
    let mut output = String::new();
    output.push_str("// Generated OCCT bindings\n");
    output.push_str("// Nested C++ types use Parent_Child naming, which is intentional\n");
    output.push_str("#![allow(non_camel_case_types)]\n\n");

    // FFI type definitions + per-toolkit extern modules
    output.push_str("// FFI type definitions (opaque structs, POD structs, handles)\n");
    output.push_str("pub(crate) mod ffi_types;\n\n");
    output.push_str("// Per-toolkit FFI extern blocks\n");
    for toolkit in split_toolkits {
        output.push_str(&format!("pub(crate) mod ffi_extern_{};\n", toolkit));
    }
    output.push('\n');

    output.push_str("// Per-module re-exports\n");

    // Collect all module rust names and sort for deterministic output
    let mut all_rust_names: Vec<&str> = modules.iter().map(|m| m.rust_name.as_str()).collect();
    for (_, rust_name) in extra_modules {
        all_rust_names.push(rust_name);
    }
```
</details>

Now let me add more timing points to identify where the remaining time is spent. I need to add timing around: the resolver build, the wrapper generation loop for each toolkit, the module re-export generation, and rustfmt.

<details>
<summary>Reading lines 1010-1080 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1010-L1081">main.rs</a></summary>

```
            .expect("config_dir required for toolkit split")
            .join(occt_src);
        config::discover_toolkit_modules(&occt_source_dir)?
    } else {
        HashMap::new()
    };

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();
    // 1. Generate ffi type definitions and per-toolkit extern blocks
    println!("Generating ffi_types.rs + per-toolkit extern modules...");
    let (ffi_code, nested_types, toolkit_files) = codegen::rust::generate_ffi_split(
        all_classes,
        &all_functions,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
        non_allocatable_classes,
        &module_to_toolkit,
    );
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());
    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
        let filename = format!("ffi_extern_{}.rs", toolkit_name);
        let path = args.output.join(&filename);
        std::fs::write(&path, code)?;
        generated_rs_files.push(path.clone());
        println!("  Wrote: {}", path.display());
    }
    println!("  {} toolkit extern modules", toolkit_files.len());

    // 2. Generate wrappers (split by toolkit or monolithic)
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        // Build HeaderResolver once for all toolkits (avoids redundant file I/O)
        let include_dir = args.include_dirs.first().map(|p| p.as_path());
        let class_header_map: HashMap<String, String> = all_classes.iter()
            .map(|c| (c.name.clone(), c.source_header.clone()))
            .collect();
        let resolver = include_dir.map(|dir| {
            codegen::cpp::HeaderResolver::build(
                &all_bindings,
                template_instantiations,
                known_headers,
                &class_header_map,
                dir,
            )
        });

        // Generate preamble header
        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, resolver.as_ref());
        let preamble_path = args.output.join("occt_preamble.hxx");
        std::fs::write(&preamble_path, &preamble)?;
        println!("  Wrote: {}", preamble_path.display());

        // Helper to extract C++ module prefix from a name
        fn cpp_module(name: &str) -> &str {
            let base = name.split("::").next().unwrap_or(name);
            base.split('_').next().unwrap_or(base)
        }

        // Group ClassBindings by toolkit
```
</details>

<details>
<summary>Reading lines 1080-1180 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1080-L1181">main.rs</a></summary>

```
        // Group ClassBindings by toolkit
        let mut tk_bindings: HashMap<String, Vec<&codegen::bindings::ClassBindings>> = HashMap::new();
        for b in &all_bindings {
            let tk = module_to_toolkit.get(b.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_bindings.entry(tk).or_default().push(b);
        }

        // Group ParsedClass by toolkit (for header collection)
        let mut tk_classes: HashMap<String, Vec<&&ParsedClass>> = HashMap::new();
        for c in all_classes {
            let tk = module_to_toolkit.get(c.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_classes.entry(tk).or_default().push(c);
        }

        // Group FunctionBindings by toolkit (use namespace = C++ module)
        let mut tk_functions: HashMap<String, Vec<&codegen::bindings::FunctionBinding>> = HashMap::new();
        for f in &all_function_bindings {
            let tk = module_to_toolkit.get(f.namespace.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_functions.entry(tk).or_default().push(f);
        }

        // Group collections by toolkit (derive C++ module from typedef prefix)
        let mut tk_collections: HashMap<String, Vec<&codegen::collections::CollectionInfo>> = HashMap::new();
        for c in &all_collections {
            let module = cpp_module(&c.typedef_name);
            let tk = module_to_toolkit.get(module).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_collections.entry(tk).or_default().push(c);
        }

        // Group nested types by toolkit (derive C++ module from cpp_name prefix)
        let mut tk_nested: HashMap<String, Vec<&codegen::rust::NestedTypeInfo>> = HashMap::new();
        for nt in &nested_types {
            let module = cpp_module(&nt.cpp_name);
            let tk = module_to_toolkit.get(module).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_nested.entry(tk).or_default().push(nt);
        }

        // Assign template instantiations to toolkits by their module
        let mut tk_templates: HashMap<String, HashMap<String, config::TemplateInstantiation>> = HashMap::new();
        for (spelling, inst) in template_instantiations {
            let tk = module_to_toolkit.get(inst.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_templates.entry(tk).or_default()
                .insert(spelling.clone(), inst.clone());
        }

        // Collect all toolkit names
        let mut all_toolkits: HashSet<String> = HashSet::new();
        all_toolkits.extend(tk_bindings.keys().cloned());
        all_toolkits.extend(tk_classes.keys().cloned());
        all_toolkits.extend(tk_functions.keys().cloned());
        all_toolkits.extend(tk_collections.keys().cloned());
        all_toolkits.extend(tk_nested.keys().cloned());
        all_toolkits.extend(tk_templates.keys().cloned());
        let mut sorted_toolkits: Vec<_> = all_toolkits.into_iter().collect();
        sorted_toolkits.sort();

        // Build own_class_names per toolkit (classes + template aliases)
        let mut tk_own_classes: HashMap<String, HashSet<String>> = HashMap::new();
        for c in all_classes {
            let tk = module_to_toolkit.get(c.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_own_classes.entry(tk).or_default().insert(c.name.clone());
        }
        // Template alias names belong to the toolkit of their module
        for (spelling, inst) in template_instantiations {
            let tk = module_to_toolkit.get(inst.module.as_str()).cloned()
                .unwrap_or_else(|| "misc".to_string());
            tk_own_classes.entry(tk).or_default()
                .insert(config::template_alias_name(spelling));
        }

        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)

... (truncated, 5239 chars)
```
</details>

<details>
<summary>Reading lines 1179-1220 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1179-L1221">main.rs</a></summary>

```
            let cpp_code = codegen::cpp::generate_wrappers_for_group(
                toolkit,
                &classes,
                collections,
                known_headers,
                bindings,
                functions,
                nested,
                &handle_able_classes,
                template_instantiations,
                "occt_preamble.hxx",
                own_classes,
                &class_header_map,
                resolver.as_ref(),
            );
            let cpp_path = args.output.join(format!("wrappers_{}.cpp", toolkit));
            std::fs::write(&cpp_path, &cpp_code)?;
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
        }
    } else {
        // Monolithic wrappers.cpp (original behavior)
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
            template_instantiations,
        );
        let cpp_path = args.output.join("wrappers.cpp");
        std::fs::write(&cpp_path, &cpp_code)?;
        println!("  Wrote: {}", cpp_path.display());
    }

    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

```
</details>

Now I have a clear picture. Let me add more specific timing around: resolver build, the per-toolkit wrapper loop, ffi_split generation, and module re-exports.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1053-L1055">main.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1068-L1070">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1161-L1163">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1201-L1203">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1222-L1224">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1053:vr",
      "content": "        eprintln!(\"[TIMING] pre-resolver: {:?}\", t_gen.elapsed());\n        // Build HeaderResolver once for all toolkits (avoids redundant file I/O)"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1067:dd",
      "content": "        eprintln!(\"[TIMING] post-resolver: {:?}\", t_gen.elapsed());\n"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1159:xs",
      "content": "        eprintln!(\"[TIMING] pre-wrapper-loop: {:?}\", t_gen.elapsed());\n        for toolkit in &sorted_toolkits {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1198:cy",
      "content": "        }\n        eprintln!(\"[TIMING] post-wrapper-loop: {:?}\", t_gen.elapsed());"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1218:qk",
      "content": "    eprintln!(\"[TIMING] pre-reexports: {:?}\", t_gen.elapsed());\n    // 3. Generate per-module re-export files"
    }
  ]
}
```
**Output:**
```
{
  "applied": 5,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 1053,
      "endLine": 1223
    }
  ]
}
```
</details>

Also need timing around the `generate_ffi_split` call:

<details>
<summary>Reading lines 1017-1040 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1017-L1041">main.rs</a></summary>

```
    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();
    // 1. Generate ffi type definitions and per-toolkit extern blocks
    println!("Generating ffi_types.rs + per-toolkit extern modules...");
    let (ffi_code, nested_types, toolkit_files) = codegen::rust::generate_ffi_split(
        all_classes,
        &all_functions,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
        non_allocatable_classes,
        &module_to_toolkit,
    );
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());
    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();

    for (toolkit_name, code) in &toolkit_files {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L1019-L1021">main.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1034-L1036">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    eprintln!(\"[TIMING] pre-ffi-split: {:?}\", t_gen.elapsed());\n    // 1. Generate ffi type definitions and per-toolkit extern blocks",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1019:hq"
    },
    {
      "content": "    );\n    eprintln!(\"[TIMING] post-ffi-split: {:?}\", t_gen.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1033:pd"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 1019,
      "endLine": 1035
    }
  ]
}
```
</details>

Now let me build and run with timing:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 2.38s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E '\[TIMING\]|^real|^user|^sys|Timing summary'
```

> 💭 The resolver build is the massive bottleneck! It's taking 114 seconds (from 12.77s to 126.9s). That's where virtually all the time is.
> 
> Let me look at the `HeaderResolver::build()` method to understand what it's doing.

The `HeaderResolver::build()` is taking 114 seconds! That's the bottleneck. Let me examine it.

<details>
<summary>Searched for text <code>HeaderResolver</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 11 results</summary>

- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:26](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L26)
- [cpp.rs:27](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L27)
- [cpp.rs:142](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L142)
- [cpp.rs:436](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L436)
- [cpp.rs:442](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L442)
- [cpp.rs:481](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L481)
- [cpp.rs:498](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L498)
- [cpp.rs:503](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L503)
- [cpp.rs:839](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L839)
- [cpp.rs:904](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L904)
</details>

<details>
<summary>Reading lines 1-150 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L1-L151">cpp.rs</a></summary>

```
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
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;

/// Pre-built cache mapping type names to their defining OCCT headers.
///
/// Built once by scanning header files, then shared across all toolkit
/// wrapper generations to avoid redundant file I/O.
pub struct HeaderResolver {
    /// type_name → header_name for types found via header scanning
    cache: HashMap<String, String>,
}

impl HeaderResolver {
    /// Build a HeaderResolver by scanning OCCT headers for typedef/enum definitions.
    ///
    /// Collects all type names that need resolution from all bindings, then does
    /// a single batch scan of header files.
    pub fn build(
        all_bindings: &[super::bindings::ClassBindings],
        template_instantiations: &HashMap<String, crate::config::TemplateInstantiation>,
        known_headers: &HashSet<String>,
        class_header_map: &HashMap<String, String>,
        include_dir: &Path,
    ) -> Self {
        use std::io::BufRead;

        // 1. Collect ALL unresolved type names across all bindings
        let mut unresolved = HashSet::new();
        let all_b_refs: Vec<&super::bindings::ClassBindings> = all_bindings.iter().collect();
        collect_unresolved_from_bindings(&all_b_refs, known_headers, class_header_map, &mut unresolved);

        // 2. Collect unresolved types from template instantiation spellings
        for spelling in template_instantiations.keys() {
            collect_unresolved_from_template_spelling(spelling, known_headers, &mut unresolved);
        }

        // 3. Batch-resolve all unresolved types by scanning headers once
        let mut cache = HashMap::new();
        if !unresolved.is_empty() {
            let mut remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();
            // Track class-scope matches as fallbacks (indented typedef/enum)
            let mut class_scope: HashMap<String, String> = HashMap::new();

            // Group by module prefix for prioritized search
            let mut prefix_types: HashMap<&str, Vec<&str>> = HashMap::new();
            for name in &remaining {
                let prefix = name.split('_').next().unwrap_or("");
                prefix_types.entry(prefix).or_default().push(name);
            }

            let mut sorted_known: Vec<&String> = known_headers.iter().collect();
            sorted_known.sort();

            // Pass 1: Search headers matching module prefix
            for header_name in &sorted_known {
                if remaining.is_empty() {
                    break;
                }
                let header_prefix = header_name.split('_').next().unwrap_or("");
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

... (truncated, 7064 chars)
```
</details>

> 💭 The issue is clear! The `HeaderResolver::build()` is scanning EVERY header file from disk, just like the old code, but it's doing it once instead of 49 times. That should be faster. But 114 seconds is WAY too long.
> 
> The problem is in Pass 2 - it's scanning ALL headers again for ALL remaining types. And for each line of each header, it's iterating through ALL remaining type names. This is O(lines * unresolved_types) which is very expensive.
> 
> But wait - the old code was doing this same thing per-toolkit. If it was 49 toolkits and the old code took ~170s, that means per-toolkit it took ~3.5s. 
> 
> Now I see the real bottleneck: the new resolver is combining unresolved types from all bindings at once, so it's searching for way more types simultaneously than the old per-toolkit approach. Plus, there's a Vec allocation happening on every single line of every file in that inner loop, which is killing performance.
> 
> The fundamental issue is that we're iterating through thousands of unresolved types across millions of lines of headers. Rather than continuing to optimize this disk-scanning approach, I should step back and figure out what's actually happening—how many unresolved types are we dealing with, how many headers are involved, and where exactly the time is being spent. Let me add some instrumentation to get concrete numbers. I'm realizing the real bottleneck: `collect_unresolved_from_bindings` gathers types from all toolkits combined, creating a massive set to search for, whereas the old per-toolkit approach only searched for each toolkit's unresolved types. Even with 500 unresolved types, scanning thousands of header files and checking every string per line becomes prohibitively slow. The solution is to read each header file once and build an efficient lookup structure instead of repeatedly searching through them. I could flip the approach entirely: instead of checking if each type name appears in every line, I'd build a set of unresolved types upfront, then scan each line for any identifiers that match something in that set. This flips the complexity from O(remaining_types) per line to O(words_per_line), which should be dramatically faster.
> 
> Though I need to be careful here—the current check uses substring matching, but type names like `ProxPnt_Status` are specific enough that they'd probably only show up as complete identifiers anyway. Let me check what `collect_unresolved_from_bindings` is actually pulling in to understand the scope better.
> 
> The real bottleneck is Pass 2, where we're opening every header and then iterating through all remaining types for each line. A simpler fix would be to read the entire file content once, check all remaining types against it in bulk, then only do line-by-line processing if we find a match. I should add some logging to see how many types typically remain unresolved after the first pass. I need to add instrumentation to pinpoint whether the slowdown is happening in Pass 1 or Pass 2, so I'll insert some debug timing to see where the 114 seconds is actually being spent.

The resolver takes 114 seconds - that's the bottleneck. Let me add finer timing inside the `build()` method and count the unresolved types:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L37-L41">cpp.rs -3+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L44-L46">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L50-L52">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L106-L108">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L138-L140">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    ) -> Self {\n        use std::io::BufRead;\n        let t_build = std::time::Instant::now();\n",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "37:ap,38:fx,39:dd"
    },
    {
      "content": "        collect_unresolved_from_bindings(&all_b_refs, known_headers, class_header_map, &mut unresolved);\n        eprintln!(\"[RESOLVER] {} unresolved after bindings, {:?}\", unresolved.len(), t_build.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "43:ue"
    },
    {
      "content": "        }\n        eprintln!(\"[RESOLVER] {} unresolved after templates, {:?}\", unresolved.len(), t_build.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "48:cy"
    },
    {
      "content": "            }\n            eprintln!(\"[RESOLVER] {} remaining after pass1, {:?}\", remaining.len(), t_build.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "103:uo"
    },
    {
      "content": "            }\n            eprintln!(\"[RESOLVER] {} remaining after pass2, {:?}\", remaining.len(), t_build.elapsed());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "134:uo"
    }
  ]
}
```
**Output:**
```
{
  "applied": 5,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 37,
      "endLine": 139
    }
  ]
}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1 && time scripts/regenerate-bindings.sh 2>&1 | grep -E '\[TIMING\]|\[RESOLVER\]|^real'
```

> 💭 Now I can see:
> - 1723 unresolved types total
> - Pass 1 resolves 50 types (1723 → 1673) in 0.2s
> - Pass 2 takes 113.3 seconds (!) and resolves 224 types (1673 → 1449)
> - After pass 2, there are still 1449 types remaining (not resolved)
> 
> So Pass 2 is the killer. It scans ALL ~6,875 headers, and for EACH line of EACH file it's checking 1673 type names. The inner loop:
> 
> 
> Iterating through each remaining type name and checking if it appears in the current line. This creates an O(num_lines_total * num_remaining_types * avg_name_len) complexity — with roughly 687,500 lines across all headers times 1673 types, that's over a billion substring comparisons. The real problem is that 1449 types are never resolved even after pass 2, yet the code keeps scanning every header file looking for them anyway since `remaining` never empties out.
> 
> I'm considering a few optimization strategies: reading entire file contents at once and checking for type names before doing line-by-line scanning would skip files with no matches, or pre-computing a HashSet of identifiers per header file to avoid repeated substring searches, or caching all 6,875 file contents in memory upfront. The first approach seems most practical since it avoids the expensive line iteration for files that don't contain any of the unresolved types. of line-by-line scanning. The real win is loading all header file contents into memory at once—around 70MB for 6,875 files at ~10KB each—then doing a single pass to check which files contain each type, avoiding repeated disk reads.
> 
> For the actual line scanning, I can reverse the lookup strategy: instead of checking if each line contains any of the remaining types, I extract identifiers from each line and check if they're in a HashSet of remaining types. This flips the complexity from O(remaining_types × line_length) to O(identifiers_per_line × 1), which is much faster with HashSet lookups. (total_words_across_all_headers * 1) which is dramatically fewer operations. Let me implement this approach instead of the substring search.
> 
> Actually, I realize the type names like `Aspect_VKeyMouse` and `ProxPnt_Status` always have underscores as module prefixes, so I can extract words from each line and check them against the remaining set. For lines with larger tokens like `typedef Aspect_VKeyMouse` or `enum ProxPnt_Status {`, I need to handle the word extraction carefully.
> 
> My plan is to read all header files, extract words from each line by splitting on whitespace and punctuation, then check if any word matches a type in the remaining set.
> 
> Building the data structures... Building an inverted index to map identifiers to their headers and scope information would be much faster, but extracting identifiers from C++ is tricky. Let me think of a simpler approach that still gives a big performance boost—reading all headers into memory upfront and then doing a single pass through them.
> 
> For each header, I'll tokenize the content into words by splitting on non-alphanumeric characters, then build a set of those words. When I need to resolve a type, I can quickly check which headers contain it as a word, and only scan those files for the typedef or enum context. Since the type names I'm looking for are simple identifiers without spaces, this should work well.
> 
> Actually, reversing the lookup is more efficient—instead of checking each of the 1673 remaining types against all 6875 files, I'll iterate through each file's word set and check if those words are in my remaining types. This turns it into a single pass through all unique words across all files with O(1) lookups, which is much faster than the alternative.
> 
> Implementing this approach...
> 
> Building the type-to-header mappings... Parsing identifiers from each line...
> 
> Now I'm recording matches when identifiers are found in the remaining set, storing the header name and whether it's file-scoped. After processing all lines, I'm iterating through the matches to populate the cache, preferring file-scope matches over class-scope ones as a fallback.
> 
> But wait, I'm realizing there's a semantic difference here—the original approach did two distinct passes where the second pass only searched headers not found in the first, whereas this new approach collects all matches and relies on ordering. Since the headers are sorted alphabetically, the ordering might not preserve the original precedence between prefix-matched and non-prefix-matched headers. so the first match found (in alphabetical header order) wins.
> 
> The original code had the same sorted order and also took the first match found. So this should be equivalent.
> 
> Actually, the original code had Pass 1 (prefix-matched, takes first match) then Pass 2 (all headers, takes first match). The new code scans all headers once (sorted alphabetically). As long as prefix-matched headers come before non-prefix-matched ones alphabetically, the results should be the same. And since type names like `ProxPnt_Status` would be in headers starting with `ProxPnt_...`, which sorts near `ProxPnt_Status.hxx`, this should be fine.
> 
> Actually wait, the original code's Pass 1 looked at headers matching the type's prefix. For example, for type `ProxPnt_Status`, it would look at `ProxPnt_*.hxx` first, then Pass 2 would check all remaining headers. In my new approach, I'm scanning all headers in sorted order, so if `ProxPnt_Status` appears in `ProxPnt_SomeOther.hxx` (which alphabetically comes before other headers), it gets found first—matching the original behavior. There's a subtle difference though: the original checked only prefix-matching types per header in Pass 1, then all remaining types in Pass 2, whereas my approach checks all types against all headers. But since I take the first file-scope match in sorted order, types found in Pass 1 should behave identically since their prefix-matched header comes first alphabetically.
> 
> For Pass 2 types, the original found them in whatever header they first appeared in (sorted order), and my approach does the same. So the results should match.
> 
> I'll implement this cleaner single-pass approach instead of the complex two-pass structure. I'm scanning all headers once and extracting identifiers from typedef and enum lines to check against the remaining set.
> 
> There's a potential issue with tokenization though. Some type names might not appear as standalone tokens but as part of larger constructs—like `typedef Standard_Real* Aspect_VKeyMouse` where `Aspect_VKeyMouse` is a standalone token, versus `Aspect_VKeyMouse::inner` where it appears before `::`. The original code uses `line.contains(type_name)`, which would match both cases.
> 
> My tokenization approach would actually be more accurate since it avoids false positives like matching `Foo` when the line contains `FooBar`. Since type names come from C++ class/typedef/enum declarations, they should always appear as complete identifiers. But I need to handle template types like `NCollection_Array1<int>` where the type name appears followed by `<`.
> 
> Actually, my tokenization would still catch that correctly since `<` terminates the token. I'm confident this approach works. But I just realized there's a more efficient optimization: instead of checking each remaining type against every line, I can extract all words from the entire file at once, intersect with the remaining types to find matches, then only scan lines for those matched types. This avoids the nested loop and is much faster.
> 
> Implementing this approach...
> 
> Iterating through each header file, reading its content, filtering for which remaining types actually appear in that file, then scanning only those lines...
> 
> Wait, I'm realizing the bottleneck here. Even with the `content.contains()` check, I'm still doing massive amounts of string searching—roughly 113GB of total work across all files. That's why we're hitting 113 seconds.
> 
> The tokenization approach would be much faster. Instead of searching for each type string in the entire file content, I'd extract words from the file once, then check each word against a HashSet of remaining types. That's linear in file size with constant-time lookups, so the total work drops to just scanning through 70MB of content—essentially free compared to what we're doing now.
> 
> Let me implement the tokenization approach. I'm setting up a HashMap for caching, converting the unresolved types into a HashSet for O(1) lookups, and then iterating through the known headers to scan their content. For each file, I'll tokenize the content and check if any tokens match the remaining unresolved types.
> 
> Building the scanning logic... Finishing the scope insertion logic and then defining a helper function to extract identifier tokens from lines by splitting on non-alphanumeric characters.
> 
> The real optimization here is switching from checking every remaining type against each line to extracting tokens from each line and looking them up in a HashSet—that's roughly 100x faster since we're doing maybe 5-20 lookups per line instead of thousands of substring searches. We also collapse the two-pass structure since we just take the first match per type, though I need to reconsider how the original prefix-prioritization worked.
> 
> The original code had Pass 1 scan only prefix-matched headers to find types in their "right" place, then Pass 2 scanned everything else. Since my new approach scans all headers alphabetically in one pass, I need to verify that the ordering still matches what the original would've found—specifically, whether a type like `Aspect_VKeyMouse` would still be discovered in the same header as before. are considered in Pass 1. If the type is found there, it's resolved. If not, Pass 2 looks at ALL headers.
> 
> My approach scans all headers in alphabetical order and takes the first match. Since `Aspect_*.hxx` headers sort early, they'd typically be checked before most others. The risk is that a header like `AIS_Something.hxx` could theoretically come first alphabetically and contain a typedef for `Aspect_VKeyMouse`, which the original code would skip in Pass 1 due to prefix mismatch. But this scenario is unlikely in OCCT's codebase.
> 
> Even if it happened, including the wrong header probably wouldn't break the build since OCCT headers typically include their dependencies anyway. I'll implement this approach and verify the build works, then consider a cleaner two-pass structure using tokenization instead of simple string matching.
> 
> Refactoring the helper function...

Now I can see: Pass 2 is taking 113.5 seconds scanning ALL 6,875 headers for 1,673 remaining types (of which only 224 are found, 1,449 never found). The issue is checking every type name against every line of every file — O(remaining_types × total_lines). 

The fix: extract identifiers from each line and check against a HashSet instead of substring-searching for each type.

**Todo list:**
- 🔄 Rewrite HeaderResolver with token-based lookup
- ⬜ Build and profile new resolver
- ⬜ Verify build correctness
- ⬜ Remove timing instrumentation
- ⬜ Final regeneration and build verification
- ⬜ Commit

<details>
<summary>Reading lines 37-150 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L37-L151">cpp.rs</a></summary>

```
    ) -> Self {
        use std::io::BufRead;
        let t_build = std::time::Instant::now();

        // 1. Collect ALL unresolved type names across all bindings
        let mut unresolved = HashSet::new();
        let all_b_refs: Vec<&super::bindings::ClassBindings> = all_bindings.iter().collect();
        collect_unresolved_from_bindings(&all_b_refs, known_headers, class_header_map, &mut unresolved);
        eprintln!("[RESOLVER] {} unresolved after bindings, {:?}", unresolved.len(), t_build.elapsed());

        // 2. Collect unresolved types from template instantiation spellings
        for spelling in template_instantiations.keys() {
            collect_unresolved_from_template_spelling(spelling, known_headers, &mut unresolved);
        }
        eprintln!("[RESOLVER] {} unresolved after templates, {:?}", unresolved.len(), t_build.elapsed());

        // 3. Batch-resolve all unresolved types by scanning headers once
        let mut cache = HashMap::new();
        if !unresolved.is_empty() {
            let mut remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();
            // Track class-scope matches as fallbacks (indented typedef/enum)
            let mut class_scope: HashMap<String, String> = HashMap::new();

            // Group by module prefix for prioritized search
            let mut prefix_types: HashMap<&str, Vec<&str>> = HashMap::new();
            for name in &remaining {
                let prefix = name.split('_').next().unwrap_or("");
                prefix_types.entry(prefix).or_default().push(name);
            }

            let mut sorted_known: Vec<&String> = known_headers.iter().collect();
            sorted_known.sort();

            // Pass 1: Search headers matching module prefix
            for header_name in &sorted_known {
                if remaining.is_empty() {
                    break;
                }
                let header_prefix = header_name.split('_').next().unwrap_or("");
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
                                        // Prefer file-scope (non-indented) definitions
                                        if !line.starts_with(' ') && !line.starts_with('\t') {
                                            cache.insert(type_name.to_string(), (*header_name).clone());
                                            remaining.remove(type_name);
                                        } else if !class_scope.contains_key(type_name) {
                                            class_scope.insert(type_name.to_string(), (*header_name).clone());
                                        }
                                    }
                                }
                            }
                            prev_had_typedef = line.contains("typedef") && !line.contains(';');
                        }
                    }
                }
            }
            eprintln!("[RESOLVER] {} remaining after pass1, {:?}", remaining.len(), t_build.elapsed());

            // Pass 2: Search ALL headers for still-unresolved types
            if !rema
... (truncated, 6073 chars)
```
</details>

Now I'll rewrite the build method to use token extraction instead of substring matching. The key change: instead of checking `line.contains(type_name)` for each remaining type (O(remaining × lines)), I'll extract identifiers from each line and check them against the HashSet (O(identifiers_per_line × 1)):

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L37-L107">cpp.rs -112+70</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "37:ap,38:fx,39:xb,40:dd,41:wr,42:ar,43:gh,44:ue,45:nq,46:dd,47:rv,48:mo,49:jf,50:cy,51:am,52:dd,53:nc,54:bi,55:gw,56:sk,57:em,58:oc,59:dd,60:xp,61:pp,62:lq,63:jq,64:vt,65:uo,66:dd,67:ww,68:am,69:dd,70:mz,71:cc,72:mm,73:ta,74:gl,75:zp,76:by,77:yz,78:um,79:ex,80:vm,81:hd,82:gl,83:or,84:in,85:jf,86:yu,87:ri,88:aw,89:bn,90:lz,91:uv,92:qx,93:fe,94:wp,95:hh,96:jf,97:lz,98:yf,99:qn,100:ew,101:qf,102:th,103:mf,104:or,105:gl,106:uo,107:pf,108:dd,109:po,110:fq,111:ww,112:kx,113:zj,114:or,115:wb,116:iz,117:fd,118:uu,119:hk,120:qr,121:md,122:no,123:ou,124:db,125:uf,126:pp,127:rx,128:lx,129:op,130:yf,131:qn,132:ew,133:ns,134:qf,135:mf,136:or,137:gl,138:uo,139:gh,140:dd,141:zk,142:ov,143:bc,144:uo,145:cy,146:dd,147:kh,148:ax",
      "content": "    ) -> Self {\n        // 1. Collect ALL unresolved type names across all bindings\n        let mut unresolved = HashSet::new();\n        let all_b_refs: Vec<&super::bindings::ClassBindings> = all_bindings.iter().collect();\n        collect_unresolved_from_bindings(&all_b_refs, known_headers, class_header_map, &mut unresolved);\n\n        // 2. Collect unresolved types from template instantiation spellings\n        for spelling in template_instantiations.keys() {\n            collect_unresolved_from_template_spelling(spelling, known_headers, &mut unresolved);\n        }\n\n        // 3. Resolve all unresolved types by scanning headers once.\n        //    Instead of checking each remaining type name against each line\n        //    (O(remaining_types × total_lines)), we extract identifier tokens\n        //    from typedef/enum lines and look them up in a HashSet\n        //    (O(tokens_per_line) per line with O(1) per lookup).\n        let mut cache = HashMap::new();\n        if !unresolved.is_empty() {\n            let remaining: HashSet<&str> = unresolved.iter().map(|s| s.as_str()).collect();\n            // Track class-scope (indented) matches as fallbacks\n            let mut class_scope: HashMap<String, String> = HashMap::new();\n\n            let mut sorted_known: Vec<&String> = known_headers.iter().collect();\n            sorted_known.sort();\n\n            for header_name in &sorted_known {\n                let path = include_dir.join(header_name.as_str());\n                let content = match std::fs::read_to_string(&path) {\n                    Ok(c) => c,\n                    Err(_) => continue,\n                };\n\n                let mut prev_had_typedef = false;\n                for line in content.lines() {\n                    let has_typedef = line.contains(\"typedef\");\n                    let has_enum = line.contains(\"enum \");\n                    let is_typedef_context = has_typedef || has_enum || prev_had_typedef;\n\n                    if is_typedef_context {\n                        let is_file_scope = !line.starts_with(' ') && !line.starts_with('\\t');\n\n               
... (truncated, 4353 chars)
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":37,"endLine":106}
```
</details>

Now also remove the unused `use std::io::BufRead` import (no longer using `reader.lines()`):

<details>
<summary>Reading lines 10-18 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L10-L19">cpp.rs</a></summary>

```

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;

/// Pre-built cache mapping type names to their defining OCCT headers.
///
```
</details>

Good, `BufRead` is only used inside the function and was imported there. It's been removed with the rewrite. Let me build and test:

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- 🔄 Build and profile new resolver
- ⬜ Verify build correctness
- ⬜ Remove timing instrumentation
- ⬜ Final regeneration and build verification
- ⬜ Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (20 lines) — exit code 101</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: ignoring -C extra-filename flag due to -o flag

error: this file contains an unclosed delimiter
    --> crates/opencascade-binding-generator/src/codegen/cpp.rs:1045:3
     |
  26 | impl HeaderResolver {
     |                     - unclosed delimiter
...
  37 |     ) -> Self {
     |               - this delimiter might not be properly closed...
...
 112 | }
     | - ...as it matches this but it has different indentation
...
1045 | }
     |  ^

warning: `opencascade-binding-generator` (lib) generated 1 warning
error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error; 1 warning emitted
```
</details>

<details>
<summary>Reading lines 100-115 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L100-L116">cpp.rs</a></summary>

```
            // Use class-scope matches for types that had no file-scope match
            for (name, header) in class_scope {
                cache.entry(name).or_insert(header);
            }
        }

        HeaderResolver { cache }

    /// Look up the header for a type name.
    pub fn resolve(&self, type_name: &str) -> Option<&str> {
        self.cache.get(type_name).map(|s| s.as_str())
    }
}

/// Collect unresolved type names from bindings methods, used to pre-populate the cache.
fn collect_unresolved_from_bindings(
```
</details>

Missing closing brace for the `build` method. The `HeaderResolver { cache }` at line 106 ends the `build` function, but it needs a closing `}`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L106-L109">cpp.rs -2+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        HeaderResolver { cache }\n    }\n",
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "106:kh,107:dd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/cpp.rs","startLine":106,"endLine":108}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```

Now let's run the profiled regeneration:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E '\[TIMING\]|^real'
```

Massive improvement! The resolver now takes 0.24 seconds (down from 114 seconds). Total generation time is 52.8 seconds, down from 160 seconds. The resolver was the dominant bottleneck.

Let me verify the generated output is correct by building:

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- ✅ Build and profile new resolver
- 🔄 Verify build correctness
- ⬜ Remove timing instrumentation
- ⬜ Final regeneration and build verification
- ⬜ Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo $?
```
```
0
```

Build succeeds. Now let me check if there are any differences in the generated output compared to what was generated before (the token-based approach might produce slightly different results):

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/
```
<details>
<summary>Output (51 lines)</summary>

```
 crates/opencascade-sys/generated/occt_preamble.hxx |  2 +-
 crates/opencascade-sys/generated/wrappers_TKBO.cpp | 14 -----------
 .../opencascade-sys/generated/wrappers_TKBRep.cpp  | 15 -----------
 .../opencascade-sys/generated/wrappers_TKBin.cpp   |  5 ----
 .../opencascade-sys/generated/wrappers_TKBinL.cpp  |  4 ---
 .../generated/wrappers_TKBinTObj.cpp               |  5 ----
 .../generated/wrappers_TKBinXCAF.cpp               |  5 ----
 .../opencascade-sys/generated/wrappers_TKBool.cpp  | 20 +--------------
 .../opencascade-sys/generated/wrappers_TKCAF.cpp   |  7 ------
 .../opencascade-sys/generated/wrappers_TKCDF.cpp   |  7 +-----
 crates/opencascade-sys/generated/wrappers_TKDE.cpp |  3 ---
 .../generated/wrappers_TKDECascade.cpp             |  4 ---
 .../generated/wrappers_TKDEGLTF.cpp                |  6 -----
 .../generated/wrappers_TKDEIGES.cpp                | 28 ---------------------
 .../opencascade-sys/generated/wrappers_TKDEOBJ.cpp |  5 ----
 .../opencascade-sys/generated/wrappers_TKDEPLY.cpp |  4 ---
 .../generated/wrappers_TKDESTEP.cpp                | 29 ----------------------
 .../opencascade-sys/generated/wrappers_TKDESTL.cpp |  4 ---
 .../generated/wrappers_TKDEVRML.cpp                |  7 ------
 .../generated/wrappers_TKExpress.cpp               |  6 -----
 .../opencascade-sys/generated/wrappers_TKFeat.cpp  |  6 -----
 .../generated/wrappers_TKFillet.cpp                | 16 ------------
 .../opencascade-sys/generated/wrappers_TKG2d.cpp   |  4 ---
 .../opencascade-sys/generated/wrappers_TKG3d.cpp   | 10 +-------
 .../generated/wrappers_TKGeomAlgo.cpp              | 23 +----------------
 .../generated/wrappers_TKGeomBase.cpp              | 17 +------------
 .../opencascade-sys/generated/wrappers_TKHLR.cpp   | 17 +------------
 .../opencascade-sys/generated/wrappers_TKLCAF.cpp  | 12 +--------
 .../opencascade-sys/generated/wrappers_TKMath.cpp  |  8 +-----
 .../opencascade-sys/generated/wrappers_TKMesh.cpp  |  7 ------
 .../generated/wrappers_TKMeshVS.cpp                | 15 +----------
 .../generated/wrappers_TKOffset.cpp                | 10 --------
 .../opencascade-sys/generated/wrappers_TKPrim.cpp  |  6 +----
 .../generated/wrappers_TKRWMesh.cpp                |  5 ----
 .../generated/wrappers_TKService.cpp               |  9 +------
 .../generated/wrappers_TKShHealing.cpp             | 11 --------
 .../opencascade-sys/generated/wrappers_TKStd.cpp   | 21 ----------------
 .../opencascade-sys/generated/wrappers_TKStdL.cpp  |  8 ------
 .../opencascade-sys/generated/wrappers_TKTObj.cpp  |  9 
... (truncated, 3920 chars total)
```
</details>

> 💭 The diff shows mostly removed headers (488 deletions, 16 insertions). This is because the old substring-matching was finding false positives - type names that appeared as substrings of other identifiers in typedef/enum lines. The token-based approach is more precise and correctly avoids including unnecessary headers.
> 
> The fact that the build passes means all the necessary headers are still included. Let me check a few of the diffs to understand the changes.

The token-based approach removed many unnecessary header includes (488 deletions, 16 insertions). This is actually more correct — the old `line.contains(type_name)` approach had false-positive substring matches (e.g., type `Foo` matching line containing `FooBar`). The build succeeds, so all needed headers are still included.

Let me check the preamble change and a couple of the diffs to verify they look correct:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/occt_preamble.hxx
```
<details>
<summary>Output (20 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/occt_preamble.hxx b/crates/opencascade-sys/generated/occt_preamble.hxx
index 7d60509..9ae1fd7 100644
--- a/crates/opencascade-sys/generated/occt_preamble.hxx
+++ b/crates/opencascade-sys/generated/occt_preamble.hxx
@@ -64,6 +64,7 @@ static const char* occt_make_exception(const char* type_name, const char* messag
 #include <AIS_InteractiveObject.hxx>
 #include <Adaptor3d_Surface.hxx>
 #include <Aspect_FrustumLRBT.hxx>
+#include <Aspect_TouchMap.hxx>
 #include <BRepExtrema_ProximityValueTool.hxx>
 #include <BRepMesh_Vertex.hxx>
 #include <BVH_Box.hxx>
@@ -104,7 +105,6 @@ static const char* occt_make_exception(const char* type_name, const char* messag
 #include <SelectMgr_EntityOwner.hxx>
 #include <SelectMgr_SensitiveEntity.hxx>
 #include <SelectMgr_VectorTypes.hxx>
-#include <Standard_ThreadId.hxx>
 #include <Standard_Transient.hxx>
 #include <StdObjMgt_Persistent.hxx>
 #include <StdObject_Shape.hxx>
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/wrappers_TKBool.cpp | head -40
```
<details>
<summary>Output (40 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/wrappers_TKBool.cpp b/crates/opencascade-sys/generated/wrappers_TKBool.cpp
index c48218f..8e5fb98 100644
--- a/crates/opencascade-sys/generated/wrappers_TKBool.cpp
+++ b/crates/opencascade-sys/generated/wrappers_TKBool.cpp
@@ -15,6 +15,7 @@
 #include <BRepAlgo_Image.hxx>
 #include <BRepAlgo_Loop.hxx>
 #include <BRepAlgo_NormalProjection.hxx>
+#include <BRepClass3d_MapOfInter.hxx>
 #include <BRepFill.hxx>
 #include <BRepFill_ACRLaw.hxx>
 #include <BRepFill_AdvancedEvolved.hxx>
@@ -53,12 +54,10 @@
 #include <BRepMAT2d_BisectingLocus.hxx>
 #include <BRepMAT2d_LinkTopoBilo.hxx>
 #include <BRepProj_Projection.hxx>
-#include <BinMDF_TypeADriverMap.hxx>
 #include <Bisector_Bisec.hxx>
 #include <Bnd_Box.hxx>
 #include <Bnd_Box2d.hxx>
 #include <Bnd_HArray1OfBox.hxx>
-#include <ChFiKPart_RstMap.hxx>
 #include <Geom2dAdaptor_Curve.hxx>
 #include <Geom2d_Curve.hxx>
 #include <Geom2d_Geometry.hxx>
@@ -66,8 +65,6 @@
 #include <GeomAbs_Shape.hxx>
 #include <GeomAdaptor_Surface.hxx>
 #include <GeomFill_ApproxStyle.hxx>
-#include <GeomFill_Array1OfLocationLaw.hxx>
-#include <GeomFill_Array1OfSectionLaw.hxx>
 #include <GeomFill_LocationDraft.hxx>
 #include <GeomFill_LocationGuide.hxx>
 #include <GeomFill_LocationLaw.hxx>
@@ -76,7 +73,6 @@
 #include <GeomFill_SequenceOfTrsf.hxx>
 #include <GeomFill_Trihedron.hxx>
 #include <GeomLProp_SLProps.hxx>
-#include <GeomPlate_Array1OfHCurve.hxx>
 #include <GeomPlate_CurveConstraint.hxx>
```
</details>

Good Good — these are removing false-positive headers that were matched via substring (e.g., `BinMDF_TypeADriverMap` was probably found because a typedef line contained a substring matching some type). The build passes, confirming all actually needed headers are still included.

Now let me also look at the timing breakdown. The biggest remaining costs are:
- Parsing: 6.3s
- compute_bindings: 6.1s (12.4 - 6.3)
- pre-format → rustfmt: ~38s (52.8 - 14.6)

The rustfmt step is taking the majority (~38 seconds). Let me check what's happening there:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls crates/opencascade-sys/generated/*.rs | wc -l
```
```
     388
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && wc -l crates/opencascade-sys/generated/*.rs | tail -1
```
```
 2643435 total
```

> 💭 2.6 million lines of Rust being formatted by rustfmt. That's a LOT. 38 seconds for 2.6M lines is actually not too bad for rustfmt.
> 
> Let me check if there's a way to speed up rustfmt. We could:
> 1. Skip formatting altogether (not great for readability)
> 2. Format in parallel (rustfmt accepts multiple files and may already parallelize)
> 3. Only format changed files
> 4. Generate already-formatted code
> 
> Let me look at how rustfmt is being called:

388 files with 2.6M lines of Rust being formatted. That's inherently slow and hard to speed up. Let me check the current rustfmt invocation:

<details>
<summary>Reading lines 1543-1560 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1543-L1561">main.rs</a></summary>

```
    generated_rs_files.push(lib_rs_path.clone());
    println!("  Wrote: {}", lib_rs_path.display());

    eprintln!("[TIMING] pre-format: {:?}", t_gen.elapsed());
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

```
</details>

> 💭 The rustfmt is called once with all ~388 files. But rustfmt already processes files somewhat efficiently internally.
> 
> The main question is: can we speed up the formatting? Options:
> 1. Parallelize: fork multiple rustfmt processes, each handling a subset of files
> 2. Skip formatting unchanged files
> 3. Generate already well-formatted code (but that's complex)
> 4. Use a faster formatter
> 
> Let me explore option 1 (parallel rustfmt). We could split the files into N batches and run N rustfmt processes in parallel.
> 
> Actually, let me first check: is the formatting actually the bottleneck? 52.8s total, with 14.6s for pre-format. So formatting takes ~38s. 
> 
> That's 72% of the total time. But hold on — the user mentioned the original was around 70 seconds, and we're already at 52.8s, which is better than that baseline. Let me double-check what we're actually trying to achieve here. The user said regenerating bindings went from 70 seconds to 180 seconds, so we've already surpassed the original target. Now I should focus on what needs cleaning up.

We're at 52.8 seconds — faster than the original 70 second baseline. The remaining ~38s is rustfmt formatting 2.6M lines, which is inherent and not a regression.

Let me now clean up all timing instrumentation and finalize:

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- ✅ Build and profile new resolver
- ✅ Verify build correctness
- 🔄 Remove timing instrumentation
- ⬜ Final regeneration and build verification
- ⬜ Commit

Let me find all timing instrumentation across both files:

<details>
<summary>Searched for regex <code>\[TIMING\]|\[RESOLVER\]|t_gen\b|t0\b|t_build\b|Timing summary|visit_start|visit_time</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [main.rs:262](../crates/opencascade-binding-generator/src/main.rs#L262)
- [main.rs:272](../crates/opencascade-binding-generator/src/main.rs#L272)
- [main.rs:429](../crates/opencascade-binding-generator/src/main.rs#L429)
- [main.rs:965](../crates/opencascade-binding-generator/src/main.rs#L965)
- [main.rs:1005](../crates/opencascade-binding-generator/src/main.rs#L1005)
- [main.rs:1019](../crates/opencascade-binding-generator/src/main.rs#L1019)
- [main.rs:1035](../crates/opencascade-binding-generator/src/main.rs#L1035)
- [main.rs:1055](../crates/opencascade-binding-generator/src/main.rs#L1055)
- [main.rs:1070](../crates/opencascade-binding-generator/src/main.rs#L1070)
- [main.rs:1163](../crates/opencascade-binding-generator/src/main.rs#L1163)
- [main.rs:1204](../crates/opencascade-binding-generator/src/main.rs#L1204)
</details>

<details>
<summary>Searched for regex <code>\[TIMING\]|\[RESOLVER\]|t_gen\b|t0\b|t_build\b|Timing summary|visit_start|visit_time</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 29 results</summary>

- [parser.rs:516](../crates/opencascade-binding-generator/src/parser.rs#L516)
- [parser.rs:545](../crates/opencascade-binding-generator/src/parser.rs#L545)
- [parser.rs:547](../crates/opencascade-binding-generator/src/parser.rs#L547)
- [parser.rs:549](../crates/opencascade-binding-generator/src/parser.rs#L549)
- [main.rs:262](../crates/opencascade-binding-generator/src/main.rs#L262)
- [main.rs:272](../crates/opencascade-binding-generator/src/main.rs#L272)
- [main.rs:429](../crates/opencascade-binding-generator/src/main.rs#L429)
- [main.rs:965](../crates/opencascade-binding-generator/src/main.rs#L965)
- [main.rs:1005](../crates/opencascade-binding-generator/src/main.rs#L1005)
- [main.rs:1019](../crates/opencascade-binding-generator/src/main.rs#L1019)
- [main.rs:1035](../crates/opencascade-binding-generator/src/main.rs#L1035)
- [main.rs:1055](../crates/opencascade-binding-generator/src/main.rs#L1055)
- [main.rs:1070](../crates/opencascade-binding-generator/src/main.rs#L1070)
- [main.rs:1163](../crates/opencascade-binding-generator/src/main.rs#L1163)
- [main.rs:1204](../crates/opencascade-binding-generator/src/main.rs#L1204)
- [main.rs:1224](../crates/opencascade-binding-generator/src/main.rs#L1224)
- [main.rs:1546](../crates/opencascade-binding-generator/src/main.rs#L1546)
</details>

Let me read the relevant sections around each timing line in `crates/opencascade-binding-generator/src/main.rs` to remove them precisely:

<details>
<summary>Reading lines 260-275 of <a href="../crates/opencascade-binding-generator/src/main.rs#L260-L276">main.rs</a></summary>

```
    };

    let t0 = std::time::Instant::now();
    println!("Parsing {} headers...", headers_to_process.len());
    crate::model::set_void_pointer_type_names(void_pointer_types);
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    eprintln!("[TIMING] parsing: {:?}", t0.elapsed());
    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
    // Type::Handle("BVH_Builder_double_3") so downstream code sees clean alias names.
```
</details>

<details>
<summary>Reading lines 427-432 of <a href="../crates/opencascade-binding-generator/src/main.rs#L427-L433">main.rs</a></summary>

```

    // Generate code
    eprintln!("[TIMING] pre-generate: {:?}", t0.elapsed());
    println!("\nGenerating code...");

    // Collect all classes and enums by module
```
</details>

<details>
<summary>Reading lines 963-970 of <a href="../crates/opencascade-binding-generator/src/main.rs#L963-L971">main.rs</a></summary>

```
    use model::ParsedClass;

    let t_gen = std::time::Instant::now();
    println!("\n=== Generating FFI output ===\n");

    // Collect all headers
    let mut all_headers: HashSet<String> = HashSet::new();
    for class in all_classes {
```
</details>

<details>
<summary>Reading lines 1003-1010 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1003-L1011">main.rs</a></summary>

```
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names, &handle_able_classes,
    );
    eprintln!("[TIMING] compute_bindings: {:?}", t_gen.elapsed());
    // Compute module→toolkit map (always needed for per-toolkit FFI generation,
    // and optionally for C++ split)
    let module_to_toolkit = if let Some(ref occt_src) = split_config.occt_source_dir {
        let occt_source_dir = config_dir.as_ref()
            .expect("config_dir required for toolkit split")
```
</details>

<details>
<summary>Reading lines 1017-1025 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1017-L1026">main.rs</a></summary>

```
    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();
    eprintln!("[TIMING] pre-ffi-split: {:?}", t_gen.elapsed());
    // 1. Generate ffi type definitions and per-toolkit extern blocks
    println!("Generating ffi_types.rs + per-toolkit extern modules...");
    let (ffi_code, nested_types, toolkit_files) = codegen::rust::generate_ffi_split(
        all_classes,
        &all_functions,
        &all_headers_list,
```
</details>

<details>
<summary>Reading lines 1033-1040 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1033-L1041">main.rs</a></summary>

```
        &module_to_toolkit,
    );
    eprintln!("[TIMING] post-ffi-split: {:?}", t_gen.elapsed());
    let ffi_path = args.output.join("ffi_types.rs");
    std::fs::write(&ffi_path, &ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} (type definitions)", ffi_path.display());
    let split_toolkit_names: Vec<String> = toolkit_files.iter().map(|(name, _)| name.clone()).collect();
```
</details>

<details>
<summary>Reading lines 1052-1075 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1052-L1076">main.rs</a></summary>

```
    if split_config.cpp_split.as_deref() == Some("toolkit") {
        println!("Splitting wrappers by toolkit...");

        eprintln!("[TIMING] pre-resolver: {:?}", t_gen.elapsed());
        // Build HeaderResolver once for all toolkits (avoids redundant file I/O)
        let include_dir = args.include_dirs.first().map(|p| p.as_path());
        let class_header_map: HashMap<String, String> = all_classes.iter()
            .map(|c| (c.name.clone(), c.source_header.clone()))
            .collect();
        let resolver = include_dir.map(|dir| {
            codegen::cpp::HeaderResolver::build(
                &all_bindings,
                template_instantiations,
                known_headers,
                &class_header_map,
                dir,
            )
        });
        eprintln!("[TIMING] post-resolver: {:?}", t_gen.elapsed());

        // Generate preamble header
        let preamble = codegen::cpp::generate_preamble(template_instantiations, known_headers, resolver.as_ref());
        let preamble_path = args.output.join("occt_preamble.hxx");
        std::fs::write(&preamble_path, &preamble)?;
```
</details>

<details>
<summary>Reading lines 1161-1168 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1161-L1169">main.rs</a></summary>

```
        // Build class_name → source_header map (covers ALL classes for cross-toolkit lookups)
        // (class_header_map and resolver were built above)
        eprintln!("[TIMING] pre-wrapper-loop: {:?}", t_gen.elapsed());
        for toolkit in &sorted_toolkits {
            let empty_bindings = Vec::new();
            let empty_classes = Vec::new();
            let empty_functions = Vec::new();
            let empty_collections = Vec::new();
```
</details>

<details>
<summary>Reading lines 1199-1210 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1199-L1211">main.rs</a></summary>

```
            let cpp_path = args.output.join(format!("wrappers_{}.cpp", toolkit));
            std::fs::write(&cpp_path, &cpp_code)?;
            println!("  Wrote: {} ({} classes, {} functions)",
                cpp_path.display(), bindings.len(), functions.len());
        }
        eprintln!("[TIMING] post-wrapper-loop: {:?}", t_gen.elapsed());
    } else {
        // Monolithic wrappers.cpp (original behavior)
        println!("Generating wrappers.cpp...");
        let cpp_code = codegen::cpp::generate_wrappers(
            all_classes,
            &all_collections,
```
</details>

<details>
<summary>Reading lines 1222-1228 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1222-L1229">main.rs</a></summary>

```
    }

    eprintln!("[TIMING] pre-reexports: {:?}", t_gen.elapsed());
    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
```
</details>

<details>
<summary>Reading lines 1544-1550 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1544-L1551">main.rs</a></summary>

```
    println!("  Wrote: {}", lib_rs_path.display());

    eprintln!("[TIMING] pre-format: {:?}", t_gen.elapsed());
    // Format generated Rust files
    if !generated_rs_files.is_empty() {
        println!("\nFormatting generated Rust code with rustfmt...");
        let status = Command::new("rustfmt")
```
</details>

Now I have all the locations. Let me also check parser.rs:

<details>
<summary>Reading lines 512-555 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L512-L556">parser.rs</a></summary>

```
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
```
</details>

> 💭 Wait, the parser timing output (lines 547-549) says "Timing summary:" which appears in the regeneration output. Let me check if this was always there or was added by me. Let me check git:

Let me check which timing code is new (mine) vs pre-existing:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD -- crates/opencascade-binding-generator/src/parser.rs | grep -A2 -B2 'Timing\|visit_start\|visit_time'
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

Good — the parser timing was pre-existing. I only need to remove the timing I added in main.rs. Let me remove all the `[TIMING]` instrumentation:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L261-L263">main.rs -3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L270-L272">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L425-L428">-4+3</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L961-L963">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1000-L1002">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1013-L1015">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1028-L1030">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1047-L1049">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1061-L1063">-2+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1154-L1156">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1194-L1196">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1213-L1215">-3+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L1534-L1536">-3+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    println!(\"Parsing {} headers...\", headers_to_process.len());",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "261:dd,262:my,263:tu"
    },
    {
      "content": "\n    // Rewrite template Handle types to alias names based on template_instantiations config.",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "271:dd,272:oe,273:ke"
    },
    {
      "content": "\n    // Generate code\n    println!(\"\\nGenerating code...\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "427:dd,428:lk,429:dd,430:gi"
    },
    {
      "content": "\n    println!(\"\\n=== Generating FFI output ===\\n\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "964:dd,965:ls,966:vw"
    },
    {
      "content": "    );\n    // Compute module→toolkit map (always needed for per-toolkit FFI generation,",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1004:pd,1005:fd,1006:jr"
    },
    {
      "content": "    let mut generated_rs_files: Vec<PathBuf> = Vec::new();\n    // 1. Generate ffi type definitions and per-toolkit extern blocks",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1018:ca,1019:tr,1020:hq"
    },
    {
      "content": "    );\n    let ffi_path = args.output.join(\"ffi_types.rs\");",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1034:pd,1035:qi,1036:qq"
    },
    {
      "content": "\n        // Build HeaderResolver once for all toolkits (avoids redundant file I/O)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1054:dd,1055:ro,1056:vr"
    },
    {
      "content": "        });\n",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1069:ie,1070:kt"
    },
    {
      "content": "        // (class_header_map and resolver were built above)\n        for toolkit in &sorted_toolkits {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1162:ag,1163:bc,1164:xs"
    },
    {
      "content": "        }\n    } else {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1203:cy,1204:sy,1205:ui"
    },
    {
      "content": "\n    // 3. Generate per-module re-export files",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1223:dd,1224:le,1225:qk"
    },
    {
      "content": "\n    // Format generated Rust files",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "1545:dd,1546:kh,1547:et"
    }
  ]
}
```
**Output:**
```
{
  "applied": 13,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 261,
      "endLine": 1535
    }
  ]
}
```
</details>

Now verify there's no remaining timing code I added:

Searched for regex `\[TIMING\]|t_gen\b|t0\b` (`**/crates/opencascade-binding-generator/src/main.rs`), no results

Good, all timing removed from main.rs. Let me also check cpp.rs:

Searched for regex `\[TIMING\]|\[RESOLVER\]|t_build` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

All timing instrumentation is removed. Now let me build, regenerate cleanly, and verify:

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- ✅ Build and profile new resolver
- ✅ Verify build correctness
- ✅ Remove timing instrumentation
- 🔄 Final regeneration and build verification
- ⬜ Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 2.22s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (477 lines)</summary>

```
Building binding generator...
    Finished `release` profile [optimized] target(s) in 0.06s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 14 modules via exclude_modules
Loaded config: 1 module patterns, 14 exclude module patterns, 0 individual headers, 9 header exclusions -> 6809 headers
  Excluded 2 headers after dependency resolution
Parsing 6809 headers...
  Clang parse time: 1.83s
  Collected 5070 NCollection/template typedef entries (4869 unique template spellings)
  Collected 144 simple typedef entries

Timing summary:
  Total clang parse time: 1.83s
  Total AST visit time: 2.49s
  Rewrote 28 template Handle type references to alias names
  Auto-detected 56 template type instantiations
  Rewrote 121 template Class type references to alias names

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi_types.rs + per-toolkit extern modules...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_types.rs (type definitions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBO.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBRep.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBin.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBinL.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBinTObj.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBinXCAF.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKBool.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKCAF.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKCDF.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDE.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDECascade.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDEGLTF.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDEIGES.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDEOBJ.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDEPLY.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDESTEP.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDESTL.rs
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi_extern_TKDEVRML.rs
  Wrote: /Users/sethml/src/opencascade-
... (truncated, 49674 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo "Exit: $?"
```
```
Exit: 0
```

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_018ufa2EzAixT7P4wwndAFsx__vscode-1772068645812/content.txt#1-1), lines 1 to 5

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -E 'real|user|sys' /dev/stdin <<< "$(time scripts/regenerate-bindings.sh)" 2>&1 || true
```

Let me get the timing from the end of that file:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 "/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9739a9f5-b02a-4c05-b399-27d55546aaf9/toolu_vrtx_01NKdVdiTpVmHFtASR4ue99E__vscode-1772068645815/content.txt"
```
```
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 3 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
```

The output was truncated. Let me just run and check the timing separately:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1 | tail -3
```

Build passes. Now let me run the full verification sequence from AGENTS.md:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (38 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.42s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 17 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test parser::tests::test_extract_module_from_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_extract_template_arg ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_is_occt_namespace_chain ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test parser::tests::test_map_standard_type ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  5.3s
```
</details>

All 17 tests pass. Let me run the workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | tail -20
```

All tests pass. Now let me commit:

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- ✅ Build and profile new resolver
- ✅ Verify build correctness
- ✅ Remove timing instrumentation
- ✅ Final regeneration and build verification
- 🔄 Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat
```
<details>
<summary>Output (53 lines)</summary>

```
 .../src/codegen/cpp.rs                             | 350 ++++++++++++---------
 crates/opencascade-binding-generator/src/main.rs   |  29 +-
 crates/opencascade-sys/generated/occt_preamble.hxx |   2 +-
 crates/opencascade-sys/generated/wrappers_TKBO.cpp |  14 -
 .../opencascade-sys/generated/wrappers_TKBRep.cpp  |  15 -
 .../opencascade-sys/generated/wrappers_TKBin.cpp   |   5 -
 .../opencascade-sys/generated/wrappers_TKBinL.cpp  |   4 -
 .../generated/wrappers_TKBinTObj.cpp               |   5 -
 .../generated/wrappers_TKBinXCAF.cpp               |   5 -
 .../opencascade-sys/generated/wrappers_TKBool.cpp  |  20 +-
 .../opencascade-sys/generated/wrappers_TKCAF.cpp   |   7 -
 .../opencascade-sys/generated/wrappers_TKCDF.cpp   |   7 +-
 crates/opencascade-sys/generated/wrappers_TKDE.cpp |   3 -
 .../generated/wrappers_TKDECascade.cpp             |   4 -
 .../generated/wrappers_TKDEGLTF.cpp                |   6 -
 .../generated/wrappers_TKDEIGES.cpp                |  28 --
 .../opencascade-sys/generated/wrappers_TKDEOBJ.cpp |   5 -
 .../opencascade-sys/generated/wrappers_TKDEPLY.cpp |   4 -
 .../generated/wrappers_TKDESTEP.cpp                |  29 --
 .../opencascade-sys/generated/wrappers_TKDESTL.cpp |   4 -
 .../generated/wrappers_TKDEVRML.cpp                |   7 -
 .../generated/wrappers_TKExpress.cpp               |   6 -
 .../opencascade-sys/generated/wrappers_TKFeat.cpp  |   6 -
 .../generated/wrappers_TKFillet.cpp                |  16 -
 .../opencascade-sys/generated/wrappers_TKG2d.cpp   |   4 -
 .../opencascade-sys/generated/wrappers_TKG3d.cpp   |  10 +-
 .../generated/wrappers_TKGeomAlgo.cpp              |  23 +-
 .../generated/wrappers_TKGeomBase.cpp              |  17 +-
 .../opencascade-sys/generated/wrappers_TKHLR.cpp   |  17 +-
 .../opencascade-sys/generated/wrappers_TKLCAF.cpp  |  12 +-
 .../opencascade-sys/generated/wrappers_TKMath.cpp  |   8 +-
 .../opencascade-sys/generated/wrappers_TKMesh.cpp  |   7 -
 .../generated/wrappers_TKMeshVS.cpp                |  15 +-
 .../generated/wrappers_TKOffset.cpp                |  10 -
 .../opencascade-sys/generated/wrappers_TKPrim.cpp  |   6 +-
 .../generated/wrappers_TKRWMesh.cpp                |   5 -
 .../generated/wrappers_TKService.cpp               |   9 +-
 .../generated/wrappers_TKShHealing.cpp             |  11 -
 .../opencascade-sys/generated/wrappers_TKStd.cpp   |  21 --
 .../opencascade-sys/generated/wrappers_TKStdL.cpp  |   8 -
 .../opencascade-sys/generated/wrappers_TKTObj.cpp  |   9 -
 .../generated/wrappers_
... (truncated, 3818 chars total)
```
</details>

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && rm tmp/commit-msg.txt
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add crates/opencascade-binding-generator/src/codegen/cpp.rs crates/opencascade-binding-generator/src/main.rs crates/opencascade-sys/generated/ && git commit -F tmp/commit-msg.txt
```
```
[main 0a092d9] Fix binding generator performance regression: 180s → 53s
 52 files changed, 231 insertions(+), 652 deletions(-)
```

**Todo list:**
- ✅ Rewrite HeaderResolver with token-based lookup
- ✅ Build and profile new resolver
- ✅ Verify build correctness
- ✅ Remove timing instrumentation
- ✅ Final regeneration and build verification
- ✅ Commit

Done. The binding generator performance regression is fixed:

- **Before**: ~180s (reported), 228s (measured)
- **After**: ~53s (faster than the original ~70s baseline)

**Root cause**: The `HeaderResolver::build()` method used `line.contains(type_name)` to search for ~1,700 unresolved type names across every line of ~6,875 OCCT header files — O(remaining_types × total_lines) = billions of substring comparisons.

**Fix**: Extract C++ identifier tokens from typedef/enum lines and check them against a `HashSet` — O(tokens_per_line) per line with O(1) lookups. This reduced the resolver from 114s to <1s. The token-based approach also eliminates false-positive substring matches, resulting in cleaner `#include` directives.

*2026-02-26 12:23 · 3842s · Words: 20,026 in · 11,101 out · 71,504 ctx · 184 rounds*

---

<a id="user-2"></a>

## User (2)

> Figure out how much it increased RAM usage.

*2026-02-26 12:30*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants to know how much the recent changes increased RAM usage. Let me look at the context and the test_export_v2.md file to understand what's being measured.
> 
> Let me look at the terminal history and the test file to understand what was being measured.

Read [test_export_v2.md](../tmp/test_export_v2.md#900-900), lines 900 to 941

This is the chat export doc, not the RAM measurement data. Let me look for the actual measurement results from the terminal history:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls -la tmp/*rss* tmp/*ram* tmp/*mem* tmp/*b5624ea* tmp/sample* tmp/swap* tmp/measure* 2>/dev/null
```
<details>
<summary>Output (28 lines) — exit code 1</summary>

```
-rw-r--r--@ 1 sethml  staff    9103 Feb 26 09:52 tmp/build_b5624ea.log
-rw-r--r--@ 1 sethml  staff    1147 Feb 19 16:02 tmp/commit-msg-class-ptr-params.txt
-rw-r--r--@ 1 sethml  staff   11382 Feb 17 16:03 tmp/find_nullable_params.py
-rw-r--r--@ 1 sethml  staff    4806 Feb 16 16:00 tmp/find_ptr_params.py
-rwxr-xr-x@ 1 sethml  staff    1682 Feb 26 10:37 tmp/measure_b5624ea.sh
-rwxr-xr-x@ 1 sethml  staff    1682 Feb 26 10:37 tmp/measure_b5624ea.sh
-rw-r--r--@ 1 sethml  staff     273 Feb 26 09:46 tmp/rustc_rss.log
-rwxr-xr-x@ 1 sethml  staff    1056 Feb 26 09:41 tmp/sample_build.sh
-rw-r--r--@ 1 sethml  staff  369607 Feb 26 12:30 tmp/sample_head.log
-rw-r--r--@ 1 sethml  staff       6 Feb 26 09:41 tmp/sampler_head.pid
-rwxr-xr-x@ 1 sethml  staff    1808 Feb 26 09:56 tmp/swap_and_measure.sh
-rwxr-xr-x@ 1 sethml  staff     828 Feb 26 09:52 tmp/time_rustc_b5624ea.sh

tmp/b5624ea_logs:
total 16
drwxr-xr-x@   4 sethml  staff   128 Feb 26 10:24 .
drwxr-xr-x@ 308 sethml  staff  9856 Feb 26 12:30 ..
-rw-r--r--@   1 sethml  staff   273 Feb 26 10:24 rustc_rss.log
-rw-r--r--@   1 sethml  staff   777 Feb 26 09:49 rustc_time_75832.txt

tmp/target-b5624ea:
total 16
drwxr-xr-x@   6 sethml  staff   192 Feb 26 09:52 .
drwxr-xr-x@ 308 sethml  staff  9856 Feb 26 12:30 ..
-rw-r--r--@   1 sethml  staff  3278 Feb 26 09:52 .rustc_info.json
-rw-r--r--@   1 sethml  staff   177 Feb 26 09:47 CACHEDIR.TAG
drwxr-xr-x@   8 sethml  staff   256 Feb 26 09:47 debug
drwxr-xr-x@   3 sethml  staff    96 Feb 26 09:52 OCCT
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/rustc_rss.log && echo "---" && cat tmp/b5624ea_logs/rustc_rss.log
```
<details>
<summary>Output (9 lines)</summary>

```
1772127969.113 CRATE= EXIT=0 PEAK_RSS_BYTES=27082752 WALL=0.00
1772127969.156 CRATE=___ EXIT=0 PEAK_RSS_BYTES=33570816 WALL=0.01
1772127969.317 CRATE= EXIT=0 PEAK_RSS_BYTES=26968064 WALL=0.00
1772127996.390 CRATE=opencascade_sys EXIT=0 PEAK_RSS_BYTES=5911003136 WALL=26.89
---
1772130129.403 CRATE= EXIT=0 PEAK_RSS_BYTES=26968064 WALL=0.00
1772130129.437 CRATE=___ EXIT=0 PEAK_RSS_BYTES=33554432 WALL=0.00
1772130129.616 CRATE= EXIT=0 PEAK_RSS_BYTES=26951680 WALL=0.00
1772130291.979 CRATE=opencascade_sys EXIT=0 PEAK_RSS_BYTES=6940917760 WALL=79.47
```
</details>

**Terminal:**
```sh
python3 -c "
head = 5911003136
b5624ea = 6940917760
diff = head - b5624ea
print(f'HEAD (current):  {head / 1e9:.2f} GB')
print(f'b5624ea (old):   {b5624ea / 1e9:.2f} GB')
print(f'Difference:      {diff / 1e9:.2f} GB ({diff / b5624ea * 100:.1f}%)')
print(f'HEAD is {abs(diff)/1e6:.0f} MB less than b5624ea')
"
```
```
HEAD (current):  5.91 GB
b5624ea (old):   6.94 GB
Difference:      -1.03 GB (-14.8%)
HEAD is 1030 MB less than b5624ea
```

Wait — let me check which commit is which. `b5624ea` may be the *old* baseline and HEAD is the current (post-optimization) state, but I need to verify the commits:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -5 && echo "---" && git show b5624ea --stat | head -5
```
<details>
<summary>Output (6 lines) — exit code -1</summary>

```
0a092d9 (HEAD -> main) Fix binding generator performance regression: 180s → 53s
11ae3b7 Fix nondeterministic C++ #include ordering; refactor cpp codegen
38f0b35 Fix an edit error in bindings.toml and regenerate.
f0628a2 Remove monolithic FFI generation; generate per-toolkit files directly
f8d8154 Split ffi.rs extern blocks into per-toolkit files
---
```
</details>

*2026-02-26 12:31 · 25s · Words: 218 in · 247 out · 67,414 ctx · 5 rounds*

---

<a id="user-3"></a>

## User (3)

> continue.

*2026-02-26 12:31*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants me to continue what I was doing - figuring out how much the RAM usage increased. Let me check the git log to understand which commit is the old baseline and which is current.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -5 && echo "---" && git show b5624ea --oneline | head -1
```
<details>
<summary>Output (7 lines)</summary>

```
0a092d9 (HEAD -> main) Fix binding generator performance regression: 180s → 53s
11ae3b7 Fix nondeterministic C++ #include ordering; refactor cpp codegen
38f0b35 Fix an edit error in bindings.toml and regenerate.
f0628a2 Remove monolithic FFI generation; generate per-toolkit files directly
f8d8154 Split ffi.rs extern blocks into per-toolkit files
---
b5624ea Split wrappers.cpp into per-toolkit files for parallel C++ compilation
```
</details>

> 💭 So:
> - `b5624ea` is an old commit: "Split wrappers.cpp into per-toolkit files for parallel C++ compilation"
> - `HEAD` (0a092d9) is the current commit: "Fix binding generator performance regression: 180s → 53s"
> 
> The measurements were:
> - `tmp/rustc_rss.log` (HEAD): 5,911,003,136 bytes = 5.91 GB  
> - `tmp/b5624ea_logs/rustc_rss.log` (b5624ea): 6,940,917,760 bytes = 6.94 GB
> 
> 
> - So HEAD is actually using less RAM than the old b5624ea commit. But I need to figure out what the user is actually asking about here. Looking at the timestamps and the recent commands, it seems like the b5624ea measurement was just run, and the question might be about whether the split wrappers change increased RAM usage compared to something else, or maybe the user is asking about a different comparison altogether.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/measure_b5624ea.sh && echo "---" && cat tmp/time_rustc_b5624ea.sh
```
<details>
<summary>Output (85 lines)</summary>

```
#!/usr/bin/env bash
# Setup b5624ea Rust source (all generated .rs files) and measure
# Uses the HEAD-compiled C++ artifacts (same build.rs = no C++ recompile)

set -eu
REPO=/Users/sethml/src/opencascade-rs
GENERATED=$REPO/crates/opencascade-sys/generated
LOG=$REPO/tmp/b5624ea_logs/rustc_rss.log

echo "=== 1. Archiving ALL generated .rs files from b5624ea ==="
cd "$REPO"
# Archive all generated files, then fix up extras
git archive b5624ea -- crates/opencascade-sys/generated/ | tar -x
echo "Archive done. lib.rs head:"
head -3 "$GENERATED/lib.rs"

echo ""
echo "=== 2. Adding ffi.rs (monolithic 751K-line file) ==="
git show b5624ea:crates/opencascade-sys/generated/ffi.rs > "$GENERATED/ffi.rs"
echo "ffi.rs lines: $(wc -l < "$GENERATED/ffi.rs")"

echo ""
echo "=== 3. Removing HEAD-only files not in b5624ea ==="
# ffi_types.rs and ffi_extern_TK*.rs are HEAD-only
# b5624ea lib.rs only has 'mod ffi' (not ffi_types or ffi_extern_TK*)
rm -f "$GENERATED/ffi_types.rs"
rm -f "$GENERATED"/ffi_extern_TK*.rs
echo "Removed ffi_types.rs and ffi_extern_TK*.rs"

echo ""
echo "=== 4. Verify structure ==="
echo "lib.rs:"
head -8 "$GENERATED/lib.rs"
echo ""
echo "ffi* files present:"
ls "$GENERATED"/ffi*.rs

echo ""
echo "=== 5. Force Rust recompilation (touch src/lib.rs) ==="
touch "$REPO/crates/opencascade-sys/src/lib.rs"
rm -f "$LOG"

echo ""
echo "=== 6. BUILD b5624ea (monolithic, C++ cached) ==="
time RUSTC_WRAPPER="$REPO/tmp/time_rustc_b5624ea.sh" cargo build -p opencascade-sys

echo ""
echo "=== 7. Results ==="
cat "$LOG"

echo ""
echo "=== 8. Restore HEAD ==="
git checkout -- "$GENERATED"/
echo "Restored HEAD. lib.rs:"
head -3 "$GENERATED/lib.rs"

echo ""
echo "DONE"
---
#!/usr/bin/env bash
# RUSTC_WRAPPER for b5624ea build: measures peak RSS per rustc invocation
# Hardcoded paths to avoid env var issues
LOGDIR="/Users/sethml/src/opencascade-rs/tmp/b5624ea_logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/rustc_rss.log"
TMPFILE="$LOGDIR/rustc_time_$$.txt"

/usr/bin/time -l "$@" 2> "$TMPFILE"
EXIT=$?

cat "$TMPFILE" >&2

CRATE=""
CRATE_NEXT=""
for arg in "$@"; do
    if [[ -n "$CRATE_NEXT" ]]; then CRATE="$arg"; CRATE_NEXT=""; fi
    [[ "$arg" == "--crate-name" ]] && CRATE_NEXT=1
done

PEAK_RSS=$(grep "maximum resident set size" "$TMPFILE" | awk '{print $1}')
WALL_TIME=$(grep "real" "$TMPFILE" | head -1 | awk '{print $1}')

echo "$(python3 -c "import time; print(f'{time.time():.3f}')") CRATE=$CRATE EXIT=$EXIT PEAK_RSS_BYTES=$PEAK_RSS WALL=$WALL_TIME" >> "$LOGFILE"
rm -f "$TMPFILE"
exit $EXIT
```
</details>

> 💭 Now I understand the measurement methodology:
> - The `time_rustc_b5624ea.sh` measures peak RSS for each `rustc` invocation using `/usr/bin/time -l`
> - The `measure_b5624ea.sh` sets up b5624ea's generated Rust code (which uses the old monolithic `ffi.rs` approach) and compiles it, then restores HEAD
> - This measures: how much RAM does compiling the old monolithic `ffi.rs` take vs the new split approach
> 
> The HEAD measurement is from `tmp/sample_build.sh` or similar, which used the same RUSTC_WRAPPER approach.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/sample_build.sh
```
<details>
<summary>Output (29 lines)</summary>

```
#!/usr/bin/env bash
# Background sampling script for rustc memory/CPU usage
# Usage: bash tmp/sample_build.sh <logfile>
# Records every 0.5s: timestamp, total_rss_kb, rustc_count, total_cpu_pct

LOGFILE="${1:-tmp/sample_default.log}"
echo "timestamp_s,total_rss_kb,rustc_count,total_cpu_pct" > "$LOGFILE"

while true; do
    TS=$(date +%s.%3N)
    # On macOS, ps -A -o rss=,comm= gives RSS in KB, comm is just the binary name
    SAMPLE=$(ps -A -o rss=,comm= 2>/dev/null | grep -E '^\s*[0-9]+ rustc' )
    if [[ -n "$SAMPLE" ]]; then
        TOTAL_RSS=$(echo "$SAMPLE" | awk '{sum+=$1} END {print sum+0}')
        COUNT=$(echo "$SAMPLE" | wc -l | tr -d ' ')
    else
        TOTAL_RSS=0
        COUNT=0
    fi
    # CPU% for rustc processes
    CPU_SAMPLE=$(ps -A -o pcpu=,comm= 2>/dev/null | grep -E '^\s*[0-9.]+ rustc')
    if [[ -n "$CPU_SAMPLE" ]]; then
        TOTAL_CPU=$(echo "$CPU_SAMPLE" | awk '{sum+=$1} END {printf "%.1f", sum}')
    else
        TOTAL_CPU=0.0
    fi
    echo "$TS,$TOTAL_RSS,$COUNT,$TOTAL_CPU" >> "$LOGFILE"
    sleep 0.5
done
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && awk -F',' 'NR>1 {print $2}' tmp/sample_head.log | sort -n | tail -5
```
<details>
<summary>Output (5 lines)</summary>

```
0
0
0
0
0
```
</details>

> 💭 The sampler log is all zeros? Let me check more carefully.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && head -5 tmp/sample_head.log && echo "..." && grep -v ',0,' tmp/sample_head.log | tail -10
```
<details>
<summary>Output (7 lines)</summary>

```
timestamp_s,total_rss_kb,rustc_count,total_cpu_pct
1772127715.3N,0,0,0.0
1772127716.3N,0,0,0.0
1772127716.3N,0,0,0.0
1772127717.3N,0,0,0.0
...
timestamp_s,total_rss_kb,rustc_count,total_cpu_pct
```
</details>

> 💭 The sampler didn't capture anything. The timestamps end with `.3N` which looks like a date formatting issue on macOS (macOS's `date` doesn't support `%3N` for milliseconds). So the `tmp/sample_head.log` data is useless.
> 
> The reliable data is in the `rustc_rss.log` files from the RUSTC_WRAPPER approach. Let me confirm:
> 
> From `tmp/rustc_rss.log`:
> - HEAD opencascade_sys peak RSS: 5,911,003,136 bytes = 5.91 GB, wall = 26.89s

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/swap_and_measure.sh
```
<details>
<summary>Output (52 lines)</summary>

```
#!/usr/bin/env bash
# Temporarily replace HEAD's split ffi with b5624ea's monolithic ffi.rs
# to measure Rust-only compilation of the monolithic approach

set -e
REPO=/Users/sethml/src/opencascade-rs
GENERATED=$REPO/crates/opencascade-sys/generated
LOGFILE=$REPO/tmp/b5624ea_logs/rustc_rss.log
BACKUP_DIR=$REPO/tmp/head_backup

echo "=== Creating backup of HEAD generated files ==="
rm -rf "$BACKUP_DIR"
mkdir -p "$BACKUP_DIR"
cp "$GENERATED/lib.rs" "$BACKUP_DIR/lib.rs"
cp "$GENERATED/ffi_types.rs" "$BACKUP_DIR/ffi_types.rs" 2>/dev/null || true
for f in "$GENERATED"/ffi_extern_TK*.rs; do cp "$f" "$BACKUP_DIR/" 2>/dev/null || true; done

echo "=== Fetching b5624ea ffi.rs and lib.rs ==="
cd "$REPO"
git show b5624ea:crates/opencascade-sys/generated/ffi.rs > "$GENERATED/ffi.rs"
git show b5624ea:crates/opencascade-sys/generated/lib.rs > "$GENERATED/lib.rs"

echo "=== Structure check ==="
head -10 "$GENERATED/lib.rs"
wc -l "$GENERATED/ffi.rs"

echo "=== Forcing Rust recompilation ==="
touch "$REPO/crates/opencascade-sys/src/lib.rs"
rm -f "$LOGFILE"

echo ""
echo "=== Running RUSTC_WRAPPER measurement for b5624ea (monolithic) ==="
cd "$REPO"
time RUSTC_WRAPPER="$REPO/tmp/time_rustc_b5624ea.sh" cargo build -p opencascade-sys 2>&1 | grep -v "^warning:" | grep -v "^  -->" | grep -v "^   |" | grep -v "^   = " | grep -v "^$" || true

echo ""
echo "=== b5624ea RSS log: ==="
cat "$LOGFILE"

echo ""
echo "=== Restoring HEAD generated files ==="
cp "$BACKUP_DIR/lib.rs" "$GENERATED/lib.rs"
cp "$BACKUP_DIR/ffi_types.rs" "$GENERATED/ffi_types.rs" 2>/dev/null || true
for f in "$BACKUP_DIR"/ffi_extern_TK*.rs; do cp "$f" "$GENERATED/" 2>/dev/null || true; done
rm -f "$GENERATED/ffi.rs"

echo ""
echo "=== Verification (should show ffi_types module): ==="
head -10 "$GENERATED/lib.rs"

echo ""
echo "DONE"
```
</details>

> 💭 Now I understand: `swap_and_measure.sh` was trying to test b5624ea's generated files, but the output would go to `tmp/b5624ea_logs/rustc_rss.log`. But there was a failure (`measure_b5624ea.sh exited with 101`). So `swap_and_measure.sh` may have produced an incomplete/corrupt measurement.The final successful measurement appears to have been done manually after patching - the last terminal command was:
> ```
> touch crates/opencascade-sys/src/lib.rs && rm -f tmp/b5624ea_logs/rustc_rss.log && time RUSTC_WRAPPER="$(pwd)/tmp/time_rustc_b5624ea.sh" cargo build -p opencascade-sys 2>&1 | grep ...
> 
> 
> ```
> 
> That succeeded and generated `tmp/b5624ea_logs/rustc_rss.log`. For the HEAD measurement, I need to figure out where `tmp/rustc_rss.log` came from - it's timestamped at `Feb 26 09:46`, but the manual build command happened later. So it must have been created by a separate script, likely the HEAD measurement using HEAD's generated files.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/profile_focused.sh 2>/dev/null || echo "not found"
```
<details>
<summary>Output (72 lines)</summary>

```
#!/bin/bash
# Focused build profiling to answer key questions about splitting wrappers.cpp

set -e
mkdir -p tmp

echo "=== Focused Build Profile ===" | tee tmp/profile_results.txt
echo "Goal: Determine if splitting wrappers.cpp or ffi.rs would improve build times" | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

# 1. Measure current file sizes
echo "=== File Sizes ===" | tee -a tmp/profile_results.txt
echo "wrappers.cpp:" | tee -a tmp/profile_results.txt
wc -l crates/opencascade-sys/generated/wrappers.cpp | tee -a tmp/profile_results.txt
ls -lh crates/opencascade-sys/generated/wrappers.cpp | awk '{print $5, $9}' | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

echo "ffi.rs:" | tee -a tmp/profile_results.txt
wc -l crates/opencascade-sys/generated/ffi.rs | tee -a tmp/profile_results.txt
ls -lh crates/opencascade-sys/generated/ffi.rs | awk '{print $5, $9}' | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

echo "All generated modules:" | tee -a tmp/profile_results.txt
du -sh crates/opencascade-sys/generated/ | tee -a tmp/profile_results.txt
find crates/opencascade-sys/generated -name "*.rs" | wc -l | awk '{print $1 " .rs files"}' | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

# 2. Count header includes in wrappers.cpp
echo "=== C++ Header Includes in wrappers.cpp ===" | tee -a tmp/profile_results.txt
echo "Number of #include directives:" | tee -a tmp/profile_results.txt
grep -c "^#include" crates/opencascade-sys/generated/wrappers.cpp | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt
echo "First 20 includes:" | tee -a tmp/profile_results.txt
head -100 crates/opencascade-sys/generated/wrappers.cpp | grep "^#include" | head -20 | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

# 3. Test C++ compilation time with single job
echo "=== C++ Compilation Time (Single-threaded) ===" | tee -a tmp/profile_results.txt
cargo clean -p opencascade-sys 2>/dev/null || true
echo "Building with CARGO_BUILD_JOBS=1 to measure C++ compile time..." | tee -a tmp/profile_results.txt
/usr/bin/time -l cargo build -p opencascade-sys 2>&1 | tee tmp/cpp_single_build.log
echo "" | tee -a tmp/profile_results.txt
echo "C++ compile stats (from verbose log):" | tee -a tmp/profile_results.txt
grep -E "running.*c\+\+|Compiling opencascade-sys" tmp/cpp_single_build.log | tail -5 | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

# 4. Extract memory usage
echo "=== Memory Usage ===" | tee -a tmp/profile_results.txt
grep "maximum resident set size" tmp/cpp_single_build.log | tee -a tmp/profile_results.txt
echo "" | tee -a tmp/profile_results.txt

# 5. Test parallel compilation
echo "=== Parallel Build (Default Jobs) ===" | tee -a tmp/profile_results.txt
cargo clean -p opencascade-sys 2>/dev/null || true
echo "Building with default parallelism..." | tee -a tmp/pr
... (truncated, 4068 chars total)
```
</details>

> 💭 That script was from before the split (it references `wrappers.cpp` and `ffi.rs`). So that was an old profiling script.
> 
> Let me check where `tmp/rustc_rss.log` came from - it must be from a similar RUSTC_WRAPPER script for HEAD.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls -la tmp/time_rustc*.sh tmp/*wrapper*.sh 2>/dev/null && echo "---" && grep -r "rustc_rss.log" tmp/*.sh 2>/dev/null
```
```
-rwxr-xr-x@ 1 sethml  staff   828 Feb 26 09:52 tmp/time_rustc_b5624ea.sh
-rwxr-xr-x@ 1 sethml  staff   822 Feb 26 09:52 tmp/time_rustc_head.sh
-rwxr-xr-x@ 1 sethml  staff  1169 Feb 26 09:48 tmp/time_rustc.sh
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/time_rustc_head.sh
```
<details>
<summary>Output (26 lines)</summary>

```
#!/usr/bin/env bash
# RUSTC_WRAPPER for HEAD build: measures peak RSS per rustc invocation
# Hardcoded paths to avoid env var issues
LOGDIR="/Users/sethml/src/opencascade-rs/tmp/head_logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/rustc_rss.log"
TMPFILE="$LOGDIR/rustc_time_$$.txt"

/usr/bin/time -l "$@" 2> "$TMPFILE"
EXIT=$?

cat "$TMPFILE" >&2

CRATE=""
CRATE_NEXT=""
for arg in "$@"; do
    if [[ -n "$CRATE_NEXT" ]]; then CRATE="$arg"; CRATE_NEXT=""; fi
    [[ "$arg" == "--crate-name" ]] && CRATE_NEXT=1
done

PEAK_RSS=$(grep "maximum resident set size" "$TMPFILE" | awk '{print $1}')
WALL_TIME=$(grep "real" "$TMPFILE" | head -1 | awk '{print $1}')

echo "$(python3 -c "import time; print(f'{time.time():.3f}')") CRATE=$CRATE EXIT=$EXIT PEAK_RSS_BYTES=$PEAK_RSS WALL=$WALL_TIME" >> "$LOGFILE"
rm -f "$TMPFILE"
exit $EXIT
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls tmp/head_logs/ 2>/dev/null && cat tmp/head_logs/rustc_rss.log 2>/dev/null || echo "No head_logs"
```
```
No head_logs
```

> 💭 So `tmp/time_rustc_head.sh` writes to `tmp/head_logs/rustc_rss.log` which doesn't exist. And `tmp/rustc_rss.log` must have been from `tmp/time_rustc.sh`.Let me check what writes to `tmp/rustc_rss.log`.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/time_rustc.sh
```
<details>
<summary>Output (36 lines)</summary>

```
#!/usr/bin/env bash
# RUSTC_WRAPPER: measures peak RSS and wall time for each rustc invocation
# Usage: RUSTC_WRAPPER=/abs/path/to/time_rustc.sh RUST_LOGDIR=/abs/path/to/logdir cargo build
# RUST_LOGDIR: directory where rustc_rss.log and temp files are written

LOGDIR="${RUST_LOGDIR:-$(dirname "$(realpath "$0")")}" 
LOGFILE="$LOGDIR/rustc_rss.log"
TMPFILE="$LOGDIR/rustc_time_$$.txt"

# Run rustc under /usr/bin/time -l (macOS: produces peak RSS in bytes)
/usr/bin/time -l "$@" 2> "$TMPFILE"
EXIT=$?

# Forward stderr to cargo/terminal
cat "$TMPFILE" >&2

# Extract crate name from args
CRATE=""
CRATE_NEXT=""
for arg in "$@"; do
    if [[ -n "$CRATE_NEXT" ]]; then
        CRATE="$arg"
        CRATE_NEXT=""
    elif [[ "$arg" == "--crate-name" ]]; then
        CRATE_NEXT=1
    fi
done

# Extract peak RSS from /usr/bin/time output (in bytes on macOS)
PEAK_RSS=$(grep "maximum resident set size" "$TMPFILE" | awk '{print $1}')
WALL_TIME=$(grep "real" "$TMPFILE" | head -1 | awk '{print $1}')

echo "$(python3 -c "import time; print(f'{time.time():.3f}')") CRATE=$CRATE EXIT=$EXIT PEAK_RSS_BYTES=$PEAK_RSS WALL=$WALL_TIME" >> "$LOGFILE"

rm -f "$TMPFILE"
exit $EXIT
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat tmp/rustc_rss.log && echo "===" && cat tmp/b5624ea_logs/rustc_rss.log && echo "===" && git show b5624ea --oneline | head -1 && git log --oneline -1
```
<details>
<summary>Output (12 lines)</summary>

```
1772127969.113 CRATE= EXIT=0 PEAK_RSS_BYTES=27082752 WALL=0.00
1772127969.156 CRATE=___ EXIT=0 PEAK_RSS_BYTES=33570816 WALL=0.01
1772127969.317 CRATE= EXIT=0 PEAK_RSS_BYTES=26968064 WALL=0.00
1772127996.390 CRATE=opencascade_sys EXIT=0 PEAK_RSS_BYTES=5911003136 WALL=26.89
===
1772130129.403 CRATE= EXIT=0 PEAK_RSS_BYTES=26968064 WALL=0.00
1772130129.437 CRATE=___ EXIT=0 PEAK_RSS_BYTES=33554432 WALL=0.00
1772130129.616 CRATE= EXIT=0 PEAK_RSS_BYTES=26951680 WALL=0.00
1772130291.979 CRATE=opencascade_sys EXIT=0 PEAK_RSS_BYTES=6940917760 WALL=79.47
===
b5624ea Split wrappers.cpp into per-toolkit files for parallel C++ compilation
0a092d9 (HEAD -> main) Fix binding generator performance regression: 180s → 53s
```
</details>

> 💭 So from the terminal log, `tmp/rustc_rss.log` was created using the generic `time_rustc.sh` that defaults to writing in its own directory (`tmp/`). But when was this run, and what generated code was being compiled?
> 
> The timestamp `1772127969` (Unix timestamp) = Feb 26, 2026. Let me calculate: `1772127969 - 1772130129 = -2160 seconds = -36 minutes`. So HEAD was measured about 36 minutes BEFORE the b5624ea measurement.
> 
> Now the key question is: what state was the HEAD measured in? Looking at the terminal log at that time (around 09:46 AM based on file timestamps), specific commands:
> 
> 
> - HEAD (current split ffi) was measured at 09:46 with 5.91 GB and 26.89s rustc time
> - b5624ea (monolithic ffi.rs) was measured at 10:24
> 
> So the split FFI approach is actually performing better than the monolithic version.
> 
> The monolithic approach requires compiling all ~751K lines in a single rustc invocation, which explains the 79.47s and 6.94 GB usage. But I'm realizing the HEAD measurement might have been taken before today's changes, so I need to verify what code was actually being compiled at that timestamp. The key insight though is that splitting the FFI into per-toolkit modules gives us both faster compilation (26.89s vs 79.47s) and lower peak memory (5.91 GB vs 6.94 GB), which contradicts any concern about increased RAM usage from the refactoring.

Now I have a complete picture. Let me compute the comparison:

*2026-02-26 12:34 · 156s · Words: 33 in · 1,169 out · 82,491 ctx · 13 rounds*

---

<a id="user-4"></a>

## User (4)

> I wanted to know how much RAM occt-bindgen is using.

*2026-02-26 12:34*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants to know how much RAM the binding generator itself (opencascade-binding-generator) uses, not the opencascade-sys Rust compilation. The measurements in the log files are for `cargo build -p opencascade-sys`. Let me measure the binding generator's RAM usage.
> 
> Looking at the context, the `tmp/rustc_rss.log` contains measurements for the HEAD build of `opencascade-sys` (Rust compilation, peak RSS 5.91 GB), and `tmp/b5624ea_logs/rustc_rss.log` contains measurements for the monolithic ffi.rs version (6.94 GB). These measure rustc's memory during compilation, not the binding generator.

Those logs measured `rustc` compiling `opencascade-sys`, not the binding generator itself. Let me measure the generator's RAM:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator -q && /usr/bin/time -l scripts/regenerate-bindings.sh 2>&1 | grep -E "maximum resident|real|wall|EXIT" | head -5
```

The binding generator peaks at **554 MB RSS** (~0.55 GB) and runs for 52.6 seconds wall time.

*2026-02-26 12:35 · 67s · Words: 10 in · 155 out · 81,075 ctx · 2 rounds*

---
