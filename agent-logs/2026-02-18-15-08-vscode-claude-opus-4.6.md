# Bug Fix Request

- **Date:** 2026-02-18 14:23 – 15:09
- **Models:** Claude Haiku 4.5, Claude Opus 4.6, Claude Sonnet 4.6
- **Turns:** 5
- **Tool calls:** 209
- **Thinking blocks:** 20
- **Input words:** 6,817
- **Output words:** 11,240
- **Prompt tokens (last round):** 229,228
- **API rounds:** 210
- **Total elapsed:** 1873s

## Table of Contents

1. [Fix this bug.](#user-1) (Claude Sonnet 4.6)
2. [@agent Try Again](#user-2) (Claude Sonnet 4.6)
3. [Why is val of type i32 rather than the appropriate enum type here?](#user-3) (Claude Sonnet 4.6) ⚠️ canceled
4. [Is that consistent with other enum types? Is this a problem that&#x27;s unique to enums passed by mut ...](#user-4) (Claude Opus 4.6)
5. [commit](#user-5) (Claude Haiku 4.5)

---

<a id="user-1"></a>

## User (1)

> Fix this bug.

*2026-02-18 14:24*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants to fix the codegen bug related to `ShapeProcess_ShapeContext.hxx` which is excluded with the comment "Codegen bug: val_enum_ undeclared identifier."
> 
> Let me investigate this bug by looking at the header file and the binding generator code.

Let me investigate the bug by looking at the relevant files.

**Terminal:**
```sh
find /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src -name "ShapeProcess_ShapeContext.hxx" | head -5
```
```
/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx
```

> 💭 Let me read the header file to understand what it contains.

Read [ShapeProcess_ShapeContext.hxx](../crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx#1-1), lines 1 to 100

> 💭 Now let me look at the binding generator code to find where `val_enum_` is generated:

Read [ShapeProcess_ShapeContext.hxx](../crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx#100-100), lines 100 to 153

Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/**`), no results

> 💭 Let me search in the entire codebase for `val_enum_`:

Searching for text `val_enum_`

*2026-02-18 14:31 · 390s · Words: 3 in · 130 out · 5 rounds*

---

<a id="user-2"></a>

## User (2)

> @agent Try Again

*2026-02-18 14:31*

### Assistant — Claude Sonnet 4.6

Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/**`), no results

The `val_enum_` identifier isn't in the generator source. Let me try to reproduce the error first to understand what's happening.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls crates/opencascade-binding-generator/src/
```
```
codegenheader_deps.rsmain.rsmodule_graph.rsresolver.rs
config.rslib.rsmodel.rsparser.rstype_mapping.rs
```

Searched for text `ContinuityVal` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `GeomAbs_Shape` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `enum_val` (`**/crates/opencascade-binding-generator/**`), no results

<details>
<summary>Searched for regex <code>val_|_val</code> (<code>**/crates/opencascade-binding-generator/**</code>), 20 results</summary>

- [codegen-flow.dot:48](../crates/opencascade-binding-generator/codegen-flow.dot#L48)
- [codegen-flow.dot:119](../crates/opencascade-binding-generator/codegen-flow.dot#L119)
- [codegen-flow.svg:142](../crates/opencascade-binding-generator/codegen-flow.svg#L142)
- [codegen-flow.svg:318](../crates/opencascade-binding-generator/codegen-flow.svg#L318)
- [main.rs:31](../crates/opencascade-binding-generator/src/main.rs#L31)
- [main.rs:47](../crates/opencascade-binding-generator/src/main.rs#L47)
- [resolver.rs:618](../crates/opencascade-binding-generator/src/resolver.rs#L618)
- [resolver.rs:628](../crates/opencascade-binding-generator/src/resolver.rs#L628)
- [resolver.rs:742](../crates/opencascade-binding-generator/src/resolver.rs#L742)
- [resolver.rs:744](../crates/opencascade-binding-generator/src/resolver.rs#L744)
- [resolver.rs:746](../crates/opencascade-binding-generator/src/resolver.rs#L746)
- [resolver.rs:756](../crates/opencascade-binding-generator/src/resolver.rs#L756)
- [resolver.rs:758](../crates/opencascade-binding-generator/src/resolver.rs#L758)
- [resolver.rs:1119](../crates/opencascade-binding-generator/src/resolver.rs#L1119)
- [resolver.rs:1178](../crates/opencascade-binding-generator/src/resolver.rs#L1178)
- [resolver.rs:1406](../crates/opencascade-binding-generator/src/resolver.rs#L1406)
- [rust.rs:541](../crates/opencascade-binding-generator/src/codegen/rust.rs#L541)
- [rust.rs:543](../crates/opencascade-binding-generator/src/codegen/rust.rs#L543)
- [rust.rs:545](../crates/opencascade-binding-generator/src/codegen/rust.rs#L545)
- [rust.rs:546](../crates/opencascade-binding-generator/src/codegen/rust.rs#L546)
</details>

Let me look at the codegen directory in detail, focusing on what generates C++ code with "val_enum_":

**Terminal:**
```sh
ls /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/
```
```
bindings.rscollections.rscpp.rsmod.rsrust.rs
```

<details>
<summary>Searched for regex <code>val_enum_|Continuity|GeomAbs_Shape|enum.*param|by.value.*enum</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [rust.rs:631](../crates/opencascade-binding-generator/src/codegen/rust.rs#L631)
- [bindings.rs:141](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L141)
- [bindings.rs:147](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L147)
- [bindings.rs:148](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L148)
- [bindings.rs:246](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L246)
- [bindings.rs:281](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L281)
- [bindings.rs:283](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L283)
- [bindings.rs:317](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L317)
- [bindings.rs:769](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L769)
- [bindings.rs:771](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L771)
- [bindings.rs:779](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L779)
- [bindings.rs:780](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L780)
- [bindings.rs:822](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L822)
- [bindings.rs:823](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L823)
- [bindings.rs:826](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L826)
- [bindings.rs:1006](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1006)
- [bindings.rs:1108](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1108)
- [bindings.rs:1165](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1165)
- [bindings.rs:1181](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1181)
- [bindings.rs:2637](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2637)
</details>

Searched for regex `val_enum|MutRefEnum|enum_param|local.*enum|writeback` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

<details>
<summary>Searched for regex <code>val_enum_|MutRefEnum|writeback|local.*var</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 17 results</summary>

- [bindings.rs:147](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L147)
- [bindings.rs:148](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L148)
- [bindings.rs:283](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L283)
- [bindings.rs:823](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L823)
- [bindings.rs:1006](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1006)
- [bindings.rs:1166](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1166)
- [bindings.rs:1181](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1181)
- [bindings.rs:1191](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1191)
- [bindings.rs:3548](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3548)
- [bindings.rs:3551](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3551)
- [bindings.rs:3557](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3557)
- [bindings.rs:3593](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3593)
- [bindings.rs:3784](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3784)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3540-3540), lines 3540 to 3700

Now I can see the general pattern. Let me look at where `val_enum_` would appear in the generated code - it's likely a naming pattern. Let me look at the preamble generation:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3750-3750), lines 3750 to 3800

Now I need to understand what  is set to for MutRefEnum params. Let me look at where it gets set:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1165-1165), lines 1165 to 1250

Found it. On line 1192, when a `&mut` enum parameter is detected,  is set to . So if the parameter name is , the local variable would be `val_enum_`.

Now let's look at the  header more carefully. The  method has  as a parameter - that's the issue.  is an enum and  is a `&mut` output param, so  becomes `val_enum_`. Let me check what the generated code looks like to understand the actual compile error:

**Terminal:**
```sh
ls /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ | grep -i shapeprocess | head -20
```

**Terminal:**
```sh
ls /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
adaptor2d.rs
adaptor3d.rs
adv_app2_var.rs
adv_approx.rs
app_blend.rs
app_cont.rs
app_par_curves.rs
approx.rs
b_rep_adaptor.rs
b_rep_algo_api.rs
b_rep_algo.rs
b_rep_blend.rs
b_rep_bnd_lib.rs
b_rep_builder_api.rs
b_rep_check.rs
b_rep_class.rs
b_rep_class3d.rs
b_rep_extrema.rs
b_rep_feat.rs
b_rep_fill.rs
```
</details>

Let me reproduce the bug by running the binding generator on just `crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx`:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build -p opencascade-binding-generator 2>&1 | tail -5
```
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

Good - already compiled. Let me run the binding generator on just that header:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat scripts/regenerate-bindings.sh
```
<details>
<summary>Output (61 lines)</summary>

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

# Build the generator
echo "Building binding generator..."
cargo build --release -p opencascade-binding-generator

echo "Generating bindings from $CONFIG_FILE..."

# Clean generated directory
echo "Cleaning $OUTPUT_DIR..."
rm -f "$OUTPUT_DIR"/*.rs "$OUTPUT_DIR"/*.hxx

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

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat crates/opencascade-binding-generator/README.md | head -80
```
<details>
<summary>Output (80 lines)</summary>

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

Topological shapes form an inheritance hierarchy: `Shape` is the base, with subtypes `Vertex`, `Edge`, `Wire`, `Face`,
... (truncated, 3342 chars total)
````
</details>

Now I understand the structure well. The issue is that when  is processed, the parameter  becomes a `&mut` enum output param, so  is `val_enum_`. 

Then in the generated C++ wrapper, it creates:
```cpp
auto val_enum_ = static_cast<GeomAbs_Shape>(val);
```
and then passes `val_enum_` to the method, which in unmodified C++ would work but might conflict with something.

Wait—the error message says `val_enum_ undeclared identifier`. Let me think about this more carefully. Let me look at what exactly the C++ wrapper would look like for :

The method signature is:
```cpp
Standard_Boolean GetContinuity(const Standard_CString param, GeomAbs_Shape& val) const;
```

The  is a  (which is `const char*`) - that's a string parameter.
The  - that's a `&mut` enum output param.

So the generated wrapper should look like:
```cpp
extern "C" bool ShapeProcess_ShapeContext_GetContinuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
    auto result_ = self_->GetContinuity(param, val_enum_);
    val = static_cast<int32_t>(val_enum_);
    return result_;
}
```

That looks right. But wait - could there be an issue with  for the  parameter? Let me look at the code that builds args_str to understand what  is for a string param:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1250-1250), lines 1250 to 1350

This code path looks fine. Let me now try to actually generate the binding for `crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx` temporarily to see the error. Let me look at what test outputs exist for the binding generator:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls crates/opencascade-binding-generator/tests/ 2>/dev/null || echo "No tests dir"
```
```
No tests dir
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat scripts/run-binding-generator-tests.sh
```
<details>
<summary>Output (23 lines)</summary>

```
#!/bin/bash
# Run opencascade-binding-generator tests
#
# This script sets up the environment needed to run the binding generator
# tests, particularly DYLD_LIBRARY_PATH for libclang on macOS.
#
# Usage:
#   ./scripts/run-binding-generator-tests.sh [cargo test args...]
#
# Examples:
#   ./scripts/run-binding-generator-tests.sh
#   ./scripts/run-binding-generator-tests.sh -- --test-threads=1
#   ./scripts/run-binding-generator-tests.sh --test golden -- --test-threads=1

set -euo pipefail

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

exec cargo test -p opencascade-binding-generator "$@"
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls crates/opencascade-binding-generator/src/
```
```
codegenheader_deps.rsmain.rsmodule_graph.rsresolver.rs
config.rslib.rsmodel.rsparser.rstype_mapping.rs
```

Let me try generating the binding for that header to reproduce the error. First let me make a temporary toml:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && ls target/OCCT/include/ShapeProcess_ShapeContext.hxx 2>/dev/null && echo "exists"
```
```
target/OCCT/include/ShapeProcess_ShapeContext.hxx
exists
```

Created [test_shapeprocess.toml](../tmp/test_shapeprocess.toml)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && mkdir -p tmp/test_gen && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen 2>&1
```
<details>
<summary>Output (48 lines)</summary>

```
    Finished `release` profile [optimized] target(s) in 0.16s
     Running `target/release/occt-bindgen --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen`
Loaded config: 0 modules, 1 individual headers, 0 exclusions -> 1 headers
Parsing 86 headers...
  Clang parse time: 0.37s
  Collected 35 NCollection/template typedefs

Timing summary:
  Total clang parse time: 0.37s
  Total AST visit time: 0.57s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

  Detected 1 NCollection classes with non-primitive element types
Generating ffi.rs...
  Wrote: tmp/test_gen/ffi.rs (37 classes, 41 functions)
Generating wrappers.cpp...
  Wrote: tmp/test_gen/wrappers.cpp
Generating module re-exports...
  Wrote: tmp/test_gen/geom_abs.rs (0 types, 0 extra)
  Wrote: tmp/test_gen/standard.rs (13 types, 6 extra)
  Wrote: tmp/test_gen/t_collection.rs (4 types, 0 extra)
  Wrote: tmp/test_gen/message.rs (3 types, 3 extra)
  Wrote: tmp/test_gen/n_collection.rs (8 types, 0 extra)
  Wrote: tmp/test_gen/osd.rs (1 types, 0 extra)
  Wrote: tmp/test_gen/top_abs.rs (0 types, 0 extra)
  Wrote: tmp/test_gen/top_loc.rs (2 types, 2 extra)
  Wrote: tmp/test_gen/topo_ds.rs (2 types, 1 extra)
  Wrote: tmp/test_gen/shape_process.rs (2 types, 0 extra)
  Wrote: tmp/test_gen/std_fail.rs (1 types, 0 extra)
  Wrote: tmp/test_gen/t_col_std.rs (1 types, 1 extra)
  Wrote: tmp/test_gen/top_tools.rs (extra types only, 6 types)
  Wrote: tmp/test_gen/resource.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/shape_build.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/shape_extend.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/b_rep_tools.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/gp.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/lib.rs

Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  12 modules generated
```
</details>

**Terminal:**
```sh
grep -n "val_enum_\|GetContinuity\|ContinuityVal" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp
```
```
128:    return self_->GetContinuity(param, val_enum_);
131:    return static_cast<int32_t>(self_->ContinuityVal(param, static_cast<GeomAbs_Shape>(def)));
```

**Terminal:**
```sh
grep -n "val_enum_" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp -B10 -A5
```
<details>
<summary>Output (16 lines)</summary>

```
118-// ShapeProcess_ShapeContext wrappers
119-// ========================
120-
121-extern "C" ShapeProcess_ShapeContext* ShapeProcess_ShapeContext_ctor_charptr2(const char* file, const char* seq) {
122-    return new ShapeProcess_ShapeContext(file, seq);
123-}
124-extern "C" ShapeProcess_ShapeContext* ShapeProcess_ShapeContext_ctor_shape_charptr2(const TopoDS_Shape* S, const char* file, const char* seq) {
125-    return new ShapeProcess_ShapeContext(*S, file, seq);
126-}
127-extern "C" Standard_Boolean ShapeProcess_ShapeContext_get_continuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
128:    return self_->GetContinuity(param, val_enum_);
129-}
130-extern "C" int32_t ShapeProcess_ShapeContext_continuity_val(const ShapeProcess_ShapeContext* self_, const char* param, int32_t def) {
131-    return static_cast<int32_t>(self_->ContinuityVal(param, static_cast<GeomAbs_Shape>(def)));
132-}
133-extern "C" void ShapeProcess_ShapeContext_set_detalisation(ShapeProcess_ShapeContext* self_, int32_t level) {
```
</details>

Found the bug. The `get_continuity` wrapper at line 127-129 uses `val_enum_` but never declared it. The preamble (which should create ) is missing, and the postamble (which writes back) is also missing.

This is happening in the  wrapper for `get_continuity`. Let me understand why the preamble is missing. Looking at line 3593 in `crates/opencascade-binding-generator/src/codegen/bindings.rs` - let me check if the method `get_continuity` is classified as :

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#780-780), lines 780 to 870

I see the problem now. Look at line 816-823:  checks for  BEFORE checking for . The method  takes both a  (which is a C string) AND a  (which is a `&mut` enum). 

Since  is true, it classifies as , not . Then the  code path doesn't handle the `val_enum_` local variable at all!

Let me look at the  wrapper generation:

Searched for text `CStringParam wrapper methods` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3201](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3201)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3201-3201), lines 3201 to 3340

Now I can see the bug clearly. The  code path (section 3) doesn't  code path (section 3) doesn't emit the preamble/postamble for `&mut enum` output params. It just uses  directly in , but  for a `&mut enum` param is `val_enum_` (the local variable name), which is never declared in this code path.

The fix should be: when we have a method with BOTH a CString param AND a `&mut enum` param (or `&mut enum` takes priority), we either:

1. Prioritize  over  when both conditions are true, and handle the cstring within ; OR
2. Add the preamble/postamble to the  wrapper generation.

Option 2 is cleaner. Let me add preamble/postamble handling to the  section:

<details>
<summary>Reading lines 3201-3310 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3201-L3311">bindings.rs</a></summary>

```
    // 3. CStringParam wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // Determine return behaviour
        let returns_cstring = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type == "const char*")
            .unwrap_or(false);
        let returns_reference = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type.contains('&'))
            .unwrap_or(false);

        if returns_cstring {
            writeln!(
                output,
                "extern \"C\" const char* {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if returns_reference {
            let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;
            writeln!(
                output,
                "extern \"C\" {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if wm.return_type.is_none() {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            let rt = wm.return_type.as_ref().unwrap();
            let ret_cpp = &rt.cpp_type;
            writeln!(
                output,
                "extern \"C\" {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            if rt.enum_cpp_name.is_some() {
                writeln!(
                    output,
                    "    return static_cast<int32_t>(self_->{method}({args_str}));",
                    method = wm.cpp_method_name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    return self_->{method}({args_str});",
                    method = wm.cpp_method_name
                )
                .unwrap();
            }
        }
        writeln!(output, "}}").unwrap();
    }

```
</details>

Now I understand the structure. I need to add preamble/postamble handling for `&mut enum` params in the  section. The fix needs to:

1. After opening the `extern "C" {...` body, emit preamble (create local enum vars)
2. After the call, emit postamble (write back enum values to int32_t& output params)
3. Adjust the return type to `int32_t` for any enum return

Actually, looking more closely - the  function should prefer  when a method has both. The  handling for C strings is straightforward (pass them through directly), while the  case needs special preamble/postamble. The simplest fix is to change the priority in  so  takes precedence over , and then ensure the  code handles C string params correctly (it should, because  for a `const char*` param is just the param name).

Let me verify how  affects the , and whether the  CString params would be handled correctly:

Searched for text `CString conversion` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4252](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4252)

Searched for text `is_c_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:799](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L799)
- [bindings.rs:803](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L803)

Searched for regex `CStringParam.*build_param|build_param.*CString` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `WrapperKind::CStringParam.*=>|CStringParam =>|match.*CStringParam` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me understand whether CString params have a different  than their regular name. Looking at  -  would be a  in the type system, which would map through  as `const char*`. The  would be set via . Let me read that function:

Searched for text `fn param_to_cpp_extern_c_arg` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:873](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L873)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#873-873), lines 873 to 910

For a  type,  would return  (dereference it). But wait, for C strings, it might be handled differently. Let me look at  for the C string case more carefully. 

Actually,  may just be `const char*` (a ), not a . Let me check:

<details>
<summary>Searched for regex <code>c_string|CString|Standard_CString</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:138](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L138)
- [bindings.rs:140](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L140)
- [bindings.rs:799](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L799)
- [bindings.rs:800](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800)
- [bindings.rs:803](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L803)
- [bindings.rs:816](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L816)
- [bindings.rs:817](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L817)
- [bindings.rs:818](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L818)
- [bindings.rs:819](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L819)
- [bindings.rs:3185](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3185)
- [bindings.rs:3187](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3187)
- [bindings.rs:3191](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3191)
- [bindings.rs:3192](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3192)
- [bindings.rs:3201](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3201)
- [bindings.rs:3205](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3205)
- [bindings.rs:3232](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3232)
- [bindings.rs:3243](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3243)
- [bindings.rs:3311](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3311)
</details>

Searched for text `is_c_string` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result
- [model.rs:494](../crates/opencascade-binding-generator/src/model.rs#L494)

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#488-488), lines 488 to 510

So  returns true for  (i.e., `const char*`). For such a param,  returns just  (it falls through to ). Good - so C string params pass through correctly in the  code path.

The root cause is clear:  checks  before . The correct fix is to check  first (it should take priority), OR add preamble/postamble handling to the  section.

The cleanest fix is to check  in , since the  code already handles enum return types and other wrapper concerns, and C string params work fine in that path (they're just passed through).

Let me look at the full relevant section around line 816:

<details>
<summary>Reading lines 797-833 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L797-L834">bindings.rs</a></summary>

```
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

```
</details>

Now I'll fix the priority: move  check before  (and also before ), since  already handles enums in all param/return positions correctly:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L814-L824">bindings.rs -10+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "814:ww,815:ar,816:eh,817:pp,818:lh,819:go,820:ww,821:ae,822:rz,823:lc",
      "content": "    if returns_by_value {\n        WrapperKind::ByValueReturn\n    } else if has_mut_ref_enum_params(&method.params, all_enums) {\n        WrapperKind::MutRefEnumParam\n    } else if has_cstring_param {\n        WrapperKind::CStringParam\n    } else if returns_cstring {\n        WrapperKind::CStringReturn\n    } else if resolver::has_const_mut_return_mismatch(method) {\n        WrapperKind::ConstMutReturnFix"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":814,"endLine":823}
```
</details>

But wait - the  code path also needs to handle `const char*` params in its FFI declaration. Let me verify by checking how  would handle a `const char*` (C string) param in the  context.

Looking at :
- For  (a C string), it falls through to the  path since it's not a , not an enum, and not nullable (probably). 
-  for  where inner is  → the match arm for  returns `"const char*"` ... wait, let me re-read that function:

```rust
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if returns `"const char*"` ... wait, let me re-read that function:

```rust
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
```

Yes,  returns `"const char*"` correctly.
And  falls through to  = .

So in the  code path,  (a C string) would have  and  - which is correct.

And for  (a ), .

The generated code in  section would be:
```cpp
extern "C" Standard_Boolean ShapeProcess_ShapeContext_get_continuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
    auto result_ = self_->GetContinuity(param, val_enum_);
    val = static_cast<int32_t>(val_enum_);
    return result_;
}
```

That's correct.

Now let me also check the static method equivalent. Let me look at `classify_static_wrapper_kind` (if it exists):

Searched for text `fn classify_static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `classify.*static|StaticMethod.*wrapper|cstring.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3187](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3187)

Searched for regex `fn classify_wrapper_kind|static.*wrapper.*kind|wrapper.*kind.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:798](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L798)

Good - there's only one  for instance methods. Let me also check for static method classification in `crates/opencascade-binding-generator/src/resolver.rs`:

Searched for regex `cstring.*static|static.*cstring|mut_ref_enum.*static` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

Now let me also check if the same -vs- priority bug exists in the static method wrapper generation in bindings.rs:

Searched for regex `static.*method.*wrapper|wrapper.*static|StaticWrapper|WrapperKind.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:3184](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3184)
- [bindings.rs:3185](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3185)
- [bindings.rs:3733](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3733)
- [bindings.rs:4110](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4110)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3733-3733), lines 3733 to 3840

Good - the static method wrapper *already* handles  correctly. The static wrapper uses  as an explicit check before other types. Only the instance method's  had the priority bug.

Now let me build the generator and regenerate the test output to verify the fix:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 5.42s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && rm -rf tmp/test_gen && mkdir -p tmp/test_gen && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen 2>&1
```
<details>
<summary>Output (48 lines)</summary>

```
    Finished `release` profile [optimized] target(s) in 0.07s
     Running `target/release/occt-bindgen --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen`
Loaded config: 0 modules, 1 individual headers, 0 exclusions -> 1 headers
Parsing 86 headers...
  Clang parse time: 0.35s
  Collected 35 NCollection/template typedefs

Timing summary:
  Total clang parse time: 0.35s
  Total AST visit time: 0.57s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

  Detected 1 NCollection classes with non-primitive element types
Generating ffi.rs...
  Wrote: tmp/test_gen/ffi.rs (37 classes, 41 functions)
Generating wrappers.cpp...
  Wrote: tmp/test_gen/wrappers.cpp
Generating module re-exports...
  Wrote: tmp/test_gen/geom_abs.rs (0 types, 0 extra)
  Wrote: tmp/test_gen/standard.rs (13 types, 6 extra)
  Wrote: tmp/test_gen/t_collection.rs (4 types, 0 extra)
  Wrote: tmp/test_gen/message.rs (3 types, 3 extra)
  Wrote: tmp/test_gen/n_collection.rs (8 types, 0 extra)
  Wrote: tmp/test_gen/osd.rs (1 types, 0 extra)
  Wrote: tmp/test_gen/top_abs.rs (0 types, 0 extra)
  Wrote: tmp/test_gen/top_loc.rs (2 types, 2 extra)
  Wrote: tmp/test_gen/topo_ds.rs (2 types, 1 extra)
  Wrote: tmp/test_gen/shape_process.rs (2 types, 0 extra)
  Wrote: tmp/test_gen/std_fail.rs (1 types, 0 extra)
  Wrote: tmp/test_gen/t_col_std.rs (1 types, 1 extra)
  Wrote: tmp/test_gen/b_rep_tools.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/shape_build.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/gp.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/resource.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/top_tools.rs (extra types only, 6 types)
  Wrote: tmp/test_gen/shape_extend.rs (extra types only, 1 types)
  Wrote: tmp/test_gen/lib.rs

Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  12 modules generated
```
</details>

**Terminal:**
```sh
grep -n "val_enum_\|GetContinuity\|ContinuityVal" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp -A5 -B3
```
<details>
<summary>Output (20 lines)</summary>

```
125-    return new ShapeProcess_ShapeContext(*S, file, seq);
126-}
127-extern "C" int32_t ShapeProcess_ShapeContext_continuity_val(const ShapeProcess_ShapeContext* self_, const char* param, int32_t def) {
128:    return static_cast<int32_t>(self_->ContinuityVal(param, static_cast<GeomAbs_Shape>(def)));
129-}
130-extern "C" void ShapeProcess_ShapeContext_set_detalisation(ShapeProcess_ShapeContext* self_, int32_t level) {
131-    self_->SetDetalisation(static_cast<TopAbs_ShapeEnum>(level));
132-}
133-extern "C" int32_t ShapeProcess_ShapeContext_get_detalisation(const ShapeProcess_ShapeContext* self_) {
134-    return static_cast<int32_t>(self_->GetDetalisation());
135-}
136-extern "C" Standard_Boolean ShapeProcess_ShapeContext_get_continuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
137:    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
138:    auto result_ = self_->GetContinuity(param, val_enum_);
139:    val = static_cast<int32_t>(val_enum_);
140-    return result_;
141-}
142-extern "C" void ShapeProcess_ShapeContext_init(ShapeProcess_ShapeContext* self_, const TopoDS_Shape* S) {
143-    self_->Init(*S);
144-}
```
</details>

The fix is correct. Now `get_continuity` generates proper preamble (line 137) and postamble (line 139) for the `&mut enum` parameter.

Now let's remove the exclusion from  and run the full test suite to make sure everything passes:

<details>
<summary>Reading lines 244-260 of <a href="../crates/opencascade-sys/bindings.toml#L244-L261">bindings.toml</a></summary>

```

    # BVH template classes: abstract base or complex template type params
    # that the generator can't resolve correctly.
    "BVH_Properties.hxx",

    # Codegen bug: val_enum_ undeclared identifier.
    "ShapeProcess_ShapeContext.hxx",

    # Protected operator delete (TDF_LabelNode).
    "TDF_LabelNode.hxx",

    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # XSControl_Vars: const char*& output parameter (name) mishandled by generator.
    "XSControl_Vars.hxx",

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L249-L251">bindings.toml -4+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    # Protected operator delete (TDF_LabelNode).",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "249:hq,250:ay,251:dd,252:lw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":249,"endLine":250}
```
</details>

Read [bindings.toml](../crates/opencascade-sys/bindings.toml#244-244), lines 244 to 265

Good. Now run the full binding regeneration and build:

**Todo list:**
- ✅ Fix priority bug in classifier
- ✅ Remove ShapeProcess exclusion from bindings.toml
- 🔄 Regenerate bindings
- ⬜ Cargo build and tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (267 lines)</summary>

```
Building binding generator...
    Finished `release` profile [optimized] target(s) in 0.07s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
Loaded config: 168 modules, 0 individual headers, 11 exclusions -> 2939 headers
  Excluded 3 dependency headers after resolution
Parsing 3230 headers...
/Users/sethml/src/opencascade-rs/target/OCCT/include/OSD_WNT.hxx:23:12: fatal error: 'windows.h' file not found
  Clang parse time: 0.87s
  Collected 1303 NCollection/template typedefs

Timing summary:
  Total clang parse time: 0.87s
  Total AST visit time: 1.51s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

  Detected 100 NCollection classes with non-primitive element types
  Detected 71 NCollection classes with primitive element types
Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (2307 classes, 1042 functions)
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
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/app_blend.rs (1 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/o
... (truncated, 26653 chars total)
```
</details>

**Terminal:**
```sh
tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5ba2a68f-95ae-4c06-8770-168913b7677c/toolu_bdrk_01NscyaAvAPGUgRQ95ie5ex9__vscode-1771312782054/content.txt"
```
<details>
<summary>Output (20 lines)</summary>

```
solid.rs (extra types only, 11 types)                                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_
view.rs (extra types only, 1 types)                                               Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/prox_
pnt.rs (extra types only, 1 types)                                                Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
ap203.rs (extra types only, 4 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 1 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  213 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  9.9s
```
</details>

Bindings regenerated successfully. Now let's check that  appears in the generated output:

**Terminal:**
```sh
grep -n "get_continuity\|val_enum_" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/*.cpp 2>/dev/null | head -20
```
<details>
<summary>Output (7 lines)</summary>

```
16897:extern "C" Standard_Integer math_GlobOptMin_get_continuity(const math_GlobOptMin* self_) {
88027:extern "C" Standard_Boolean ShapeProcess_ShapeContext_get_continuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
88028:    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
88029:    auto result_ = self_->GetContinuity(param, val_enum_);
88030:    val = static_cast<int32_t>(val_enum_);
101513:extern "C" Standard_Integer IGESToBRep_Actor_get_continuity(const IGESToBRep_Actor* self_) {
102059:extern "C" Standard_Integer IGESToBRep_CurveAndSurface_get_continuity(const IGESToBRep_CurveAndSurface* self_) {
```
</details>

The fix is in the generated code. Now build:

**Todo list:**
- ✅ Fix priority bug in classifier
- ✅ Remove ShapeProcess exclusion from bindings.toml
- ✅ Regenerate bindings
- 🔄 Cargo build and tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (7 lines)</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
   Compiling examples v0.2.0 (/Users/sethml/src/opencascade-rs/examples)
   Compiling viewer v0.2.0 (/Users/sethml/src/opencascade-rs/crates/viewer)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.97s
real  23.0s
```
</details>

Build passes. Now run the tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-9f2064aa9f77643d)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
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

real  3.8s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```
<details>
<summary>Output (345 lines)</summary>

```
   Compiling examples v0.2.0 (/Users/sethml/src/opencascade-rs/examples)
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
   Compiling viewer v0.2.0 (/Users/sethml/src/opencascade-rs/crates/viewer)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.88s
     Running unittests src/lib.rs (target/debug/deps/examples-638415a022e1f97e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/write_model.rs (target/debug/deps/write_model-4fb5f107f01498a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/golden.rs (target/debug/deps/golden-a67f7fbf81fc69ec)

running 23 tests
test high_level_bottle ... ignored, non-deterministic STEP output

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpcV7hF5/airfoil.step(222 ents)  Write  Done
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpUkQalK/bounding-box.step(625 ents)  Write  Done
test airfoil ... ok
test bounding_box ... ok

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******
** WorkSession : Sending all data

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******

*******************************************************************
******        Statistics on T
... (truncated, 16950 chars total)
```
</details>

**Terminal:**
```sh
tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5ba2a68f-95ae-4c06-8770-168913b7677c/toolu_bdrk_015y89M53Znm7NsX6sswCKV2__vscode-1771312782060/content.txt"
```
<details>
<summary>Output (20 lines)</summary>

```
   Doc-tests model_api

running 1 test
test crates/model-api/src/wasm.rs - wasm::__export_model_world_impl (line 11) ..
. ok                                                                            
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.38s                                                                   
   Doc-tests occt_sys

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.00s                                                                   
   Doc-tests opencascade

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.00s                                                                   
real  42.2s
```
</details>

All tests pass. Let me clean up the temp files:

**Terminal:**
```sh
rm -rf /Users/sethml/src/opencascade-rs/tmp/test_gen /Users/sethml/src/opencascade-rs/tmp/test_shapeprocess.toml
```
```
(base) whitelily-7:opencascade-rs sethml$
```

**Todo list:**
- ✅ Fix priority bug in classifier
- ✅ Remove ShapeProcess exclusion from bindings.toml
- ✅ Regenerate bindings
- ✅ Cargo build and tests

**Root cause:** In `crates/opencascade-binding-generator/src/codegen/bindings.rs`,  was checked *before* .  param *and* a  (`&mut` enum output) param, so it was incorrectly classified as . The  code path doesn't emit the preamble/postamble local variable pattern needed for `&mut enum` params, so the generated C++ used `val_enum_` without declaring it.

**Fix:** Moved the  check above  (and ) in  code path handles C string params correctly since they pass through as `const char*` without special transformation. The exclusion of `crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx` from `crates/opencascade-sys/bindings.toml` was also removed.

*2026-02-18 14:38 · 412s · Words: 567 in · 2,471 out · 67,202 ctx · 65 rounds*

---

<a id="user-3"></a>

## User (3)

> Why is val of type i32 rather than the appropriate enum type here?

> **⚠️ This request was canceled.**

*2026-02-18 14:45*

### Assistant — Claude Sonnet 4.6

> 💭 The user is asking about why `get_continuity` has `val: &mut i32` instead of `val: &mut crate::geom_abs::Shape`. Looking at the generated code:
> 
> ```rust
> pub fn get_continuity(&self, param: &str, val: &mut i32) -> bool {
> ```
> 
> The C++ signature is:
> ```cpp
> Standard_Boolean GetContinuity(const Standard_CString param, GeomAbs_Shape& val) const;
> ```
> 
> So `val` should be `&mut crate::geom_abs::Shape` instead of `&mut i32`.
> 
> The reason is in `build_param_binding` in bindings.rs. When a `MutRef(Class(enum_name))` parameter is encountered, the generated Rust type in `rust_reexport_type` is `"&mut i32"` (not the Rust enum type). Let me look at the code again:
> 
> 
> I'm seeing the issue now—when handling mutable references to enum classes, the binding generator is hardcoding both the FFI type and the reexport type as `&mut i32`, which means it's always using the raw integer representation instead of the actual Rust enum type.
> 
> To fix this properly, I'd need to look up the enum's Rust type (similar to how value enums are handled) and then generate conversion logic in the Rust wrapper that takes the enum reference, converts it to i32 for the FFI call, and converts the result back to the enum type afterward. This is more involved than the value enum case since it requires a local variable pattern in the wrapper rather than just a simple `.into()` call.

Looking at  in the binding generator — when a  is detected, the  is hardcoded to `"&mut i32"`:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1180-1180), lines 1180 to 1205

> 💭 The root cause is clear. The `rust_reexport_type` for `&mut enum` params is hardcoded to `"&mut i32"` with the comment "No enum conversion at Rust level — C++ wrapper handles int32_t ↔ enum". 
> 
> To fix this properly, we'd need to:
> 1. Look up the Rust enum type for the `enum_name` 
> 2. Set `rust_reexport_type` to `&mut EnumType`
> 3. In the re-export method, generate a local variable pattern: `let mut val_int32 = *val as i32; ffi(..., &mut val_int32); *val = EnumType::try_from(val_int32).unwrap();`

The root cause:  is hardcoded to `"&mut i32"` with the comment explaining the C++ wrapper handles the `int32_t ↔ enum` conversion. The re-export layer exposes the raw FFI type directly without lifting it back to the Rust enum type.

Fixing it properly requires the Rust re-export wrapper to generate a local variable pattern — analogous to what the C++ wrapper does:
```rust
// Instead of passing val directly, we'd need:
let mut val_i32 = *val as i32;
ffi::ShapeProcess_ShapeContext_get_continuity(self, c_param.as_ptr(), &mut val_i32);
*val = geom_abs::Shape::try_from(val_i32).unwrap();
```

Let me check whether the re-export codegen supports this kind of pre/post-call pattern:

Searched for regex `preamble|postamble|local.*var|mut_ref_enum|enum_rust_type` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 2 results
- [rust.rs:636](../crates/opencascade-binding-generator/src/codegen/rust.rs#L636)
- [rust.rs:661](../crates/opencascade-binding-generator/src/codegen/rust.rs#L661)

> 💭 The rust.rs codegen handles `enum_rust_type` for regular enum params (converting with `.into()`) but doesn't have a pattern for `&mut enum` output params. Let me look at the broader context of how method bodies are generated in rust.rs.

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#600-600), lines 600 to 700

> 💭 Now I see the full picture. In the Rust re-export code generation (`rust.rs`), the arg-building logic for method params handles:
> - `&str` params: generate CString prelude and pass `c_name.as_ptr()`
> - `enum_rust_type.is_some()`: pass `name.into()`
> - Everything else: pass `name` directly
> 
> For `&mut enum` output params (`mut_ref_enum_cpp_name.is_some()`), since `rust_reexport_type = "&mut i32"` and `enum_rust_type = None`, the param is just passed through directly as `name`. The type is `&mut i32` in the re-export signature, which compiles and works but isn't ergonomic.
> 
> 
> To fix this properly, I'd need to track the enum rust type for mutable references and generate a conversion pattern that transforms the enum value to its underlying integer representation before the call, then passes the mutable reference to that temporary.

The Rust re-export codegen's arg-building loop has no case for `mut_ref_enum_cpp_name`. Since  is  and  is `"&mut i32"`, the param just passes through as-is. The proper fix needs:

1. A new field (e.g. ) on `ParamBinding` to hold the Rust enum type
2.  to set  and populate the field 
3. The Rust re-export method emitter to generate a local `i32` variable, pass it to FFI, then write back via `try_from`

Let me look at the method body emitter to understand the full change needed:

Searched for text `fn emit_method_wrapper` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

> 💭 Let me look for where method bodies are emitted in rust.rs.

Searched for regex `fn emit_|wrapper_method|reexport_method|WrapperMethod` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 3 results
- [rust.rs:526](../crates/opencascade-binding-generator/src/codegen/rust.rs#L526)
- [rust.rs:602](../crates/opencascade-binding-generator/src/codegen/rust.rs#L602)
- [rust.rs:858](../crates/opencascade-binding-generator/src/codegen/rust.rs#L858)

*2026-02-18 14:46 · 63s · Words: 13 in · 694 out · 6 rounds*

---

<a id="user-4"></a>

## User (4)

> Is that consistent with other enum types? Is this a problem that&#x27;s unique to enums passed by mut ref? Anyway, see if you can make it consistent with other enums, exposing the rust enum type in the public API.

*2026-02-18 14:48*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to fix the `&mut enum` output parameters so they use the proper Rust enum type instead of `&mut i32` in the public re-export API. Currently, by-value and const-ref enum params use typed Rust enums, but `&mut` enum output params use `&mut i32`.
> 
> Let me plan this out:
> 
> 1. In `build_param_binding`, when we detect a `MutRef(Class(enum_name))` that's an enum, we need to:
>    - Set `rust_reexport_type` to `&mut <EnumType>` instead of `&mut i32`
>    - Add or repurpose a field to carry the Rust enum type info for the re-export codegen
>    
> 2. For the re-export method generation, I need to wrap the mutable enum parameter with a conversion pattern: convert the typed enum to `i32` before the FFI call, then convert the result back to the enum type afterward.
> 
> Now I need to trace through how the re-export method bodies are actually constructed to see where this conversion logic should be inserted.

**Todo list:**
- 🔄 Understand re-export method codegen
- ⬜ Add mut_ref_enum_rust_type to ParamBinding
- ⬜ Update build_param_binding for &mut enum
- ⬜ Update Rust re-export codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Yes — by-value and const-ref enum params get typed Rust enums, but `&mut` enum output params are stuck at `&mut i32`. Let me trace the full re-export codegen to understand what needs to change.

Searched for regex `fn generate_method_body|fn emit_method|wrapper_methods|method_body|reexport.*method|method.*reexport` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

<details>
<summary>Searched for regex <code>fn generate_module|impl.*block|wrapper_kind|method.*impl</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 10 results</summary>

- [rust.rs:4](../crates/opencascade-binding-generator/src/codegen/rust.rs#L4)
- [rust.rs:329](../crates/opencascade-binding-generator/src/codegen/rust.rs#L329)
- [rust.rs:361](../crates/opencascade-binding-generator/src/codegen/rust.rs#L361)
- [rust.rs:523](../crates/opencascade-binding-generator/src/codegen/rust.rs#L523)
- [rust.rs:692](../crates/opencascade-binding-generator/src/codegen/rust.rs#L692)
- [rust.rs:693](../crates/opencascade-binding-generator/src/codegen/rust.rs#L693)
- [rust.rs:749](../crates/opencascade-binding-generator/src/codegen/rust.rs#L749)
- [rust.rs:768](../crates/opencascade-binding-generator/src/codegen/rust.rs#L768)
- [rust.rs:857](../crates/opencascade-binding-generator/src/codegen/rust.rs#L857)
- [rust.rs:1110](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1110)
</details>

<details>
<summary>Reading lines 768-1000 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L768-L1001">rust.rs</a></summary>

```
    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");

        for bindings in header_bindings {
            output.push_str(&super::bindings::emit_reexport_class(bindings, module_name));
        }
    }

    // Re-export additional types (handles, opaque references, collection iterators)
    // that appear in ffi.rs but aren't covered by ClassBindings or collections.
    // Skip types already re-exported by ClassBindings (directly_exported_handles or base_handle_reexports).
    if !extra_types.is_empty() {
        let mut extra_lines = Vec::new();
        for (ffi_name, short_name) in extra_types {
            // Skip handle types that are already re-exported by emit_reexport_class (has_to_handle or has_handle_get)
            // or by the base handle re-exports section above.
            if directly_exported_handles.contains(ffi_name.as_str())
                || base_handle_reexports.contains(ffi_name.as_str())
            {
                continue;
            }
            if ffi_name == short_name {
                extra_lines.push(format!("pub use crate::ffi::{};\n", ffi_name));
            } else {
                extra_lines.push(format!(
                    "pub use crate::ffi::{} as {};\n",
                    ffi_name, short_name
                ));
            }
        }
        if !extra_lines.is_empty() {
            output.push_str("// ========================\n");
            output.push_str("// Additional type re-exports\n");
            output.push_str("// ========================\n\n");
            for line in &extra_lines {
                output.push_str(line);
            }
            output.push('\n');
        }
    }

    output
}
/// Generate an `impl` block for a collection type, re-exporting its FFI helper functions as methods.
fn emit_collection_impl(coll: &super::colle
... (truncated, 12281 chars)
```
</details>

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#768-768), lines 768 to 1000

That's the collection codegen. I need the method wrapper generation from . Let me find it:

Searched for text `fn emit_reexport_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4183](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4183)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4183-4183), lines 4183 to 4350

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4350-4350), lines 4350 to 4550

Now let me look at , , and  to understand the full conversion chain:

Searched for regex `fn convert_arg\b|fn cstr_prelude|fn build_reexport_body` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:4112](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4112)
- [bindings.rs:4146](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4146)
- [bindings.rs:4153](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4153)
- [bindings.rs:4163](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4163)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4110-4110), lines 4110 to 4183

**Todo list:**
- ✅ Understand re-export method codegen
- 🔄 Add mut_ref_enum_rust_type to ParamBinding
- ⬜ Update build_param_binding for &mut enum
- ⬜ Update Rust re-export codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now I understand the full picture. Here's my plan:

1. **`ParamBinding`**: Add  field
2. ****: For `&mut enum` params, set  and populate `mut_ref_enum_rust_type`
3. **** / ****: For `mut_ref_enum_rust_type`, generate a local temp variable name instead of passing the param directly
4. ****: Add preamble for mut_ref_enum params (create local `i32`, convert from enum)
5. **Method body codegen**: Add postamble that writes back the enum value after the FFI call
6. **`ResolvedParamBinding`**: Needs the same field

Let me look at both `ParamBinding` and `ResolvedParamBinding`:

Searched for text `struct ParamBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:268](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L268)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#260-260), lines 260 to 340

Now I need to understand the approach. Currently:
-  runs BEFORE the `unsafe` block and produces  lines
-  produces the arg expressions used in the FFI call
-  wraps the FFI call in `unsafe { ... }`

For `&mut enum` params, the generated Rust re-export needs to be structured like:

```rust
pub fn get_continuity(&self, param: &str, val: &mut geom_abs::Shape) -> bool {
    let c_param = std::ffi::CString::new(param).unwrap();
    let mut val_i32_: i32 = (*val).into();
    let result_ = unsafe { crate::ffi::ShapeProcess_ShapeContext_get_continuity(self as *const Self, c_param.as_ptr(), &mut val_i32_) };
    *val = crate::geom_abs::Shape::try_from(val_i32_).unwrap();
    result_
}
```

This requires a more complex body generation than simply wrapping the call. The key changes:
1. Prelude: create `let mut val_i32_: i32 = (*val).into();`
2. Arg conversion: pass `&mut val_i32_` instead of 
3. Store result in `let result_ = unsafe { ... };`
4. Postamble: 
5. Return `result_`

The simplest approach is to detect `mut_ref_enum` params at the method body construction site and generate the full body specially. Let me now also check where `ResolvedParamBinding` gets constructed, since inherited methods need the same fix:

Searched for text `ResolvedParamBinding {"` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `ResolvedParamBinding \{` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:308](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L308)
- [bindings.rs:2609](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2609)
- [bindings.rs:2643](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2643)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2595-2595), lines 2595 to 2670

Now let me check what  looks like for `&mut enum` params in the resolver:

Searched for regex `mut_ref.*enum|enum.*mut_ref|MutRef.*enum_cpp` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

<details>
<summary>Searched for regex <code>ResolvedParam|enum_cpp_name</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 17 results</summary>

- [resolver.rs:151](../crates/opencascade-binding-generator/src/resolver.rs#L151)
- [resolver.rs:172](../crates/opencascade-binding-generator/src/resolver.rs#L172)
- [resolver.rs:199](../crates/opencascade-binding-generator/src/resolver.rs#L199)
- [resolver.rs:232](../crates/opencascade-binding-generator/src/resolver.rs#L232)
- [resolver.rs:284](../crates/opencascade-binding-generator/src/resolver.rs#L284)
- [resolver.rs:295](../crates/opencascade-binding-generator/src/resolver.rs#L295)
- [resolver.rs:325](../crates/opencascade-binding-generator/src/resolver.rs#L325)
- [resolver.rs:1051](../crates/opencascade-binding-generator/src/resolver.rs#L1051)
- [resolver.rs:1052](../crates/opencascade-binding-generator/src/resolver.rs#L1052)
- [resolver.rs:1100](../crates/opencascade-binding-generator/src/resolver.rs#L1100)
- [resolver.rs:1101](../crates/opencascade-binding-generator/src/resolver.rs#L1101)
- [resolver.rs:1163](../crates/opencascade-binding-generator/src/resolver.rs#L1163)
- [resolver.rs:1164](../crates/opencascade-binding-generator/src/resolver.rs#L1164)
- [resolver.rs:1232](../crates/opencascade-binding-generator/src/resolver.rs#L1232)
- [resolver.rs:1233](../crates/opencascade-binding-generator/src/resolver.rs#L1233)
- [resolver.rs:1331](../crates/opencascade-binding-generator/src/resolver.rs#L1331)
- [resolver.rs:1349](../crates/opencascade-binding-generator/src/resolver.rs#L1349)
</details>

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#1300-1300), lines 1300 to 1360

:

Searched for text `fn extract_enum_name_from_type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:1365](../crates/opencascade-binding-generator/src/resolver.rs#L1365)

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#1365-1365), lines 1365 to 1395

Good. The resolver's  deliberately skips  (line 1369 comment says "NOT MutRef (output params need special handling)"). So  falls through to 's default path — it won't set . 

This means in the inherited method codegen (line 2643 area), a `&mut enum` param doesn't have  set, so it just gets treated as a regular ref type. Let me check what that looks like:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2620-2620), lines 2620 to 2665

So for inherited methods, a `&mut enum` param (where  is  because  skips ) just falls through and gets type `&mut EnumType` at the FFI level — but the C++ wrapper also handles this with the `MurRefEnumParam` pattern. Let me check — does the inherited method builder know about ?

Actually, for inherited methods, the `resolve_inhabited_method` code path in `crates/opencascade-binding-generator/src/codegen/bindings.rs` _also_ calls into the same C++ codegen that handles . So the C++ wrappers are already correct. The issue is only in the Rust re-export layer. Let me understand the situation for inherited methods vs own methods:

For **own methods** (wrapper_methods):  handles the `&mut enum` → `ParamBinding` with `mut_ref_enum_cpp_name`, , etc.

For **inherited methods**: The code above at line 2643 constructs `ResolvedParamBinding`. The `&mut enum` case doesn't have  set (because the resolver skips MutRef), so  and  would be... let me check what  returns for :

Searched for text `fn type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:920](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L920)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#920-920), lines 920 to 990

So for inherited methods with  returns `&mut crate::geom_abs::Shape` — the correct Rust enum type! But the FFI function expects `&mut i32`. So for inherited methods, the re-export type is already right but the arg conversion is wrong (it passes the enum ref directly to an `&mut i32` parameter).

Wait, I need to check — what does the inherited method's C++ wrapper look like for a `&mut enum` param? Let me check if the  handling applies to inherited wrappers too.

Let me look at how inherited methods are built in the C++ generation:

<details>
<summary>Searched for regex <code>inherited.*wrapper|InheritedMethod|inherited_methods</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 16 results</summary>

- [bindings.rs:56](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L56)
- [bindings.rs:245](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L245)
- [bindings.rs:1806](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1806)
- [bindings.rs:1816](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1816)
- [bindings.rs:1827](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1827)
- [bindings.rs:1868](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1868)
- [bindings.rs:2431](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2431)
- [bindings.rs:2679](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2679)
- [bindings.rs:4011](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4011)
- [bindings.rs:4012](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4012)
- [bindings.rs:4483](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4483)
- [bindings.rs:4484](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4484)
- [bindings.rs:4791](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4791)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4011-4011), lines 4011 to 4090

I see — the inherited method C++ codegen (section 10) does NOT handle  at all! It doesn't generate preamble/postamble. So inherited methods with `&mut enum` params also have the same C++ bug where the local variable is used but never declared.

However, in practice it might not be hit because inherited methods with `&mut enum` params may not exist yet or may be filtered. Still, I should fix both the Rust re-export AND add the C++ preamble/postamble for inherited methods.

Let me now plan the full implementation. This is a multi-site change:

**Changes needed:**

1. **`ParamBinding` struct**: Add  
2. **`ResolvedParamBinding` struct**: Add 
3. ****: For `&mut enum`, look up and set  to `&mut EnumType` and `mut_ref_enum_rust_type`
4. **Inherited method's `ResolvedParamBinding` construction**: Detect  and set up properly
5. ** / **: For `mut_ref_enum`, pass `&mut <name>_i32_` instead
6. **Prelude generation**: Add `let mut <name>_i32_: i32 = (*<name>).into();` for mut_ref_enum params
7. **Method body**: For methods with mut_ref_enum params, generate multi-line body with postamble
8. **Inherited method C++ codegen (section 10)**: Add preamble/postamble for `&mut enum` params
9. **Free function codegen in `crates/opencascade-binding-generator/src/codegen/rust.rs`**: Also handle mut_ref_enum there
10. **PORTING.md**: Update the limitation

Let me start implementing:

**Todo list:**
- ✅ Understand re-export method codegen
- 🔄 Add mut_ref_enum_rust_type fields
- ⬜ Update build_param_binding
- ⬜ Update inherited param binding
- ⬜ Update convert_arg functions
- ⬜ Update prelude generation
- ⬜ Update method body generation
- ⬜ Fix inherited C++ codegen
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Let me read all the sections I need to modify:

<details>
<summary>Reading lines 268-300 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L268-L301">bindings.rs</a></summary>

```
pub struct ParamBinding {
    /// Original C++ parameter name (for use in C++ wrapper declarations)
    pub cpp_name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    /// Type as it appears in ffi.rs (e.g. "f64", "&gp_Pnt", "*mut gp_Pnt")
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt" or enum type)
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx parameter (e.g. "Standard_Real", "const gp_Pnt&")
    pub cpp_type: String,
    /// C++ argument expression when calling OCCT (e.g. param name, or "std::string(x).c_str()")
    pub cpp_arg_expr: String,
    /// If this is a value enum param, the qualified Rust enum type (e.g. "crate::top_abs::Orientation")
    pub enum_rust_type: Option<String>,
    /// If this is a &mut enum output param, the C++ enum name for local var + writeback pattern
    pub mut_ref_enum_cpp_name: Option<String>,
    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)
    pub is_nullable_ptr: bool,
}

/// A return type binding with info for all three output targets.
#[derive(Debug, Clone)]
pub struct ReturnTypeBinding {
    /// Type as it appears in ffi.rs
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx
    pub cpp_type: String,
    /// Whether the C++ return needs std::unique_ptr wrapping
    pub needs_unique_ptr: bool,
    /// If this is an enum return, the original C++ enum name (for static_cast)
```
</details>

<details>
<summary>Reading lines 306-325 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L306-L326">bindings.rs</a></summary>

```
/// A resolved parameter binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedParamBinding {
    pub name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt" or enum type)
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub cpp_arg_expr: String,
    /// If this is a value enum param, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)
    pub is_nullable_ptr: bool,
}

/// A resolved return type binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedReturnTypeBinding {
```
</details>

Now I also need to check all places that construct `ParamBinding` and `ResolvedParamBinding` to make sure I add the new field everywhere. Let me search for all construction sites:

<details>
<summary>Searched for regex <code>ParamBinding \{</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 9 results</summary>

- [bindings.rs:268](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L268)
- [bindings.rs:308](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L308)
- [bindings.rs:1177](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1177)
- [bindings.rs:1185](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1185)
- [bindings.rs:1209](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1209)
- [bindings.rs:1252](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1252)
- [bindings.rs:1285](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1285)
- [bindings.rs:2609](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2609)
- [bindings.rs:2643](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2643)
</details>

Good — 5 `ParamBinding` constructions and 2 Good — 5 `ParamBinding` constructions and 2 `ResolvedParamBinding` constructions. Let me read all of them to add the new field. Let me also check what the `FunctionBinding` params use:

Searched for regex `struct FunctionBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:341](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L341)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#341-341), lines 341 to 380

`FunctionBinding` uses  (same struct). Good.

Now let me read the full  function and all 4 construction sites of `ParamBinding`:

<details>
<summary>Reading lines 1177-1300 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1177-L1301">bindings.rs</a></summary>

```
fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                return ParamBinding {
                    cpp_name,
                    rust_name,
                    rust_ffi_type: "&mut i32".to_string(),
                    rust_reexport_type: "&mut i32".to_string(),
                    cpp_type: "int32_t&".to_string(),
                    // The arg expression uses the local variable name (preamble creates it)
                    cpp_arg_expr: format!("{}_enum_", name),
                    // No enum conversion at Rust level — C++ wrapper handles int32_t ↔ enum
                    enum_rust_type: None,
                    mut_ref_enum_cpp_name: Some(enum_name.clone()),
                    is_nullable_ptr: false,
                };
            }
        }
    }

    // Check if this parameter is an enum type (by value or const ref)
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        // Look up the Rust enum type for value enums
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            cpp_arg_expr: format!("static_cast<{}>({})", enum_cpp_name, name),
            enum_rust_type,
            mut_ref_enum_cpp_name: None,
            is_nullable_ptr: false,
        };
    }

    // Nullable pointer params: const T* = NULL -> Option<&T>, T* = NULL -> Option<&mut T>
    // In ffi.rs: *const T / *mut T (raw pointers, nullable)
    // In re-export: Option<&T> / Option<&mut T>
    // In C++: const T* / T* (passed through directly)
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = type_to_cpp(inner);
                (
                    format!("*const {}", inner_ffi),
                    format!("Option<&{}>", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = type_to_cpp(inner);
                (
                    format!("*mut {}", inner_ffi),
                    format!("Option<&mut {}>", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            is_nullable_ptr: true,
        };
    }

    // By-value class/handle params — opaque types
    // must be passed by reference. We convert them to const T& at the FFI
    // boundary; the C++ wrapper passes the reference to the original method
    // w
... (truncated, 5176 chars)
```
</details>

Now I have a clear picture. Let me implement all the changes. I'll work through the files methodically.

**Step 1: Add `mut_ref_enum_rust_type` to both structs and update **

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L283-L287">bindings.rs -2+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L319-L325">-4+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "283:ta,284:ar",
      "content": "    /// If this is a &mut enum output param, the C++ enum name for local var + writeback pattern\n    pub mut_ref_enum_cpp_name: Option<String>,\n    /// If this is a &mut enum output param, the qualified Rust enum type (e.g. \"crate::geom_abs::Shape\")\n    pub mut_ref_enum_rust_type: Option<String>,"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "317:dx,318:rk,319:ev,320:ru",
      "content": "    /// If this is a value enum param, the qualified Rust enum type\n    pub enum_rust_type: Option<String>,\n    /// If this is a &mut enum output param, the qualified Rust enum type\n    pub mut_ref_enum_rust_type: Option<String>,\n    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)\n    pub is_nullable_ptr: bool,"
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
      "startLine": 283,
      "endLine": 324
    }
  ]
}
```
</details>

Now update  for the `&mut enum` case to look up the Rust enum type:

<details>
<summary>Reading lines 1181-1205 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1181-L1206">bindings.rs</a></summary>

```
fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                return ParamBinding {
                    cpp_name,
                    rust_name,
                    rust_ffi_type: "&mut i32".to_string(),
                    rust_reexport_type: "&mut i32".to_string(),
                    cpp_type: "int32_t&".to_string(),
                    // The arg expression uses the local variable name (preamble creates it)
                    cpp_arg_expr: format!("{}_enum_", name),
                    // No enum conversion at Rust level — C++ wrapper handles int32_t ↔ enum
                    enum_rust_type: None,
                    mut_ref_enum_cpp_name: Some(enum_name.clone()),
                    is_nullable_ptr: false,
                };
            }
        }
    }

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1185-L1212">bindings.rs -20+27</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Check for &mut enum output params — these need special local var + writeback handling\n    if let Type::MutRef(inner) = ty {\n        if let Type::Class(enum_name) = inner.as_ref() {\n            if ffi_ctx.all_enums.contains(enum_name) {\n                let enum_rust_type = ffi_ctx.enum_rust_types\n                    .and_then(|map| map.get(enum_name))\n                    .cloned();\n                let reexport_type = enum_rust_type.as_ref()\n                    .map(|t| format!(\"&mut {}\", t))\n                    .unwrap_or_else(|| \"&mut i32\".to_string());\n                return ParamBinding {\n                    cpp_name,\n                    rust_name,\n                    rust_ffi_type: \"&mut i32\".to_string(),\n                    rust_reexport_type: reexport_type,\n                    cpp_type: \"int32_t&\".to_string(),\n                    // The arg expression uses the local variable name (preamble creates it)\n                    cpp_arg_expr: format!(\"{}_enum_\", name),\n                    // No value enum conversion at Rust level\n                    enum_rust_type: None,\n                    mut_ref_enum_cpp_name: Some(enum_name.clone()),\n                    mut_ref_enum_rust_type: enum_rust_type,\n                    is_nullable_ptr: false,\n                };\n            }\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1185:al,1186:yn,1187:ma,1188:rb,1189:ai,1190:kr,1191:nc,1192:yb,1193:oo,1194:wy,1195:yr,1196:uw,1197:cn,1198:af,1199:zz,1200:fm,1201:fj,1202:uo,1203:cy,1204:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1185,"endLine":1211}
```
</details>

Now I need to add  to all other `ParamBinding` and `ResolvedParamBinding` construction sites. Let me find them:

<details>
<summary>Reading lines 1215-1230 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1215-L1231">bindings.rs</a></summary>

```
        // Look up the Rust enum type for value enums
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            cpp_arg_expr: format!("static_cast<{}>({})", enum_cpp_name, name),
            enum_rust_type,
            mut_ref_enum_cpp_name: None,
            is_nullable_ptr: false,
        };
```
</details>

<details>
<summary>Reading lines 1260-1275 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1260-L1276">bindings.rs</a></summary>

```
            }
            _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            is_nullable_ptr: true,
        };
    }

```
</details>

<details>
<summary>Reading lines 1295-1310 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1295-L1311">bindings.rs</a></summary>

```

    ParamBinding {
        cpp_name,
        rust_name,
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        cpp_arg_expr,
        enum_rust_type: None,
        mut_ref_enum_cpp_name: None,
        is_nullable_ptr: false,
    }
}

fn build_return_type_binding(ty: &Type, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ReturnTypeBinding {
    // Check if this return type is an enum
```
</details>

Now let me read the two `ResolvedParamBinding` construction sites:

<details>
<summary>Reading lines 2609-2630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2609-L2631">bindings.rs</a></summary>

```
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*mut {}", inner_ffi),
                                        format!("Option<&mut {}>", inner_rust),
                                        format!("{}*", inner_cpp),
                                    )
                                }
                                _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
                            };
                            return ResolvedParamBinding {
                                name: p.name.clone(),
                                rust_name: p.rust_name.clone(),
                                rust_ffi_type,
                                rust_reexport_type,
                                cpp_type,
                                cpp_arg_expr: p.name.clone(),
                                enum_rust_type: None,
                                is_nullable_ptr: true,
                            };
                        }
```
</details>

<details>
<summary>Reading lines 2630-2680 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2630-L2681">bindings.rs</a></summary>

```
                        }

                        // Convert by-value class/handle params to const ref (same as build_param_binding)
                        let effective_ty = match &p.ty.original {
                            Type::Class(name) if name != "char" && p.ty.enum_cpp_name.is_none() => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            Type::Handle(_) => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            _ => p.ty.original.clone(),
                        };
                        let cpp_arg_expr = if let Some(ref enum_name) = p.ty.enum_cpp_name {
                            format!("static_cast<{}>({})", enum_name, p.name)
                        } else {
                            p.name.clone()
                        };
                        let cpp_param_type = if p.ty.enum_cpp_name.is_some() {
                            // Enum params are passed as int32_t at the extern "C" boundary;
                            // the static_cast in cpp_arg_expr converts to the actual enum type.
                            "int32_t".to_string()
                        } else {
                            type_to_cpp(&effective_ty)
                        };
                        ResolvedParamBinding {
                            name: p.name.clone(),
                            rust_name: p.rust_name.clone(),
                            rust_ffi_type: if p.ty.enum_cpp_name.is_some() { "i32".to_string() } else { type_to_ffi_full_name(&effective_ty) },
                            rust_reexport_type: if let Some(ref enum_name) = p.ty.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
                                type_to_rust_string(&effective_ty, reexport_ctx)
                            },
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),
                            is_nullable_ptr: false,
                        }
                    })
                    .collect();

                let return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        let enum_rust_type = rt.enum_cpp_name.as_ref()
                            .and_then(|n| symbol_table.enum_rust_types.get(n))
                            .cloned();
                        ResolvedReturnTypeBinding {
                            rust_ffi_type: if rt.enum_cpp_name.is_some() { "i32".to_string() } else { return_type_to_ffi_full_name(&rt.original) },
                            rust_reexport_type: if let Some(ref enum_name) = rt.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
```
</details>

Now I need to handle the inherited method case. The `&mut enum` param needs detection here too. The  would be  (because the resolver skips MutRef). I need to add a check.

Let me now apply all the changes in a batch. First the 4 `ParamBinding` sites and 2 `ResolvedParamBinding` sites that need , then the inherited method ResolvedParamBinding that needs the new `&mut enum` detection:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1227-L1231">bindings.rs -3+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1271-L1275">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1305-L1309">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2630-L2633">-2+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            enum_rust_type,\n            mut_ref_enum_cpp_name: None,\n            mut_ref_enum_rust_type: None,\n            is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1227:mw,1228:lp,1229:xt"
    },
    {
      "content": "            enum_rust_type: None,\n            mut_ref_enum_cpp_name: None,\n            mut_ref_enum_rust_type: None,\n            is_nullable_ptr: true,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1270:mr,1271:lp,1272:cm"
    },
    {
      "content": "        enum_rust_type: None,\n        mut_ref_enum_cpp_name: None,\n        mut_ref_enum_rust_type: None,\n        is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1303:qp,1304:bs,1305:tw"
    },
    {
      "content": "                                enum_rust_type: None,\n                                mut_ref_enum_rust_type: None,\n                                is_nullable_ptr: true,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2627:cn,2628:ig"
    }
  ]
}
```
**Output:**
```
{
  "applied": 4,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 1227,
      "endLine": 2632
    }
  ]
}
```
</details>

Now the main ResolvedParamBinding construction at line ~2654. I need to add `&mut enum` detection here. Let me re-read it:

<details>
<summary>Reading lines 2634-2680 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2634-L2681">bindings.rs</a></summary>

```
                        }

                        // Convert by-value class/handle params to const ref (same as build_param_binding)
                        let effective_ty = match &p.ty.original {
                            Type::Class(name) if name != "char" && p.ty.enum_cpp_name.is_none() => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            Type::Handle(_) => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            _ => p.ty.original.clone(),
                        };
                        let cpp_arg_expr = if let Some(ref enum_name) = p.ty.enum_cpp_name {
                            format!("static_cast<{}>({})", enum_name, p.name)
                        } else {
                            p.name.clone()
                        };
                        let cpp_param_type = if p.ty.enum_cpp_name.is_some() {
                            // Enum params are passed as int32_t at the extern "C" boundary;
                            // the static_cast in cpp_arg_expr converts to the actual enum type.
                            "int32_t".to_string()
                        } else {
                            type_to_cpp(&effective_ty)
                        };
                        ResolvedParamBinding {
                            name: p.name.clone(),
                            rust_name: p.rust_name.clone(),
                            rust_ffi_type: if p.ty.enum_cpp_name.is_some() { "i32".to_string() } else { type_to_ffi_full_name(&effective_ty) },
                            rust_reexport_type: if let Some(ref enum_name) = p.ty.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
                                type_to_rust_string(&effective_ty, reexport_ctx)
                            },
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),
                            is_nullable_ptr: false,
                        }
                    })
                    .collect();

                let return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        let enum_rust_type = rt.enum_cpp_name.as_ref()
                            .and_then(|n| symbol_table.enum_rust_types.get(n))
                            .cloned();
                        ResolvedReturnTypeBinding {
```
</details>

I need to add a check for  BEFORE the  match, similar to what  does. Let me insert it:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2635-L2660">bindings.rs -2+25</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n                        // Check for &mut enum output params — same as build_param_binding\n                        if let Type::MutRef(inner) = &p.ty.original {\n                            if let Type::Class(enum_name) = inner.as_ref() {\n                                if all_enum_names.contains(enum_name) {\n                                    let enum_rust_type = symbol_table.enum_rust_types.get(enum_name).cloned();\n                                    let reexport_type = enum_rust_type.as_ref()\n                                        .map(|t| format!(\"&mut {}\", t))\n                                        .unwrap_or_else(|| \"&mut i32\".to_string());\n                                    return ResolvedParamBinding {\n                                        name: p.name.clone(),\n                                        rust_name: p.rust_name.clone(),\n                                        rust_ffi_type: \"&mut i32\".to_string(),\n                                        rust_reexport_type: reexport_type,\n                                        cpp_type: \"int32_t&\".to_string(),\n                                        cpp_arg_expr: format!(\"{}_enum_\", p.name),\n                                        enum_rust_type: None,\n                                        mut_ref_enum_rust_type: enum_rust_type,\n                                        is_nullable_ptr: false,\n                                    };\n                                }\n                            }\n                        }\n\n                        // Convert by-value class/handle params to const ref (same as build_param_binding)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2635:dd,2636:gh"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2635,"endLine":2659}
```
</details>

Now add  to the remaining `ResolvedParamBinding` construction:

<details>
<summary>Reading lines 2680-2700 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2680-L2701">bindings.rs</a></summary>

```
                        };
                        ResolvedParamBinding {
                            name: p.name.clone(),
                            rust_name: p.rust_name.clone(),
                            rust_ffi_type: if p.ty.enum_cpp_name.is_some() { "i32".to_string() } else { type_to_ffi_full_name(&effective_ty) },
                            rust_reexport_type: if let Some(ref enum_name) = p.ty.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
                                type_to_rust_string(&effective_ty, reexport_ctx)
                            },
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),
                            is_nullable_ptr: false,
                        }
                    })
                    .collect();

                let return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        let enum_rust_type = rt.enum_cpp_name.as_ref()
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2692-L2695">bindings.rs -2+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),\n                            mut_ref_enum_rust_type: None,\n                            is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2692:zu,2693:bp"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2692,"endLine":2694}
```
</details>

**Todo list:**
- ✅ Understand re-export method codegen
- ✅ Add mut_ref_enum_rust_type fields
- ✅ Update build_param_binding
- ✅ Update inherited param binding
- 🔄 Update convert_arg functions
- ⬜ Update prelude generation
- ⬜ Update method body generation
- ⬜ Fix inherited C++ codegen
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now I need to update the Rust re-export codegen. The key insight: for methods with `&mut enum` params, we can't use the simple  pattern. We need a multi-statement body with:
1. Prelude: `let mut <name>_i32_: i32 = (*<name>).into();`
2. Call: `let result_ = unsafe { ffi::foo(self_, ..., &mut <name>_i32_) };`
3. Postamble: 
4. Return: `result_`

Let me update  first, then the prelude functions, and then the body generation sites.

For , when we see a `mut_ref_enum_rust_type`, the arg should be `&mut <name>_i32_`:

<details>
<summary>Reading lines 4114-4155 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4114-L4156">bindings.rs</a></summary>

```
            writeln!(
                output,
                "    return self->{method}({args_str});",
                method = im.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    self->{method}({args_str});",
                method = im.cpp_method_name
            )
            .unwrap();
        }

        writeln!(output, "}}").unwrap();
    }

    // 11. Destructor wrapper
    if !bindings.has_protected_destructor {
        writeln!(
            output,
            "extern \"C\" void {ffi_cn}_destructor({cn}* self_) {{ delete self_; }}"
        )
        .unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
```
</details>

<details>
<summary>Reading lines 4151-4185 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4151-L4186">bindings.rs</a></summary>

```
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", p.rust_name)
        }
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", p.rust_name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", p.rust_name)
    } else {
        p.rust_name.clone()
    }
}

fn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", name)
        }
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", name)
    } else {
        name.to_string()
    }
}

/// Generate CString let-bindings for all &str parameters.
/// These must appear before the unsafe block so the CStrings live long enough.
fn cstr_prelude_params(params: &[ParamBinding]) -> String {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4151-L4186">bindings.rs -31+35</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn convert_arg(p: &ParamBinding) -> String {\n    if p.is_nullable_ptr {\n        if p.rust_ffi_type.starts_with(\"*const\") {\n            format!(\"{}.map_or(std::ptr::null(), |r| r as *const _)\", p.rust_name)\n        } else {\n            format!(\"{}.map_or(std::ptr::null_mut(), |r| r as *mut _)\", p.rust_name)\n        }\n    } else if p.mut_ref_enum_rust_type.is_some() {\n        format!(\"&mut {}_i32_\", p.rust_name)\n    } else if p.rust_reexport_type == \"&str\" {\n        format!(\"c_{}.as_ptr()\", p.rust_name)\n    } else if p.enum_rust_type.is_some() {\n        format!(\"{}.into()\", p.rust_name)\n    } else {\n        p.rust_name.clone()\n    }\n}\n\nfn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {\n    if p.is_nullable_ptr {\n        if p.rust_ffi_type.starts_with(\"*const\") {\n            format!(\"{}.map_or(std::ptr::null(), |r| r as *const _)\", name)\n        } else {\n            format!(\"{}.map_or(std::ptr::null_mut(), |r| r as *mut _)\", name)\n        }\n    } else if p.mut_ref_enum_rust_type.is_some() {\n        format!(\"&mut {}_i32_\", name)\n    } else if p.rust_reexport_type == \"&str\" {\n        format!(\"c_{}.as_ptr()\", name)\n    } else if p.enum_rust_type.is_some() {\n        format!(\"{}.into()\", name)\n    } else {\n        name.to_string()\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4151:ly,4152:wr,4153:mg,4154:mz,4155:ag,4156:ge,4157:cy,4158:lz,4159:mv,4160:gg,4161:br,4162:ui,4163:ho,4164:ax,4165:io,4166:dd,4167:si,4168:wr,4169:mg,4170:he,4171:ag,4172:ze,4173:cy,4174:lz,4175:rf,4176:gg,4177:sl,4178:ui,4179:mn,4180:ax,4181:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4151,"endLine":4185}
```
</details>

**Todo list:**
- ✅ Understand re-export method codegen
- ✅ Add mut_ref_enum_rust_type fields
- ✅ Update build_param_binding
- ✅ Update inherited param binding
- ✅ Update convert_arg functions
- 🔄 Update prelude + body generation
- ⬜ Fix inherited C++ codegen
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now let me update both  to also generate the `let mut <name>_i32_` bindings. And I need a helper to generate the postamble. Then I need to update all the method body generation sites to use the multi-statement pattern when `mut_ref_enum` params exist.

Let me first update the prelude functions and add a postamble function:

<details>
<summary>Reading lines 4186-4210 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4186-L4211">bindings.rs</a></summary>

```

/// Generate CString let-bindings for all &str parameters.
/// These must appear before the unsafe block so the CStrings live long enough.
fn cstr_prelude_params(params: &[ParamBinding]) -> String {
    params.iter()
        .filter(|p| p.rust_reexport_type == "&str")
        .map(|p| format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name))
        .collect()
}

fn cstr_prelude_resolved(params: &[ResolvedParamBinding], names: &[String]) -> String {
    params.iter().zip(names.iter())
        .filter(|(p, _)| p.rust_reexport_type == "&str")
        .map(|(_, name)| format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", name, name))
        .collect()
}


/// Build the body expression for a re-export method call.
/// Handles the conversion from FFI raw pointer returns to Rust references/OwnedPtr.
fn build_reexport_body(raw_call: &str, reexport_type: Option<&str>, is_enum: Option<&str>, needs_owned_ptr: bool) -> String {
    if let Some(enum_type) = is_enum {
        format!("unsafe {{ {}::try_from({}).unwrap() }}", enum_type, raw_call)
    } else if needs_owned_ptr {
        format!("unsafe {{ crate::OwnedPtr::from_raw({}) }}", raw_call)
```
</details>

I'll replace the prelude functions with versions that also handle `mut_ref_enum`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4187-L4214">bindings.rs -15+27</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Generate let-bindings for CString (&str) params and &mut enum params.\n/// These must appear before the unsafe block so the temporaries live long enough.\nfn cstr_prelude_params(params: &[ParamBinding]) -> String {\n    let mut result = String::new();\n    for p in params {\n        if p.rust_reexport_type == \"&str\" {\n            result.push_str(&format!(\"        let c_{} = std::ffi::CString::new({}).unwrap();\\n\", p.rust_name, p.rust_name));\n        }\n        if p.mut_ref_enum_rust_type.is_some() {\n            result.push_str(&format!(\"        let mut {}_i32_: i32 = (*{}).into();\\n\", p.rust_name, p.rust_name));\n        }\n    }\n    result\n}\n\nfn cstr_prelude_resolved(params: &[ResolvedParamBinding], names: &[String]) -> String {\n    let mut result = String::new();\n    for (p, name) in params.iter().zip(names.iter()) {\n        if p.rust_reexport_type == \"&str\" {\n            result.push_str(&format!(\"        let c_{} = std::ffi::CString::new({}).unwrap();\\n\", name, name));\n        }\n        if p.mut_ref_enum_rust_type.is_some() {\n            result.push_str(&format!(\"        let mut {}_i32_: i32 = (*{}).into();\\n\", name, name));\n        }\n    }\n    result\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4187:af,4188:yr,4189:cy,4190:st,4191:qh,4192:py,4193:yi,4194:io,4195:dd,4196:rv,4197:vn,4198:sb,4199:mg,4200:yi,4201:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4187,"endLine":4213}
```
</details>

Now I need to add a postamble helper and update the body builder to handle `&mut enum` with a multi-statement body. Let me add a postamble generator and update  to support it. Actually, the cleanest approach is to add a new function that checks if `&mut enum` params need the multi-statement pattern, and produce the whole body:

<details>
<summary>Reading lines 4214-4240 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4214-L4241">bindings.rs</a></summary>

```


/// Build the body expression for a re-export method call.
/// Handles the conversion from FFI raw pointer returns to Rust references/OwnedPtr.
fn build_reexport_body(raw_call: &str, reexport_type: Option<&str>, is_enum: Option<&str>, needs_owned_ptr: bool) -> String {
    if let Some(enum_type) = is_enum {
        format!("unsafe {{ {}::try_from({}).unwrap() }}", enum_type, raw_call)
    } else if needs_owned_ptr {
        format!("unsafe {{ crate::OwnedPtr::from_raw({}) }}", raw_call)
    } else if let Some(rt) = reexport_type {
        if rt == "String" {
            format!("unsafe {{ std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned() }}", raw_call)
        } else if rt.starts_with("&mut ") {
            format!("unsafe {{ &mut *({}) }}", raw_call)
        } else if rt.starts_with('&') {
            format!("unsafe {{ &*({}) }}", raw_call)
        } else {
            format!("unsafe {{ {} }}", raw_call)
        }
    } else {
        format!("unsafe {{ {} }}", raw_call)
    }
}

pub fn emit_reexport_class(bindings: &ClassBindings, module_name: &str) -> String {
    let cn = &bindings.cpp_name;
    let short_name = &bindings.short_name;
```
</details>

**Todo list:**
- 🔄 Add mut_ref_enum postamble helper
- ⬜ Update wrapper method body gen
- ⬜ Update direct method body gen
- ⬜ Update static method body gen
- ⬜ Update inherited method body gen
- ⬜ Fix inherited C++ codegen preamble
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Let me first read the relevant method body generation sites to understand the patterns:

<details>
<summary>Reading lines 4300-4420 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4300-L4421">bindings.rs</a></summary>

```
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        let doc = format_reexport_doc(&source_attr, &ctor.doc_comment);

        if let Some(ref conv) = ctor.convenience_of {
            // Convenience constructor: Rust-only wrapper that delegates to full-arg version
            // Use raw param names (no CString conversion) since the target method handles it
            let convenience_args: Vec<String> = ctor.params.iter().map(|p| p.rust_name.clone()).collect();
            let mut all_args = convenience_args;
            all_args.extend(conv.default_exprs.iter().cloned());
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                prelude,
                ctor.ffi_fn_name,
                args.join(", ")
            ));
        }
    }

    // 2. Wrapper methods (impl delegates to ffi free functions)
    for wm in &bindings.wrapper_methods {
        let self_param = if wm.is_const {
            "&self".to_string()
        } else {
            "&mut self".to_string()
        };

        let self_arg = if wm.is_const {
            "self as *const Self".to_string()
        } else {
            "self as *mut Self".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                wm.params
                    .iter()
                    .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type)),
            )
            .collect();
        let args: Vec<String> = std::iter::once(self_arg)
            .chain(wm.params.iter().map(|p| convert_arg(p)))
            .collect();

        let return_type = wm
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        let raw_call = format!("crate::ffi::{}({})", wm.ffi_fn_name, args.join(", "));
        let is_enum_return = wm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = wm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = wm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&wm.params);

        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr);

        let source_attr = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        let doc = format_reexport_doc(&source_attr, &wm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            wm.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // 2b. Direct methods (also delegates to ffi free functions, same pattern as wrappers)
    for dm in &bindings.direct_methods {
        let self_param = if dm.is_const {
            "&self".to_string()
        } else {
            "&mut self".to_string()
        };

        let self_arg = if dm.is_const {
            "self as *const Self".to_string()
        } else {
            "self as *mut Self".to_string()
        };

        let params: Vec<String> = st
... (truncated, 4539 chars)
```
</details>

<details>
<summary>Reading lines 4420-4570 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4420-L4571">bindings.rs</a></summary>

```
            .unwrap_or_default();

        let ffi_fn_name = format!("{}_{}", cn, dm.rust_name);
        let raw_call = format!("crate::ffi::{}({})", ffi_fn_name, args.join(", "));
        let is_enum_return = dm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = dm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = dm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&dm.params);

        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr);

        let source_attr = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        let doc = format_reexport_doc(&source_attr, &dm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            dm.rust_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // 3. Static methods
    for sm in &bindings.static_methods {
        let params: Vec<String> = sm
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = sm.params.iter().map(|p| convert_arg(p)).collect();

        let return_type = sm
            .return_type
            .as_ref()
            .map(|rt| {
                let mut ty_str = rt.rust_reexport_type.clone();
                if sm.needs_static_lifetime
                    && ty_str.starts_with('&')
                    && !ty_str.contains("'static")
                {
                    ty_str = ty_str.replacen('&', "&'static ", 1);
                }
                format!(" -> {}", ty_str)
            })
            .unwrap_or_default();

        let source_attr = format_source_attribution(
            &bindings.source_header,
            sm.source_line,
            &format!("{}::{}()", cn, sm.cpp_method_name),
        );
        let doc = format_reexport_doc(&source_attr, &sm.doc_comment);
        let raw_call = format!("crate::ffi::{}({})", sm.ffi_fn_name, args.join(", "));
        let is_enum_return = sm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = sm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = sm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&sm.params);

        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr);

        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            sm.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // 4. Upcast methods
    for up in &bindings.upcasts {
        let ret_type = if up.base_module == module_name {
            up.base_short_name.clone()
        } else {
            let rust_mod = module_graph::module_to_rust_name(&up.base_module);
            format!("crate::{}::{}", rust_mod, up.base_short_name)
        };

        impl_methods.push(format!(
            "    /// Upcast to {}\n    pub fn {}(&self) -> &{} {{\n        unsafe {{ &*(crate::ffi::{}(self as *const Self)) }}\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name
        ));

        impl_methods.push(format!(
            "    /// Upcast to {} (mutable)\n    pub fn {}_mut(&mut self) -> &mut {} {{\n        unsafe {{ &mut *(crate::ffi::{}(self as *mut Self)) }}\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name_mut
        ));
    }

    // 5. to_owned
    if bindings.has_to_owned {
        let ffi_fn_name =
... (truncated, 6052 chars)
```
</details>

<details>
<summary>Reading lines 4570-4630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4570-L4631">bindings.rs</a></summary>

```
            .unwrap_or_default();

        let raw_call = format!("crate::ffi::{}({})", im.ffi_fn_name, args.join(", "));
        let is_enum_return = im.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = im.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = im.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_resolved(&im.params, &param_names);

        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr);

        let no_doc: Option<String> = None;
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            format_reexport_doc(
                &format!("Inherited: {}", format_source_attribution(
                    &im.source_header,
                    im.source_line,
                    &format!("{}::{}()", im.source_class, im.cpp_method_name),
                )),
                &no_doc,
            ),
            im.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // Generate the impl block
    if !impl_methods.is_empty() {
        output.push_str(&format!("impl {} {{\n", short_name));
        for method in impl_methods {
            output.push_str(&method);
        }
        output.push_str("}\n\n");
    }

    // Handle type re-export, CppDeletable, get method, and handle upcast methods
    if bindings.has_handle_get {
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
        // Re-export the handle type so external crates can name it
        output.push_str(&format!(
            "pub use crate::ffi::{};\n\n",
            handle_type_name
        ));

        // CppDeletable for handle type
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            handle_type_name, handle_type_name
        ));

        // Generate impl block with get(), get_mut(), and upcast methods
        output.push_str(&format!("impl {} {{\n", handle_type_name));
        // get() - dereference handle to &T
        output.push_str(&format!(
            "    /// Dereference this Handle to access the underlying {}\n    pub fn get(&self) -> &crate::ffi::{} {{\n        unsafe {{ &*(crate::ffi::{}_get(self as *const Self)) }}\n    }}\n",
            cn, cn, handle_type_name
        ));
```
</details>

> 💭 Now I understand the pattern. All the method body generation sites use the same format:
> 
> ```rust
> impl_methods.push(format!(
>     "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
>     doc,
>     method_name,
>     params.join(", "),
>     return_type,
>     prelude,
>     body,
> ));
> ```
> 
> Where `body` is produced by `build_reexport_body()` which wraps everything in a single `unsafe { ... }` expression.
> 
> For `&mut enum` params, we need a multi-statement body:
> 
> 
> Modifying `build_reexport_body` to accept an optional postamble would let us handle the enum conversion after the FFI call, keeping the unsafe block focused on just the foreign call while handling the type conversion and assignment separately. I'm realizing the issue: when there's both an enum return and `&mut enum` parameters, I need to handle the unsafe call separately from the conversions. The pattern would be to assign the unsafe FFI call result to a variable, apply the parameter postamble (the writeback for `&mut enum`), then convert and return the result—but I'm running into the problem that the enum return also needs its own conversion wrapped in `try_from().unwrap()`.
> 
> The cleanest solution is to refactor this with a helper function that generates the postamble lines for mutable enum parameters, then modify the body builder to handle multi-statement cases where I can split the unsafe call from the conversions. I'm realizing the cleanest approach is to check at each call site whether there are mutable reference enum parameters, and if so, wrap the generated body in a multi-statement block that handles the postamble logic based on whether the function returns a value or not. I see the issue now—when there's no return value, the body needs to be a statement rather than an expression, so I'm wrapping it differently. For void returns with mutable enum parameters, I'm executing the unsafe call as a statement and then applying the postamble conversion. When there is a return value, I can bind the result and still apply the postamble. Now I'm working through the case where the function itself returns an enum type alongside the mutable enum parameter conversion. I'm creating helper functions to generate the postamble code that converts mutable enum references back from their i32 representations, then applying these at each call site to wrap the unsafe FFI call with the necessary cleanup statements. Wrapping the unsafe conversions in a `let result_` binding with the postamble works correctly across different return types—whether it's enum conversions, owned pointers, or string handling, the pattern keeps the unsafe operations contained and lets me append the cleanup code before returning the result.
> 
> Now I need to implement this by adding helper functions for the mutable reference enum postamble, then applying that wrapping logic at all four call sites plus the free function handling in rust.rs. Let me check the current state of `convert_arg` and `cstr_prelude_params` to see what's already been set up.

Now let me also check the free function codegen in rust.rs:

<details>
<summary>Reading lines 590-680 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L590-L681">rust.rs</a></summary>

```
    }
    writeln!(output, "            _ => Err(value),").unwrap();
    writeln!(output, "        }}").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();
}

/// Emit a wrapper function for a free function in the public module.
/// All free functions are real functions (not pub use re-exports) so that
/// IDE "go to definition" lands in the public module, not ffi::.
/// Includes source attribution and doc comments.
fn emit_free_function_wrapper(
    output: &mut String,
    func: &super::bindings::FunctionBinding,
) {
    use std::fmt::Write;

    // Source attribution + doc comment
    let source_attr = format_source_attribution(
        &func.source_header,
        func.source_line,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    writeln!(output, "/// {}", source_attr).unwrap();
    if let Some(ref comment) = func.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }

    // Build parameter list using pre-computed re-export types
    let params: Vec<String> = func.params.iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
        .collect();

    // Build args with .into() for enum params, CString conversion for &str params
    let args: Vec<String> = func.params.iter()
        .map(|p| {
            if p.rust_reexport_type == "&str" {
                format!("c_{}.as_ptr()", p.rust_name)
            } else if p.enum_rust_type.is_some() {
                format!("{}.into()", p.rust_name)
            } else {
                p.rust_name.clone()
            }
        })
        .collect();

    // Generate CString prelude for &str params
    let prelude: String = func.params.iter()
        .filter(|p| p.rust_reexport_type == "&str")
        .map(|p| format!("    let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name))
        .collect();

    // Build return type string
    let return_type_str = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", rt.rust_reexport_type))
        .unwrap_or_default();

    // Build call expression
    let call_expr = format!("crate::ffi::{}({})", func.cpp_wrapper_name, args.join(", "));

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
            call_expr
        }
    } else {
        call_expr
    };
```
</details>

<details>
<summary>Reading lines 680-710 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L680-L711">rust.rs</a></summary>

```
    };

    writeln!(output, "pub fn {}({}){} {{", func.rust_ffi_name, params.join(", "), return_type_str).unwrap();
    write!(output, "{}", prelude).unwrap();
    writeln!(output, "    unsafe {{ {} }}", body).unwrap();
    writeln!(output, "}}").unwrap();
}

/// Generate a module re-export file
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    module_fn_bindings: &[&super::bindings::FunctionBinding],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

```
</details>

> 💭 Now I have a good picture. Let me see what `convert_arg()` and the prelude currently look like (the modified versions):

Let me check the current state of  and the prelude functions:

<details>
<summary>Reading lines 4140-4215 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4140-L4216">bindings.rs</a></summary>

```

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", p.rust_name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", p.rust_name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", p.rust_name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", p.rust_name)
    } else {
        p.rust_name.clone()
    }
}

fn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", name)
    } else {
        name.to_string()
    }
}

/// Generate let-bindings for CString (&str) params and &mut enum params.
/// These must appear before the unsafe block so the temporaries live long enough.
fn cstr_prelude_params(params: &[ParamBinding]) -> String {
    let mut result = String::new();
    for p in params {
        if p.rust_reexport_type == "&str" {
            result.push_str(&format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name));
        }
        if p.mut_ref_enum_rust_type.is_some() {
            result.push_str(&format!("        let mut {}_i32_: i32 = (*{}).into();\n", p.rust_name, p.rust_name));
        }
    }
    result
}

fn cstr_prelude_resolved(params: &[ResolvedParamBinding], names: &[String]) -> String {
    let mut result = String::new();
    for (p, name) in params.iter().zip(names.iter()) {
        if p.rust_reexport_type == "&str" {
            result.push_str(&format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", name, name));
        }
        if p.mut_ref_enum_rust_type.is_some() {
            result.push_str(&format!("        let mut {}_i32_: i32 = (*{}).into();\n", name, name));
        }
    }
    result
}


```
</details>

Good, the prelude and convert_arg are already updated. Now I need to:
1. Add postamble helper functions
2. Modify the method body generation at each call site

Let me add the postamble helpers right after the prelude functions, then update  to accept a postamble:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4214-L4256">bindings.rs +42</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4213:io|",
      "insertAfter": true,
      "content": "\n/// Generate the postamble for &mut enum params: write back from i32 to typed enum.\nfn mut_ref_enum_postamble_params(params: &[ParamBinding], indent: &str) -> String {\n    let mut result = String::new();\n    for p in params {\n        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {\n            result.push_str(&format!(\"{}*{} = {}::try_from({}_i32_).unwrap();\\n\", indent, p.rust_name, enum_type, p.rust_name));\n        }\n    }\n    result\n}\n\nfn mut_ref_enum_postamble_resolved(params: &[ResolvedParamBinding], names: &[String], indent: &str) -> String {\n    let mut result = String::new();\n    for (p, name) in params.iter().zip(names.iter()) {\n        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {\n            result.push_str(&format!(\"{}*{} = {}::try_from({}_i32_).unwrap();\\n\", indent, name, enum_type, name));\n        }\n    }\n    result\n}\n\n/// Wrap a reexport body expression with &mut enum writeback postamble.\n/// When postamble is non-empty, splits into multi-statement body:\n///   let result_ = <body>;\n///   <postamble>\n///   result_\n/// For void returns (has_return=false):\n///   <body>;\n///   <postamble trimmed>\nfn wrap_body_with_postamble(body: &str, postamble: &str, has_return: bool) -> String {\n    if postamble.is_empty() {\n        return body.to_string();\n    }\n    if has_return {\n        format!(\"let result_ = {};\\n{}        result_\", body, postamble)\n    } else {\n        // Void return: body as statement, then postamble (trim trailing newline for last line)\n        let trimmed_postamble = postamble.trim_end_matches('\\n');\n        format!(\"{};\\n{}\", body, trimmed_postamble)\n    }\n}"
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
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4213:io|",
      "error": "hash mismatch at line 4213: expected 'io|', got 'io'"
    }
  ]
}
```
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4210-4210), lines 4210 to 4220

Replacing 5 lines with 46 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

```

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- 🔄 Update wrapper method body gen
- ⬜ Update direct method body gen
- ⬜ Update static method body gen
- ⬜ Update inherited method body gen
- ⬜ Fix inherited C++ codegen preamble
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now let me update the wrapper method body generation site. I need to read the current state to get exact line numbers:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4410-4410), lines 4410 to 4440

Now I need to add the postamble wrapping at each of the 4 sites. Let me update all four at once:

Using "Replace String in File"

Now the direct methods site:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4466-4466), lines 4466 to 4490

Replacing 19 lines with 22 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

Now the static methods site:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4525-4525), lines 4525 to 4545

Replacing 5 lines with 8 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

Now the inherited methods site:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4618-4618), lines 4618 to 4640

Replacing 5 lines with 8 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- ✅ Update wrapper method body gen
- ✅ Update direct method body gen
- ✅ Update static method body gen
- ✅ Update inherited method body gen
- 🔄 Fix inherited C++ codegen preamble
- ⬜ Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now let me check the inherited C++ codegen (section 10) to see if it handles `&mut enum` preamble/postamble:

Searched for text `Section 10` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for text `inherited wrapper` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4588](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4588)

Searched for regex `inherited.*C\+\+|inherited.*cpp|emit_inherited|generate.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:1834](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1834)
- [bindings.rs:2454](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2454)

Let me search for where inherited method C++ wrappers are generated:

Searched for regex `inherited.*ffi|inherited.*wrapper_fn|inherited_methods.*cpp|generate_inherited_cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:376](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L376)
- [bindings.rs:1821](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1821)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#370-370), lines 370 to 400

> 💭 I need to find where the C++ code for inherited method wrappers is generated. Let me search for the C++ output generation pattern for inherited methods.

Let me find where the C++ wrapper code for inherited methods is emitted:

Searched for regex `emit_inherited|emit_cpp.*inherited|generate_cpp.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for regex <code>InheritedMethod|inherited_method.*cpp</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:56](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L56)
- [bindings.rs:245](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L245)
- [bindings.rs:1830](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1830)
- [bindings.rs:2445](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2445)
- [bindings.rs:2718](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2718)
</details>

Searched for regex `cpp_inherited|inherited.*cpp_output|inherited.*writeln|inherited.*push_str.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me search for where the C++ wrapper functions are generated - they should use `cpp_output` or similar:

Searched for regex `cpp_wrapper|cpp_code|emit.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:349](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L349)
- [bindings.rs:3079](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3079)
- [bindings.rs:3109](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3109)
- [bindings.rs:3132](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3132)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3132-3132), lines 3132 to 3200

Let me search for the MutRefEnumParam wrapper kind section:

<details>
<summary>Searched for text <code>MutRefEnumParam</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:148](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L148)
- [bindings.rs:821](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L821)
- [bindings.rs:1010](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1010)
- [bindings.rs:3587](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3587)
- [bindings.rs:3596](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3596)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3587-3587), lines 3587 to 3700

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3700-3700), lines 3700 to 3720

Good — the section 4e C++ wrapper for  is already properly implemented with preamble/postamble. Now let me check the inherited methods C++ section to see if it has the same handling:

Searched for regex `Inherited method C\+\+|inherited_methods.*cpp|section.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `Inherited.*wrapper|10\.|emit.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:3238](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3238)
- [bindings.rs:4050](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4050)
- [bindings.rs:4588](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4588)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4140

> 💭 I can see the inherited method C++ codegen (section 10) doesn't handle `&mut enum` params at all. It just passes `cpp_arg_expr` directly without creating local enum variables and writing back. This is the same bug pattern as the original issue.
> 
> However, for inherited methods, the `ResolvedParamBinding` doesn't have a `mut_ref_enum_cpp_name` field. Let me check what fields `ResolvedParam` has.
> 
> Wait, the inherited methods use `ResolvedParam` from the resolver, not `ParamBinding`. Let me check the `ResolvedParam` struct and how inherited methods are constructed.

The inherited C++ codegen (section 10) doesn't handle `&mut enum` params. It needs preamble/postamble like section 4e. Let me check the `ResolvedParam` struct:

Searched for regex `struct InheritedMethodBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:245](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L245)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#245-245), lines 245 to 285

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#285-285), lines 285 to 330

Now I see that `ResolvedParamBinding` has `mut_ref_enum_rust_type` but NOT `mut_ref_enum_cpp_name`. The C++ codegen for inherited methods (section 10) uses `ResolvedParam` from the resolver (not `ResolvedParamBinding`). Let me check what `ResolvedParam` looks like and how the C++ side uses inherited params:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4075

> 💭 The section 10 C++ codegen for inherited methods uses `im.params` which are of type `ResolvedParamBinding`. The `cpp_arg_expr` for `&mut enum` params was set to `format!("{}_enum_", p.name)` in the inherited `ResolvedParamBinding` construction that I updated earlier. So the call uses `val_enum_` as the arg expression, but the C++ codegen doesn't create the local variable or write it back.
> 
> I need to add preamble/postamble similar to section 4e. But the `ResolvedParamBinding` doesn't have a `mut_ref_enum_cpp_name` field. I need to:
> 
> 
> 1. Add `mut_ref_enum_cpp_name: Option<String>` to `ResolvedParamBinding`
> 2. Populate it in the inherited method binding constructions
> 3. Add preamble/postamble to section 10
> 
> Let me trace back through the inherited ResolvedParamBinding construction to verify what was actually set for `cpp_arg_expr` when handling `&mut enum` parameters.

Let me check the ResolvedParamBinding construction for inherited methods to see how `&mut enum` params are set up:

<details>
<summary>Searched for text <code>_enum_</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:284](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L284)
- [bindings.rs:286](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L286)
- [bindings.rs:322](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L322)
- [bindings.rs:784](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L784)
- [bindings.rs:820](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L820)
- [bindings.rs:1061](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1061)
- [bindings.rs:1171](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1171)
- [bindings.rs:1175](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1175)
- [bindings.rs:1202](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1202)
- [bindings.rs:1205](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1205)
- [bindings.rs:1206](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1206)
- [bindings.rs:1214](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1214)
- [bindings.rs:1228](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1228)
- [bindings.rs:1229](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1229)
- [bindings.rs:1272](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1272)
- [bindings.rs:1273](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1273)
- [bindings.rs:1306](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1306)
- [bindings.rs:1307](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1307)
- [bindings.rs:1314](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1314)
- [bindings.rs:1590](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1590)
</details>

Now let me see where the inherited ResolvedParamBinding construction handles `&mut enum`:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2660-2660), lines 2660 to 2750

> 💭 I see that the inherited `ResolvedParamBinding` construction at line ~2682 currently has `mut_ref_enum_rust_type: None` for all params. It needs to detect `&mut enum` params and set the appropriate fields.
> 
> Looking at the `cpp_arg_expr` — for by-value enum params, it's `format!("static_cast<{}>({})", enum_name, p.name)`. But for `&mut enum` params (MutRef(Class(enum_name)) where enum_name is an enum), I need to:
> 1. Set `cpp_arg_expr` to `format!("{}_enum_", p.name)` (local variable name)
> 
> 
> 2. Set `cpp_type` to `"int32_t&"` so the C++ side can write back to the reference
> 3. Set `rust_ffi_type` to `"&mut i32"` and `rust_reexport_type` to the typed Rust enum like `"&mut crate::geom_abs::Shape"`
> 4. Set `mut_ref_enum_rust_type` to `Some(enum_rust_type)` and add a `mut_ref_enum_cpp_name` field to `ResolvedParamBinding` so the C++ codegen can generate the proper conversion code
> 
> Now I'm realizing the issue—when the resolver processes `&mut enum`, it wraps it as `Type::MutRef(Box::new(Type::Class(enum_name)))`, but the `enum_cpp_name` field only gets populated for by-value enums since the resolver deliberately skips extracting enum names from mutable references. I need to check if `p.ty.original` matches the pattern of `MutRef(Class(name))` where that name exists in `all_enums`, then look at how the resolver structures these resolved parameters.

I need to add `mut_ref_enum_cpp_name` to `ResolvedParamBinding` and handle `&mut enum` detection in the inherited method construction. Let me check the resolver's type structure first:

Searched for regex `pub enum_cpp_name|enum_cpp_name.*is.*set|extract_enum_name_from_type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 4 results
- [resolver.rs:325](../crates/opencascade-binding-generator/src/resolver.rs#L325)
- [resolver.rs:1322](../crates/opencascade-binding-generator/src/resolver.rs#L1322)
- [resolver.rs:1365](../crates/opencascade-binding-generator/src/resolver.rs#L1365)
- [resolver.rs:1370](../crates/opencascade-binding-generator/src/resolver.rs#L1370)

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#1365-1365), lines 1365 to 1395

OK, so the resolver's  deliberately doesn't unwrap . That means  won't be set for `&mut enum` params. I need to detect this case in the inherited method construction.

Now let me add the `mut_ref_enum_cpp_name` field to `ResolvedParamBinding` and update the inherited method construction:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#310-310), lines 310 to 328

Replacing 16 lines with 18 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

to all the existing `ResolvedParamBinding` construction sites. Let me find them:

Searched for text `ResolvedParamBinding {` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:310](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L310)
- [bindings.rs:2625](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2625)
- [bindings.rs:2646](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2646)
- [bindings.rs:2683](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2683)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2618-2618), lines 2618 to 2700

I need to add `mut_ref_enum_cpp_name` to all three `ResolvedParamBinding` construction sites:

Replacing 37 lines with 40 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

Now I need to update section 10 (inherited C++ wrappers) to add the preamble/postamble for `&mut enum` params, just like section 4e does:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4145

Now I need to add preamble/postamble to section 10. The `ResolvedParamBinding` now has `mut_ref_enum_cpp_name`. I need to:
1. Before the call: create local enum vars from int32_t
2. After the call: write back to int32_t

Also need to handle the return type for `&mut enum` — the section currently uses a simple `return` pattern but needs to be multi-statement when `&mut enum` params are present.

Using "Replace String in File"

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- ✅ Update wrapper method body gen
- ✅ Update direct method body gen
- ✅ Update static method body gen
- ✅ Update inherited method body gen
- ✅ Fix inherited C++ codegen preamble
- 🔄 Update free function codegen
- ⬜ Update PORTING.md
- ⬜ Regenerate, build, and test

Now let me update the free function codegen in `crates/opencascade-binding-generator/src/codegen/rust.rs`. It needs to handle `&mut enum` params in its args building, prelude, and body:

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#626-626), lines 626 to 690

I need to update the free function codegen to handle `&mut enum` params — add them to args, prelude, and postamble:

Replacing 61 lines with 96 lines in [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs)

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- ✅ Update wrapper method body gen
- ✅ Update direct method body gen
- ✅ Update static method body gen
- ✅ Update inherited method body gen
- ✅ Fix inherited C++ codegen preamble
- ✅ Update free function codegen
- 🔄 Update PORTING.md
- ⬜ Regenerate, build, and test

Now let me update PORTING.md to remove the limitation about `&mut i32`:

Searched for regex `mut.*i32|mut.*enum.*out` (`**/crates/opencascade-sys/PORTING.md`), 2 results
- [PORTING.md:422](../crates/opencascade-sys/PORTING.md#L422)
- [PORTING.md:424](../crates/opencascade-sys/PORTING.md#L424)

Read [PORTING.md](../crates/opencascade-sys/PORTING.md#418-418), lines 418 to 430

Replacing 7 lines with 8 lines in [PORTING.md](../crates/opencascade-sys/PORTING.md)

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- ✅ Update wrapper method body gen
- ✅ Update direct method body gen
- ✅ Update static method body gen
- ✅ Update inherited method body gen
- ✅ Fix inherited C++ codegen preamble
- ✅ Update free function codegen
- ✅ Update PORTING.md
- 🔄 Regenerate, build, and test

Now let me first check if the binding generator compiles before regenerating:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (19 lines) — exit code 101</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error[E0425]: cannot find value `all_enum_names` in this scope
    --> crates/opencascade-binding-generator/src/codegen/bindings.rs:2642:36
     |
2443 |     _all_enum_names: &HashSet<String>,
     |     --------------- `_all_enum_names` defined here
...
2642 |                                 if all_enum_names.contains(enum_name) {
     |                                    ^^^^^^^^^^^^^^
     |
help: the leading underscore in `_all_enum_names` marks it as unused, consider renaming it to `all_enum_names`
     |
2443 -     _all_enum_names: &HashSet<String>,
2443 +     all_enum_names: &HashSet<String>,
     |

For more information about this error, try `rustc --explain E0425`.
error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
real  3.8s
```
</details>

Easy fix: the parameter was named  because it was unused. Now we use it. Let me rename it:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2438-2438), lines 2438 to 2448

Replacing 1 lines with 1 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

Replacing 7 lines with 7 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (11 lines) — exit code 101</summary>

```
    Blocking waiting for file lock on build directory
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error[E0425]: cannot find value `_all_enum_names` in this scope
    --> crates/opencascade-binding-generator/src/codegen/bindings.rs:2549:71
     |
2549 | ...s(name.as_str()) && !_all_enum_names.contains(name.as_str()) {
     |                         ^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `all_enum_names`

For more information about this error, try `rustc --explain E0425`.
error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
real  1.9s
```
</details>

There's another reference to  that needs updating:

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2546-2546), lines 2546 to 2552

Replacing 1 lines with 1 lines in [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on build directory
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.83s
real  3.9s
```

Binding generator compiles. Now let me regenerate bindings:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | tail -5
```
<details>
<summary>Output (6 lines)</summary>

```

Code generation complete!
  213 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated
real  11.3s
```
</details>

Regeneration succeeded. Now let me verify the generated code looks right for the  case:

Searched for text `get_continuity` (`**/crates/opencascade-sys/generated/shape_process.rs`), 2 results
- [shape_process.rs:531](../crates/opencascade-sys/generated/shape_process.rs#L531)
- [shape_process.rs:535](../crates/opencascade-sys/generated/shape_process.rs#L535)

Now  is `&mut crate::geom_abs::Shape` instead of `&mut i32`. Let me see the full method body:

Read [shape_process.rs](../crates/opencascade-sys/generated/shape_process.rs#529-529), lines 529 to 550

The generated code looks exactly right:
1. CString prelude for the  string
2. `let mut val_i32_: i32 = (*val).into();` to convert enum to i32
3. FFI call with 
4.  to write back
5.  return

Now let me build the whole project:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Clean build! Now run the binding generator tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-9f2064aa9f77643d)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
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

real  3.5s
```
</details>

All 16 tests pass. Now run workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```
<details>
<summary>Output (345 lines)</summary>

```
   Compiling examples v0.2.0 (/Users/sethml/src/opencascade-rs/examples)
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
   Compiling viewer v0.2.0 (/Users/sethml/src/opencascade-rs/crates/viewer)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.79s
     Running unittests src/lib.rs (target/debug/deps/examples-638415a022e1f97e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/write_model.rs (target/debug/deps/write_model-4fb5f107f01498a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/golden.rs (target/debug/deps/golden-a67f7fbf81fc69ec)

running 23 tests
test high_level_bottle ... ignored, non-deterministic STEP output

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpotJJ3l/airfoil.step(222 ents)  Write  Done
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpGBCRPY/bounding-box.step(625 ents)  Write  Done
test airfoil ... ok
test bounding_box ... ok

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******
** WorkSession : Sending all data
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/
... (truncated, 16946 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "^test result:|FAILED|error"
```

All tests pass. No failures.

**Todo list:**
- ✅ Add mut_ref_enum postamble helper
- ✅ Update wrapper method body gen
- ✅ Update direct method body gen
- ✅ Update static method body gen
- ✅ Update inherited method body gen
- ✅ Fix inherited C++ codegen preamble
- ✅ Update free function codegen
- ✅ Update PORTING.md
- ✅ Regenerate, build, and test

All done. Here's a summary of the changes:

**Problem:** `&mut enum` output parameters (e.g., ) were exposed as `&mut i32` in the Rust re-export API instead of using typed Rust enums like .

**Changes across 3 files:**

1. **`crates/opencascade-binding-generator/src/codegen/bindings.rs`:**
   - Added  to `ParamBinding` and `mut_ref_enum_cpp_name`/`mut_ref_enum_rust_type` to `ResolvedParamBinding`
   - Updated  and inherited `ResolvedParamBinding` construction to look up typed enum and set  to `"&mut <EnumType>"`
   - Updated /`convert_arg_resolved()` to pass `&mut <name>_i32_` for `&mut enum` params
   - Updated /`cstr_prelude_resolved()` to generate `let mut <name>_i32_: i32 = (*<name>).into();`
   - Added /`mut_ref_enum_postamble_resolved()` and `wrap_body_with_postamble()` helpers
   - Applied postamble wrapping at all 4 method body generation sites (wrapper, direct, static, inherited)
   - Updated section 10 (inherited C++ wrappers) with preamble/postamble for `&mut enum` params

2. **`crates/opencascade-binding-generator/src/codegen/rust.rs`:** Updated  with `&mut enum` handling in args, prelude, and postamble

3. **`crates/opencascade-sys/PORTING.md`:** Updated the limitation note to reflect that `&mut enum` out-params now use typed enum references

*2026-02-18 15:04 · 986s · Words: 6,233 in · 7,334 out · 74,144 ctx · 130 rounds*

---

<a id="user-5"></a>

## User (5)

> commit

*2026-02-18 15:08*

### Assistant — Claude Haiku 4.5

> 💭 The user wants me to commit the changes. Let me check what changes have been made to the workspace. According to the conversation summary, we've made changes to:
> 
> 1. crates/opencascade-binding-generator/src/codegen/bindings.rs - Major changes to add mut_ref_enum support
> 2. crates/opencascade-binding-generator/src/codegen/rust.rs - Updated free function codegen
> 3. crates/opencascade-sys/PORTING.md - Updated limitation note
> 4. crates/crates/opencascade-sys/bindings.toml - Removed ShapeProcess_ShapeContext.hxx exclusion (in Phase 1)
> 
> Actually, let me check what was already done. Looking back at the conversation, in Phase 1, the user said the exclusion was removed from bindings.toml. Let me verify what files were actually modified by checking git status first, then create a proper commit.
> 
> The instructions say:
> "Before comitting, always make sure that everything builds and tests pass with:
> ```
> scripts/regenerate-bindings.sh
> cargo build
> scripts/run-binding-generator-tests.sh
> cargo test --workspace --exclude opencascade-binding-generator
> ```"
> 
> We've already done all of that:
> 1. ✅ scripts/regenerate-bindings.sh - succeeded
> 2. ✅ cargo build - succeeded (21.3s)
> 3. ✅ scripts/run-binding-generator-tests.sh - all 16 tests passed
> 4. ✅ cargo test --workspace --exclude opencascade-binding-generator - all tests passed
> 
> Now I should commit. The instructions also say:
> "Never git commit unless explicitly asked to do so."
> 
> But the user said "commit", so they're explicitly asking. The instructions also mention:
> "When doing a git commit, write the commit message to a temporary file rather than using shell quoting."
> 
> Let me create a commit message file and commit with it.

Using "Create File"

> 💭 Now let me commit with this message file.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit_msg.txt
```
```
[main 170da28] Expose typed Rust enums for &mut enum output parameters
 21 files changed, 1232 insertions(+), 219 deletions(-)
```

> 💭 Perfect! The commit was created successfully. Let me verify the commit was created properly by checking the log.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -1
```
```
170da28 (HEAD -> main) Expose typed Rust enums for &mut enum output parameters
```

> 💭 Perfect! The commit is done. Let me provide a summary to the user.

Commit created successfully! 

**170da28** — `Expose typed Rust enums for &mut enum output parameters`

The commit includes all the changes across the binding generator to expose typed Rust enums for `&mut enum` output parameters, making them consistent with other enum parameter types. All tests pass and bindings regenerated successfully.

*2026-02-18 15:09 · 22s · Words: 1 in · 611 out · 87,882 ctx · 4 rounds*

---
