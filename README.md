# opencascade-rs

Comprehensive Rust FFI bindings to the [OpenCASCADE](https://dev.opencascade.org/) (OCCT) C++ CAD kernel, auto-generated from OCCT header files.

## Background

This is a fork of [bschwind/opencascade-rs](https://github.com/bschwind/opencascade-rs). The original project provides hand-written bindings covering a useful subset of OCCT, along with a higher-level `opencascade` crate and a WGPU-based viewer.

I ([@sethml](https://github.com/sethml)) wanted to use Rust and OpenCASCADE to vibe-code an STL-to-STEP converter, and found that `opencascade-sys` was a good start but was missing many of the bindings I needed. Rather than hand-writing wrappers one by one, I built an automated binding generator that parses OCCT headers with libclang and emits a complete Rust FFI layer.

This was an experiment in using LLM-assisted coding to bind nearly all of OCCT in a form that can be effectively used from Rust. I may have gone a bit overboard and accidentally recreated much of [bindgen](https://github.com/rust-lang/rust-bindgen) — but specialized for OCCT's idioms (Handle smart pointers, NCollection iterators, inheritance upcasts, builder patterns, etc.).

I started from bschwind's repo and kept all the existing examples working throughout — they served as an excellent way to verify that the new bindings didn't break anything. The higher-level `opencascade` crate and viewer are still fully functional.

### Status

- **8,700+ opaque types** and **164,000+ extern "C" functions** across **324 Rust modules**
- Parses **5,065 OCCT headers** with automatic dependency resolution
- Only **94 symbols** skipped (mostly platform-specific or external dependency types)
- **446 enums** generated as `#[repr(i32)]` Rust enums
- Passes unit tests and golden STEP-file tests on macOS and Linux
- Untested APIs likely have issues — this is a large surface area

### What's next

I'm honestly not sure how to proceed with releasing this to the greater Rust ecosystem. A few options:
- [@bschwind](https://github.com/bschwind) could pull these changes and repackage as a new crate version
- I could publish a separate crate (e.g., `occt-sys` or similar)
- Some other approach — suggestions welcome

## Architecture

The binding pipeline has three main pieces:

1. **`opencascade-binding-generator`** — A Rust tool (~16k lines) that parses OCCT C++ headers with libclang, builds a symbol table, and generates both Rust FFI declarations and C++ wrapper functions. See [crates/opencascade-binding-generator/README.md](crates/opencascade-binding-generator/README.md) for full details on the architecture and generated API.

2. **`scripts/regenerate-bindings.sh`** — Orchestrates the generator: builds it in release mode, cleans the output directory, and runs it against the TOML configuration.

3. **`opencascade-sys`** — The generated crate. Contains `generated/ffi.rs` (a single `extern "C"` block with all types and functions), `generated/wrappers.cpp` (C++ glue code), and 324 per-module re-export files providing ergonomic short names.

Configuration lives in `crates/opencascade-sys/bindings.toml`, which controls which OCCT modules/headers to include, symbol-level exclusions, type overrides, and template instantiations.

### Porting from the old hand-written bindings

If you're migrating code that used the previous `opencascade-sys` API, see [crates/opencascade-sys/PORTING.md](crates/opencascade-sys/PORTING.md) for a detailed guide on the API differences and migration patterns.

## Dependencies

* Rust toolchain (https://rustup.rs/)
* CMake (https://cmake.org/)
* A C++ compiler with C++11 support
* libclang (for regenerating bindings — not needed for just building)

## Building

The OCCT codebase is included as a git submodule. Clone the repo with the `--recursive` flag, or use `git submodule update --init` to fetch the submodule.

```bash
cargo build --release
```

The first build compiles OCCT from source via the `builtin` feature (enabled by default), which takes a while. Subsequent builds are fast — OCCT is only rebuilt if the submodule changes.

### Using pre-installed OpenCASCADE

If you have OCCT already installed via a package manager, you can dynamically link to it instead of building from source. Disable the `builtin` feature:

```bash
cargo build --no-default-features
```

or in your `Cargo.toml`:

```toml
[dependencies]
opencascade-sys = { version = "0.2", default-features = false }
```

If OCCT is installed in a non-standard location, set the `DEP_OCCT_ROOT` environment variable to the root directory (containing `include/` and `lib/` subdirectories).

## Regenerating Bindings

To regenerate the FFI layer after modifying the binding generator or `bindings.toml`:

```bash
# Regenerate bindings (requires libclang)
scripts/regenerate-bindings.sh

# Build everything
cargo build

# Run binding generator tests (requires libclang)
scripts/run-binding-generator-tests.sh

# Run all other tests
cargo test --workspace --exclude opencascade-binding-generator
```

On macOS, the scripts automatically set `DYLD_LIBRARY_PATH` to find libclang from the Xcode toolchain. On Linux, ensure `libclang` is available (e.g., `apt install libclang-dev`).

OCCT headers must be built first (`cargo build -p occt-sys` will do this). The generator reads them from `target/OCCT/include/`.

## Run Examples

### Higher Level

The [higher level examples](./examples/src) use ergonomic Rust APIs built on top of `opencascade-sys`:

```bash
cargo run --release --example bottle
```

### Viewer Application

There is an experimental WGPU-based viewer for visualizing models:

```bash
# Visualize an example
cargo run --release --bin viewer -- --example keycap

# View a STEP file
cargo run --release --bin viewer -- --step-file SOME_FILE.step
```

### Lower Level (opencascade-sys)

There are lower-level examples that directly call OpenCASCADE functions through the generated bindings:

- [bottle.rs](./crates/opencascade-sys/examples/bottle.rs) — the classic OCCT bottle tutorial
- [simple.rs](./crates/opencascade-sys/examples/simple.rs) — basic shape creation
- [point_info_3d.rs](./crates/opencascade-sys/examples/point_info_3d.rs) — geometry queries

### Golden Tests

STEP-file golden tests verify that the examples produce consistent output:

```bash
cargo test --workspace --exclude opencascade-binding-generator
```

## Example Model Writer

Write an example model to a file:

```bash
cargo run --bin write_model -- --help
```

## Code Formatting

### Rust Code
```bash
cargo +nightly fmt
```

### C++ Code
```bash
clang-format -i crates/opencascade-sys/include/wrapper.hxx
```

## Credits

- [bschwind/opencascade-rs](https://github.com/bschwind/opencascade-rs) — the original project this is forked from, including the higher-level `opencascade` crate, viewer, and examples
- [OpenCASCADE Technology](https://dev.opencascade.org/) — the C++ CAD kernel
- The binding generator and the 216 commits expanding `opencascade-sys` were written with extensive LLM assistance (primarily Claude)
