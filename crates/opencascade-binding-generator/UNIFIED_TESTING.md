# Testing the Unified FFI Architecture

This document describes how to test and enable the unified FFI architecture for step 4i.

## Status

- ✅ Code generation functions implemented (`generate_unified_ffi`, `generate_unified_wrappers`, `generate_module_reexports`)
- ✅ `build.rs` already supports unified architecture (auto-detects `ffi.rs` and switches mode)
- 🔲 Needs testing with actual generated code
- 🔲 Needs to be made default once verified

## Prerequisites

### Fix libclang Path (Linux)

The binding generator requires libclang. On Linux, you may need to create a symlink:

```bash
# Find your libclang installation
find /usr -name "libclang.so*" 2>/dev/null

# Create symlink (example for llvm-14)
sudo ln -sf /usr/lib/llvm-14/lib/libclang.so.1 /usr/lib/llvm-14/lib/libclang.so

# Or set environment variable
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
```

## Testing Unified Generation

### Step 1: Generate Unified Bindings

```bash
# From repository root
./scripts/regenerate-bindings.sh --unified --verbose
```

This should:
1. Build the binding generator
2. Parse all headers with dependency resolution
3. Generate:
   - `crates/opencascade-sys/generated/ffi.rs` - Single unified FFI module
   - `crates/opencascade-sys/generated/wrappers.hxx` - Single C++ wrapper header
   - `crates/opencascade-sys/generated/{module}.rs` - Per-module re-exports
   - `crates/opencascade-sys/generated/lib.rs` - Module declarations

### Step 2: Verify Generated Files

Check that the unified files were created:

```bash
ls -lh crates/opencascade-sys/generated/ffi.rs
ls -lh crates/opencascade-sys/generated/wrappers.hxx
```

The `ffi.rs` file should be large (50k+ lines) containing all types with full C++ names like:
- `type gp_Pnt;`
- `type TopoDS_Shape;`
- `type BRepPrimAPI_MakeBox;`

Module re-export files should contain grouped re-exports:

```bash
# Check a module file
head -50 crates/opencascade-sys/generated/gp.rs
```

Should see sections like:
```rust
// ========================
// From gp_Pnt.hxx
// ========================

/// A 3D Cartesian point.
pub use crate::ffi::gp_Pnt as Pnt;

impl Pnt {
    // ... methods
}

// ========================
// From gp_Vec.hxx
// ========================

/// A 3D vector.
pub use crate::ffi::gp_Vec as Vec;

impl Vec {
    // ... methods
}
```

### Step 3: Build opencascade-sys

The build.rs will automatically detect `ffi.rs` and use unified mode:

```bash
cargo build -p opencascade-sys
```

Expected output should show it compiling a single large C++ translation unit instead of many small ones.

### Step 4: Build opencascade

```bash
cargo build -p opencascade
```

This should work identically to the per-module architecture since the public API is the same:
- `use opencascade_sys::gp::Pnt;` still works
- `use opencascade_sys::topo_ds::Shape;` still works

### Step 5: Run Tests

```bash
# Run opencascade-sys tests
cargo test -p opencascade-sys

# Run opencascade tests
cargo test -p opencascade

# Run examples
cargo run -p examples --example simple
```

## Verification Checklist

- [ ] `ffi.rs` generated (large file with all types)
- [ ] `wrappers.hxx` generated (single C++ header)
- [ ] Module re-export files generated (one per module)
- [ ] `lib.rs` declares `pub mod ffi;` and module re-exports
- [ ] `opencascade-sys` compiles without errors
- [ ] `opencascade` compiles without errors
- [ ] All tests pass
- [ ] Examples run successfully
- [ ] Build time compared to per-module (should be similar or faster)

## Making Unified the Default

Once testing is successful, update `regenerate-bindings.sh` to use `--unified` by default:

```bash
# In scripts/regenerate-bindings.sh, line 77:
"$REPO_ROOT/target/release/occt-bindgen" \
    --unified \
    --resolve-deps \
    -I "$OCCT_INCLUDE" \
    -o "$OUTPUT_DIR" \
    "$@" \
    "${HEADERS[@]}"
```

This way, users can still opt out with `--no-unified` if needed (once that flag is added).

## Expected Benefits

After migration to unified architecture:

1. **All inherited methods work** - No cross-module type filtering needed
2. **Simpler codegen** - No cross-module type aliases required
3. **Faster builds** - Single C++ compilation unit instead of 88 separate ones
4. **Same public API** - Users see no difference, modules are preserved via re-exports

## Comparison: Per-Module vs Unified

### Per-Module Architecture (current)

```
generated/
├── gp.rs              # CXX bridge + impl blocks
├── wrapper_gp.hxx     # C++ wrappers
├── topo_ds.rs         # CXX bridge + impl blocks
├── wrapper_topo_ds.hxx
├── ... (88 modules)
└── lib.rs
```

Limitations:
- Cross-module type references require type aliases
- Inherited methods filtered if they use types from other modules
- 88 separate C++ compilation units

### Unified Architecture (new)

```
generated/
├── ffi.rs             # Single CXX bridge with ALL types
├── wrappers.hxx       # Single C++ wrapper header
├── gp.rs              # Re-exports + impl blocks
├── topo_ds.rs         # Re-exports + impl blocks
├── ... (88 modules)
└── lib.rs
```

Benefits:
- No cross-module type reference issues
- All inherited methods included
- Single C++ compilation unit
- Module organization preserved via re-exports

## Troubleshooting

### "No such file or directory: ffi.rs"

The unified generation did not complete successfully. Check generator output for errors.

### Compilation errors about undefined types

The module re-exports may not be correctly referencing `crate::ffi::FullName`. Check the generated re-export files.

### Linking errors

The unified `wrappers.hxx` may be missing includes. Verify all required OCCT headers are included.

## Rollback

To revert to per-module architecture:

```bash
# Delete ffi.rs to trigger per-module mode in build.rs
rm crates/opencascade-sys/generated/ffi.rs

# Regenerate with per-module architecture
./scripts/regenerate-bindings.sh

# build.rs will automatically detect absence of ffi.rs and use per-module mode
cargo build -p opencascade-sys
```
