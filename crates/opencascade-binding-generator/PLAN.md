# OCCT Binding Generator

A Rust tool using libclang to parse OCCT headers and generate CXX bridge code with a unified FFI module and per-module re-exports.

## Status

The binding generator is functional and deployed. It parses 378 OCCT headers (262 explicit + automatic dependency resolution), generating a unified `ffi.rs` (45K lines, 602 types, 6079 functions) plus 79 per-module re-export files.

The `opencascade` high-level crate compiles against the generated bindings. Some methods are stubbed due to generator limitations documented below.

See **[TRANSITION_PLAN.md](./TRANSITION_PLAN.md)** for remaining work on the `opencascade` crate migration.

## Architecture

### Unified FFI Architecture

All types and functions are in a single `#[cxx::bridge]` module (`ffi.rs`), with per-module re-export files providing ergonomic short names:

```
crates/opencascade-sys/generated/
├── ffi.rs             # Single CXX bridge with ALL types (full C++ names: gp_Pnt, TopoDS_Shape, etc.)
├── wrappers.hxx       # Single C++ wrapper header (all includes + wrapper functions)
├── gp.rs              # Re-exports: `pub use crate::ffi::gp_Pnt as Pnt;` + impl blocks
├── topo_ds.rs          # Re-exports for topo_ds module + impl blocks
├── ... (79 module files)
└── lib.rs             # `pub(crate) mod ffi;` + `pub mod gp;` etc.
```

Users write `use opencascade_sys::gp::Pnt;` -- the unified ffi module is `pub(crate)`.

### Generator Source

```
crates/opencascade-binding-generator/src/
├── main.rs           # CLI entry point
├── lib.rs            # Library API (for tests)
├── parser.rs         # libclang-based header parser
├── model.rs          # IR for parsed declarations (ParsedClass, Method, etc.)
├── resolver.rs       # Two-pass symbol table: resolves names, applies filters, builds SymbolTable
├── module_graph.rs   # Module dependency analysis
├── header_deps.rs    # Automatic header dependency resolution (--resolve-deps)
├── type_mapping.rs   # OCCT type -> Rust type mappings
└── codegen/
    ├── mod.rs
    ├── rust.rs       # Generates ffi.rs + per-module re-export files
    ├── cpp.rs        # Generates wrappers.hxx
    └── collections.rs # Generates collection type wrappers (iterators, accessors)
```

### Two-Pass Pipeline

1. **Parse**: libclang parses OCCT headers into `ParsedClass`, `Method`, etc. (`parser.rs`)
2. **Resolve**: `SymbolTable` built from parsed data -- applies all filters, computes names, determines binding status (`resolver.rs`)
3. **Generate**: Rust and C++ code emitted from resolved symbols (`codegen/`)

All method filtering (enum checks, lifetime issues, by-value params, etc.) is centralized in `resolver.rs` and applied consistently to both FFI and impl generation.

## CLI Usage

```bash
# Standard regeneration (from repo root):
./scripts/regenerate-bindings.sh

# Manual invocation:
cargo run -p opencascade-binding-generator -- \
    --resolve-deps \
    -I target/OCCT/include \
    -o crates/opencascade-sys/generated \
    $(cat crates/opencascade-sys/headers.txt | grep -v '^#' | grep -v '^$' | sed 's|^|target/OCCT/include/|')
```

**Flags:**
- `--resolve-deps` -- Auto-include header dependencies (always used)
- `--dump-symbols` -- Dump symbol table for debugging
- `--dry-run` -- Parse without generating
- `-v, --verbose` -- Verbose output
- `--module <name>` -- Filter to a specific module

## Methods Skipped Due to CXX/OCCT Limitations

The following patterns cause methods to be intentionally skipped during binding generation:

1. **Methods using enums** -- CXX requires `enum class` (C++11 scoped enums), but OCCT uses traditional unscoped enums. Methods with enum parameters or returns are skipped.

2. **Methods with ambiguous lifetimes** -- CXX cannot handle methods returning `Pin<&mut Self>` when there are also reference parameters. The lifetime of the returned reference is ambiguous.

3. **Abstract class constructors** -- Abstract classes cannot be instantiated, so constructor wrappers and `to_handle()` functions are not generated. Abstract detection walks the full inheritance hierarchy to catch classes that inherit unimplemented pure virtual methods from ancestors.

4. **Methods with by-value Handle parameters** -- CXX cannot pass `Handle<T>` by value across the FFI boundary. Methods taking `Handle<T>` parameters (not `const Handle<T>&`) are skipped.

5. **Methods with by-value class parameters** -- Similar to Handles, methods taking class types by value (not by reference) are skipped.

6. **Classes with protected destructors** -- Excluded from CXX type declarations entirely since CXX auto-generates destructor code.

7. **Inherited methods with signature mismatches** -- When a method pointer's declaring class differs from the binding class, C++ compilation fails. Inherited methods use C++ wrapper functions instead.

8. **Const/mut return mismatches** -- Methods returning `const T&` when the method is non-const are problematic for CXX's ownership model and are skipped.

### Filter Consistency

All filter functions are centralized in `resolver.rs`. When any method is filtered out of FFI generation, it is automatically filtered out of impl generation too, since both use the same `SymbolTable`.

## Key Patterns

### Wrapper Functions

CXX can't directly bind certain C++ patterns. The generator creates C++ wrapper functions for:

| C++ Pattern | Problem | Generated Wrapper |
|-------------|---------|-------------------|
| Constructor | CXX can't return `T` | `construct_unique<T>()` template |
| Return by value | CXX needs `UniquePtr<T>` | `make_unique<T>(obj.Method())` |
| Static method | No `self` | Free function wrapper |
| `Handle<T>` | Template | `typedef opencascade::handle<T> HandleT;` |
| Overloaded method | Name collision | Suffix: `_real3`, `_pnt2`, etc. |
| `const char*` param | CXX can't do `&str` to C++ | Wrapper accepts `rust::Str`, converts |
| `const char*` return | CXX can't do C++ to `&str` | Wrapper returns `rust::String` |
| Inherited method | Method pointer type mismatch | Free function calling `self.Method()` |
| Upcast (const) | CXX sees types as unrelated | `Derived_as_Base(self) -> &Base` |
| Upcast (mut) | CXX sees types as unrelated | `Derived_as_Base_mut(self) -> Pin<&mut Base>` |

### Handle Support

Classes inheriting from `Standard_Transient` get:
- `ClassName::to_handle(obj)` -- wrap in `Handle<T>`
- `handle.to_handle_base()` -- upcast Handle to base type

### Collection Types

NCollection typedefs (e.g., `TopTools_ListOfShape`) get iterator wrappers:
- C++ iterator struct wrapping `const_iterator` or indexed access
- `TypeName_iter()` / `TypeNameIterator_next()` C++ functions
- Rust `Iterator` trait impl yielding `UniquePtr<Element>`
- Impl methods: `iter()`, `from_iter()`, `append()`, etc.

### Naming Conventions

- **Types in ffi.rs**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi::X as Y;`
- **Methods**: snake_case with `#[cxx_name]` mapping to C++ names
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

## TODO

- [x] Implicit default constructors (e.g., `BRep_Builder` has no explicit constructor)
- [ ] Constructors with default enum parameters (e.g., `BRepFilletAPI_MakeFillet`)
- [ ] TColgp array constructors (template instantiation typedefs)
- [ ] System include path auto-detection (currently passed via `-I`)

---

## Completed Steps

### Step A: Remove Non-Unified Code Path ✓

Removed the old per-module code generation path. The unified architecture is now the only path.

**What was removed (~4,600 lines):**
- `main.rs`: Removed `--unified` CLI flag, old per-module code path, `generate_lib_rs()`
- `lib.rs`: Removed `GeneratedModule`, `GeneratorConfig`, `generate_bindings()`, `generate_lib_rs()`
- `codegen/rust.rs`: Removed `generate_module()` and ~30 old-only helper functions (~2,750 lines)
- `codegen/cpp.rs`: Removed `generate_module_header()` and ~15 old-only helper functions (~1,100 lines)
- `codegen/collections.rs`: Removed `collections_for_module` and 6 old per-module functions (~1,000 lines)
- `scripts/regenerate-bindings.sh`: Removed `--unified` flag
- Deleted the test that used `generate_module`

## Next Steps

### Step B: Unify Codegen with Shared Intermediate Representation *(in progress)*

Currently the three output files (ffi.rs, wrappers.hxx, per-module *.rs) are generated by independent functions that each re-derive filtering, naming, overload suffixes, and `used_names` conflict resolution. Comments throughout say "must match X exactly". This is fragile — any change to filtering or naming must be applied identically in three places.

The fix: compute all binding decisions once per class into a shared `ClassBindings` struct, then have thin emit functions for each output format consume it.

#### Phase B1: Introduce `ClassBindings` intermediate representation ✓

Created `crates/opencascade-binding-generator/src/codegen/bindings.rs` with:
- `ClassBindings` struct and all sub-structs (ConstructorBinding, DirectMethodBinding, WrapperMethodBinding, StaticMethodBinding, UpcastBinding, HandleUpcastBinding, InheritedMethodBinding, ParamBinding, ReturnTypeBinding)
- `compute_class_bindings()` — computes all filtering, naming, overload suffixes, and 3-level conflict resolution in one place
- `compute_all_class_bindings()` — top-level function that processes all classes
- Unit tests for empty classes, abstract classes, overload suffixes, and 3-level static method conflict resolution
- Vectors are ordered by source_line to preserve C++ header declaration order

The original plan proposed these structs (kept for reference):

Create a new module `crates/opencascade-binding-generator/src/codegen/bindings.rs`:

```rust
/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
/// The emit functions for ffi.rs, wrappers.hxx, and module re-exports
/// consume this struct without re-deriving any decisions.
pub struct ClassBindings {
    pub cpp_name: String,           // e.g. "gp_Pnt"
    pub short_name: String,         // e.g. "Pnt"
    pub module: String,             // e.g. "gp"
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,    // CXX self methods
    pub wrapper_methods: Vec<WrapperMethodBinding>,  // by-value return, const char*
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
}

pub struct ConstructorBinding {
    pub rust_method_name: String,   // e.g. "new_real3"
    pub ffi_fn_name: String,        // e.g. "gp_Pnt_ctor_real3"
    pub params: Vec<ParamBinding>,
    pub doc_comment: Option<String>,
    pub source_line: Option<u32>,
}

pub struct DirectMethodBinding {
    pub rust_name: String,          // e.g. "x" — bound as self method by CXX
    pub cxx_name: String,           // e.g. "X"
    pub is_const: bool,
    pub params: Vec<ParamBinding>,
    pub return_type: Option<TypeBinding>,
    pub doc_comment: Option<String>,
}

pub struct WrapperMethodBinding {
    pub rust_name: String,          // e.g. "mirrored_pnt" — name in impl block
    pub ffi_fn_name: String,        // e.g. "gp_Pnt_mirrored_pnt" — name in ffi.rs
    pub is_const: bool,
    pub params: Vec<ParamBinding>,
    pub return_type: TypeBinding,
    pub wrapper_kind: WrapperKind,  // ByValueReturn, CStringParam, CStringReturn
    pub doc_comment: Option<String>,
    pub source_line: Option<u32>,
    pub cpp_method_name: String,    // original C++ name, e.g. "Mirrored"
}

pub struct StaticMethodBinding {
    pub rust_name: String,          // name in impl block (may have _static suffix)
    pub ffi_fn_name: String,        // name in ffi.rs (may have _static suffix)
    pub params: Vec<ParamBinding>,
    pub return_type: Option<TypeBinding>,
    pub doc_comment: Option<String>,
    pub cpp_method_name: String,
}

pub struct ParamBinding {
    pub rust_name: String,
    pub rust_type: String,          // for ffi.rs and impl blocks
    pub cpp_type: String,           // for wrappers.hxx
    pub cpp_arg_expr: String,       // how to pass to C++ (may need conversion)
}

pub struct TypeBinding {
    pub rust_ffi_type: String,      // type as it appears in ffi.rs
    pub rust_return_type: String,   // type as it appears in impl return (may differ)
    pub cpp_type: String,           // C++ type
}

pub enum WrapperKind {
    ByValueReturn,
    CStringParam,
    CStringReturn,
}
```

The key function:

```rust
/// Compute all binding decisions for a class.
/// This is the SINGLE place where filtering, naming, overload suffixes,
/// and used_names conflict resolution happen.
pub fn compute_class_bindings(
    class: &ParsedClass,
    ctx: &TypeContext,
    all_enum_names: &HashSet<String>,
) -> ClassBindings { ... }
```

This function absorbs the filtering and naming logic currently duplicated across `generate_unified_wrapper_methods` (rust.rs), `generate_unified_return_by_value_wrappers` (cpp.rs), and the wrapper section of `generate_module_reexports` (rust.rs). The `used_names` → `reserved_names` flow for static method conflict detection happens inside `compute_class_bindings` instead of being threaded between separate functions.

#### Phase B2: Rewrite emit functions to consume `ClassBindings`

Rewrite the three generators to be thin formatters:

- **rust.rs `emit_ffi_class(bindings: &ClassBindings) -> String`**: Emits the `type` declaration, direct method declarations (with `self: &T`), wrapper function declarations (with `self_: &T`), static function declarations, upcast/to_owned/to_handle declarations.

- **cpp.rs `emit_cpp_class(bindings: &ClassBindings) -> String`**: Emits the matching C++ inline wrapper functions. Each `WrapperMethodBinding` has all the info needed (cpp_method_name, params with cpp_type and cpp_arg_expr).

- **rust.rs `emit_reexport_class(bindings: &ClassBindings) -> String`**: Emits the `pub use` and `impl` block. Each binding already has both `ffi_fn_name` (what to call) and `rust_name` (what to expose).

The top-level generation becomes:

```rust
// In main.rs or a new top-level codegen function:
let all_bindings: Vec<ClassBindings> = all_classes.iter()
    .map(|c| compute_class_bindings(c, &ctx, &all_enum_names))
    .collect();

let ffi_rs = emit_ffi(&all_bindings, &function_bindings, &collection_bindings);
let wrappers_hxx = emit_cpp(&all_bindings, &function_bindings, &collection_bindings);
for module in modules {
    let module_bindings: Vec<_> = all_bindings.iter()
        .filter(|b| b.module == module.rust_name)
        .collect();
    let module_rs = emit_module_reexports(&module_bindings, ...);
}
```

#### Phase B3: Switch ffi.rs generation from TokenStream to string-based

Convert `emit_ffi_class` (and its callers) from `quote!`/TokenStream to `format!`/`push_str`, matching the style already used by `generate_module_reexports` and all of cpp.rs.

This eliminates:
- The `postprocess_generated_code()` regex hack that converts `#[doc = "SECTION: ..."]` → `// ...` and `#[doc = "BLANK_LINE"]` → blank lines
- `format_tokenstream()` and `proc_macro2`/`quote`/`syn` dependencies (verify no other usage first)
- String-to-TokenStream round-trips like `ty.rust_type.parse().unwrap_or_else(|_| quote! { () })`

After this step, all three emit functions are string-based, making the codegen WYSIWYG — reading the generator shows exactly what the output looks like.

#### Ordering and verification

- **B1** can be done incrementally: introduce `ClassBindings` and `compute_class_bindings`, then migrate one generator at a time to consume it (e.g., start with wrappers.hxx since it's simplest).
- **B2** follows naturally — each generator is rewritten to use `ClassBindings`.
- **B3** is independent and can be done during B2 (convert each emit function to strings as it's written) or after.
- After each sub-step: `cargo build -p opencascade-binding-generator`, then `./scripts/regenerate-bindings.sh`, then `cargo build -p opencascade-sys` and diff the generated output to verify no changes.
