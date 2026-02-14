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
- [x] Constructors with default enum parameters (e.g., `BRepFilletAPI_MakeFillet`)
- [x] Collection-aware method filtering (methods returning/taking known collection types no longer filtered)
- [x] TColgp array constructors (template instantiation typedefs)
- [x] Utility class detection and free function generation (e.g., `gp::OX()`, `TopoDS::Vertex()`)
- [x] Default-argument convenience constructors (trailing defaulted params omitted)
- [x] Unified function naming in resolver.rs (single source of truth for CXX-unique names)
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

### Step B: Unify Codegen with Shared Intermediate Representation ✓

All binding decisions are now computed once per class into a `ClassBindings` struct in `codegen/bindings.rs`. The three generators are thin formatters that consume it.

#### Phase B1: Introduce `ClassBindings` intermediate representation ✓

Created `codegen/bindings.rs` with `ClassBindings` struct and all sub-structs. `compute_class_bindings()` computes all filtering, naming, overload suffixes, and 3-level conflict resolution in one place. `compute_all_class_bindings()` is the top-level entry point.

#### Phase B2: Rewrite emit functions to consume `ClassBindings` ✓

The three generators are now thin formatters consuming `ClassBindings`:
- `emit_ffi_class(bindings)` — emits type declarations, method declarations, upcasts for ffi.rs
- `emit_cpp_class(bindings)` — emits C++ wrapper functions for wrappers.hxx
- `emit_reexport_class(bindings, module_name)` — emits `pub use` and `impl` blocks for per-module files

`main.rs` computes all bindings once via `compute_all_class_bindings()` and passes them to all three generators. ~1,300 lines of duplicated filtering/naming logic eliminated.

#### Phase B3: Switch ffi.rs generation from TokenStream to string-based ✓

All three emit functions are now string-based (`format!`/`push_str`), eliminating the `postprocess_generated_code()` regex hack, `format_tokenstream()`, and `proc_macro2`/`quote`/`syn` dependencies. The codegen is WYSIWYG.
