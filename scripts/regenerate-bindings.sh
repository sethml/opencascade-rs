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
# Also remove hidden generated files with empty basenames (e.g. `.rs`), which
# the `*.rs` glob doesn't match on most shells.
find "$OUTPUT_DIR" -maxdepth 1 -name '.rs' -delete 2>/dev/null || true
find "$OUTPUT_DIR" -maxdepth 1 -name '.hxx' -delete 2>/dev/null || true
find "$OUTPUT_DIR" -maxdepth 1 -name '.cpp' -delete 2>/dev/null || true

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

# Run the generator
# Include both installed headers and build headers (needed for .pxx private headers)
OCCT_BUILD_INCLUDE="$REPO_ROOT/target/OCCT/build/include"
if [[ -d "$OCCT_BUILD_INCLUDE" ]]; then
    BUILD_INCLUDE_ARG=(-I "$OCCT_BUILD_INCLUDE")
else
    BUILD_INCLUDE_ARG=()
fi

"$REPO_ROOT/target/release/occt-bindgen" \
    --config "$CONFIG_FILE" \
    -I "$OCCT_INCLUDE" \
    "${BUILD_INCLUDE_ARG[@]}" \
    -o "$OUTPUT_DIR" \
    "$@"

echo ""
echo "Bindings generated in $OUTPUT_DIR"
