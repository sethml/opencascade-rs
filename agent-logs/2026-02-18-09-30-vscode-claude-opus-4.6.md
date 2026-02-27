# Fixing parsing issues with type hints in bindings.toml

- **Date:** 2026-02-18 08:40 – 09:07
- **Model:** Claude Opus 4.6
- **Turns:** 1
- **Tool calls:** 177
- **Thinking blocks:** 18
- **Input words:** 45,090
- **Output words:** 8,625
- **Prompt tokens (last round):** 117,944
- **API rounds:** 107
- **Total elapsed:** 1540s

## Table of Contents

1. [These issues all seem similar. Come up with a fix. If we need some type hints in bindings.toml to...](#user-1) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> These issues all seem similar. Come up with a fix. If we need some type hints in bindings.toml to guide parsing of the problematic types, that would be an OK solution. Regenerate, test, and commit.

*2026-02-18 08:41*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to fix the issues with headers that are excluded because of NCollection template parameter misresolution in batch parsing mode. Let me first understand the specific problems:
> 
> 1. `Message_Report.hxx` - `ActiveMetrics()` return type `NCollection_IndexedMap<Message_MetricType>` misresolved to `Standard_Integer`
> 2. `math_Matrix.hxx` - methods reference `math_Vector` (typedef of `math_VectorBase<>`) misresolved to `int`
> 3. `TObj_TIntSparseArray.hxx` - `GetIterator()` returns `NCollection_SparseArray::ConstIterator` misresolved to `Standard_Integer`
> 
> 
> 4. `XSAlgo_ShapeProcessor.hxx` - constructor takes `NCollection_DataMap` param misresolved to `int`; `ReadProcessingData` returns `std::pair<DataMap, bitset>` misresolved to `int`
> 
> All of these stem from the same issue: clang's batch parsing mode incorrectly resolves template types to primitive types. The current solution handles NCollection typedefs, but these cases involve return types and parameters that might need a different approach. I should examine the binding generator code to see how we're currently handling this and figure out where to add type hints.

**Todo list:**
- 🔄 Understand current misresolution handling
- ⬜ Examine problematic OCCT headers
- ⬜ Design type hints solution
- ⬜ Implement type hints in generator
- ⬜ Add type hints to bindings.toml
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

Research binding generator type resolution

**Terminal:**
```sh
wc -l /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/config.rs /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md
```
<details>
<summary>Output (5 lines)</summary>

```
    1609 /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs
     125 /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/config.rs
    5067 /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs
     450 /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md
    7251 total
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/parser.rs">parser.rs</a></summary>

```
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

thread_local! {
    /// Map from NCollection template display names to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: normalized template spelling (whitespace-stripped), e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: typedef name, e.g. "TDF_LabelMap"
    static TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


/// Walk the AST to collect all typedef declarations that resolve to NCollection
/// template specializations. Populates the thread-local TYPEDEF_MAP.
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        // Only record if the typedef name looks like an OCCT type
                        // (starts with uppercase, contains underscore)
                        if name.starts_with(|c: char| c.is_ascii_uppercase())
                            && name.contains('_')
                        {
                            map.insert(key, name.clone());
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    eprintln!("  Collected {} NCollection/template typedefs", map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Look up a type's display name in the typedef map.
/// Returns the typedef name if found.
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() ->
... (truncated, 62412 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/config.rs">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Expand the configuration into a list of header file paths.
///
/// - For each module in `modules`, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Expand modules: find all headers matching {Module}.hxx and {Module}_*.hxx
    for module in &config.modules {
        let mut module_headers = Vec::new();

        // Look for {Module}.hxx
        let main_header = format!("{}.hxx", module);
        let main_path = occt_include_dir.join(&main_header);
        if main_path.exists() {
            module_headers.push((main_header.clone(), main_path));
        }

        // Look for {Module}_*.hxx
        let prefix = format!("{}_", module);
        let entries = std::fs::read_dir(occt_include_dir)
            .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".hxx") {
                module_headers.push((filename, entry.path()));
            }
        }

        module_headers.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, path) in module_headers {
            if seen.insert(name) {
                headers.push(path);
            }
        }
    }

    // 2. Add individual headers
    for header_name in &config.include_headers {
        if seen.insert(header_name.clone()) {
            let path = occt_include_dir.join(header_name);
            if path.exists() {
                headers.push(path);
            } else {
                eprintln!("Warning: Header not found: {}", path.display());
            }
        }
    }

    // 3. Remove excluded headers
    if !config.exclude_headers.is_empty() {
        let exclude_set: std::collections::HashSet<&str> =
            config.exclude_headers.iter().map(|s| s.as_str()).collect();
        headers.reta
... (truncated, 4177 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/README.md">README.md</a></summary>

````
# opencascade-binding-generator

Automatically generates Rust `extern "C"` FFI bindings for the [OpenCASCADE](https://dev.opencascade.org/) C++ CAD kernel.

Given a set of OCCT header files, the generator produces a complete Rust FFI layer: type declarations, method bindings, constructor wrappers, Handle smart pointer support, collection iterators, inheritance upcasts, and per-module re-exports with ergonomic short names.

## Quick Start

```bash
# Regenerate all bindings (from repo root)
./scripts/regenerate-bindings.sh
```

This parses OCCT headers configured in `bindings.toml` (expanding with automatic dependency resolution), and writes generated code to `crates/opencascade-sys/generated/`.

## CLI Usage

```bash
# Standard regeneration (from repo root):
./scripts/regenerate-bindings.sh

# Manual invocation with TOML config:
cargo run -p opencascade-binding-generator -- \
    --config crates/opencascade-sys/bindings.toml \
    -I target/OCCT/include \
    -o crates/opencascade-sys/generated

# Or with explicit header arguments (legacy):
cargo run -p opencascade-binding-generator -- \
    -I target/OCCT/include \
    -o crates/opencascade-sys/generated \
    target/OCCT/include/gp_Pnt.hxx target/OCCT/include/TopoDS_Shape.hxx ...
```

**Flags:**
- `--config <file>` — TOML configuration file specifying headers (recommended)
- `--resolve-deps` — Auto-include header dependencies (default: true)
- `--dump-symbols` — Dump symbol table for debugging
- `--dry-run` — Parse without generating
- `-v, --verbose` — Verbose output
- `--module <name>` — Filter to a specific module

## Generated Public API

Users interact with `opencascade-sys` through per-module re-exports. Each OCCT module (gp, TopoDS, BRepPrimAPI, etc.) becomes a Rust module with short type names and idiomatic method signatures.

### Geometry Primitives

Simple value types like points, vectors, and directions get constructors, accessors, and transformation methods. All methods go through extern "C" C++ wrappers. Methods that return class types by value return `OwnedPtr<T>`.

```rust
use opencascade_sys::gp::{Pnt, Vec, Dir, Ax1, Ax2, Trsf};

// Constructors -- overloads distinguished by parameter type suffix
let origin = Pnt::new();                          // default constructor
let p = Pnt::new_real3(1.0, 2.0, 3.0);           // from coordinates
let p2 = Pnt::new_xyz(&xyz);                      // from XYZ

// Direct accessors
let x: f64 = p.x();
let y: f64 = p.y();
p.set_x(10.0);

// Transformations return OwnedPtr (by-value return needs C++ wrapper)
let mirrored: OwnedPtr<Pnt> = p.mirrored_pnt(&origin);
let rotated = p.rotated(&axis, std::f64::consts::FRAC_PI_2);
let transformed = p.transformed(&trsf);

// Copy into a new OwnedPtr
let p_copy = p.to_owned();
```

### Topology Types and Inheritance

Topological shapes form an inheritance hierarchy: `Shape` is the base, with subtypes `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`, `CompSolid`, and `Compound`. Extern "C" FFI treats these as unrelated opaque types, so the generator produces explicit upcast methods (both const and mutable) and downcast free functions.

```rust
use opencascade_sys::topo_ds::{Shape, Edge, Face, Solid, Wire};

// Upcasting -- every subtype can upcast to its ancestors
let edge = Edge::new();
let shape_ref: &Shape = edge.as_shape();                    // const upcast
let shape_mut: &mut Shape = edge.as_shape_mut();            // mutable upcast

// Downcasting -- free functions in the topo_ds module
// (wraps OCCT's TopoDS::Edge(), TopoDS::Face(), etc.)
use opencascade_sys::topo_ds;
let edge: OwnedPtr<Edge> = topo_ds::edge(&some_shape);       // const
let face: OwnedPtr<Face> = topo_ds::face_mut(&mut shape);    // mutable

// Copy semantics (TopoDS shapes are reference-counted internally)
let shape_copy = shape.to_owned();
```

### Primitive Shape Construction (Builder Pattern)

OCCT builder classes follow a pattern: construct with parameters, then extract the result by upcasting to `MakeShap
... (truncated, 20285 chars)
````
</details>

<details>
<summary>Reading lines 1-800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1-L801">bindings.rs</a></summary>

```
//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type};
use crate::module_graph;
use crate::parser;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

/// Rust keywords that need suffix escaping (FFI doesn't support raw identifiers).
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    /// Whether Handle_get/get_mut should be generated (true for all handle types, including abstract)
    pub has_handle_get: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub handle_downcasts: Vec<HandleDowncastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
    /// Whether this class is a POD struct that can be represented with real fields
    pub is_pod_struct: bool,
    /// Fields for POD structs (only populated when is_pod_struct is true)
    pub pod_fields: Vec<PodFieldBinding>,
}

/// A single field in a POD struct.
#[derive(Debug, Clone)]
pub struct PodFieldBinding {
    /// Field name in Rust (snake_case)
    pub rust_name: String,
    /// Field name in C++ (original)
    pub cpp_name: String,
    /// Rust type string, e.g. "bool" or "f64"
    pub rust_type: String,
    /// Array size if this is a fixed-size array field
    pub array_size: Option<usize>,
    /// Byte offset for offsetof check
    pub offset_index: usize,
    /// Doc comment
    pub doc_comment: Option<String>,
}

/// A constructor that will have a C++ wrapper (std::make_unique),
/// or a Rust-only convenience wrapper that delegates to a full-argument constructor.
#[derive(Debug, Clone)]
pub struct ConstructorBinding {
    /// FFI function name, e.g. "gp_Pnt_ctor_real3"
    pub ffi_fn_name: String,
    /// Impl method name in re-export, e.g. "new_real3"
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// C++ argument expressions for calling the constructor
    pub cpp_arg_exprs: Vec<String>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// If this is a convenience wrapper (fewer params
... (truncated, 32939 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Rq1t5L9iZGuKaDwLaUcBTD__vscode-1771312781571/content.txt#1-1), lines 1 to 1609

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_019BfRS5C582wo5No8whYv2D__vscode-1771312781573/content.txt#1-1), lines 1 to 450

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01WVWZR4sGfZkBkPGPcZWokP__vscode-1771312781574/content.txt)

<details>
<summary>Reading lines 800-1200 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800-L1201">bindings.rs</a></summary>

```
    let returns_cstring = method
        .return_type
        .as_ref()
        .map(|t| t.is_c_string())
        .unwrap_or(false);
    let returns_by_value = method.return_type.as_ref().map_or(false, |ty| {
        let is_class_or_handle = ty.is_class() || ty.is_handle();
        let is_enum = match ty {
            Type::Class(name) => all_enums.contains(name),
            _ => false,
        };
        is_class_or_handle && !is_enum
    });

    if returns_by_value {
        WrapperKind::ByValueReturn
    } else if has_cstring_param {
        WrapperKind::CStringParam
    } else if returns_cstring {
        WrapperKind::CStringReturn
    } else if resolver::has_const_mut_return_mismatch(method) {
        WrapperKind::ConstMutReturnFix
    } else if has_mut_ref_enum_params(&method.params, all_enums) {
        WrapperKind::MutRefEnumParam
    } else if resolver::method_uses_enum(method, all_enums) {
        WrapperKind::EnumConversion
    } else if has_by_value_class_or_handle_params(&method.params, all_enums) {
        WrapperKind::ByValueParam
    } else {
        // Simple method wrapper (primitives, void, etc.)
        WrapperKind::Simple
    }
}

/// Convert a Type to C++ type string
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "Standard_Real".to_string(),
        Type::ConstRef(inner) => format!("const {}&", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}&", type_to_cpp(inner)),
        Type::RValueRef(inner) => format!("{}&&", type_to_cpp(inner)),
        Type::ConstPtr(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutPtr(inner) => format!("{}*", type_to_cpp(inner)),
        Type::Handle(name) => format!("opencascade::handle<{}>", name),
        Type::Class(name) => name.clone(),
    }
}


/// Convert a Type to C++ parameter type for extern "C" wrapper functions (pointers instead of references)
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "const char*".to_string()
        }
        _ => type_to_cpp(ty),
    }
}

/// Convert a parameter to C++ argument expression for extern "C" wrappers.
/// Dereferences pointers to match C++ method signatures (which take references).
fn param_to_cpp_extern_c_arg(param_name: &str, ty: &Type) -> String {
    match ty {
        Type::ConstRef(_) | Type::MutRef(_) => format!("*{}", param_name),
        _ => param_name.to_string(),
    }
}

/// Context for resolving C++ class names to their public re-exported Rust type
/// paths instead of raw `crate::ffi::` paths.
pub struct ReexportTypeContext<'a> {
    /// Maps C++ class name (original, may contain ::) → (rust_module_name, short_name)
    pub class_public_info: &'a HashMap<String, (String, String)>,
    /// The Rust module name of the class/function currently being generated
    pub current_module_rust: String,
}

impl<'a> ReexportTypeContext<'a> {
    fn resolve_class(&self, cpp_name: &str) -> String {
        if let Some((module_rust, short)) = self.class_public_info.get(cpp_name) {
            if *module_rust == self.current_module_rust {
                short.clone()
            } else {
                format!("crate::{}::{}", module_rust, short)
            }
        } else {
            format!("crate::ffi::{}", Type::ffi_s
... (truncated, 15468 chars)
```
</details>

Searched for text `is_constructor_bindable` (`**/crates/opencascade-binding-generator/**`), 3 results
- [bindings.rs:1055](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1055)
- [bindings.rs:2068](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2068)
- [bindings.rs:2082](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2082)

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_019y9fmMoYJHCLNZ1XfA1sLV__vscode-1771312781578/content.txt)

<details>
<summary>Reading lines 1900-2200 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1900-L2201">bindings.rs</a></summary>

```
}

// ── Constructor bindings ─────────────────────────────────────────────────


/// Adapt a C++ default value expression to be valid for the corresponding Rust type.
///
/// C++ allows implicit conversions (e.g., `0` for `double`, `0` for `nullptr`).
/// This function returns `None` if the default can't be properly expressed in Rust.
fn adapt_default_for_rust_type(default_expr: &str, param_type: &Type) -> Option<String> {
    // Unwrap references since the default applies to the underlying type
    let inner_type = match param_type {
        Type::ConstRef(inner) | Type::MutRef(inner) => inner.as_ref(),
        _ => param_type,
    };

    match inner_type {
        Type::Bool => {
            // Bool defaults should already be "true" or "false"
            match default_expr {
                "true" | "false" => Some(default_expr.to_string()),
                "0" => Some("false".to_string()),
                "1" => Some("true".to_string()),
                _ => None,
            }
        }
        Type::F64 | Type::F32 => {
            // C++ allows integer literals for floating types (e.g., `0` for `0.0`)
            if default_expr.contains('.') {
                Some(default_expr.to_string())
            } else if let Ok(_) = default_expr.parse::<i64>() {
                Some(format!("{}.0", default_expr))
            } else {
                None
            }
        }
        Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::Long | Type::ULong | Type::Usize => {
            // Integer literals should work directly
            if default_expr.parse::<i64>().is_ok() || default_expr.parse::<u64>().is_ok() {
                Some(default_expr.to_string())
            } else {
                None
            }
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            // const char* defaults — `0`/`nullptr` means null pointer, not expressible as &str
            None
        }
        _ => {
            // For other types (classes, handles, etc.), we can't express defaults
            None
        }
    }
}

/// A constructor, possibly with trailing defaulted params trimmed.
struct TrimmedConstructor<'a> {
    original: &'a Constructor,
    /// How many params to include (may be less than original.params.len())
    trimmed_param_count: usize,
    /// If this is a convenience wrapper, the index of the full-argument parent
    /// in the regular_ctors vec, plus that parent's trimmed_param_count.
    convenience_parent: Option<(usize, usize)>,
}

/// Check if a slice of params passes all bindability filters.
fn is_params_bindable(
    params: &[Param],
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> bool {
    // By-value class/handle params are now supported via C++ wrappers (const T& conversion).
    if params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr()) {
        return false;
    }
    if params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    // Check for unknown class types
    if params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    true
}

/// Compute overload suffix for a param slice (used for trimmed constructors).
/// Combine a base name with an overload suffix, avoiding double underscores.
/// If base_name ends with '_' (e.g. keyword-escaped "type_") and suffix starts with '_',
/// we merge them to avoid "type__suffix" → "type_suffix" instead.
fn combine_name_suffix(base: &str, suffix: &str) -> String {
    if base.ends_with('_') && suffix.starts_with('_') {
        format!("{}{}", base, &suffix[1..])
    } else {
        format!("{}{}", base, suffix)
    }
}

fn overload_suffix_for_params(params: &[Param]) -> String {
    let types: Vec<Type> = params.iter().map(|p| p.ty.clone()).collect();
    overload_suffix_for_types(&types)
}

/// 
... (truncated, 11577 chars)
```
</details>

<details>
<summary>Searched for regex <code>ncollection|NCollection|misresol|three.layer|guard|type.hint</code> (<code>**/crates/opencascade-sys/bindings.toml</code>), 20 results</summary>

- [bindings.toml:17](../crates/opencascade-sys/bindings.toml#L17)
- [bindings.toml:271](../crates/opencascade-sys/bindings.toml#L271)
- [bindings.toml:272](../crates/opencascade-sys/bindings.toml#L272)
- [bindings.toml:276](../crates/opencascade-sys/bindings.toml#L276)
- [bindings.toml:279](../crates/opencascade-sys/bindings.toml#L279)
- [bindings.toml:280](../crates/opencascade-sys/bindings.toml#L280)
- [bindings.toml:283](../crates/opencascade-sys/bindings.toml#L283)
- [bindings.toml:284](../crates/opencascade-sys/bindings.toml#L284)
- [bindings.toml:302](../crates/opencascade-sys/bindings.toml#L302)
- [bindings.toml:304](../crates/opencascade-sys/bindings.toml#L304)
- [bindings.toml:305](../crates/opencascade-sys/bindings.toml#L305)
- [bindings.toml:309](../crates/opencascade-sys/bindings.toml#L309)
- [bindings.toml:310](../crates/opencascade-sys/bindings.toml#L310)
- [bindings.toml:311](../crates/opencascade-sys/bindings.toml#L311)
- [bindings.toml:313](../crates/opencascade-sys/bindings.toml#L313)
- [bindings.toml:316](../crates/opencascade-sys/bindings.toml#L316)
- [bindings.toml:319](../crates/opencascade-sys/bindings.toml#L319)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_016LULMimXpNN7eRrov3pLcw__vscode-1771312781581/content.txt)

<details>
<summary>Reading lines 260-345 of <a href="../crates/opencascade-sys/bindings.toml#L260-L346">bindings.toml</a></summary>

```
    "ShapeProcess_ShapeContext.hxx",

    # Protected operator delete (TDF_LabelNode).
    "TDF_LabelNode.hxx",

    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

    # Message_Report: ActiveMetrics() return type NCollection_IndexedMap<Message_MetricType>
    # is misresolved to Standard_Integer by clang in batch parsing mode.
    "Message_Report.hxx",

    # math_Matrix: methods reference math_Vector (typedef of math_VectorBase<>)
    # which clang misresolves to int in batch parsing mode.
    "math_Matrix.hxx",

    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray::ConstIterator
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "TObj_TIntSparseArray.hxx",

    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param misresolved to int;
    # ReadProcessingData returns std::pair<DataMap, bitset> misresolved to int.
    "XSAlgo_ShapeProcessor.hxx",
]

# Include specific individual headers (from modules not fully included above).
include_headers = []

[general]
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
# =============================================================================
#
# The following categories of problems prevent certain OCCT headers from being
# included in the binding generation. Each category describes the root cause,
# affected headers, and potential solutions.
#
# 1. NCollection Template Parameter Misresolution (SOLVED)
#    ─────────────────────────────────────────────────────────────────
#    Root cause: When clang processes NCollection template specializations
#    (e.g., NCollection_DataMap<TDF_Label, TDF_Label>) that it cannot fully
#    instantiate, it falls back to canonical type "int" (Standard_Integer).
#    This causes constructor and method parameters/returns to have wrong types.
#
#    Solution implemented: Three-layer guard in parser.rs:
#    a) AST-based typedef map — collect_ncollection_typedefs() pre-scans all
#       typedef declarations, building a map from NCollection template spelling
#       to typedef name. lookup_typedef() then resolves template display names
#       back to their original typedef (e.g., NCollection_DataMap<K,V> →
#       TopTools_DataMapOfShapeShape). normalize_template_spelling() handles
#       Standard_Integer→int, Standard_Real→double type alias mismatches.
#    b) spelling_looks_like_class guard — when a non-template, non-namespace
#       display name looks like an OCCT class but canonical type is "int",
#       checks the underlying typedef declaration to distinguish real primitive
#       typedefs from misresolved NCollection types.
#    c) Reference/pointer canonical guard — when the outer ref/ptr type's
#       display name identifies an OCCT class/template/namespace but canonical
#       resolves to "int", constructs the Class type directly instead of
#       recursing into the pointee (which would lose the typedef info).
#
#    Additionally, is_constructor_bindable() in codegen/bindings.rs now checks
#    type_uses_unknown_type() to filter constructors with unresolvable types.
#
#    ~120 headers successfully UNBLOCKED by these combined fixes.
#
#
# 2. Primitive Typedef vs Template Typedef Ambiguity
#    ────────────────────────────────────────────────
#    Root cause: Some OCCT types are typedefs to primitives (e.g.,
#    `typedef unsigned int Poly_MeshPurpose`), while NCollection typedefs
#    are ALSO canonicalized to primitives by clang (incorrectly). The parser
#    uses a heuristic guard (`spelling_looks_like_class`) to distinguish
#    these, checking the typedef declaration's underlying type kind.
#
#    Current status: Mostly solved. The guard now checks whether the type's
#    declaration is a TypedefDecl with a primitive u
... (truncated, 4242 chars)
```
</details>

<details>
<summary>Searched for regex <code>type_uses_unknown_class|type_uses_unknown_handle|TypeContext</code> (<code>**/crates/opencascade-binding-generator/src/type_mapping.rs</code>), 8 results</summary>

- [type_mapping.rs:281](../crates/opencascade-binding-generator/src/type_mapping.rs#L281)
- [type_mapping.rs:308](../crates/opencascade-binding-generator/src/type_mapping.rs#L308)
- [type_mapping.rs:323](../crates/opencascade-binding-generator/src/type_mapping.rs#L323)
- [type_mapping.rs:329](../crates/opencascade-binding-generator/src/type_mapping.rs#L329)
- [type_mapping.rs:331](../crates/opencascade-binding-generator/src/type_mapping.rs#L331)
- [type_mapping.rs:351](../crates/opencascade-binding-generator/src/type_mapping.rs#L351)
- [type_mapping.rs:358](../crates/opencascade-binding-generator/src/type_mapping.rs#L358)
- [type_mapping.rs:442](../crates/opencascade-binding-generator/src/type_mapping.rs#L442)
</details>

<details>
<summary>Searched for regex <code>type_uses_unknown|all_class_names|handle_able_classes|typedef_names</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 20 results</summary>

- [resolver.rs:348](../crates/opencascade-binding-generator/src/resolver.rs#L348)
- [resolver.rs:350](../crates/opencascade-binding-generator/src/resolver.rs#L350)
- [resolver.rs:768](../crates/opencascade-binding-generator/src/resolver.rs#L768)
- [resolver.rs:770](../crates/opencascade-binding-generator/src/resolver.rs#L770)
- [resolver.rs:773](../crates/opencascade-binding-generator/src/resolver.rs#L773)
- [resolver.rs:803](../crates/opencascade-binding-generator/src/resolver.rs#L803)
- [resolver.rs:806](../crates/opencascade-binding-generator/src/resolver.rs#L806)
- [resolver.rs:811](../crates/opencascade-binding-generator/src/resolver.rs#L811)
- [resolver.rs:818](../crates/opencascade-binding-generator/src/resolver.rs#L818)
- [resolver.rs:821](../crates/opencascade-binding-generator/src/resolver.rs#L821)
- [resolver.rs:825](../crates/opencascade-binding-generator/src/resolver.rs#L825)
- [resolver.rs:848](../crates/opencascade-binding-generator/src/resolver.rs#L848)
- [resolver.rs:849](../crates/opencascade-binding-generator/src/resolver.rs#L849)
- [resolver.rs:923](../crates/opencascade-binding-generator/src/resolver.rs#L923)
- [resolver.rs:1200](../crates/opencascade-binding-generator/src/resolver.rs#L1200)
- [resolver.rs:1201](../crates/opencascade-binding-generator/src/resolver.rs#L1201)
</details>

<details>
<summary>Reading lines 275-360 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L275-L361">type_mapping.rs</a></summary>

```
    } else {
        class_name.to_string()
    }
}

/// Context for type mapping within a specific module
pub struct TypeContext<'a> {
    /// The current module name (e.g., "gp")
    pub current_module: &'a str,
    /// Classes defined in the current module (full C++ names like "gp_Pnt")
    pub module_classes: &'a std::collections::HashSet<String>,
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type && !has_protected_destructor)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    match ty {
        Type::Class(class_
... (truncated, 4034 chars)
```
</details>

<details>
<summary>Reading lines 755-860 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L755-L861">resolver.rs</a></summary>

```
}

/// Build the symbol table from parsed headers and module graph
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Compute handle-able classes (inherit from Standard_Transient and no protected destructor)
    let mut handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
                    set.insert(name.clone());
                }
            }
            crate::model::Type::ConstRef(inner)
            | crate::model::Type::MutRef(inner)
            | crate::model::Type::ConstPtr(inner)
            | crate::model::Type::MutPtr(inner)
            | crate::model::Type::RValueRef(inner) => {
                collect_handle_types(inner, set);
            }
            _ => {}
        }
    }
    for class in all_classes {
        for method in &class.methods {
            for param in &method.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
            if let Some(ret) = &method.return_type {
                collect_handle_types(ret, &mut handle_able_classes);
            }
        }
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
        }
    }
    // Also scan standalone functions
    for func in all_functions {
        for param in &func.params {
            collect_handle_types(&param.ty, &mut handle_able_classes);
        }
        if let Some(ret) = &func.return_type {
            collect_handle_types(ret, &mut handle_able_classes);
        }
    }
    // These referenced Handle types also need to be known class names
    all_class_names.extend(handle_able_classes.iter().cloned());
    
    // Build authoritative type→module mapping from parsed header data.
    // This is the single source of truth for module membership.
    let type_to_module: HashMap<String, String> = all_classes
        .iter()
        .map(|c| (c.name.clone(), c.module.clone()))
        .chain(all_enums.iter().map(|e| (e.name.clone(), e.module.clone())))
        .collect();
    // Keep a reference copy for resolve_* functions (avoids borrow conflicts with table)
    let type_to_module_ref = type_to_module.clone();
    
    let mut table = SymbolTable {
        classes: HashMap::new(),
        constructors: HashMap::new(),
        methods: HashMap::new(),
        static_methods: HashMap::new(),
        functions: HashMap::new(),
        enums: HashMap::new(),
        classes_by_module: HashMap::new(),
        functions_by_module: HashMap::new(),
        enums_by_module: HashMap::new(),
        all_enum_names: all_enum_names.clone(),
        all_class_names: all_class_names.clone(),
        handle_able_classes: handle_able_classes.clone(),
        cross_module_types:
... (truncated, 4318 chars)
```
</details>

<details>
<summary>Reading lines 260-345 of <a href="../crates/opencascade-sys/bindings.toml#L260-L346">bindings.toml</a></summary>

```
    "ShapeProcess_ShapeContext.hxx",

    # Protected operator delete (TDF_LabelNode).
    "TDF_LabelNode.hxx",

    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

    # Message_Report: ActiveMetrics() return type NCollection_IndexedMap<Message_MetricType>
    # is misresolved to Standard_Integer by clang in batch parsing mode.
    "Message_Report.hxx",

    # math_Matrix: methods reference math_Vector (typedef of math_VectorBase<>)
    # which clang misresolves to int in batch parsing mode.
    "math_Matrix.hxx",

    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray::ConstIterator
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "TObj_TIntSparseArray.hxx",

    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param misresolved to int;
    # ReadProcessingData returns std::pair<DataMap, bitset> misresolved to int.
    "XSAlgo_ShapeProcessor.hxx",
]

# Include specific individual headers (from modules not fully included above).
include_headers = []

[general]
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
# =============================================================================
#
# The following categories of problems prevent certain OCCT headers from being
# included in the binding generation. Each category describes the root cause,
# affected headers, and potential solutions.
#
# 1. NCollection Template Parameter Misresolution (SOLVED)
#    ─────────────────────────────────────────────────────────────────
#    Root cause: When clang processes NCollection template specializations
#    (e.g., NCollection_DataMap<TDF_Label, TDF_Label>) that it cannot fully
#    instantiate, it falls back to canonical type "int" (Standard_Integer).
#    This causes constructor and method parameters/returns to have wrong types.
#
#    Solution implemented: Three-layer guard in parser.rs:
#    a) AST-based typedef map — collect_ncollection_typedefs() pre-scans all
#       typedef declarations, building a map from NCollection template spelling
#       to typedef name. lookup_typedef() then resolves template display names
#       back to their original typedef (e.g., NCollection_DataMap<K,V> →
#       TopTools_DataMapOfShapeShape). normalize_template_spelling() handles
#       Standard_Integer→int, Standard_Real→double type alias mismatches.
#    b) spelling_looks_like_class guard — when a non-template, non-namespace
#       display name looks like an OCCT class but canonical type is "int",
#       checks the underlying typedef declaration to distinguish real primitive
#       typedefs from misresolved NCollection types.
#    c) Reference/pointer canonical guard — when the outer ref/ptr type's
#       display name identifies an OCCT class/template/namespace but canonical
#       resolves to "int", constructs the Class type directly instead of
#       recursing into the pointee (which would lose the typedef info).
#
#    Additionally, is_constructor_bindable() in codegen/bindings.rs now checks
#    type_uses_unknown_type() to filter constructors with unresolvable types.
#
#    ~120 headers successfully UNBLOCKED by these combined fixes.
#
#
# 2. Primitive Typedef vs Template Typedef Ambiguity
#    ────────────────────────────────────────────────
#    Root cause: Some OCCT types are typedefs to primitives (e.g.,
#    `typedef unsigned int Poly_MeshPurpose`), while NCollection typedefs
#    are ALSO canonicalized to primitives by clang (incorrectly). The parser
#    uses a heuristic guard (`spelling_looks_like_class`) to distinguish
#    these, checking the typedef declaration's underlying type kind.
#
#    Current status: Mostly solved. The guard now checks whether the type's
#    declaration is a TypedefDecl with a primitive u
... (truncated, 4242 chars)
```
</details>

**Terminal:**
```sh
grep -n "ActiveMetrics\|NCollection_IndexedMap" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Message/Message_Report.hxx 2>/dev/null | head -20
```
<details>
<summary>Output (5 lines)</summary>

```
22:#include <NCollection_IndexedMap.hxx>
111:  const NCollection_IndexedMap<Message_MetricType>& ActiveMetrics() const
113:    return myActiveMetrics;
122:  void ClearMetrics() { myActiveMetrics.Clear(); }
185:  NCollection_IndexedMap<Message_MetricType> myActiveMetrics; //!< metrics to compute on alerts
```
</details>

**Terminal:**
```sh
grep -n "math_Vector\|math_VectorBase\|NCollection" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/math/math_Matrix.hxx 2>/dev/null | head -20 && echo "---" && grep -n "GetIterator\|NCollection_SparseArray\|ConstIterator" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx 2>/dev/null | head -20 && echo "---" && grep -n "NCollection_DataMap\|ReadProcessingData\|std::pair" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx 2>/dev/null | head -20
```
<details>
<summary>Output (24 lines)</summary>

```
22:#include <NCollection_Allocator.hxx>
33:class math_VectorBase;
79:  friend class math_VectorBase<>;
292:  Standard_EXPORT void SetRow(const Standard_Integer Row, const math_VectorBase<>& V);
299:  Standard_EXPORT void SetCol(const Standard_Integer Col, const math_VectorBase<>& V);
306:  Standard_EXPORT math_VectorBase<> Row(const Standard_Integer Row) const;
309:  Standard_EXPORT math_VectorBase<> Col(const Standard_Integer Col) const;
336:  Standard_EXPORT void Multiply(const math_VectorBase<>& Left, const math_VectorBase<>& Right);
386:  Standard_NODISCARD Standard_EXPORT math_VectorBase<> Multiplied(
387:    const math_VectorBase<>& Right) const;
388:  Standard_NODISCARD Standard_EXPORT math_VectorBase<> operator*(
389:    const math_VectorBase<>& Right) const;
---
21:#include <NCollection_SparseArray.hxx>
24:typedef NCollection_SparseArray<Standard_Integer> TObj_TIntSparseArray_VecOfData;
25:typedef NCollection_SparseArray<Standard_Integer> TObj_TIntSparseArray_MapOfData;
32: * The supporting underlying data structure is NCollection_SparseArray of integers.
56:  typedef TObj_TIntSparseArray_VecOfData::ConstIterator Iterator;
59:  Iterator GetIterator() const { return Iterator(myVector); }
---
35:  using ParameterMap   = NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>;
36:  using ProcessingData = std::pair<ParameterMap, ShapeProcess::OperationsFlags>;
40:  using ProcessingFlags = std::pair<ShapeProcess::OperationsFlags, bool>;
100:  Standard_EXPORT static ProcessingData ReadProcessingData(
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01YMbamUFpyDp5P1UB8DELVh__vscode-1771312781569/content.txt)

**Todo list:**
- ✅ Understand current misresolution handling
- 🔄 Examine problematic OCCT headers
- ⬜ Design type hints solution
- ⬜ Implement type hints in generator
- ⬜ Add type hints to bindings.toml
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

Let me examine the actual OCCT headers and the parser code to understand the exact misresolution patterns.

Examine OCCT headers and parser details

Searched for files matching `**/OCCT/src/**/Message_Report.hxx`, 1 match
- [Message_Report.hxx](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx)

Searched for files matching `**/OCCT/src/**/math_Matrix.hxx`, 1 match
- [math_Matrix.hxx](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx)

Searched for files matching `**/OCCT/src/**/TObj_TIntSparseArray.hxx`, 1 match
- [TObj_TIntSparseArray.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx)

Searched for files matching `**/OCCT/src/**/XSAlgo_ShapeProcessor.hxx`, 1 match
- [XSAlgo_ShapeProcessor.hxx](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx)

Searched for files matching `**/OCCT/src/**/math_Vector*`, 3 matches
- [math_Vector.hxx](../crates/occt-sys/OCCT/src/math/math_Vector.hxx)
- [math_VectorBase.hxx](../crates/occt-sys/OCCT/src/math/math_VectorBase.hxx)
- [math_VectorBase.lxx](../crates/occt-sys/OCCT/src/math/math_VectorBase.lxx)

Searched for text `ActiveMetrics` (`**/Message_Report.hxx`), 4 results
- [Message_Report.hxx:111](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx#L111)
- [Message_Report.hxx:113](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx#L113)
- [Message_Report.hxx:122](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx#L122)
- [Message_Report.hxx:185](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx#L185)

<details>
<summary>Searched for text <code>math_Vector</code> (<code>**/math/math_Matrix.hxx</code>), 12 results</summary>

- [math_Matrix.hxx:33](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L33)
- [math_Matrix.hxx:79](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L79)
- [math_Matrix.hxx:292](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L292)
- [math_Matrix.hxx:299](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L299)
- [math_Matrix.hxx:306](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L306)
- [math_Matrix.hxx:309](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L309)
- [math_Matrix.hxx:336](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L336)
- [math_Matrix.hxx:386](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L386)
- [math_Matrix.hxx:387](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L387)
- [math_Matrix.hxx:388](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L388)
- [math_Matrix.hxx:389](../crates/occt-sys/OCCT/src/math/math_Matrix.hxx#L389)
</details>

<details>
<summary>Searched for regex <code>GetIterator|Iterator|typedef</code> (<code>**/TObj_TIntSparseArray.hxx</code>), 9 results</summary>

- [TObj_TIntSparseArray.hxx:24](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx#L24)
- [TObj_TIntSparseArray.hxx:25](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx#L25)
- [TObj_TIntSparseArray.hxx:56](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx#L56)
- [TObj_TIntSparseArray.hxx:58](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx#L58)
- [TObj_TIntSparseArray.hxx:59](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx#L59)
</details>

<details>
<summary>Searched for regex <code>ReadProcessingData|constructor|ShapeProcessor</code> (<code>**/XSAlgo/XSAlgo_ShapeProcessor.hxx</code>), 10 results</summary>

- [XSAlgo_ShapeProcessor.hxx:14](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L14)
- [XSAlgo_ShapeProcessor.hxx:15](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L15)
- [XSAlgo_ShapeProcessor.hxx:32](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L32)
- [XSAlgo_ShapeProcessor.hxx:43](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L43)
- [XSAlgo_ShapeProcessor.hxx:48](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L48)
- [XSAlgo_ShapeProcessor.hxx:51](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L51)
- [XSAlgo_ShapeProcessor.hxx:53](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L53)
- [XSAlgo_ShapeProcessor.hxx:100](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L100)
- [XSAlgo_ShapeProcessor.hxx:133](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L133)
- [XSAlgo_ShapeProcessor.hxx:206](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx#L206)
</details>

<details>
<summary>Searched for regex <code>typedef|math_Vector</code> (<code>**/math/math_Vector.hxx</code>), 8 results</summary>

- [math_Vector.hxx:15](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L15)
- [math_Vector.hxx:16](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L16)
- [math_Vector.hxx:18](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L18)
- [math_Vector.hxx:24](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L24)
- [math_Vector.hxx:29](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L29)
- [math_Vector.hxx:44](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L44)
- [math_Vector.hxx:48](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#L48)
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1-L101">parser.rs</a></summary>

```
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

thread_local! {
    /// Map from NCollection template display names to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: normalized template spelling (whitespace-stripped), e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: typedef name, e.g. "TDF_LabelMap"
    static TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


/// Walk the AST to collect all typedef declarations that resolve to NCollection
/// template specializations. Populates the thread-local TYPEDEF_MAP.
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        // Only record if the typedef name looks like an OCCT type
                        // (starts with uppercase, contains underscore)
                        if name.starts_with(|c: char| c.is_ascii_uppercase())
                            && name.contains('_')
                        {
                            map.insert(key, name.clone());
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    eprintln!("  Collected {} NCollection/template typedefs", map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Look up a type's display name in the typedef map.
/// Returns the typedef name if found.
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() ->
... (truncated, 4326 chars)
```
</details>

<details>
<summary>Reading lines 1140-1350 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1140-L1351">parser.rs</a></summary>

```
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
    let clean_for_lookup = trimmed_spelling
        .trim_start_matches("const ")
        .trim_start_matches("struct ")
        .trim_start_matches("class ")
        .trim_start_matches("typename ")
        .trim();
    if clean_for_lookup.contains('<') && !clean_for_lookup.starts_with("opencascade::handle<") && !clean_for_lookup.starts_with("Handle(") {
        if let Some(typedef_name) = lookup_typedef(clean_for_lookup) {
            return Type::Class(typedef_name);
        }
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = canonical_spelling
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim();
    // Guard against clang misresolving NCollection template specializations.
    // When clang can't fully instantiate templates like NCollection_DataMap<A,B>,
    // it falls back to canonical type "int". Detect this by checking if the
    // display name is clearly a class/typedef (not a known primitive typedef)
    // while the canonical says it's a primitive.
    //
    // However, legitimate typedefs to primitives (e.g., `typedef unsigned int Poly_MeshPurpose`)
    // must still resolve to their canonical primitive type. We distinguish these by
    // checking the typedef's underlying type: if it's a builtin primitive or another
    // typedef (i.e., a chain like Graphic3d_ZLayerId -> Standard_Integer -> int),
    // it's a genuine primitive typedef. NCollection typedefs have underlying types
    // that are template specializations (Record/Elaborated/Unexposed), not primitives.
    let spelling_looks_like_class = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("struct ")
            .trim_start_matches("class ")
            .trim_start_matches("typename ")
            .trim();
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && ma
... (truncated, 9568 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/config.rs">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Expand the configuration into a list of header file paths.
///
/// - For each module in `modules`, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Expand modules: find all headers matching {Module}.hxx and {Module}_*.hxx
    for module in &config.modules {
        let mut module_headers = Vec::new();

        // Look for {Module}.hxx
        let main_header = format!("{}.hxx", module);
        let main_path = occt_include_dir.join(&main_header);
        if main_path.exists() {
            module_headers.push((main_header.clone(), main_path));
        }

        // Look for {Module}_*.hxx
        let prefix = format!("{}_", module);
        let entries = std::fs::read_dir(occt_include_dir)
            .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".hxx") {
                module_headers.push((filename, entry.path()));
            }
        }

        module_headers.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, path) in module_headers {
            if seen.insert(name) {
                headers.push(path);
            }
        }
    }

    // 2. Add individual headers
    for header_name in &config.include_headers {
        if seen.insert(header_name.clone()) {
            let path = occt_include_dir.join(header_name);
            if path.exists() {
                headers.push(path);
            } else {
                eprintln!("Warning: Header not found: {}", path.display());
            }
        }
    }

    // 3. Remove excluded headers
    if !config.exclude_headers.is_empty() {
        let exclude_set: std::collections::HashSet<&str> =
            config.exclude_headers.iter().map(|s| s.as_str()).collect();
        headers.reta
... (truncated, 4177 chars)
```
</details>

<details>
<summary>Reading lines 495-700 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L495-L701">bindings.rs</a></summary>

```
pub fn build_ncollection_element_types(
    all_classes: &[&ParsedClass],
) -> (HashMap<String, String>, HashSet<String>) {
    let typedef_map = parser::get_typedef_map();

    // Build reverse map: typedef_name → element_type(s)
    // The typedef map is: normalized_template_spelling → typedef_name
    let mut typedef_to_element: HashMap<String, String> = HashMap::new();
    for (template_spelling, typedef_name) in &typedef_map {
        if let Some(elem_type) = extract_element_type_from_template(template_spelling) {
            // Strip Handle() wrapper if present
            let clean_elem = if elem_type.starts_with("Handle(") {
                elem_type.trim_start_matches("Handle(").trim_end_matches(')').to_string()
            } else if elem_type.starts_with("opencascade::handle<") {
                elem_type.trim_start_matches("opencascade::handle<").trim_end_matches('>').to_string()
            } else {
                elem_type
            };
            typedef_to_element.insert(typedef_name.clone(), clean_elem);
        }
    }

    // For each class, check if it inherits from an NCollection typedef
    // or directly from an NCollection template class
    let mut nonprimitive = HashMap::new();
    let mut primitive = HashSet::new();
    for class in all_classes {
        for base in &class.base_classes {
            // Check if base is a known typedef with element type
            if let Some(elem_type) = typedef_to_element.get(base) {
                if is_primitive_element_type(elem_type) {
                    primitive.insert(class.name.clone());
                } else {
                    nonprimitive.insert(class.name.clone(), elem_type.clone());
                }
                break;
            }
            // Check if base is a NCollection template like
            // NCollection_HArray1<gp_Pnt, TColgp_Array1OfPnt>
            if base.starts_with("NCollection_") {
                if let Some(elem_type) = extract_element_type_from_template(base) {
                    // Strip Handle() wrapper if present
                    let clean_elem = if elem_type.starts_with("Handle(") {
                        elem_type.trim_start_matches("Handle(").trim_end_matches(')').to_string()
                    } else if elem_type.starts_with("opencascade::handle<") {
                        elem_type.trim_start_matches("opencascade::handle<").trim_end_matches('>').to_string()
                    } else {
                        elem_type
                    };
                    if is_primitive_element_type(&clean_elem) {
                        primitive.insert(class.name.clone());
                    } else {
                        nonprimitive.insert(class.name.clone(), clean_elem);
                    }
                    break;
                }
            }
        }
    }

    // Fallback: detect H-classes by DEFINE_HARRAY1/HARRAY2/HSEQUENCE name patterns
    // When clang can't resolve the base class typedef in batch mode, the base
    // specifier is lost. We can still identify H-classes by their naming pattern
    // and look up the corresponding Array/Sequence typedef in the typedef map.
    for class in all_classes {
        if nonprimitive.contains_key(&class.name) || primitive.contains(&class.name) {
            continue;
        }
        let name = &class.name;
        // Try each DEFINE_H* pattern: HClassName -> ArrayType naming convention
        // Standard patterns: Foo_HArray1OfBar -> Foo_Array1OfBar
        // Non-standard patterns: Foo_HBarArray1 -> Foo_BarArray1 (strip "H" before "Array1")
        let array_typedef_name = if let Some(pos) = name.find("HArray1Of") {
            Some(format!("{}Array1Of{}", &name[..pos], &name[pos + "HArray1Of".len()..]))
        } else if let Some(pos) = name.find("HArray2Of") {
            Some(format!("{}Array2Of{}", &name[..pos], &name[pos + "HArray2Of".len()..]))
        } else if let Some(pos) = name.find("HSequenceOf") {
            Some(format!("{}Sequenc
... (truncated, 9667 chars)
```
</details>

<details>
<summary>Reading lines 740-800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L801">bindings.rs</a></summary>

```
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            param_uses_unknown_handle(inner, handle_able_classes)
        }
        _ => false,
    }
}

/// Check if a type uses an unknown class/handle given the TypeContext.
/// Enum types (Type::Class that are in all_enums) are known — they map to i32.
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    // Enum types are known (mapped to i32), so skip them
    match ty {
        Type::Class(name) if ctx.all_enums.contains(name) => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
            }
        }
        _ => {}
    }
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a method has by-value class or handle parameters (not enums).
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && name != "char",
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
fn has_mut_ref_enum_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| {
        if let Type::MutRef(inner) = &p.ty {
            if let Type::Class(name) = inner.as_ref() {
                return all_enums.contains(name);
            }
        }
        false
    })
}

/// Determine if a method needs a C++ wrapper function
fn needs_wrapper_function(_method: &Method, _all_enums: &HashSet<String>) -> bool {
    // With extern "C" FFI, all methods need C++ wrapper functions
    true
}

/// Classify the wrapper kind for a method that needs_wrapper_function
fn classify_wrapper_kind(method: &Method, all_enums: &HashSet<String>) -> WrapperKind {
    let has_cstring_param = method.params.iter().any(|p| p.ty.is_c_string());
    let returns_cstring = method
```
</details>

<details>
<summary>Reading lines 1050-1100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1050-L1101">bindings.rs</a></summary>

```
    }
    true
}

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> bool {
    // By-value class/handle params are now supported: C++ wrappers accept const T&
    // and the C++ compiler handles the copy.
    if ctor.has_unbindable_types() {
        return false;
    }
    if ctor
        .params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    // Also check for unknown class types in parameters.
    // This catches NCollection typedef types (e.g., TDF_LabelMap) that aren't
    // declared in the extern "C" FFI.
    if ctor
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip constructors where a nullable pointer param's inner type is unknown
    if ctor.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return false;
    }
    true
}

/// Filter for static methods
fn is_static_method_bindable(method: &StaticMethod, ctx: &TypeContext) -> bool {
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01FxF8Lnu8u3PT5jLgttBVQ9__vscode-1771312781606/content.txt)

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_014VhWg2VJF7xEbTKJn6wrkd__vscode-1771312781608/content.txt)

Read [XSAlgo_ShapeProcessor.hxx](../crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx)

Read [TObj_TIntSparseArray.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_TIntSparseArray.hxx)

Read [Message_Report.hxx](../crates/occt-sys/OCCT/src/Message/Message_Report.hxx#100-100), lines 100 to 130

Read [math_VectorBase.hxx](../crates/occt-sys/OCCT/src/math/math_VectorBase.hxx#1-1), lines 1 to 60

Read [math_Vector.hxx](../crates/occt-sys/OCCT/src/math/math_Vector.hxx#1-1), lines 1 to 50

<details>
<summary>Searched for regex <code>ConstIterator|class.*Iterator</code> (<code>**/OCCT/src/NCollection/NCollection_SparseArray.hxx</code>), 5 results</summary>

- [NCollection_SparseArray.hxx:137](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#L137)
- [NCollection_SparseArray.hxx:141](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#L141)
- [NCollection_SparseArray.hxx:144](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#L144)
- [NCollection_SparseArray.hxx:165](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#L165)
- [NCollection_SparseArray.hxx:173](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#L173)
</details>

Searched for regex `template|class NCollection_IndexedMap` (`**/OCCT/src/NCollection/NCollection_IndexedMap.hxx`), 2 results
- [NCollection_IndexedMap.hxx:39](../crates/occt-sys/OCCT/src/NCollection/NCollection_IndexedMap.hxx#L39)
- [NCollection_IndexedMap.hxx:40](../crates/occt-sys/OCCT/src/NCollection/NCollection_IndexedMap.hxx#L40)

<details>
<summary>Searched for text <code>Message_Report|math_Matrix|TObj_TIntSparseArray|XSAlgo_ShapeProcessor</code> (<code>**/bindings.toml</code>), 8 results</summary>

- [bindings.toml:271](../crates/opencascade-sys/bindings.toml#L271)
- [bindings.toml:273](../crates/opencascade-sys/bindings.toml#L273)
- [bindings.toml:275](../crates/opencascade-sys/bindings.toml#L275)
- [bindings.toml:277](../crates/opencascade-sys/bindings.toml#L277)
- [bindings.toml:279](../crates/opencascade-sys/bindings.toml#L279)
- [bindings.toml:281](../crates/opencascade-sys/bindings.toml#L281)
- [bindings.toml:283](../crates/opencascade-sys/bindings.toml#L283)
- [bindings.toml:285](../crates/opencascade-sys/bindings.toml#L285)
</details>

<details>
<summary>Reading lines 265-295 of <a href="../crates/opencascade-sys/bindings.toml#L265-L296">bindings.toml</a></summary>

```
    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

    # Message_Report: ActiveMetrics() return type NCollection_IndexedMap<Message_MetricType>
    # is misresolved to Standard_Integer by clang in batch parsing mode.
    "Message_Report.hxx",

    # math_Matrix: methods reference math_Vector (typedef of math_VectorBase<>)
    # which clang misresolves to int in batch parsing mode.
    "math_Matrix.hxx",

    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray::ConstIterator
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "TObj_TIntSparseArray.hxx",

    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param misresolved to int;
    # ReadProcessingData returns std::pair<DataMap, bitset> misresolved to int.
    "XSAlgo_ShapeProcessor.hxx",
]

# Include specific individual headers (from modules not fully included above).
include_headers = []

[general]
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
```
</details>

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- 🔄 Design type hints solution
- ⬜ Implement type hints in generator
- ⬜ Add type hints to bindings.toml
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

> 💭 Let me think about the approach:
> 
> 1. **Message_Report** - `ActiveMetrics()` returns `NCollection_IndexedMap<Message_MetricType>&` with no typedef. We need to either skip this method or provide a type hint that maps this template spelling to a known type.
> 
> 2. **math_Matrix** - `math_VectorBase<>` doesn't match the typedef map because the key stores `math_VectorBase<double>`. Fix: expand empty template args `<>` by looking up the class template defaults, or add a manual mapping.
> 
> 3. **TObj_TIntSparseArray** - Returns a nested iterator type that's fundamentally unbindable. 
> 
> For the remaining problematic cases, I should skip those methods entirely. **XSAlgo_ShapeProcessor** has similar issues with class-scoped `using` declarations and `std::pair` types that can't be bound. The real solution is to improve the generator's detection of unbindable types—methods with nested iterators, `std::pair`, or raw templates without typedefs should already be filtered out by the existing `type_uses_unknown_type` check, but it seems that logic isn't catching all these cases.
> 
> The core problem is that unresolvable template types are getting misidentified as `int` instead of being marked as unknown. I could add a type hints configuration to manually map template spellings to their typedef names, and for truly unbindable methods, add explicit exclusion rules in the configuration.
> 
> But the real solution is to improve the parser itself—either by expanding empty template arguments against known typedefs, or by detecting when a type has been misresolved and marking it as unbindable so codegen naturally skips those methods.
> 
> Let me trace through what actually happens in `parse_type` when it encounters a return type like `const NCollection_IndexedMap<Message_MetricType>&` to understand where the misresolution is occurring.

Let me look at the parser code in detail and try enabling these headers to see what actually happens.

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1-L101">parser.rs</a></summary>

```
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

thread_local! {
    /// Map from NCollection template display names to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: normalized template spelling (whitespace-stripped), e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: typedef name, e.g. "TDF_LabelMap"
    static TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


/// Walk the AST to collect all typedef declarations that resolve to NCollection
/// template specializations. Populates the thread-local TYPEDEF_MAP.
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        // Only record if the typedef name looks like an OCCT type
                        // (starts with uppercase, contains underscore)
                        if name.starts_with(|c: char| c.is_ascii_uppercase())
                            && name.contains('_')
                        {
                            map.insert(key, name.clone());
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    eprintln!("  Collected {} NCollection/template typedefs", map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Look up a type's display name in the typedef map.
/// Returns the typedef name if found.
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() ->
... (truncated, 4326 chars)
```
</details>

<details>
<summary>Reading lines 1140-1360 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1140-L1361">parser.rs</a></summary>

```
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
    let clean_for_lookup = trimmed_spelling
        .trim_start_matches("const ")
        .trim_start_matches("struct ")
        .trim_start_matches("class ")
        .trim_start_matches("typename ")
        .trim();
    if clean_for_lookup.contains('<') && !clean_for_lookup.starts_with("opencascade::handle<") && !clean_for_lookup.starts_with("Handle(") {
        if let Some(typedef_name) = lookup_typedef(clean_for_lookup) {
            return Type::Class(typedef_name);
        }
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = canonical_spelling
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim();
    // Guard against clang misresolving NCollection template specializations.
    // When clang can't fully instantiate templates like NCollection_DataMap<A,B>,
    // it falls back to canonical type "int". Detect this by checking if the
    // display name is clearly a class/typedef (not a known primitive typedef)
    // while the canonical says it's a primitive.
    //
    // However, legitimate typedefs to primitives (e.g., `typedef unsigned int Poly_MeshPurpose`)
    // must still resolve to their canonical primitive type. We distinguish these by
    // checking the typedef's underlying type: if it's a builtin primitive or another
    // typedef (i.e., a chain like Graphic3d_ZLayerId -> Standard_Integer -> int),
    // it's a genuine primitive typedef. NCollection typedefs have underlying types
    // that are template specializations (Record/Elaborated/Unexposed), not primitives.
    let spelling_looks_like_class = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("struct ")
            .trim_start_matches("class ")
            .trim_start_matches("typename ")
            .trim();
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && ma
... (truncated, 9890 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/config.rs">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Expand the configuration into a list of header file paths.
///
/// - For each module in `modules`, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Expand modules: find all headers matching {Module}.hxx and {Module}_*.hxx
    for module in &config.modules {
        let mut module_headers = Vec::new();

        // Look for {Module}.hxx
        let main_header = format!("{}.hxx", module);
        let main_path = occt_include_dir.join(&main_header);
        if main_path.exists() {
            module_headers.push((main_header.clone(), main_path));
        }

        // Look for {Module}_*.hxx
        let prefix = format!("{}_", module);
        let entries = std::fs::read_dir(occt_include_dir)
            .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".hxx") {
                module_headers.push((filename, entry.path()));
            }
        }

        module_headers.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, path) in module_headers {
            if seen.insert(name) {
                headers.push(path);
            }
        }
    }

    // 2. Add individual headers
    for header_name in &config.include_headers {
        if seen.insert(header_name.clone()) {
            let path = occt_include_dir.join(header_name);
            if path.exists() {
                headers.push(path);
            } else {
                eprintln!("Warning: Header not found: {}", path.display());
            }
        }
    }

    // 3. Remove excluded headers
    if !config.exclude_headers.is_empty() {
        let exclude_set: std::collections::HashSet<&str> =
            config.exclude_headers.iter().map(|s| s.as_str()).collect();
        headers.reta
... (truncated, 4177 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Xuz8ASnjNq8C1PTWuQ6ynd__vscode-1771312781624/content.txt)

Now let me read the rest of `parse_type` and the codegen parts:

<details>
<summary>Reading lines 1360-1470 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1360-L1471">parser.rs</a></summary>

```
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
        .trim_start_matches("enum ")
        .trim_end_matches(" &")
        .trim_end_matches(" *")
        .trim();
    
    // If the spelling contains :: or < (nested/template type), try typedef map first,
    // then try to use canonical
    if clean_name.contains("::") || clean_name.contains('<') {
        // For template types, check if this is a known typedef
        if clean_name.contains('<') {
            if let Some(typedef_name) = lookup_typedef(clean_name) {
                return Type::Class(typedef_name);
            }
        }
        let canonical_clean = canonical_spelling

            .trim_start_matches("const ")
            .trim_start_matches("class ")
            .trim_start_matches("struct ")
            .trim_start_matches("enum ")
            .trim_end_matches(" &")
            .trim_end_matches(" *")
            .trim();
        
        // Only use canonical if it's simpler (no :: or <)
        if !canonical_clean.contains("::") && !canonical_clean.contains('<') && !canonical_clean.is_empty() {
            return Type::Class(canonical_clean.to_string());
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
fn map_standar
... (truncated, 4261 chars)
```
</details>

<details>
<summary>Reading lines 740-810 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L811">bindings.rs</a></summary>

```
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            param_uses_unknown_handle(inner, handle_able_classes)
        }
        _ => false,
    }
}

/// Check if a type uses an unknown class/handle given the TypeContext.
/// Enum types (Type::Class that are in all_enums) are known — they map to i32.
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    // Enum types are known (mapped to i32), so skip them
    match ty {
        Type::Class(name) if ctx.all_enums.contains(name) => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
            }
        }
        _ => {}
    }
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a method has by-value class or handle parameters (not enums).
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && name != "char",
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
fn has_mut_ref_enum_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| {
        if let Type::MutRef(inner) = &p.ty {
            if let Type::Class(name) = inner.as_ref() {
                return all_enums.contains(name);
            }
        }
        false
    })
}

/// Determine if a method needs a C++ wrapper function
fn needs_wrapper_function(_method: &Method, _all_enums: &HashSet<String>) -> bool {
    // With extern "C" FFI, all methods need C++ wrapper functions
    true
}

/// Classify the wrapper kind for a method that needs_wrapper_function
fn classify_wrapper_kind(method: &Method, all_enums: &HashSet<String>) -> WrapperKind {
    let has_cstring_param = method.params.iter().any(|p| p.ty.is_c_string());
    let returns_cstring = method
        .return_type
        .as_ref()
        .map(|t| t.is_c_string())
        .unwrap_or(false);
    let returns_by_value = method.return_type.as_ref().map_or(false, |ty| {
        let is_class_or_handle = ty.is_class() || ty.is_handle();
        let is_enum = match ty {
            Type::Class(name) => all_enums.contains(name),
            _ => false,
        };
```
</details>

<details>
<summary>Reading lines 1050-1120 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1050-L1121">bindings.rs</a></summary>

```
    }
    true
}

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> bool {
    // By-value class/handle params are now supported: C++ wrappers accept const T&
    // and the C++ compiler handles the copy.
    if ctor.has_unbindable_types() {
        return false;
    }
    if ctor
        .params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    // Also check for unknown class types in parameters.
    // This catches NCollection typedef types (e.g., TDF_LabelMap) that aren't
    // declared in the extern "C" FFI.
    if ctor
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip constructors where a nullable pointer param's inner type is unknown
    if ctor.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return false;
    }
    true
}

/// Filter for static methods
fn is_static_method_bindable(method: &StaticMethod, ctx: &TypeContext) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    // Skip static methods with const char*& or const char* const& params (need manual bindings)
    if resolver::static_method_has_string_ref_param(method).is_some() {
        return false;
    }
    // &mut enum output params are now handled via C++ wrappers.
    if method
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip static methods where a nullable pointer param's inner type is unknown
    if method.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
```
</details>

> 💭 Now I understand the full picture. Let me look at what happens for methods specifically - where are methods filtered for unknown return types?

Let me also check how methods (not just constructors) are filtered for unknown return types:

<details>
<summary>Searched for regex <code>is_method_bindable|method_bindable|return_type.*unknown|unknown.*return</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 6 results</summary>

- [bindings.rs:993](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L993)
- [bindings.rs:1100](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1100)
- [bindings.rs:1133](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1133)
- [bindings.rs:1607](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1607)
- [bindings.rs:1642](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1642)
- [bindings.rs:1721](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1721)
</details>

<details>
<summary>Reading lines 993-1055 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L993-L1056">bindings.rs</a></summary>

```
fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if resolver::method_has_string_ref_param(method).is_some() {
        return false;
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return false;
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if resolver::method_needs_explicit_lifetimes(method) {
        return false;
    }
    if method
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if method.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return false;
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return false;
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return false;
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return false;
        }
    }
    true
}

/// Filter for constructors
fn is_constructor_bindable(
```
</details>

> 💭 Now I understand the issue more deeply. The key problem: when clang misresolves a type like `NCollection_IndexedMap<Message_MetricType>` to `int`, the parse_type function's guards *should* catch this because the display name contains `<`. Let me trace through what happens:
> 
> 1. Display name: `const NCollection_IndexedMap<Message_MetricType> &` (for a const ref return)
> 2. Kind: LValueReference
> 3. Enter parse_type
> 
> Wait, but for a return type parsed through `parse_return_type`, the function calls `parse_type(&result_type)` where `result_type` is the full return type. 
> 
> So for `const NCollection_IndexedMap<Message_MetricType>&`, I'm checking how the cleaning and lookup process handles this: the kind is LValueReference, the spelling includes the full type with const and the reference, and when I clean it for lookup by stripping the const prefix, I'm left with the template type still intact—but the trailing & remains, which means the lookup against standard types won't match, and the template check should catch the `<` character.
> 
> Then when I get to the reference guard, I need to strip the trailing & or * from the canonical form to get the actual base type for further processing. So the three-layer guard should actually catch this case—the return type gets wrapped as `ConstRef(Class("NCollection_IndexedMap<Message_MetricType>"))`, and then when checking if the method is bindable, it would detect that the template specialization isn't in the known classes list and filter it out. That suggests the exclusion comment for `Message_Report.hxx` might be outdated from before these safeguards were in place. Now I'm looking at the other problematic files like `math_Matrix.hxx` to see if they follow the same pattern. as parameter types. Let's trace:
> - For a method like `void SetRow(const Standard_Integer Row, const math_VectorBase<>& V)`:
>   - The param `V` has type `const math_VectorBase<> &`
>   - Kind: LValueReference
>   - spelling: "const math_VectorBase<> &" (approximately)
>   - The pointee type would be `const math_VectorBase<>`
>   - When parse_type recurses into the pointee...
>   
>   Actually, wait. Layer (c) fires for LValueReference types. Let's trace:
>   - canonical_base = "int" (misresolved)
>   - base = "math_VectorBase<>" (after stripping const, &)
>   - base_looks_like_class = false (contains '<')
>   - base_looks_like_type = true (contains '<')
>   - Returns ConstRef(Class("math_VectorBase<>"))
>   
>   Then `type_uses_unknown_type` checks if "math_VectorBase<>" is in all_classes. It's not, so the method gets skipped.
> 
> For by-value returns like `math_VectorBase<> Row(...)`, the type isn't a reference or pointer, so layer (c) doesn't apply. The spelling gets normalized through template handling to find the base class name in the registry.
> 
> The issue is that when the typedef map has "math_VectorBase<double>" but the code uses "math_VectorBase<>", they don't match. The fallback logic then incorrectly resolves to "int" instead of finding the actual class. I need to re-examine how the canonical type resolution works in those lines. I'm tracing through the logic flow to understand why `math_VectorBase<>` gets converted to `Type::Class("int")`. The `spelling_looks_like_class` check fails because the spelling contains `<`, so that's false. Then at the primitive guard, since `spelling_is_template_or_namespaced` is true (it has `<`), we skip the primitive matching and move into layer (c), but `math_VectorBase<>` doesn't match the expected patterns there either.
> 
> By the time we reach line 1398, the code checks if `clean_name` contains `::` or `<`, which it does, so it attempts a typedef lookup that fails. The canonical form becomes just `"int"` with no template or namespace qualifiers, and the function returns `Type::Class("int")` instead of preserving the original template type. This is the bug—when a by-value return type is a template with no matching typedef, the code strips away the template information and returns the canonical form as a class type.
> 
> Looking at the other cases, `TObj_TIntSparseArray.hxx` has the same issue: `NCollection_SparseArray<Standard_Integer>::ConstIterator` gets canonicalized to `"int"` and incorrectly returned as `Type::Class("int")`. For `XSAlgo_ShapeProcessor.hxx`, the constructor parameter `const ParameterMap&` involves a class-scoped alias, so I need to figure out what clang would show for the display name in that context.
> 
> The core fix is straightforward: in the parse_type function around lines 1398-1419, when `clean_name` contains `<` or `::` but the canonical resolves to something primitive like `"int"` or `"double"`, I should return `Type::Class(clean_name)` instead of `Type::Class("int")`. This preserves the actual template or namespaced type name, and then `type_uses_unknown_type` will naturally filter out the method. The check needs to detect when the canonical is a simple primitive despite the clean_name being complex. I need to add a check to prevent using primitive type names as the canonical form when the display name contains template syntax—if the canonical resolves to something like "int" but the display name has '<', I should keep the display name since it's likely a misresolution and returning the unknown type is safer than using the wrong primitive. the display name has '<' or '::' indicating misresolution, so I should only use the canonical form if it actually looks like a class name—starting with an uppercase letter and without template or scope resolution syntax. This filters out primitives like "int" that sneak through as canonical forms, letting template types fall back to preserving their original names instead.
> 
> With this approach, unknown types get naturally caught by the existing filter since they won't be in the class registry—so template instantiations like `NCollection_IndexedMap<Message_MetricType>` and nested iterators would all be handled consistently across the different problematic cases. I'm realizing the core issue: if `type_uses_unknown_class` correctly identifies "int" as unknown, then these methods should already be getting skipped, which means the headers should compile fine without exclusion. So either "int" is being treated as a known type somewhere in the logic, or there's something else causing the compilation failure that I'm missing.
> 
> Looking at the subagent's research, the actual problem seems to be with class declarations themselves—specifically that certain return types like `NCollection_IndexedMap<Message_MetricType>` are being misresolved by clang's batch parsing mode. Maybe I should just try removing these exclusions and regenerating to see if the three-layer guard already handles them properly.
> 
> But wait, if the type is being misresolved to `Standard_Integer` or `Type::I32`, that's actually a valid primitive type, so the method wouldn't get filtered out. The codegen would happily generate a C++ wrapper that returns an int from a method that actually returns a complex map type—which would cause a compile error or crash at runtime.
> 
> The real issue is that `parse_type` is misresolving the template type to a primitive before any of the safety checks can catch it. If clang's display name already shows "const int &" instead of the actual template, then there's no way for the guards to know something went wrong. The fix has to happen at the type parsing level, not in the filtering logic.
> 
> Let me test this by removing those excluded headers and regenerating to see what compilation errors actually surface. I should also fix the `Type::Class("int")` bug in `parse_type` directly, since that's a real issue regardless of what clang reports.
> 
> Actually, thinking about this differently—the comments mention "batch parsing mode," which suggests the binding generator processes all headers in a single libclang invocation. Maybe the issue is how clang resolves types in that context. But the user mentioned type hints as a potential solution, so let me consider how a manual typedef mapping like `"math_VectorBase<>" = "math_Vector"` would help when that template appears in the parsed output.
> 
> For the `Message_Report` case with `NCollection_IndexedMap<Message_MetricType>`, there's no existing typedef to fall back on. If it's returned by const reference, the binding needs to know the type, but since it's a template specialization without a pre-existing typedef, I can't easily bind it. The practical solution is to skip the method entirely rather than try to create synthetic typedefs.
> 
> My strategy is to fix the `Type::Class("int")` bug so misresolved types stay as their display names, which means methods with unknown types get filtered out naturally. Then I can remove those excluded headers and let the rest of each class work fine. The tricky part is when clang completely mangles the display name to just "int" and loses all template information—at that point, the parser has no way to know it should be a class.
> 
> Rather than add debug output to trace what clang produces for these specific types, I'll just try removing the excluded headers and regenerating to see what actually happens.

Let me take the pragmatic approach: fix a parser bug I've identified, remove the exclusions, regenerate, and see what happens.

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- 🔄 Fix parser bug and remove exclusions
- ⬜ Regenerate and test build
- ⬜ Fix remaining issues
- ⬜ Commit changes

I found a bug in `parse_type`: when a template/namespace-scoped type (display name has `<` or `::`) has its canonical type misresolved to a primitive like `"int"`, the code at line 1416 uses the canonical, returning `Type::Class("int")`. This makes a misresolved method appear bindable (int is a valid type), leading to incorrect C++ wrapper generation.

The fix: when the canonical is a primitive name and the display had `<` or `::`, keep the display name so the method gets properly filtered as unknown.

Let me also look at what the OCCT headers actually contain and then fix both parser and bindings.toml:Now let me fix the parser bug and remove the exclusions:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1398-L1426">parser.rs -22+28</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1398:pa,1399:dz,1400:gs,1401:hk,1402:yp,1403:uo,1404:cy,1405:sg,1406:dd,1407:ej,1408:te,1409:qa,1410:ko,1411:yh,1412:iw,1413:vg,1414:bf,1415:xh,1416:jv,1417:ry,1418:cy,1419:ax",
      "content": "    if clean_name.contains(\"::\") || clean_name.contains('<') {\n        // For template types, check if this is a known typedef\n        if clean_name.contains('<') {\n            if let Some(typedef_name) = lookup_typedef(clean_name) {\n                return Type::Class(typedef_name);\n            }\n        }\n        let canonical_clean = canonical_spelling\n\n            .trim_start_matches(\"const \")\n            .trim_start_matches(\"class \")\n            .trim_start_matches(\"struct \")\n            .trim_start_matches(\"enum \")\n            .trim_end_matches(\" &\")\n            .trim_end_matches(\" *\")\n            .trim();\n        \n        // Only use canonical if it's simpler (no :: or <) AND still looks like a class name.\n        // When clang misresolves NCollection templates, canonical becomes \"int\" or another\n        // primitive — using that would produce Type::Class(\"int\") which is nonsensical. By\n        // keeping the template/namespaced spelling, type_uses_unknown_type() will properly\n        // filter methods with unresolvable types.\n        let canonical_looks_like_class = canonical_clean\n            .starts_with(|c: char| c.is_ascii_uppercase());\n        if !canonical_clean.contains(\"::\") && !canonical_clean.contains('<') && !canonical_clean.is_empty() && canonical_looks_like_class {\n            return Type::Class(canonical_clean.to_string());\n        }\n    }"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/parser.rs","startLine":1398,"endLine":1425}
```
</details>

Now remove the 4 headers from the exclude list in bindings.toml:

<details>
<summary>Reading lines 265-300 of <a href="../crates/opencascade-sys/bindings.toml#L265-L301">bindings.toml</a></summary>

```
    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

    # Message_Report: ActiveMetrics() return type NCollection_IndexedMap<Message_MetricType>
    # is misresolved to Standard_Integer by clang in batch parsing mode.
    "Message_Report.hxx",

    # math_Matrix: methods reference math_Vector (typedef of math_VectorBase<>)
    # which clang misresolves to int in batch parsing mode.
    "math_Matrix.hxx",

    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray::ConstIterator
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "TObj_TIntSparseArray.hxx",

    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param misresolved to int;
    # ReadProcessingData returns std::pair<DataMap, bitset> misresolved to int.
    "XSAlgo_ShapeProcessor.hxx",
]

# Include specific individual headers (from modules not fully included above).
include_headers = []

[general]
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
# =============================================================================
#
# The following categories of problems prevent certain OCCT headers from being
# included in the binding generation. Each category describes the root cause,
# affected headers, and potential solutions.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L271">bindings.toml -15</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "271:ik,272:kr,273:hi,274:dd,275:jh,276:sc,277:fc,278:dd,279:dt,280:zy,281:zh,282:dd,283:oq,284:jv,285:zp"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":271,"endLine":271}
```
</details>

Now let's regenerate bindings and see what happens:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (268 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 5.07s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
Loaded config: 168 modules, 0 individual headers, 18 exclusions -> 2932 headers
  Excluded 5 dependency headers after resolution
Parsing 3223 headers...
/Users/sethml/src/opencascade-rs/target/OCCT/include/OSD_WNT.hxx:23:12: fatal error: 'windows.h' file not found
  Clang parse time: 0.93s
  Collected 1302 NCollection/template typedefs

Timing summary:
  Total clang parse time: 0.93s
  Total AST visit time: 1.55s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

  Detected 100 NCollection classes with non-primitive element types
  Detected 71 NCollection classes with primitive element types
Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (2299 classes, 1042 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 10 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom2d.rs (22 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/adaptor2d.rs (3 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom.rs (41 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 31 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 4 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/math.rs (49 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/adaptor3d.rs (8 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/adv_app2_var.rs (1 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_colgp.rs (28 types, 8 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/adv_approx.rs (2 types, 1 extra)
  Wrote: /Users/sethml/s
... (truncated, 26788 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -30 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01C8UYFzfgmm98vJW7wrXa74__vscode-1771312781636/content.txt
```
<details>
<summary>Output (30 lines)</summary>

```
rans.rs (1 types, 0 extra)                                                        Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/units
.rs (20 types, 3 extra)                                                           Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/units
_api.rs (0 types, 0 extra)                                                        Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_
app.rs (1 types, 0 extra)                                                         Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xs_al
go.rs (1 types, 0 extra)                                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_
view.rs (extra types only, 1 types)                                               Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/iges_
basic.rs (extra types only, 1 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hlr_t
opo_b_rep.rs (extra types only, 1 types)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/iges_
solid.rs (extra types only, 11 types)                                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_o
pe_b_rep_build.rs (extra types only, 1 types)                                     Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
ap203.rs (extra types only, 4 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/iges_
geom.rs (extra types only, 22 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
ap214.rs (extra types only, 9 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/prox_
pnt.rs (extra types only, 1 types)                                                Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 1 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wr
... (truncated, 3785 chars total)
```
</details>

Binding generation succeeded. Let me check for the new modules:

**Terminal:**
```sh
grep -l "Message_Report\|math_Matrix\|TObj_TIntSparseArray\|XSAlgo_ShapeProcessor" crates/opencascade-sys/generated/*.rs 2>/dev/null || echo "No matches in generated files"
```
<details>
<summary>Output (12 lines)</summary>

```
crates/opencascade-sys/generated/ffi.rs
crates/opencascade-sys/generated/iges_control.rs
crates/opencascade-sys/generated/iges_to_b_rep.rs
crates/opencascade-sys/generated/igescaf_control.rs
crates/opencascade-sys/generated/math.rs
crates/opencascade-sys/generated/message.rs
crates/opencascade-sys/generated/step_control.rs
crates/opencascade-sys/generated/stepcaf_control.rs
crates/opencascade-sys/generated/t_obj.rs
crates/opencascade-sys/generated/transfer.rs
crates/opencascade-sys/generated/xs_algo.rs
crates/opencascade-sys/generated/xs_control.rs
```
</details>

Good, they're being generated. Now let's build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1
```
<details>
<summary>Output (219 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:17090:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')
warning: opencascade-sys@0.2.0:  17090 |     return self_->Row(Row);
warning: opencascade-sys@0.2.0:        |            ^~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:17093:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')
warning: opencascade-sys@0.2.0:  17093 |     return self_->Col(Col);
warning: opencascade-sys@0.2.0:        |            ^~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:116566:12: error: no viable conversion from returned value of type 'Iterator' (aka 'NCollection_SparseArray<int>::ConstIterator') to function return type 'Standard_Integer' (aka 'int')
warning: opencascade-sys@0.2.0:  116566 |     return self_->GetIterator();
warning: opencascade-sys@0.2.0:         |            ^~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:124819:16: error: no matching constructor for initialization of 'XSAlgo_ShapeProcessor'
warning: opencascade-sys@0.2.0:  124819 |     return new XSAlgo_ShapeProcessor(*theParameters, *theShapeFixParameters);
warning: opencascade-sys@0.2.0:         |                ^                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx:48:19: note: candidate constructor not viable: no known conversion from 'const Standard_Integer' (aka 'const int') to 'const ParameterMap' (aka 'const NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>') for 1st argument
warning: opencascade-sys@0.2.0:    48 |   Standard_EXPORT XSAlgo_ShapeProcessor(const ParameterMap&          theParameters,
warning: opencascade-sys@0.2.0:       |                   ^                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/XSAlgo/XSAlgo_ShapeProcessor.hxx:53:19: note: candidate constructor not viable: requires single argument 'theParameters', but 2 arguments were provided
warning: opencascade-sys@0.2.0:    53 |   Standard_EXPORT XSAlgo_ShapeProcessor(const DE_ShapeFixParameters& theParameters);
warning: opencascade-sys@0.2.0:       |
... (truncated, 16356 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Vs7J2mMMukvU7crVdKAs9i__vscode-1771312781639/content.txt && echo "---" && grep "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Vs7J2mMMukvU7crVdKAs9i__vscode-1771312781639/content.txt
```
```
1
---
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
```

**Terminal:**
```sh
grep -E "^/|error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Vs7J2mMMukvU7crVdKAs9i__vscode-1771312781639/content.txt | grep -v "generated from" | grep -v "note:"
```
<details>
<summary>Output (11 lines)</summary>

```
ade-sys/generated/wrappers.cpp:17090:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')                                                                     warning: opencascade-sys@0.2.0:  17090 |     return self_->Row(Row);
ade-sys/generated/wrappers.cpp:17093:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')                                                                     warning: opencascade-sys@0.2.0:  17093 |     return self_->Col(Col);
ade-sys/generated/wrappers.cpp:116566:12: error: no viable conversion from returned value of type 'Iterator' (aka 'NCollection_SparseArray<int>::ConstIterator') to function return type 'Standard_Integer' (aka 'int')                         warning: opencascade-sys@0.2.0:  116566 |     return self_->GetIterator();
ade-sys/generated/wrappers.cpp:124819:16: error: no matching constructor for initialization of 'XSAlgo_ShapeProcessor'                                          warning: opencascade-sys@0.2.0:  124819 |     return new XSAlgo_ShapeProcessor(*
ade-sys/generated/wrappers.cpp:124828:12: error: no viable conversion from returned value of type 'ProcessingData' (aka 'pair<NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>, bitset<Operation::Last + 1>>') to function return type 'Standard_Integer' (aka 'int')                                      warning: opencascade-sys@0.2.0:  124828 |     return XSAlgo_ShapeProcessor::Read
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
d/wrappers.cpp:17090:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')       cargo:warning= 17090 |     return self_->Row(Row);
d/wrappers.cpp:17093:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')       cargo:warning= 17093 |     return self_->Col(Col);
d/wrappers.cpp:116566:12: error: no viable conversion from returned value of type 'Iterator' (aka 'NCollection_SparseArray<int>::ConstIterator') to function return type 'Standard_Integer' (aka 'int')                                           cargo:warning= 116566 |     return self_->GetIterator();
d/wrappers.cpp:124819:16: error: no matching constructor for initialization of 'XSAlgo_ShapeProcessor'                                                            cargo:warning= 124819 |     return new XSAlgo_ShapeProcessor(*theParameters, *
d/wrappers.cpp:124828:12: error: no viable conversion from returned value of type 'ProcessingData' (aka 'pair<NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>, bitset<Operation::Last + 1>>') to function return type 'Standard_Integer' (aka 'int')                                                        cargo:warning= 124828 |     return XSAlgo_Shap
... (truncated, 3032 chars total)
```
</details>

> 💭 Now I can see exactly what's

> 💭 Now I can see exactly what's happening. The errors are:
> 
> 1. **math_Matrix::Row() and Col()** - return `math_VectorBase<>` but wrapper declares return type as `Standard_Integer` (int)
> 2. **TObj_TIntSparseArray::GetIterator()** - returns `Iterator` (NCollection_SparseArray<int>::ConstIterator) but wrapper declares return type as `Standard_Integer` (int)
> 3. **XSAlgo_ShapeProcessor constructor** - param type misresolved
> 4. **XSAlgo_ShapeProcessor::ReadProcessingData()** - returns `ProcessingData` (pair<...>) but wrapper declares return type as `Standard_Integer` (int)
> 
> 
> 5. The display names clang produces are showing these as simple types rather than templates, which means `parse_type` is returning `Type::I32` for all of them. The issue is that when clang fully misresolves the type to "int", the standard type mapping fails because it's looking for "Standard_*" prefixed names.
> 
> 6. Both the display name and canonical form are corrupted to "int", so my parser fix—which relied on the display name preserving template information—doesn't help here. The parser has no way to recover that this should be a template type when both representations are lost.
> 
> The real issue is that clang itself is losing the template information in batch mode, not just in the canonical form. I need either type hints to guide the parser, method-level exclusions to skip these problematic cases, or to figure out why clang's display name gets corrupted and fix that at the source. Looking at the pattern across these errors, template types consistently get misresolved to `Standard_Integer`, which then maps to `int` through the standard type mapping.
> 
> The most promising approach might be cross-referencing the actual C++ headers to verify method signatures when clang's type information seems unreliable.
> 
> But that's getting complicated. The practical fix is to add method exclusions to the bindings configuration for the known problematic cases like `math_Matrix.Row`, `math_Matrix.Col`, and the others. That handles the immediate issue, though it doesn't address the underlying pattern the user noticed about why these failures cluster together.
> 
> Looking at the actual errors, they all seem to stem from clang misresolving C++ template types to `Standard_Integer` when they should resolve to more complex types like `math_Vector` or custom iterators. The real systematic fix would require improving how the parser handles template type resolution, but for now the method-level exclusion approach is the most practical solution without major refactoring. Standard_Integer`/`int`. The display name also shows the misresolved type, so we can't detect it at the clang API level.
> 
> BUT: we could detect it by cross-referencing with the actual C++ header source. If a method in the C++ header returns `math_VectorBase<>` or `Iterator` or `ProcessingData`, but clang says it returns `Standard_Integer`, we know there's a misresolution. 
> 
> Actually, there's a simpler heuristic: **if a method's DISPLAY signature in the C++ header doesn't match what clang resolved, reject it.** But this requires parsing the header ourselves...
> 
> Wait, I have another idea. The key observation is that ALL misresolutions go to `Standard_Integer` (= int). This is because `Standard_Integer` is the fallback canonical type when clang can't instantiate templates. What if we detect this specific pattern? These types are all class-local typedefs or template uses—`math_VectorBase<>`, `Iterator`, `ParameterMap`, `ProcessingData`—so I could check the method declarations in the AST and see if there's a mismatch between what the declaration says and what clang resolves.
> 
> The simpler approach is to add method exclusions to the bindings config to skip known problematic methods. But the user specifically asked about type hints to guide parsing, which suggests they want something more elegant than just blacklisting methods. A type hint approach would let us specify the correct type for these parameters directly in the config, which is cleaner and more maintainable. certain class::method return types or parameters:
> ```toml
> [type_overrides]
> "math_Matrix::Row.return" = "math_Vector"
> "math_Matrix::Col.return" = "math_Vector"
> ```
> 
> This would override the clang-parsed type with the correct one. The parser would check these overrides and use them instead of clang's resolution. This is more useful than simple method exclusions because it actually ENABLES the methods instead of skipping them.
> 
> For the math_Matrix case, the methods that return math_Vector (a typedef of math_VectorBase<double>) could be bound correctly:
> - Row() returns math_Vector by value → wrap in UniquePtr
> - Col() returns math_Vector by value → wrap in UniquePtr
> 
> For TObj_TIntSparseArray::GetIterator(), the return type is a nested iterator that's fundamentally unbindable, so that needs exclusion. XSAlgo_ShapeProcessor has similar issues—the constructor parameter is a complex map type and ReadProcessingData returns std::pair, neither of which can be bound. So I need both approaches: type overrides for cases where the correct type exists and is bindable, and method exclusions for types that are inherently problematic.
> 
> Actually, I'm realizing math_Vector might not even be in the collected typedefs because it's a typedef starting with lowercase 'm', and the collection filter requires uppercase first letter and an underscore. Let me trace through the logic—the filter checks if the name starts with an uppercase letter and contains an underscore, so "math_Vector" would fail that check and wouldn't be collected. The generated output shows math_Matrix is now included, but the Row() and Col() methods that return math_VectorBase<> were misresolved to int, causing incorrect wrappers. I think the best approach is to add method exclusions in bindings.toml for unbindable types and fix the typedef map to capture lowercase OCCT aliases like math_Vector. Looking at the user's original request about the 4 problematic headers, I'm seeing errors across 2 methods in math_Matrix alone.
> 
> Now I'm counting the remaining issues: TObj_TIntSparseArray has GetIterator() returning Iterator misresolved to int, and XSAlgo_ShapeProcessor has constructor and ReadProcessingData() problems—that's 3 more methods across 2 headers. Since the user explicitly asked for a fix and mentioned type hints in bindings.toml would be acceptable, I think method-level exclusions are the cleanest solution here. These are cases where clang completely corrupts the type information, so no parser-level fix can help—we need external configuration to exclude them. I'll implement an exclude_methods option in bindings.toml to handle all these cases uniformly.
> 
> For constructors specifically, I'll need to figure out the naming convention—either "ClassName::new" or just matching by class name. Let me check the generated code to see what else is in XSAlgo_ShapeProcessor and whether excluding the whole header would be simpler than picking individual methods.

I can see exactly 5 C++ compilation errors - they're all cases where clang's display name is ALSO misresolved (not just canonical), so the parser can't detect the issue. Let me check what was generated:

**Terminal:**
```sh
grep -n "math_Matrix\|TObj_TIntSparseArray\|XSAlgo_ShapeProcessor\|Message_Report" crates/opencascade-sys/generated/wrappers.cpp | grep -E "Row|Col|GetIterator|ReadProcessingData|XSAlgo_ShapeProcessor\("
```
<details>
<summary>Output (15 lines)</summary>

```
16996:extern "C" math_Matrix* math_Matrix_ctor_int4(Standard_Integer LowerRow, Standard_Integer UpperRow, Standard_Integer LowerCol, Standard_Integer UpperCol) {
16997:    return new math_Matrix(LowerRow, UpperRow, LowerCol, UpperCol);
16999:extern "C" math_Matrix* math_Matrix_ctor_int4_real(Standard_Integer LowerRow, Standard_Integer UpperRow, Standard_Integer LowerCol, Standard_Integer UpperCol, Standard_Real InitialValue) {
17000:    return new math_Matrix(LowerRow, UpperRow, LowerCol, UpperCol, InitialValue);
17035:extern "C" Standard_Real& math_Matrix_value(math_Matrix* self_, Standard_Integer Row, Standard_Integer Col) {
17089:extern "C" Standard_Integer math_Matrix_row(const math_Matrix* self_, Standard_Integer Row) {
17092:extern "C" Standard_Integer math_Matrix_col(const math_Matrix* self_, Standard_Integer Col) {
17095:extern "C" void math_Matrix_swap_row(math_Matrix* self_, Standard_Integer Row1, Standard_Integer Row2) {
17098:extern "C" void math_Matrix_swap_col(math_Matrix* self_, Standard_Integer Col1, Standard_Integer Col2) {
124819:    return new XSAlgo_ShapeProcessor(*theParameters, *theShapeFixParameters);
124822:    return new XSAlgo_ShapeProcessor(*theParameters);
124827:extern "C" Standard_Integer XSAlgo_ShapeProcessor_read_processing_data(const TCollection_AsciiString* theFileResourceName, const TCollection_AsciiString* theScopeResourceName) {
124828:    return XSAlgo_ShapeProcessor::ReadProcessingData(*theFileResourceName, *theScopeResourceName);
132978:extern "C" Standard_Integer BSplCLib_solve_banded_system_matrix_int2_array1ofpnt2d(const math_Matrix* Matrix, Standard_Integer UpperBandWidth, Standard_Integer LowerBandWidth, TColgp_Array1OfPnt2d* Array) { return BSplCLib::SolveBandedSystem(*Matrix, UpperBandWidth, LowerBandWidth, *Array); }
132979:extern "C" Standard_Integer BSplCLib_solve_banded_system_matrix_int2_array1ofpnt(const math_Matrix* Matrix, Standard_Integer UpperBandWidth, Standard_Integer LowerBandWidth, TColgp_Array1OfPnt* Array) { return BSplCLib::SolveBandedSystem(*Matrix, UpperBandWidth, LowerBandWidth, *Array); }
```
</details>

**Terminal:**
```sh
grep -n "TObj_TIntSparseArray" crates/opencascade-sys/generated/wrappers.cpp
```
<details>
<summary>Output (53 lines)</summary>

```
2474:#include <TObj_TIntSparseArray.hxx>
3545:typedef opencascade::handle<TObj_TIntSparseArray> HandleTObjTIntSparseArray;
116547:// TObj_TIntSparseArray wrappers
116550:extern "C" TObj_TIntSparseArray* TObj_TIntSparseArray_ctor() {
116551:    return new TObj_TIntSparseArray();
116553:extern "C" opencascade::handle<TDF_Attribute>* TObj_TIntSparseArray_new_empty(const TObj_TIntSparseArray* self_) {
116556:extern "C" opencascade::handle<TDF_Attribute>* TObj_TIntSparseArray_backup_copy(const TObj_TIntSparseArray* self_) {
116559:extern "C" const Standard_GUID& TObj_TIntSparseArray_id(const TObj_TIntSparseArray* self_) {
116562:extern "C" size_t TObj_TIntSparseArray_size(const TObj_TIntSparseArray* self_) {
116565:extern "C" Standard_Integer TObj_TIntSparseArray_get_iterator(const TObj_TIntSparseArray* self_) {
116568:extern "C" Standard_Boolean TObj_TIntSparseArray_has_value(const TObj_TIntSparseArray* self_, size_t theId) {
116571:extern "C" Standard_Integer TObj_TIntSparseArray_value(const TObj_TIntSparseArray* self_, size_t theId) {
116574:extern "C" void TObj_TIntSparseArray_set_value(TObj_TIntSparseArray* self_, size_t theId, Standard_Integer theValue) {
116577:extern "C" void TObj_TIntSparseArray_unset_value(TObj_TIntSparseArray* self_, size_t theId) {
116580:extern "C" void TObj_TIntSparseArray_clear(TObj_TIntSparseArray* self_) {
116583:extern "C" void TObj_TIntSparseArray_restore(TObj_TIntSparseArray* self_, const opencascade::handle<TDF_Attribute>* theDelta) {
116586:extern "C" void TObj_TIntSparseArray_paste(const TObj_TIntSparseArray* self_, const opencascade::handle<TDF_Attribute>* theInto, const opencascade::handle<TDF_RelocationTable>* theRT) {
116589:extern "C" void TObj_TIntSparseArray_before_commit_transaction(TObj_TIntSparseArray* self_) {
116592:extern "C" Standard_Boolean TObj_TIntSparseArray_after_undo(TObj_TIntSparseArray* self_, const opencascade::handle<TDF_AttributeDelta>* theDelta, Standard_Boolean toForce) {
116595:extern "C" void TObj_TIntSparseArray_set_do_backup(TObj_TIntSparseArray* self_, Standard_Boolean toDo) {
116598:extern "C" void TObj_TIntSparseArray_clear_delta(TObj_TIntSparseArray* self_) {
116601:extern "C" const opencascade::handle<Standard_Type>& TObj_TIntSparseArray_dynamic_type(const TObj_TIntSparseArray* self_) {
116604:extern "C" const Standard_GUID& TObj_TIntSparseArray_get_id() {
116605:    return TObj_TIntSparseArray::GetID();
116607:extern "C" const char* TObj_TIntSparseArray_get_type_name() {
116608:    return TObj_TIntSparseArray::get_type_name();
116610:extern "C" const opencascade::handle<Standard_Type>& TObj_TIntSparseArray_get_type_descriptor() {
116611:    return TObj_TIntSparseArray::get_type_descriptor();
116613:extern "C" const TDF_Attribute* TObj_TIntSparseArray_as_TDF_Attribute(const TObj_TIntSparseArray* self_) { return static_cast<const TDF_Attribute*>(self_); }
116614:extern "C" TDF_Attribute* TObj_TIntSparseArray_as_TDF_Attribute_mut(TObj_TIntSparseArray
... (truncated, 5824 chars total)
```
</details>

> 💭 I can see that:
> 
> 1. **TObj_TIntSparseArray**: Only `GetIterator()` is problematic. All other methods work fine (constructor, NewEmpty, BackupCopy, Size, HasValue, Value, SetValue, etc.)
> 
> 2. **math_Matrix**: Only `Row()` and `Col()` are problematic (return `math_VectorBase<>` misresolved to int). Other methods work fine.
> 
> 3. **XSAlgo_ShapeProcessor**: Constructor parameter and ReadProcessingData return are problematic.
> 
> 4. **Message_Report**: Seems to be fine (no errors)! The three-layer guard may have fixed it, or it might be that the methods using NCollection_IndexedMap<Message_MetricType> were properly filtered out.
>  
> Now I'm going to add a method exclusion mechanism to the config. I'll add an `exclude_methods` field to bindings.toml that takes a list of fully qualified method names, then implement the filtering logic in the codegen to skip those methods entirely. For XSAlgo_ShapeProcessor, I'll need to exclude both the problematic constructor and the static method.

**Terminal:**
```sh
grep -n "XSAlgo_ShapeProcessor" crates/opencascade-sys/generated/wrappers.cpp
```
<details>
<summary>Output (28 lines)</summary>

```
2676:#include <XSAlgo_ShapeProcessor.hxx>
100117:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& XSControl_Reader_get_shape_process_flags(const XSControl_Reader* self_) {
100763:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& IGESControl_ActorWrite_inherited_GetShapeProcessFlags(const IGESControl_ActorWrite* self) {
100945:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& IGESControl_Reader_inherited_GetShapeProcessFlags(const IGESControl_Reader* self) {
101074:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& IGESCAFControl_Reader_inherited_GetShapeProcessFlags(const IGESCAFControl_Reader* self) {
101155:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& IGESToBRep_Actor_inherited_GetProcessingFlags(const IGESToBRep_Actor* self) {
103224:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPControl_ActorRead_inherited_GetProcessingFlags(const STEPControl_ActorRead* self) {
103285:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPControl_ActorWrite_inherited_GetShapeProcessFlags(const STEPControl_ActorWrite* self) {
103412:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPControl_Reader_inherited_GetShapeProcessFlags(const STEPControl_Reader* self) {
103442:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPControl_Writer_get_shape_process_flags(const STEPControl_Writer* self_) {
103499:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPCAFControl_ActorWrite_inherited_GetShapeProcessFlags(const STEPCAFControl_ActorWrite* self) {
103782:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPCAFControl_Reader_get_shape_process_flags(const STEPCAFControl_Reader* self_) {
103863:extern "C" const XSAlgo_ShapeProcessor::ProcessingFlags& STEPCAFControl_Writer_get_shape_process_flags(const STEPCAFControl_Writer* self_) {
124815:// XSAlgo_ShapeProcessor wrappers
124818:extern "C" XSAlgo_ShapeProcessor* XSAlgo_ShapeProcessor_ctor_int_shapefixparameters(const Standard_Integer* theParameters, const DE_ShapeFixParameters* theShapeFixParameters) {
124819:    return new XSAlgo_ShapeProcessor(*theParameters, *theShapeFixParameters);
124821:extern "C" XSAlgo_ShapeProcessor* XSAlgo_ShapeProcessor_ctor_shapefixparameters(const DE_ShapeFixParameters* theParameters) {
124822:    return new XSAlgo_ShapeProcessor(*theParameters);
124824:extern "C" Standard_Boolean XSAlgo_ShapeProcessor_check_p_curve(const TopoDS_Edge* theEdge, const TopoDS_Face* theFace, Standard_Real thePrecision, Standard_Boolean theIsSeam) {
124825:    return XSAlgo_ShapeProcessor::CheckPCurve(*theEdge, *theFace, thePrecision, theIsSeam);
124827:extern "C" Standard_Integer XSAlgo_ShapeProcessor_read_processing_data(const TCollection_AsciiString* theFileResourceName, const TCollection_AsciiString* theScopeResourceName) {
124828:    return XSAlgo_ShapeProcessor::ReadProcessingData(*theFileResourceName, *theScopeResourceName);
124830:extern "C" void XSAlgo_ShapeProcessor_prepare_for_transf
... (truncated, 3646 chars total)
```
</details>

> 💭 I see that XSAlgo_ShapeProcessor has:
> 1. `XSAlgo_ShapeProcessor_ctor_int_shapefix` - constructor with misresolved ParameterMap type → ERROR  
> 2. `XSAlgo_ShapeProcessor_ctor_shapefixparameters` - constructor with only DE_ShapeFixParameters → this one seems wrong too. Wait... `XSAlgo_ShapeProcessor(*theParameters)` where theParameters is `DE_ShapeFixParameters*`... but the C++ constructor requires ParameterMap + optional DE_ShapeFixParameters. So this constructor must be the default ParameterMap one, but with wrong number of args.
> 
> 
> 3. The first constructor is taking a `Standard_Integer*` instead of the NCollection_DataMap that should represent the ParameterMap, which will definitely fail at runtime.
> 4. The second constructor at line 124821-124822 takes only `DE_ShapeFixParameters*` and calls `new XSAlgo_ShapeProcessor(*theParameters)`, but I need to verify if the C++ class actually has a constructor that accepts just the ShapeFixParameters without the ParameterMap first.
> 
> Looking at the actual C++ declaration, the constructor signature is `XSAlgo_ShapeProcessor(const ParameterMap& theParameters, const DE_ShapeFixParameters& theShapeFixParameters = {})` — so the ParameterMap is always required as the first parameter. This means both generated constructors are incorrect since they're missing that required first argument. The error report only flagged line 124819, but line 124822 should also fail for the same reason.
> 
> The compiler likely hit its error limit and stopped reporting after five errors. So my approach is to add an `exclude_methods` configuration to bindings.toml and exclude the problematic methods: `math_Matrix::Row` and `math_Matrix::Col` which return `math_VectorBase<>`, `TObj_TIntSparseArray::GetIterator` which returns an iterator type, and the `XSAlgo_ShapeProcessor` constructor variants.
> 
> For the exclusion format, I'll use "ClassName::MethodName" where constructors are represented as "ClassName::ClassName" since that's the C++ constructor naming convention. Let me implement this approach.
> 
> Actually, I'm reconsidering—maybe instead of method-level exclusions, I could use a type blacklist where any method containing certain problematic types in its signature gets automatically excluded, which would be cleaner than maintaining a manual list.
> 
> But wait, that won't work because by the time we're at codegen, the C++ type names have already been resolved to internal representations, so I can't match against the original type strings.
> 
> So method exclusions it is—I'll add a config option that lets me explicitly list methods to skip, which is straightforward and doesn't require complex type matching logic.
> 
> Now I need to wire this through: parse the exclude_methods list from the config, pass it to the codegen phase, and check against it in the bindability functions for methods, static methods, and constructors.
> 
> Actually, let me double-check those other XSAlgo_ShapeProcessor methods that reference ProcessingFlags—they seem to compile fine since ProcessingFlags is a nested type that's actually accessible.
> 
> Implementing the config parsing and bindability checks... at the full error output to see what the compiler actually reported for each line.

Let me see the full error output and check for constructor issues:

**Terminal:**
```sh
grep -c "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Vs7J2mMMukvU7crVdKAs9i__vscode-1771312781639/content.txt
```
```
11
```

**Terminal:**
```sh
grep "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01Vs7J2mMMukvU7crVdKAs9i__vscode-1771312781639/content.txt
```
<details>
<summary>Output (11 lines)</summary>

```
ade-sys/generated/wrappers.cpp:17090:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')                                                                     warning: opencascade-sys@0.2.0:  17090 |     return self_->Row(Row);
ade-sys/generated/wrappers.cpp:17093:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')                                                                     warning: opencascade-sys@0.2.0:  17093 |     return self_->Col(Col);
ade-sys/generated/wrappers.cpp:116566:12: error: no viable conversion from returned value of type 'Iterator' (aka 'NCollection_SparseArray<int>::ConstIterator') to function return type 'Standard_Integer' (aka 'int')                         warning: opencascade-sys@0.2.0:  116566 |     return self_->GetIterator();
ade-sys/generated/wrappers.cpp:124819:16: error: no matching constructor for initialization of 'XSAlgo_ShapeProcessor'                                          warning: opencascade-sys@0.2.0:  124819 |     return new XSAlgo_ShapeProcessor(*
ade-sys/generated/wrappers.cpp:124828:12: error: no viable conversion from returned value of type 'ProcessingData' (aka 'pair<NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>, bitset<Operation::Last + 1>>') to function return type 'Standard_Integer' (aka 'int')                                      warning: opencascade-sys@0.2.0:  124828 |     return XSAlgo_ShapeProcessor::Read
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
d/wrappers.cpp:17090:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')       cargo:warning= 17090 |     return self_->Row(Row);
d/wrappers.cpp:17093:12: error: no viable conversion from returned value of type 'math_VectorBase<>' to function return type 'Standard_Integer' (aka 'int')       cargo:warning= 17093 |     return self_->Col(Col);
d/wrappers.cpp:116566:12: error: no viable conversion from returned value of type 'Iterator' (aka 'NCollection_SparseArray<int>::ConstIterator') to function return type 'Standard_Integer' (aka 'int')                                           cargo:warning= 116566 |     return self_->GetIterator();
d/wrappers.cpp:124819:16: error: no matching constructor for initialization of 'XSAlgo_ShapeProcessor'                                                            cargo:warning= 124819 |     return new XSAlgo_ShapeProcessor(*theParameters, *
d/wrappers.cpp:124828:12: error: no viable conversion from returned value of type 'ProcessingData' (aka 'pair<NCollection_DataMap<TCollection_AsciiString, TCollection_AsciiString>, bitset<Operation::Last + 1>>') to function return type 'Standard_Integer' (aka 'int')                                                        cargo:warning= 124828 |     return XSAlgo_Shap
... (truncated, 3032 chars total)
```
</details>

> 💭 So 5 unique errors (each shown twice - once from the C++ compiler and once in the cargo error summary):
> 
> 1. `math_Matrix::Row()` → returns `math_VectorBase<>`, wrapper expects `Standard_Integer`
> 2. `math_Matrix::Col()` → returns `math_VectorBase<>`, wrapper expects `Standard_Integer`
> 3. `TObj_TIntSparseArray::GetIterator()` → returns `Iterator` (NCollection_SparseArray::ConstIterator), wrapper expects `Standard_Integer`
> 4. `XSAlgo_ShapeProcessor` constructor → `Standard_Integer` where it should be `ParameterMap`
> 
> 
> 5. `XSAlgo_ShapeProcessor::ReadProcessingData()` → returns `ProcessingData` (pair<...>), wrapper expects `Standard_Integer`
> 
> Looking at the second constructor at line 124822, it takes a `DE_ShapeFixParameters` pointer and tries to construct `XSAlgo_ShapeProcessor` by dereferencing it, but the actual constructor signature requires a `ParameterMap` reference plus an optional `DE_ShapeFixParameters` parameter—so there's a mismatch in how the arguments are being passed.
> 
> The compiler probably hit its error limit at 5 and stopped reporting, which is why the 6th error at line 124822 didn't show up. Rather than chase down each individual signature mismatch, I'll add an `exclude_methods` field to the bindings config that lets me specify methods and constructors to skip during code generation, using a format like `"ClassName::MethodName"` for methods and `"ClassName::ctor"` for constructors.

5 unique C++ compilation errors. All are cases where clang's display name is also misresolved so the parser can't detect it. I'll implement method-level exclusions in bindings.toml to skip these specific methods while keeping the rest of each class.

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- 🔄 Implement method exclusions
- ⬜ Add exclusions to bindings.toml
- ⬜ Regenerate and test build
- ⬜ Commit changes

Let me look at how the config flows through to codegen to understand where to hook in the exclusions:

Trace config flow through codegen

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/main.rs">main.rs</a></summary>

```
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
    let (explicit_headers, resolve_deps, exclude_set) = if let Some(ref config_path) = args.config {
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
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            pri
... (truncated, 38308 chars)
```
</details>

Searched for files matching `**/crates/opencascade-binding-generator/src/codegen/mod.rs`, 1 match
- [mod.rs](../crates/opencascade-binding-generator/src/codegen/mod.rs)

<details>
<summary>Searched for regex <code>is_method_bindable|compute_class_bindings|TypeContext|BindingConfig</code> (<code>**/crates/opencascade-binding-generator/src/**/*.rs</code>), 20 results</summary>

- [config.rs:11](../crates/opencascade-binding-generator/src/config.rs#L11)
- [config.rs:50](../crates/opencascade-binding-generator/src/config.rs#L50)
- [config.rs:53](../crates/opencascade-binding-generator/src/config.rs#L53)
- [config.rs:65](../crates/opencascade-binding-generator/src/config.rs#L65)
- [type_mapping.rs:281](../crates/opencascade-binding-generator/src/type_mapping.rs#L281)
- [type_mapping.rs:358](../crates/opencascade-binding-generator/src/type_mapping.rs#L358)
- [type_mapping.rs:442](../crates/opencascade-binding-generator/src/type_mapping.rs#L442)
- [bindings.rs:12](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L12)
- [bindings.rs:747](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L747)
- [bindings.rs:749](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L749)
- [bindings.rs:882](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L882)
- [bindings.rs:889](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L889)
- [bindings.rs:920](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L920)
- [bindings.rs:960](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L960)
- [bindings.rs:993](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L993)
- [bindings.rs:1059](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1059)
- [bindings.rs:1100](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1100)
- [bindings.rs:1133](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1133)
- [bindings.rs:1177](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1177)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_0133Bgb3VwGS95tH8vaKiGU1__vscode-1771312781649/content.txt#1-1), lines 1 to 500

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/codegen/mod.rs">mod.rs</a></summary>

```
//! Code generation for extern "C" FFI and C++ wrappers

pub mod bindings;
pub mod collections;
pub mod cpp;
pub mod rust;

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/config.rs">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Expand the configuration into a list of header file paths.
///
/// - For each module in `modules`, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Expand modules: find all headers matching {Module}.hxx and {Module}_*.hxx
    for module in &config.modules {
        let mut module_headers = Vec::new();

        // Look for {Module}.hxx
        let main_header = format!("{}.hxx", module);
        let main_path = occt_include_dir.join(&main_header);
        if main_path.exists() {
            module_headers.push((main_header.clone(), main_path));
        }

        // Look for {Module}_*.hxx
        let prefix = format!("{}_", module);
        let entries = std::fs::read_dir(occt_include_dir)
            .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".hxx") {
                module_headers.push((filename, entry.path()));
            }
        }

        module_headers.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, path) in module_headers {
            if seen.insert(name) {
                headers.push(path);
            }
        }
    }

    // 2. Add individual headers
    for header_name in &config.include_headers {
        if seen.insert(header_name.clone()) {
            let path = occt_include_dir.join(header_name);
            if path.exists() {
                headers.push(path);
            } else {
                eprintln!("Warning: Header not found: {}", path.display());
            }
        }
    }

    // 3. Remove excluded headers
    if !config.exclude_headers.is_empty() {
        let exclude_set: std::collections::HashSet<&str> =
            config.exclude_headers.iter().map(|s| s.as_str()).collect();
        headers.reta
... (truncated, 4177 chars)
```
</details>

<details>
<summary>Reading lines 270-360 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L270-L361">type_mapping.rs</a></summary>

```

/// Extract short class name (without module prefix)
pub fn extract_short_class_name(class_name: &str) -> String {
    if let Some(underscore_pos) = class_name.find('_') {
        class_name[underscore_pos + 1..].to_string()
    } else {
        class_name.to_string()
    }
}

/// Context for type mapping within a specific module
pub struct TypeContext<'a> {
    /// The current module name (e.g., "gp")
    pub current_module: &'a str,
    /// Classes defined in the current module (full C++ names like "gp_Pnt")
    pub module_classes: &'a std::collections::HashSet<String>,
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type && !has_protected_destructor)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_class
... (truncated, 4260 chars)
```
</details>

<details>
<summary>Reading lines 980-1200 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L980-L1201">bindings.rs</a></summary>

```
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────

/// Common filter for instance methods (both direct and wrapper)
/// Methods that cause ambiguous overload errors due to multiple inheritance.
/// Format: (class_name, method_name)
/// TODO: Add to bindings.toml or fix in some other way.
const AMBIGUOUS_METHODS: &[(&str, &str)] = &[
    ("BOPAlgo_ParallelAlgo", "Perform"),
];

fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if resolver::method_has_string_ref_param(method).is_some() {
        return false;
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return false;
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if resolver::method_needs_explicit_lifetimes(method) {
        return false;
    }
    if method
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if method.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return false;
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return false;
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return false;
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return false;
        }
    }
    true
}

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> bool {
    // By-value class/handle params are now supported: C++ wrappers accept const T&
    // and the C++ compiler handles the copy.
    if ctor.has_unbindable_types() {
        return false;
    }
    if ctor
        .params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    // Also check for unknown class types in parameters.
    // This catches NCollection typedef types (e.g., TDF_LabelMap) that aren't
    // declared in the extern "C" FFI.
    if ctor
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Skip constructors where a nullable pointer param's inner type is unknown
    if ctor.params.iter().any(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return fals
... (truncated, 8102 chars)
```
</details>

<details>
<summary>Reading lines 1-120 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1-L121">resolver.rs</a></summary>

```
//! Symbol resolution and binding decision making
//!
//! This module implements Pass 1 of the two-pass architecture:
//! Parse all headers and build a complete `SymbolTable` containing every symbol
//! we'll wrap, with all derived information pre-computed.
//!
//! Benefits:
//! - Single source of truth for filtering (computed once, used by both rust.rs and cpp.rs)
//! - Pre-computed naming (rust_ffi_name, rust_public_name, cpp_name)
//! - Better cross-module support (all symbols known before code generation)
//! - Debuggability (can dump symbol table to inspect what will be generated)

use crate::model::{Constructor, Method, ParsedClass, ParsedEnum, ParsedFunction, Param, StaticMethod, Type};
use crate::module_graph::{CrossModuleType, Module, ModuleGraph};
use crate::type_mapping::safe_short_name;
use heck::ToSnakeCase;
use std::collections::{HashMap, HashSet};

/// Unique identifier for a symbol in the symbol table
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub String);

impl SymbolId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The kind of symbol being bound
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// A C++ class
    Class,
    /// A class constructor
    Constructor,
    /// An instance method
    Method,
    /// A static method  
    StaticMethod,
    /// A free function (namespace-level)
    Function,
    /// An enum type
    Enum,
    /// A Handle<T> type
    HandleType,
}

/// Why a symbol is excluded from binding generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExclusionReason {
    /// Method uses an enum type that can't be bound (enum class requires integer conversion at FFI boundary)
    UsesEnum { enum_name: String },
    /// Class is abstract (has pure virtual methods)
    AbstractClass,
    /// Class has protected/private destructor
    ProtectedDestructor,
    /// Method needs explicit lifetimes (&mut self return with reference params)
    NeedsExplicitLifetimes,
    /// Method has unsupported by-value parameter (class or handle type)
    UnsupportedByValueParam { param_name: String, type_name: String },
    /// Method has const/mut return mismatch
    ConstMutReturnMismatch,
    /// Type is unbindable (streams, void pointers, arrays, etc.)
    UnbindableType { description: String },
    /// Constructor has unbindable types
    UnbindableConstructor,
    /// Static method has unbindable types
    UnbindableStaticMethod,
    /// Function has unbindable types
    UnbindableFunction,
    /// Function references Handle types for classes without Handle declarations
    UnknownHandleType,
    /// Method has const char*& or const char* const& parameter (needs manual binding)
    StringRefParam { param_name: String, type_name: String },
}

/// Binding status for a symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingStatus {
    /// Will be generated
    Included,
    /// Skipped with reason
    Excluded(ExclusionReason),
}

impl BindingStatus {
    pub fn is_included(&self) -> bool {
        matches!(self, BindingStatus::Included)
    }
    
    pub fn is_excluded(&self) -> bool {
        !self.is_included()
    }
}

/// Information about a resolved class
#[derive(Debug, Clone)]
pub struct ResolvedClass {
    /// Symbol ID for this class
    pub id: SymbolId,
    /// C++ fully qualified name (e.g., "gp_Pnt")
    pub cpp_name: String,
    /// Rust module this belongs to (e.g., "gp")
    pub rust_module: String,
    /// Rust FFI type name with escaping (e.g., "Pnt", "Vec_")
    pub rust_ffi_name: String,
    /// Rust public name for re-exports (e.g., "Vec" when ffi name is "Vec_")
    pub rust_public_name: String,
    /// Source header file
    pub source_header: String,
    /// Documentation comment
    pub doc_comment: Option<String>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this is a Handle type
    pub is_handle_type: bo
... (truncated, 4003 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01X3azBfPvayBaeZSHt9ZBrc__vscode-1771312781656/content.txt)

<details>
<summary>Reading lines 120-300 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L120-L301">resolver.rs</a></summary>

```
    pub is_handle_type: bool,
    /// Whether this class is abstract
    pub is_abstract: bool,
    /// Whether this class has a protected destructor
    pub has_protected_destructor: bool,
    /// Base classes (C++ names)
    pub base_classes: Vec<String>,
    /// Constructor symbol IDs
    pub constructors: Vec<SymbolId>,
    /// Method symbol IDs
    pub methods: Vec<SymbolId>,
    /// Static method symbol IDs
    pub static_methods: Vec<SymbolId>,
}

/// Information about a resolved constructor
#[derive(Debug, Clone)]
pub struct ResolvedConstructor {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Rust function name (e.g., "new", "new_real3")
    pub rust_name: String,
    /// C++ wrapper function name
    pub cpp_wrapper_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Binding status
    pub status: BindingStatus,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved method
#[derive(Debug, Clone)]
pub struct ResolvedMethod {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Original C++ method name
    pub cpp_name: String,
    /// Rust method name (snake_case)
    pub rust_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this method needs a C++ wrapper (returns by value)
    pub needs_wrapper: bool,
    /// C++ wrapper function name (if needs_wrapper)
    pub cpp_wrapper_name: Option<String>,
    /// Documentation comment
    pub doc_comment: Option<String>,
    /// Source line number in the header file
    pub source_line: Option<u32>,
}

/// Information about a resolved static method
#[derive(Debug, Clone)]
pub struct ResolvedStaticMethod {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Original C++ method name
    pub cpp_name: String,
    /// Rust method name (snake_case)
    pub rust_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this method needs a C++ wrapper
    pub needs_wrapper: bool,
    /// C++ wrapper function name (if needs_wrapper)
    pub cpp_wrapper_name: Option<String>,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved free function
#[derive(Debug, Clone)]
pub struct ResolvedFunction {
    /// Symbol ID
    pub id: SymbolId,
    /// C++ fully qualified name (e.g., "TopoDS::Edge")
    pub cpp_name: String,
    /// C++ short name without namespace (e.g., "Edge")
    pub short_name: String,
    /// Namespace (e.g., "TopoDS")
    pub namespace: String,
    /// Rust module
    pub rust_module: String,
    /// Rust function name (base, before dedup)
    pub rust_name: String,
    /// Deduplicated Rust FFI function name (unique across the entire FFI module)
    pub rust_ffi_name: String,
    /// C++ wrapper function name (used in both #[cxx_name] and wrappers.hxx)
    pub cpp_wrapper_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Source header
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved enum
#[derive(Debug, Clone)]
pub struct ResolvedEnum {
    /// Symbol ID
    pub id: SymbolId,
    /// C++ enum name
    pub cpp_name: String,
    /// Rust module
    pub rust_module: String,
    /// Rust type name
    pub rust_name: String,
    /// Source header
    pub so
... (truncated, 5557 chars)
```
</details>

Searched for regex `pub fn build_symbol_table|compute_class_bindings|is_method_bindable` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:758](../crates/opencascade-binding-generator/src/resolver.rs#L758)

Searched for regex `pub fn compute_class_bindings|fn compute_class_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1561](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1561)

<details>
<summary>Reading lines 758-920 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L758-L921">resolver.rs</a></summary>

```
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Compute handle-able classes (inherit from Standard_Transient and no protected destructor)
    let mut handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
                    set.insert(name.clone());
                }
            }
            crate::model::Type::ConstRef(inner)
            | crate::model::Type::MutRef(inner)
            | crate::model::Type::ConstPtr(inner)
            | crate::model::Type::MutPtr(inner)
            | crate::model::Type::RValueRef(inner) => {
                collect_handle_types(inner, set);
            }
            _ => {}
        }
    }
    for class in all_classes {
        for method in &class.methods {
            for param in &method.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
            if let Some(ret) = &method.return_type {
                collect_handle_types(ret, &mut handle_able_classes);
            }
        }
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
        }
    }
    // Also scan standalone functions
    for func in all_functions {
        for param in &func.params {
            collect_handle_types(&param.ty, &mut handle_able_classes);
        }
        if let Some(ret) = &func.return_type {
            collect_handle_types(ret, &mut handle_able_classes);
        }
    }
    // These referenced Handle types also need to be known class names
    all_class_names.extend(handle_able_classes.iter().cloned());
    
    // Build authoritative type→module mapping from parsed header data.
    // This is the single source of truth for module membership.
    let type_to_module: HashMap<String, String> = all_classes
        .iter()
        .map(|c| (c.name.clone(), c.module.clone()))
        .chain(all_enums.iter().map(|e| (e.name.clone(), e.module.clone())))
        .collect();
    // Keep a reference copy for resolve_* functions (avoids borrow conflicts with table)
    let type_to_module_ref = type_to_module.clone();
    
    let mut table = SymbolTable {
        classes: HashMap::new(),
        constructors: HashMap::new(),
        methods: HashMap::new(),
        static_methods: HashMap::new(),
        functions: HashMap::new(),
        enums: HashMap::new(),
        classes_by_module: HashMap::new(),
        functions_by_module: HashMap::new(),
        enums_by_module: HashMap::new(),
        all_enum_names: all_enum_names.clone(),
        all_class_names: all_class_names.clone(),
        handle_able_classes: handle_able_classes.clone(),
        cross_module_types: HashMap::new(),
        type_to_module,
        enum_rust_types: H
... (truncated, 6939 chars)
```
</details>

<details>
<summary>Reading lines 1561-1700 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1561-L1701">bindings.rs</a></summary>

```
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    ncollection_element_types: &HashMap<String, String>,
    ncollection_primitive_classes: &HashSet<String>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    // ── Constructors ────────────────────────────────────────────────────
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Wrapper methods (by-value return, const char*) ──────────────────
    let wrapper_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = Has
... (truncated, 6173 chars)
```
</details>

Searched for regex `fn resolve_class` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:934](../crates/opencascade-binding-generator/src/resolver.rs#L934)

<details>
<summary>Reading lines 920-1160 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L920-L1161">resolver.rs</a></summary>

```
    
    // Resolve all free functions
    for func in all_functions {
        resolve_function(&mut table, func, &all_enum_names, &all_class_names, &handle_able_classes, &type_to_module_ref);
    }
    
    // Note: Function naming (rust_ffi_name, cpp_wrapper_name) is now handled by
    // compute_all_function_bindings() in bindings.rs. The placeholder names set
    // during resolve_function() are no longer used by emitters.
    
    table
}

/// Resolve a single class and its members
fn resolve_class(
    table: &mut SymbolTable,
    class: &ParsedClass,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) {
    let class_id = SymbolId::new(format!("class::{}", class.name));
    let rust_module = crate::module_graph::module_to_rust_name(&class.module);
    let short_name = crate::type_mapping::short_name_for_module(&class.name, &class.module);
    let rust_ffi_name = safe_short_name(&short_name);
    
    // Determine class binding status
    let class_status = if class.has_protected_destructor {
        BindingStatus::Excluded(ExclusionReason::ProtectedDestructor)
    } else {
        BindingStatus::Included
    };
    
    // Resolve constructors
    let mut constructor_ids = Vec::new();
    for (idx, ctor) in class.constructors.iter().enumerate() {
        let ctor_id = SymbolId::new(format!("ctor::{}::{}", class.name, idx));
        let resolved_ctor = resolve_constructor(
            &ctor_id,
            &class_id,
            &class.name,
            ctor,
            idx,
            class.is_abstract,
            all_enum_names,
            type_to_module,
        );
        constructor_ids.push(ctor_id.clone());
        table.constructors.insert(ctor_id, resolved_ctor);
    }
    
    // Resolve methods
    let mut method_ids = Vec::new();
    for (idx, method) in class.methods.iter().enumerate() {
        let method_id = SymbolId::new(format!("method::{}::{}::{}", class.name, method.name, idx));
        let resolved_method = resolve_method(
            &method_id,
            &class_id,
            &class.name,
            method,
            all_enum_names,
            type_to_module,
        );
        method_ids.push(method_id.clone());
        table.methods.insert(method_id, resolved_method);
    }
    
    // Resolve static methods
    let mut static_method_ids = Vec::new();
    for (idx, method) in class.static_methods.iter().enumerate() {
        let method_id = SymbolId::new(format!("static::{}::{}::{}", class.name, method.name, idx));
        let resolved_method = resolve_static_method(
            &method_id,
            &class_id,
            &class.name,
            method,
            all_enum_names,
            type_to_module,
        );
        static_method_ids.push(method_id.clone());
        table.static_methods.insert(method_id, resolved_method);
    }
    
    let resolved = ResolvedClass {
        id: class_id.clone(),
        cpp_name: class.name.clone(),
        rust_module: rust_module.clone(),
        rust_ffi_name,
        rust_public_name: short_name.to_string(),
        source_header: class.source_header.clone(),
        doc_comment: class.comment.clone(),
        status: class_status,
        is_handle_type: class.is_handle_type,
        is_abstract: class.is_abstract,
        has_protected_destructor: class.has_protected_destructor,
        base_classes: class.base_classes.clone(),
        constructors: constructor_ids,
        methods: method_ids,
        static_methods: static_method_ids,
    };
    
    table.classes_by_module
        .entry(rust_module)
        .or_default()
        .push(class_id.clone());
    table.classes.insert(class_id, resolved);
}

/// Resolve a constructor
fn resolve_constructor(
    id: &SymbolId,
    class_id: &SymbolId,
    class_name: &str,
    ctor: &Constructor,
    _idx: usize,
    is_abstract: bool,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) -> ResolvedConstructor {
    //
... (truncated, 7967 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_0111kmC46r4VREEbB3c1yg8u__vscode-1771312781665/content.txt)

> 💭 Now I have a clear picture. Let me implement the method exclusions. I'll take the simplest effective approach:
> 
> 1. Add `exclude_methods` to `BindingConfig` in config.rs
> 2. Pass it through to the codegen layer (simplest: add to TypeContext or pass separately)
> 3. Check it in `is_method_bindable`, `is_static_method_bindable`, `is_constructor_bindable`
> 
> Since there are two parallel paths (resolver and codegen), I should add the check in both. But for simplicity, let me focus on the codegen path since that's where the final filtering happens.
> 
> 
> Now I need to trace through the actual function signatures to understand how to thread the config through main.rs and into the binding checks for methods, constructors, and static methods.

Now let me read the key files I need to modify:

<details>
<summary>Reading lines 1-170 of <a href="../crates/opencascade-binding-generator/src/main.rs#L1-L171">main.rs</a></summary>

```
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
    let (explicit_headers, resolve_deps, exclude_set) = if let Some(ref config_path) = args.config {
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
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            pri
... (truncated, 6225 chars)
```
</details>

<details>
<summary>Reading lines 170-350 of <a href="../crates/opencascade-binding-generator/src/main.rs#L170-L351">main.rs</a></summary>

```

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

    // Build symbol table (Pass 1 of two-pass architecture)
    // This resolves all symbols and makes binding decisions ONCE
    let ordered_modules = graph.modules_in_order();
    let symbol_table = resolver::build_symbol_table(
        &ordered_modules,
        &graph,
        &all_classes,
        &all_enums,
        &all_functions,
        &collection_type
... (truncated, 7749 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_019yHdAXci17hJm4D8dRcVKm__vscode-1771312781668/content.txt)

> 💭 Now I understand the architecture. The config is loaded in main.rs, but currently only the header-level info is extracted and the rest is dropped. I need to:
> 
> 1. Add `exclude_methods` to `BindingConfig` 
> 2. Thread it through to either the resolver's `build_symbol_table()` or the codegen's `compute_class_bindings()` or `is_method_bindable()`
> 
> Looking at the code, there's already a hardcoded `AMBIGUOUS_METHODS` array in bindings.rs that does exactly what I need. Let me find it:
> ```rust
> if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
>     return false;
> }
> ```
> 
> The cleanest approach would be to add `exclude_methods` to the bindings.toml config, extract it in main.rs after loading, and pass it into `build_symbol_table()` which already accepts multiple parameters. Then in the resolver, I'd mark methods matching the exclusion set as Excluded, and that status would propagate through to codegen. The symbol table becomes the primary source of truth for which methods should be excluded.
> 
> Actually, the resolver already has method exclusion logic in place, so I could just add the config-based exclusions there. But that requires threading the config through to the resolver. The simpler path might be to let `build_symbol_table()` return first, then iterate through the symbol table afterward and mark the excluded methods directly. This avoids changing function signatures. Let me check what parameters `build_symbol_table` currently takes...
> 
> I could add the excluded methods as a parameter, but there's a complication: `is_method_bindable` in bindings.rs runs as a separate filter independent of the symbol table, so both need to reject the method. Let me look at how `compute_class_bindings` works to understand the flow better.

<details>
<summary>Reading lines 1550-1700 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1550-L1701">bindings.rs</a></summary>

```

    // If any pure virtual method is not overridden, the class is abstract
    all_pvms.iter().any(|pvm| !all_concrete.contains(pvm))
}

// ── Main compute function ───────────────────────────────────────────────────

/// Compute all binding decisions for a class.
///
/// This is the SINGLE place where filtering, naming, overload suffixes,
/// and used_names conflict resolution happen.
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    ncollection_element_types: &HashMap<String, String>,
    ncollection_primitive_classes: &HashSet<String>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    // ── Constructors ────────────────────────────────────────────────────
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Wrapper methods (by-value return, const char*) ──────────────────
    let wrapper_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && needs_
... (truncated, 6563 chars)
```
</details>

Now let me find `AMBIGUOUS_METHODS` and the resolver's signature:

Searched for text `AMBIGUOUS_METHODS` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:989](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L989)
- [bindings.rs:1002](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1002)

<details>
<summary>Reading lines 985-1005 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L985-L1006">bindings.rs</a></summary>

```
/// Common filter for instance methods (both direct and wrapper)
/// Methods that cause ambiguous overload errors due to multiple inheritance.
/// Format: (class_name, method_name)
/// TODO: Add to bindings.toml or fix in some other way.
const AMBIGUOUS_METHODS: &[(&str, &str)] = &[
    ("BOPAlgo_ParallelAlgo", "Perform"),
];

fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if resolver::method_has_string_ref_param(method).is_some() {
        return false;
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return false;
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
```
</details>

> 💭 I see there's already a hardcoded `AMBIGUOUS_METHODS` array with a TODO to move it to bindings.toml. The simplest approach is to:
> 
> 1. Add `exclude_methods` to `BindingConfig`
> 2. Pass it into `compute_class_bindings` (or through TypeContext)
> 3. Add the check in `is_method_bindable`, `is_static_method_bindable`, and `is_constructor_bindable`
> 
> But I also need to handle the constructor case. Let me think about the format:
> 
> ```toml
> exclude_methods = [
>     "math_Matrix::Row",        # instance method
>     "math_Matrix::Col",        # instance method  
>     "TObj_TIntSparseArray::GetIterator",  # instance method
>     "XSAlgo_ShapeProcessor::XSAlgo_ShapeProcessor",  # constructor
>     "XSAlgo_ShapeProcessor::ReadProcessingData",      # static method
> ]
> ```
> 
> For constructors, using the C++ convention where the constructor name is the class name works well. Now for threading, the simplest approach given the existing architecture is to add it to `compute_class_bindings` as a parameter and check it in the various `is_*_bindable` functions.
> 
> The issue is that many of these are module-level functions without access to the exclusion set. I could move the ambiguous methods into a lazy-loaded global or pass the set through, but globals feel messy. The cleanest solution would be to add the exclusion set directly to `TypeContext` since it's already the context bag passed to all the filter functions—let me look at what TypeContext contains.

Searched for text `pub struct TypeContext` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:281](../crates/opencascade-binding-generator/src/type_mapping.rs#L281)

<details>
<summary>Reading lines 278-310 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L278-L311">type_mapping.rs</a></summary>

```
}

/// Context for type mapping within a specific module
pub struct TypeContext<'a> {
    /// The current module name (e.g., "gp")
    pub current_module: &'a str,
    /// Classes defined in the current module (full C++ names like "gp_Pnt")
    pub module_classes: &'a std::collections::HashSet<String>,
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
```
</details>

> 💭 I now have a clear plan. I'll add `exclude_methods` to TypeContext (or pass it separately). Since TypeContext is already a bag of references, adding one more field is clean. 
> 
> But actually, `is_method_bindable` takes `ctx: &TypeContext` but doesn't use it for class-level decisions. Adding method exclusions to TypeContext feels wrong - TypeContext is about TYPE resolution. 
> 
> Let me think of a better approach. Actually, the existing code has `AMBIGUOUS_METHODS` as a const array and checks it in `is_method_bindable`. The simplest change that works:
> 
> 
> I'll add the exclusion set to BindingConfig, parse it into a HashSet of method pairs, and thread it through the codegen pass into `compute_class_bindings`. Since that function already calls the bindable checks, I can pass the exclusion set down to those functions and check it alongside the existing ambiguous methods logic.

Let me check how `compute_class_bindings` is called from the code generation path:

Searched for text `compute_class_bindings(` (`**/crates/opencascade-binding-generator/src/**`), 4 results
- [bindings.rs:1561](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1561)
- [bindings.rs:2752](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2752)
- [bindings.rs:4879](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4879)
- [bindings.rs:4969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4969)

<details>
<summary>Reading lines 2730-2760 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2730-L2761">bindings.rs</a></summary>

```
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();

    // Build NCollection class element type maps for misresolution detection
    let (ncollection_element_types, ncollection_primitive_classes) = build_ncollection_element_types(all_classes);

    let class_public_info = build_class_public_info(all_classes);

    all_classes
        .iter()
        .map(|class| {
            let reexport_ctx = ReexportTypeContext {
                class_public_info: &class_public_info,
                current_module_rust: crate::module_graph::module_to_rust_name(&class.module),
            };
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, &ncollection_element_types, &ncollection_primitive_classes, Some(&reexport_ctx))
        })
        .collect()
}

// ── Free function bindings ──────────────────────────────────────────────────

/// Collect nested type names (Parent::Nested) that should be considered deletable.
/// These get destructors generated via the nested type destructor mechanism.
```
</details>

> 💭 Now I see the main codegen function. It calls `compute_class_bindings` in a tight loop. The function that builds the context and calls `compute_class_bindings` is likely `generate_all_class_bindings`. Let me find its signature.

Searched for text `fn generate_all_class_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `pub fn.*class_bindings.*all_classes` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Reading lines 2700-2720 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2700-L2721">bindings.rs</a></summary>

```
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
```
</details>

Searched for regex `^pub fn.*bindings_for_all` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Reading lines 2650-2670 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2650-L2671">bindings.rs</a></summary>

```
                    impl_method_name,
                    is_const: resolved_method.is_const,
                    params,
                    return_type,
                    cpp_method_name: resolved_method.cpp_name.clone(),
                    source_class: ancestor_name.clone(),
                    source_header: ancestor_class.source_header.clone(),
                    source_line: resolved_method.source_line,
                });
            }
        }
    }

    result
}

// ── Top-level function ──────────────────────────────────────────────────────

/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
```
</details>

<details>
<summary>Reading lines 2668-2690 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2668-L2691">bindings.rs</a></summary>

```
/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the 91 manually-specified known collections (which get generated destructors).
    // NCollection typedef names from extra_typedef_names are NOT included here.
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
```
</details>

Good. Now I have the full picture. The approach:

1. Add `exclude_methods` to `BindingConfig`
2. Pass it via `compute_all_class_bindings` →  → filter functions
3. Check it in , , `is_constructor_bindable`

Let me also check how `compute_all_class_bindings` is called from codegen output modules:

Searched for text `compute_all_class_bindings(` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [main.rs:562](../crates/opencascade-binding-generator/src/main.rs#L562)
- [bindings.rs:2671](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2671)

<details>
<summary>Reading lines 540-580 of <a href="../crates/opencascade-binding-generator/src/main.rs#L540-L581">main.rs</a></summary>

```
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
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let all_function_bindings = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers,
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
```
</details>

> 💭 Now I have a clear picture. Let me implement the changes:
> 
> 1. **config.rs**: Add `exclude_methods: Vec<String>` to `BindingConfig`
> 2. **main.rs**: Extract the exclude_methods set from config and pass it through  
> 3. **bindings.rs**: Add `exclude_methods` parameter to `compute_all_class_bindings` and `compute_class_bindings`, use it in the filter functions
> 
> For the approach, I'll:
> - Add `exclude_methods` to `BindingConfig` as `Vec<String>` (format: "ClassName::MethodName")
> 
> 
> - Parse it into a `HashSet<(String, String)>` in main.rs
> - Thread through `compute_all_class_bindings` → `compute_class_bindings`
> - In `compute_class_bindings`, filter constructors, methods, and static methods using the convention "ClassName::ClassName" to exclude all constructors
> 
> Actually, I can simplify this by just adding a `.filter()` step in `compute_class_bindings` to check against the exclusion set, without needing to modify the signatures of helper functions like `is_method_bindable`. Let me start implementing this now.

Now I have the full picture. Let me implement method exclusions across the codebase. The plan:
1. Add `exclude_methods` to config
2. Thread it through  → 
3. Filter at the iteration level (alongside )Let me make all the changes. First, the config:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/config.rs#L11-L38">config.rs -1+27</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "11:ja",
      "content": "pub struct BindingConfig {\n    /// General settings.\n    #[serde(default)]\n    pub general: GeneralConfig,\n\n    /// Include ALL headers from these OCCT modules.\n    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the\n    /// OCCT include directory will be processed.\n    #[serde(default)]\n    pub modules: Vec<String>,\n\n    /// Exclude specific headers, even if their module is included.\n    #[serde(default)]\n    pub exclude_headers: Vec<String>,\n\n    /// Include specific individual headers (from modules not fully listed in `modules`).\n    #[serde(default)]\n    pub include_headers: Vec<String>,\n\n    /// Exclude specific methods/constructors from binding generation.\n    /// Format: \"ClassName::MethodName\" for instance/static methods,\n    /// or \"ClassName::ClassName\" for constructors (C++ constructor naming convention).\n    /// Methods matching these patterns will be skipped during codegen,\n    /// even though the rest of the class is bound.\n    #[serde(default)]\n    pub exclude_methods: Vec<String>,\n}"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/config.rs","startLine":11,"endLine":37}
```
</details>

Now update `crates/opencascade-binding-generator/src/main.rs` to extract and pass the exclude_methods:

<details>
<summary>Reading lines 95-125 of <a href="../crates/opencascade-binding-generator/src/main.rs#L95-L126">main.rs</a></summary>

```
    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set) = if let Some(ref config_path) = args.config {
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
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            println!("  Expanded to {} headers", headers.len());
        }

        println!("Loaded config: {} modules, {} individual headers, {} exclusions -> {} headers",
            cfg.modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        (headers, resolve, excludes)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new())
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

```
</details>

I need to also extract `exclude_methods` from the config and thread it to the codegen. Let me update the config loading to return `exclude_methods` too:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L95-L140">main.rs -30+45</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Determine explicit headers from config file or CLI arguments\n    let (explicit_headers, resolve_deps, exclude_set, exclude_methods) = if let Some(ref config_path) = args.config {\n        let cfg = config::load_config(config_path)?;\n        let resolve = cfg.general.resolve_deps;\n\n        if args.include_dirs.is_empty() {\n            anyhow::bail!(\"--config requires at least one -I <include_dir>\");\n        }\n        let occt_include_dir = &args.include_dirs[0];\n\n        let headers = config::expand_headers(&cfg, occt_include_dir)?;\n\n        if args.verbose {\n            println!(\"Config file: {:?}\", config_path);\n            println!(\"  Modules: {:?}\", cfg.modules);\n            println!(\"  Include headers: {} entries\", cfg.include_headers.len());\n            println!(\"  Exclude headers: {} entries\", cfg.exclude_headers.len());\n            println!(\"  Exclude methods: {} entries\", cfg.exclude_methods.len());\n            println!(\"  Expanded to {} headers\", headers.len());\n        }\n\n        println!(\"Loaded config: {} modules, {} individual headers, {} exclusions -> {} headers\",\n            cfg.modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());\n\n        // Parse exclude_methods into (ClassName, MethodName) pairs\n        let method_exclusions: HashSet<(String, String)> = cfg.exclude_methods\n            .iter()\n            .filter_map(|s| {\n                let parts: Vec<&str> = s.splitn(2, \"::\").collect();\n                if parts.len() == 2 {\n                    Some((parts[0].to_string(), parts[1].to_string()))\n                } else {\n                    eprintln!(\"Warning: invalid exclude_methods entry (expected ClassName::MethodName): {}\", s);\n                    None\n                }\n            })\n            .collect();\n\n        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();\n        (headers, resolve, excludes, method_exclusions)\n    } else if !args.headers.is_empty() {\n        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), HashSet::new())\n    } else {\n        anyhow::bail!(\"Either --config <file.toml> or positional header arguments are required\");\n    };",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "95:np,96:uu,97:db,98:hg,99:dd,100:ur,101:oz,102:cy,103:wo,104:dd,105:ht,106:dd,107:ye,108:ud,109:ck,110:ex,111:ib,112:fh,113:cy,114:dd,115:ma,116:iv,117:dd,118:wk,119:bd,120:av,121:ee,122:ui,123:aw,124:jv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":95,"endLine":139}
```
</details>

Now I need to thread `exclude_methods` through to the `generate_output` function and then to . Let me find the `generate_output` function:

<details>
<summary>Reading lines 335-345 of <a href="../crates/opencascade-binding-generator/src/main.rs#L335-L346">main.rs</a></summary>

```
    } else {
        HashSet::new()
    };

    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers)
}
```
</details>

I need to pass `exclude_methods` to `generate_output`. Let me find the function definition:

Searched for text `fn generate_output(` (`**/crates/opencascade-binding-generator/src/main.rs`), 1 result
- [main.rs:547](../crates/opencascade-binding-generator/src/main.rs#L547)

<details>
<summary>Reading lines 547-590 of <a href="../crates/opencascade-binding-generator/src/main.rs#L547-L591">main.rs</a></summary>

```
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
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
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let all_function_bindings = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers,
    );

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();

    // 1. Generate ffi.rs
    println!("Generating ffi.rs...");
    let (ffi_code, nested_types) = codegen::rust::generate_ffi(
        all_classes,
```
</details>

> 💭 I need to:
> 1. Add `exclude_methods` to the `generate_output` function signature
> 2. Pass it to `compute_all_class_bindings`
> 3. Thread it through to `compute_class_bindings`
> 4. Use it in the filtering logic
> 
> Let me do all these changes. I'll also need to update the call site in main() where generate_output is called.
> 
> Let me also check the test functions that call `compute_class_bindings`:

Let me see those test call sites for :

<details>
<summary>Reading lines 4870-4905 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4870-L4906">bindings.rs</a></summary>

```
            handle_able_classes: HashSet::new(),
            cross_module_types: HashMap::new(),
            type_to_module: HashMap::new(),
            enum_rust_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("gp_Pnt".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            &HashMap::new(),
            &HashSet::new(),
            None,
        );

        assert_eq!(bindings.cpp_name, "gp_Pnt");
        assert_eq!(bindings.short_name, "Pnt");
        assert_eq!(bindings.module, "gp");
        // Non-abstract class with no explicit constructors gets a synthetic default constructor
        assert_eq!(bindings.constructors.len(), 1);
        assert_eq!(bindings.constructors[0].impl_method_name, "new");
        assert!(bindings.direct_methods.is_empty());
        assert!(bindings.wrapper_methods.is_empty());
        assert!(bindings.static_methods.is_empty());
        assert!(!bindings.has_to_handle);
        // gp module is copyable, and class is not abstract/protected
        assert!(bindings.has_to_owned);
    }

    /// Test that abstract classes don't get constructors
    #[test]
```
</details>

<details>
<summary>Reading lines 4960-4980 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4960-L4981">bindings.rs</a></summary>

```
            handle_able_classes: ["Geom_Curve".to_string()].into(),
            cross_module_types: HashMap::new(),
            type_to_module: HashMap::new(),
            enum_rust_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("Geom_Curve".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            &HashMap::new(),
            &HashSet::new(),
            None,
        );

        assert!(bindings.constructors.is_empty());
```
</details>

Now I have a clear picture of all call sites. Let me implement all changes at once. The approach: add `exclude_methods: &HashSet<(String, String)>` parameter to  and , and use it for filtering.Now let me make all the changes. I'll edit , the call sites, and  + main:

<details>
<summary>Reading lines 2668-2760 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2668-L2761">bindings.rs</a></summary>

```
/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the 91 manually-specified known collections (which get generated destructors).
    // NCollection typedef names from extra_typedef_names are NOT included here.
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
        for method in &class.methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();

    // Build NCollection class element type maps for misresolution detection
    let (ncollection_element_types, ncollection_primitive_classes) = build_ncollection_element_types(all_classes);

    let class_public_info = build_class_public_info(all_classes);

    all_classes
        .iter()
        .map(|class| {
            let reexport_ctx = ReexportTypeContext {
                class_public_info: &class_public_info,
                current_module_rust: crate::module_graph::module_to_rust_name(&class.module),
            };
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, &ncollection_element_types, &ncollection_primitive_classes, Some(&reexport_ctx))
        })
        .collect()
}

// ── Free function bindings ──────────────────────────────────────────────────

/// Col
... (truncated, 4154 chars)
```
</details>

Now I'll make all the changes. Let me do them in order:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2671-L2678">bindings.rs -6+7</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2753-L2754">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2671:dh,2672:ue,2673:ca,2674:jg,2675:td,2676:ee",
      "content": "pub fn compute_all_class_bindings(\n    all_classes: &[&ParsedClass],\n    symbol_table: &SymbolTable,\n    collection_names: &HashSet<String>,\n    extra_typedef_names: &HashSet<String>,\n    exclude_methods: &HashSet<(String, String)>,\n) -> Vec<ClassBindings> {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2752:ro",
      "content": "            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, &ncollection_element_types, &ncollection_primitive_classes, Some(&reexport_ctx), exclude_methods)"
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
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 2671,
      "endLine": 2753
    }
  ]
}
```
</details>

to accept and use `exclude_methods`:

<details>
<summary>Reading lines 1557-1575 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1557-L1576">bindings.rs</a></summary>

```
/// Compute all binding decisions for a class.
///
/// This is the SINGLE place where filtering, naming, overload suffixes,
/// and used_names conflict resolution happen.
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    ncollection_element_types: &HashMap<String, String>,
    ncollection_primitive_classes: &HashSet<String>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1561-L1572">bindings.rs -10+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "pub fn compute_class_bindings(\n    class: &ParsedClass,\n    ffi_ctx: &TypeContext,\n    symbol_table: &SymbolTable,\n    handle_able_classes: &HashSet<String>,\n    all_classes_by_name: &HashMap<String, &ParsedClass>,\n    ncollection_element_types: &HashMap<String, String>,\n    ncollection_primitive_classes: &HashSet<String>,\n    reexport_ctx: Option<&ReexportTypeContext>,\n    exclude_methods: &HashSet<(String, String)>,\n) -> ClassBindings {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1561:zr,1562:ei,1563:zw,1564:ca,1565:kf,1566:ph,1567:mn,1568:xh,1569:nj,1570:qa"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1561,"endLine":1571}
```
</details>

Now I need to add the filtering in the method/constructor/static method iteration. Let me read the constructor section and the method filter section:

<details>
<summary>Reading lines 1579-1615 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1579-L1616">bindings.rs</a></summary>

```

    // ── Constructors ────────────────────────────────────────────────────
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
```
</details>

Now let me add the filtering. I need to add exclusion checks to:
1. Constructors - filter after `compute_constructor_bindings` 
2. Direct methods - add to the `.filter()` chain
3. Wrapper methods - add to the `.filter()` chain
4. Static methods - find and add

Let me also find the static methods section:

Searched for regex `Static methods.*──|static_methods.*filter.*is_static_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:1718](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1718)
- [bindings.rs:4698](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4698)

<details>
<summary>Reading lines 1718-1740 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1718-L1741">bindings.rs</a></summary>

```
    // ── Static methods ──────────────────────────────────────────────────
    let static_methods_raw: Vec<&StaticMethod> = class
        .static_methods
        .iter()
        .filter(|m| is_static_method_bindable(m, ffi_ctx))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let static_method_names = compute_static_method_names(
        cpp_name,
        &static_methods_raw,
        &reserved_names,
        &all_instance_method_names,
    );

    let static_methods: Vec<StaticMethodBinding> = static_methods_raw
        .iter()
        .zip(static_method_names.iter())
        .map(|(method, (ffi_fn_name_base, impl_method_name))| {
            let ffi_fn_name = format!("{}_{}", cpp_name, ffi_fn_name_base);

            let params: Vec<ParamBinding> = method
                .params
```
</details>

Now let me add the exclude_methods filtering. I need a helper for checking class name matches (since cpp_name has `::` replaced with `_`):

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1580-L1608">bindings.rs -23+28</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // ── Constructors ────────────────────────────────────────────────────\n    let exclude_ctors = exclude_methods.contains(&(class.name.clone(), class.name.clone()))\n        || exclude_methods.contains(&(class.name.clone(), \"*\".to_string()));\n    let constructors = if !effectively_abstract && !class.has_protected_destructor {\n        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);\n        if exclude_ctors {\n            ctors.clear();\n        }\n        // If no bindable constructors AND no explicit constructors at all,\n        // generate a synthetic default constructor (uses C++ implicit default).\n        // We must NOT generate synthetic constructors when:\n        // - The class has explicit constructors (even if filtered out) — C++ won't\n        //   generate an implicit default constructor in that case\n        if ctors.is_empty() && !class.has_explicit_constructors {\n            ctors.push(ConstructorBinding {\n                ffi_fn_name: format!(\"{}_ctor\", cpp_name),\n                impl_method_name: \"new\".to_string(),\n                params: Vec::new(),\n                cpp_arg_exprs: Vec::new(),\n                doc_comment: Some(\"Default constructor\".to_string()),\n                source_line: None,\n                convenience_of: None,\n            });\n        }\n        ctors\n    } else {\n        Vec::new()\n    };",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1580:yw,1581:qt,1582:qn,1583:yy,1584:hu,1585:yf,1586:jx,1587:zd,1588:bu,1589:ov,1590:ut,1591:vw,1592:ov,1593:hc,1594:dk,1595:hy,1596:ua,1597:ys,1598:cy,1599:wk,1600:ui,1601:ev,1602:jv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1580,"endLine":1607}
```
</details>

Now add filtering to the direct methods and wrapper methods:

<details>
<summary>Reading lines 1607-1620 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1607-L1621">bindings.rs</a></summary>

```
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1609-L1617">bindings.rs -7+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // ── Direct methods (extern \"C\" self-pointer wrappers) ──────────────────\n    let direct_methods_raw: Vec<&Method> = class\n        .methods\n        .iter()\n        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))\n        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))\n        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))\n        .collect();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1609:ev,1610:jy,1611:lq,1612:cb,1613:ig,1614:vt,1615:nj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1609,"endLine":1616}
```
</details>

Now the wrapper methods:

<details>
<summary>Reading lines 1643-1655 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1643-L1656">bindings.rs</a></summary>

```
        .collect();

    // ── Wrapper methods (by-value return, const char*) ──────────────────
    let wrapper_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1645-L1653">bindings.rs -7+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // ── Wrapper methods (by-value return, const char*) ──────────────────\n    let wrapper_methods_raw: Vec<&Method> = class\n        .methods\n        .iter()\n        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))\n        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && needs_wrapper_function(m, all_enum_names))\n        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))\n        .collect();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1645:dd,1646:bs,1647:lq,1648:cb,1649:js,1650:vt,1651:nj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1645,"endLine":1652}
```
</details>

Now the static methods:

<details>
<summary>Reading lines 1723-1735 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1723-L1736">bindings.rs</a></summary>

```
        .collect();

    // ── Static methods ──────────────────────────────────────────────────
    let static_methods_raw: Vec<&StaticMethod> = class
        .static_methods
        .iter()
        .filter(|m| is_static_method_bindable(m, ffi_ctx))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let static_method_names = compute_static_method_names(
        cpp_name,
        &static_methods_raw,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1725-L1733">bindings.rs -7+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // ── Static methods ────────────────────────────────────────────────────\n    let static_methods_raw: Vec<&StaticMethod> = class\n        .static_methods\n        .iter()\n        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))\n        .filter(|m| is_static_method_bindable(m, ffi_ctx))\n        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))\n        .collect();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1725:zs,1726:ch,1727:mv,1728:cb,1729:ds,1730:vt,1731:nj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1725,"endLine":1732}
```
</details>

in `crates/opencascade-binding-generator/src/main.rs` and the call sites:

<details>
<summary>Reading lines 545-570 of <a href="../crates/opencascade-binding-generator/src/main.rs#L545-L571">main.rs</a></summary>

```
/// - MODULE.rs: Per-module re-exports with impl blocks
/// - lib.rs: Module declarations
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L547-L556">main.rs -8+9</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn generate_output(\n    args: &Args,\n    all_classes: &[&model::ParsedClass],\n    all_functions: &[&model::ParsedFunction],\n    graph: &module_graph::ModuleGraph,\n    symbol_table: &resolver::SymbolTable,\n    known_headers: &HashSet<String>,\n    exclude_methods: &HashSet<(String, String)>,\n) -> Result<()> {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "547:ye,548:zb,549:pv,550:mq,551:yb,552:uu,553:ai,554:gv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":547,"endLine":555}
```
</details>

Now update the `compute_all_class_bindings` call inside :

<details>
<summary>Reading lines 575-585 of <a href="../crates/opencascade-binding-generator/src/main.rs#L575-L586">main.rs</a></summary>

```
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();
    let extra_typedef_names = parser::get_collected_typedef_names();
    let all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let all_function_bindings = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers,
    );

    // Track generated files for formatting
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L577-L579">main.rs -2+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let all_bindings =\n        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods);",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "577:fy,578:vk"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":577,"endLine":578}
```
</details>

Now update the call to  in `main()`:

<details>
<summary>Reading lines 348-360 of <a href="../crates/opencascade-binding-generator/src/main.rs#L348-L361">main.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 340-365 of <a href="../crates/opencascade-binding-generator/src/main.rs#L340-L366">main.rs</a></summary>

```
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers)
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L343-L345">main.rs -2+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Generate FFI output\n    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "343:ke,344:gt"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":343,"endLine":344}
```
</details>

Now update the test call sites in `crates/opencascade-binding-generator/src/codegen/bindings.rs` to pass the new parameter:

<details>
<summary>Reading lines 4879-4895 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4879-L4896">bindings.rs</a></summary>

```
            all_class_names: ["gp_Pnt".to_string()].into(),
            handle_able_classes: HashSet::new(),
            cross_module_types: HashMap::new(),
            type_to_module: HashMap::new(),
            enum_rust_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("gp_Pnt".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            &HashMap::new(),
```
</details>

<details>
<summary>Reading lines 4889-4905 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4889-L4906">bindings.rs</a></summary>

```
        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            &HashMap::new(),
            &HashSet::new(),
            None,
        );

        assert_eq!(bindings.cpp_name, "gp_Pnt");
        assert_eq!(bindings.short_name, "Pnt");
        assert_eq!(bindings.module, "gp");
        // Non-abstract class with no explicit constructors gets a synthetic default constructor
        assert_eq!(bindings.constructors.len(), 1);
        assert_eq!(bindings.constructors[0].impl_method_name, "new");
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4889-L4900">bindings.rs -10+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let bindings = compute_class_bindings(\n            &class,\n            &ffi_ctx,\n            &symbol_table,\n            &handle_able_classes,\n            &all_classes_by_name,\n            &HashMap::new(),\n            &HashSet::new(),\n            None,\n            &HashSet::new(),\n        );",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4889:wz,4890:vo,4891:ca,4892:sa,4893:jv,4894:tk,4895:oq,4896:au,4897:xc,4898:nw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4889,"endLine":4899}
```
</details>

Now the second test call site:

<details>
<summary>Reading lines 4975-4990 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4975-L4991">bindings.rs</a></summary>

```
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("Geom_Curve".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            &HashMap::new(),
            &HashSet::new(),
            None,
        );

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4980-L4991">bindings.rs -10+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let bindings = compute_class_bindings(\n            &class,\n            &ffi_ctx,\n            &symbol_table,\n            &handle_able_classes,\n            &all_classes_by_name,\n            &HashMap::new(),\n            &HashSet::new(),\n            None,\n            &HashSet::new(),\n        );",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4980:wz,4981:vo,4982:ca,4983:sa,4984:jv,4985:tk,4986:oq,4987:au,4988:xc,4989:nw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4980,"endLine":4990}
```
</details>

Now add the `exclude_methods` entries to `bindings.toml`:

<details>
<summary>Reading lines 267-280 of <a href="../crates/opencascade-sys/bindings.toml#L267-L281">bindings.toml</a></summary>

```

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

]

# Include specific individual headers (from modules not fully included above).
include_headers = []

[general]
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L271-L300">bindings.toml -4+29</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "]\n\n# Include specific individual headers (from modules not fully included above).\ninclude_headers = []\n\n# Exclude specific methods/constructors that clang misresolves in batch parsing mode.\n# When clang can't fully instantiate certain template types, it misresolves them to\n# Standard_Integer (int). This corruption affects both canonical AND display names,\n# so the parser's heuristic guards cannot detect it. These methods are excluded\n# individually so the rest of each class remains accessible.\n# Format: \"ClassName::MethodName\" for methods, \"ClassName::ClassName\" for constructors.\nexclude_methods = [\n    # math_Matrix: Row() and Col() return math_VectorBase<> (typedef math_Vector)\n    # which clang misresolves to Standard_Integer in batch parsing mode.\n    \"math_Matrix::Row\",\n    \"math_Matrix::Col\",\n\n    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray<int>::ConstIterator\n    # which clang misresolves to Standard_Integer in batch parsing mode.\n    \"TObj_TIntSparseArray::GetIterator\",\n\n    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param (class-scoped\n    # using alias \"ParameterMap\") misresolved to Standard_Integer by clang.\n    \"XSAlgo_ShapeProcessor::XSAlgo_ShapeProcessor\",\n\n    # XSAlgo_ShapeProcessor: ReadProcessingData() returns std::pair<ParameterMap, bitset>\n    # which clang misresolves to Standard_Integer in batch parsing mode.\n    \"XSAlgo_ShapeProcessor::ReadProcessingData\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "271:cy,272:dd,273:vg,274:nu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":271,"endLine":299}
```
</details>

Now let's check if the code compiles (binding generator first):

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (15 lines) — exit code 101</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error: unexpected closing delimiter: `}`
  --> crates/opencascade-binding-generator/src/config.rs:55:1
   |
11 | pub struct BindingConfig {
   |                          - this opening brace...
...
37 | }
   | - ...matches this closing brace
...
55 | }
   | ^ unexpected closing delimiter

error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
real  0.6s
```
</details>

<details>
<summary>Reading lines 30-60 of <a href="../crates/opencascade-binding-generator/src/config.rs#L30-L61">config.rs</a></summary>

```
    /// Exclude specific methods/constructors from binding generation.
    /// Format: "ClassName::MethodName" for instance/static methods,
    /// or "ClassName::ClassName" for constructors (C++ constructor naming convention).
    /// Methods matching these patterns will be skipped during codegen,
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,
}
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
```
</details>

> 💭 I see the problem - the edit resulted in the struct fields being split across two separate `}` closings. The struct definition got malformed. Let me fix it.

The struct got malformed. Let me fix config.rs:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/config.rs#L10-L38">config.rs -46+28</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "#[derive(Debug, Deserialize)]\npub struct BindingConfig {\n    /// General settings.\n    #[serde(default)]\n    pub general: GeneralConfig,\n\n    /// Include ALL headers from these OCCT modules.\n    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the\n    /// OCCT include directory will be processed.\n    #[serde(default)]\n    pub modules: Vec<String>,\n\n    /// Exclude specific headers, even if their module is included.\n    #[serde(default)]\n    pub exclude_headers: Vec<String>,\n\n    /// Include specific individual headers (from modules not fully listed in `modules`).\n    #[serde(default)]\n    pub include_headers: Vec<String>,\n\n    /// Exclude specific methods/constructors from binding generation.\n    /// Format: \"ClassName::MethodName\" for instance/static methods,\n    /// or \"ClassName::ClassName\" for constructors (C++ constructor naming convention).\n    /// Methods matching these patterns will be skipped during codegen,\n    /// even though the rest of the class is bound.\n    #[serde(default)]\n    pub exclude_methods: Vec<String>,\n}",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "10:wb,11:ja,12:zs,13:lb,14:ob,15:dd,16:kt,17:lh,18:vk,19:lb,20:qc,21:dd,22:ff,23:lb,24:qx,25:dd,26:ha,27:lb,28:kj,29:io,30:hr,31:tp,32:el,33:ug,34:cc,35:lb,36:cx,37:io,38:zs,39:lb,40:ob,41:dd,42:kt,43:lh,44:vk,45:lb,46:qc,47:dd,48:ff,49:lb,50:qx,51:dd,52:ha,53:lb,54:kj,55:io"
    }
  ]
}
```
**Output:**
```
{
  "applied": 0,
  "failed": 1,
  "errors": [
    {
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "10:wb,11:ja,12:zs,13:lb,14:ob,15:dd,16:kt,17:lh,18:vk,19:lb,20:qc,21:dd,22:ff,23:lb,24:qx,25:dd,26:ha,27:lb,28:kj,29:io,30:hr,31:tp,32:el,33:ug,34:cc,35:lb,36:cx,37:io,38:zs,39:lb,40:ob,41:dd,42:kt,43:lh,44:vk,45:lb,46:qc,47:dd,48:ff,49:lb,50:qx,51:dd,52:ha,53:lb,54:kj,55:io",
      "error": "hash mismatch at line 29: expected 'io', got 'dd'"
    }
  ]
}
```
</details>

<details>
<summary>Reading lines 9-57 of <a href="../crates/opencascade-binding-generator/src/config.rs#L9-L58">config.rs</a></summary>

```
/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,

    /// Exclude specific methods/constructors from binding generation.
    /// Format: "ClassName::MethodName" for instance/static methods,
    /// or "ClassName::ClassName" for constructors (C++ constructor naming convention).
    /// Methods matching these patterns will be skipped during codegen,
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,
}
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,
}

/// General configuration options.
```
</details>

I see, the original struct body got duplicated. Let me fix this:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/config.rs#L37-L38">config.rs -19+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "}",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "37:io,38:zs,39:lb,40:ob,41:dd,42:kt,43:lh,44:vk,45:lb,46:qc,47:dd,48:ff,49:lb,50:qx,51:dd,52:ha,53:lb,54:kj,55:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/config.rs","startLine":37,"endLine":37}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on build directory
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.28s
real  3.3s
```

Binding generator compiles. Now let's run its tests:

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- ✅ Implement method exclusions
- 🔄 Regenerate and test build
- ⬜ Fix remaining issues
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.96s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-9f2064aa9f77643d)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-5dd5131126f4e1fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  2.8s
```
</details>

All 16 tests pass. Now regenerate and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep -E "Exclude methods|exclude_methods|Code generation complete" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/9771ee61-1749-467f-89d9-113545ba0f86/toolu_01HJfCRvceWmfkYa5cSEitKe__vscode-1771312781729/content.txt
```
```
Code generation complete!
```

Now let's verify the problematic methods are gone from wrappers.cpp:

**Terminal:**
```sh
grep -n "math_Matrix_row\|math_Matrix_col\b\|TObj_TIntSparseArray_get_iterator\|XSAlgo_ShapeProcessor_ctor\|XSAlgo_ShapeProcessor_read_processing_data" crates/opencascade-sys/generated/wrappers.cpp
```
```
17041:extern "C" Standard_Integer math_Matrix_row_number(const math_Matrix* self_) {
```

Only `row_number` remains (which is a different method - `RowNumber()`, not `Row()`). The excluded methods are gone. Now let's build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1
```
```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 20.03s
real  20.1s
```

Build succeeds. Let me also verify the full workspace builds:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

> 💭 These errors are pre-existing - they're about `Bnd_Box` being renamed to `Box_` (because "Box" is a Rust prelude name) and similarly `gp_Vec` → `Vec_`. These are naming collision issues that exist in the `opencascade` crate (the higher-level Rust wrapper), not in `opencascade-sys`. They're likely pre-existing and not related to my changes.
> 
> Let me verify by checking if these errors existed before my changes:

These `Box_`/`Vec_` naming errors are pre-existing and unrelated to my changes - they're in the higher-level `opencascade` crate. Let me verify `opencascade-sys` builds cleanly and that the generator tests pass:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git stash && time cargo build -p opencascade 2>&1 | grep "^error" ; git stash pop
```

Confirmed - the `opencascade` crate build errors are pre-existing and not caused by my changes. `opencascade-sys` builds cleanly. 

Now let me update the problem documentation in bindings.toml to reflect the new approach:

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- ✅ Implement method exclusions
- ✅ Regenerate and test build
- 🔄 Update docs in bindings.toml
- ⬜ Commit changes

Let me update the Problem Classes documentation at the bottom of bindings.toml:

<details>
<summary>Reading lines 302-345 of <a href="../crates/opencascade-sys/bindings.toml#L302-L346">bindings.toml</a></summary>

```
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
# =============================================================================
#
# The following categories of problems prevent certain OCCT headers from being
# included in the binding generation. Each category describes the root cause,
# affected headers, and potential solutions.
#
# 1. NCollection Template Parameter Misresolution (SOLVED)
#    ─────────────────────────────────────────────────────────────────
#    Root cause: When clang processes NCollection template specializations
#    (e.g., NCollection_DataMap<TDF_Label, TDF_Label>) that it cannot fully
#    instantiate, it falls back to canonical type "int" (Standard_Integer).
#    This causes constructor and method parameters/returns to have wrong types.
#
#    Solution implemented: Three-layer guard in parser.rs:
#    a) AST-based typedef map — collect_ncollection_typedefs() pre-scans all
#       typedef declarations, building a map from NCollection template spelling
#       to typedef name. lookup_typedef() then resolves template display names
#       back to their original typedef (e.g., NCollection_DataMap<K,V> →
#       TopTools_DataMapOfShapeShape). normalize_template_spelling() handles
#       Standard_Integer→int, Standard_Real→double type alias mismatches.
#    b) spelling_looks_like_class guard — when a non-template, non-namespace
#       display name looks like an OCCT class but canonical type is "int",
#       checks the underlying typedef declaration to distinguish real primitive
#       typedefs from misresolved NCollection types.
#    c) Reference/pointer canonical guard — when the outer ref/ptr type's
#       display name identifies an OCCT class/template/namespace but canonical
#       resolves to "int", constructs the Class type directly instead of
#       recursing into the pointee (which would lose the typedef info).
#
#    Additionally, is_constructor_bindable() in codegen/bindings.rs now checks
#    type_uses_unknown_type() to filter constructors with unresolvable types.
#
#    ~120 headers successfully UNBLOCKED by these combined fixes.
#
#
# 2. Primitive Typedef vs Template Typedef Ambiguity
#    ────────────────────────────────────────────────
#    Root cause: Some OCCT types are typedefs to primitives (e.g.,
#    `typedef unsigned int Poly_MeshPurpose`), while NCollection typedefs
#    are ALSO canonicalized to primitives by clang (incorrectly). The parser
```
</details>

<details>
<summary>Reading lines 338-420 of <a href="../crates/opencascade-sys/bindings.toml#L338-L421">bindings.toml</a></summary>

```
#    ~120 headers successfully UNBLOCKED by these combined fixes.
#
#
# 2. Primitive Typedef vs Template Typedef Ambiguity
#    ────────────────────────────────────────────────
#    Root cause: Some OCCT types are typedefs to primitives (e.g.,
#    `typedef unsigned int Poly_MeshPurpose`), while NCollection typedefs
#    are ALSO canonicalized to primitives by clang (incorrectly). The parser
#    uses a heuristic guard (`spelling_looks_like_class`) to distinguish
#    these, checking the typedef declaration's underlying type kind.
#
#    Current status: Mostly solved. The guard now checks whether the type's
#    declaration is a TypedefDecl with a primitive underlying type (allowing
#    canonical mapping) vs a template/record underlying type (blocking it).
#    Handles both TypeKind::Typedef and TypeKind::Elaborated (clang sugar).
#
# 3. BVH/Template-Heavy Classes
#    ──────────────────────────
#    Root cause: Some classes use deeply-nested BVH tree templates or other
#    complex template patterns that the generator cannot resolve.
#
#    Affected: BRepExtrema_OverlapTool, BRepExtrema_SelfIntersection,
#    BRepExtrema_TriangleSet, BRepExtrema_ProximityDistTool,
#    BRepExtrema_ProximityValueTool
#
#    Potential solutions: Skip classes with unresolvable template parameters
#    automatically, or pre-declare the template specializations.
#
# 4. External Dependencies
#    ─────────────────────
#    Some classes reference external libraries (e.g., RapidJSON, RWGltf DRACO
#    compression) that may not be available in the build environment.
#
#    Affected: RWGltf_GltfMaterialMap, RWGltf_GltfOStreamWriter,
#    RWGltf_GltfSceneNodeMap, RWGltf_CafReader, RWMesh_FaceIterator,
#    RWObj_CafReader, StdStorage_Root
#
# 5. Name Collisions and Method Conflicts (PARTIALLY SOLVED)
#    ─────────────────────────────────────────────────────────
#    Some headers produce duplicate method names or conflict with Rust
#    reserved words in ways the generator can't resolve.
#
#    Rust prelude collision (SOLVED): BRepCheck_Result's short name "Result"
#    collided with Rust's prelude Result<T,E>. Fixed by applying safe_short_name()
#    consistently in codegen — reserved names get trailing underscore (Result_).
#    All 8 BRepCheck headers now unblocked.
#
#    Remaining: TObj_HiddenPartition (duplicate new_label method from static
#    constructor and inherited method), TDataStd_NamedData (PascalCase vs
#    camelCase methods colliding in snake_case), IntPatch_WLine/
#    IntPatch_ImpImpIntersection (name conflicts or unmapped operator types).
#
# 6. Modules Not Included (Intentionally Excluded)
#    ─────────────────────────────────────────────
#    The following module categories are not included because they are either
#    platform-specific, visualization-only, or not relevant to CAD modeling:
#
#    - Platform-specific GUI: Cocoa, WNT, Wasm, Xw, D3DHost, OpenGl
#    - Visualization/rendering: AIS, V3d, Graphic3d, Prs3d, PrsMgr, Select3D,
#      SelectMgr, StdPrs, StdSelect, MeshVS, XSDRAW*
#    - Font/image handling: Font, Image
#    - Persistence/Storage: FSD, Storage, StdStorage, ShapeSchema, PCDM,
#      CDM, CDF, BinDrivers, XmlDrivers
#    - Draw test harness: Draw, DBRep, BRepTest, *Draw*
#    - VRML: Vrml*
#    - LDOM/XML core: LDOM, XmlObjMgt
#    - Other unused: Media, TKernel-internal headers

```
</details>

Now let me add a note to the NCollection section about the new method-level exclusion approach and update the description to reflect the fix:

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L338-L352">bindings.toml -3+14</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "#    ~120 headers successfully UNBLOCKED by these combined fixes.\n#\n#    Additionally, parse_type() now preserves template/namespaced display names\n#    instead of falling through to Type::Class(canonical) when canonical looks\n#    like a primitive. This prevents misresolved templates from generating\n#    incorrect bindings (the method is then naturally filtered out as unknown).\n#\n#    For cases where clang corrupts BOTH the canonical AND display name\n#    (resolving templates to Standard_Integer even in the display name),\n#    the `exclude_methods` config provides method-level exclusions. These are\n#    used for the handful of methods where the three-layer guard cannot help\n#    because the type information is completely lost at the clang level.\n#    The rest of each affected class remains fully accessible.\n#",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "338:xv,339:gd,340:gd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":338,"endLine":351}
```
</details>

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- ✅ Implement method exclusions
- ✅ Regenerate and test build
- ✅ Update docs in bindings.toml
- 🔄 Commit changes

Now let me write the commit message and commit:

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
rm -f /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 4ac46de] Unblock 4 headers via method-level exclusions and parser fix
 28 files changed, 4105 insertions(+), 50 deletions(-)
 create mode 100644 crates/opencascade-sys/generated/xs_algo.rs
```

**Todo list:**
- ✅ Understand current misresolution handling
- ✅ Examine problematic OCCT headers
- ✅ Design type hints solution
- ✅ Implement method exclusions
- ✅ Regenerate and test build
- ✅ Update docs in bindings.toml
- ✅ Commit changes

Done. Here's a summary of what was done:

**Two-part fix for 4 excluded headers:**

1. **Parser fix** in `crates/opencascade-binding-generator/src/parser.rs`: When `parse_type()` encounters a template/namespaced type (display name has `<` or `::`) whose canonical type is a primitive like `"int"` due to clang misresolution, it now checks that the canonical actually looks like a class name before using it. This prevents producing `Type::Class("int")` which would slip through the unknown-type filter and generate incorrect C++ wrappers.

2. **`exclude_methods` config** in `crates/opencascade-binding-generator/src/config.rs``crates/opencascade-sys/bindings.toml`: New field for method-level exclusions when clang corrupts both canonical AND display names. Format: `"ClassName::MethodName"` for methods, `"ClassName::ClassName"` for constructors. Threaded through `compute_all_class_bindings` →  and checked for instance methods, wrapper methods, static methods, and constructors.

**Results:**
- **Message_Report.hxx** — fully unblocked, no method exclusions needed (existing guards handle it)
- **math_Matrix.hxx** — unblocked, 2 methods excluded (`Row`, `Col`)
- **TObj_TIntSparseArray.hxx** — unblocked, 1 method excluded (`GetIterator`)
- **XSAlgo_ShapeProcessor.hxx** — unblocked, constructor + 1 static method excluded
- All 16 binding generator tests pass, `opencascade-sys` builds cleanly

*2026-02-18 09:07 · 1540s · Words: 45,090 in · 8,625 out · 117,944 ctx · 107 rounds*

---
