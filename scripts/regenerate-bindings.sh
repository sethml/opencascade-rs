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
HEADERS_FILE="$REPO_ROOT/crates/opencascade-sys/headers.txt"

# Check prerequisites
if [[ ! -d "$OCCT_INCLUDE" ]]; then
    echo "Error: OCCT include directory not found at $OCCT_INCLUDE"
    echo "Run 'cargo build -p occt-sys' first to build OCCT"
    exit 1
fi

if [[ ! -f "$HEADERS_FILE" ]]; then
    echo "Error: Headers file not found at $HEADERS_FILE"
    exit 1
fi

# Build the generator
echo "Building binding generator..."
cargo build --release -p opencascade-binding-generator

# Read headers from headers.txt, skipping comments and empty lines
HEADERS=()
while IFS= read -r line || [[ -n "$line" ]]; do
    # Skip comments and empty lines
    [[ "$line" =~ ^[[:space:]]*# ]] && continue
    [[ -z "${line// }" ]] && continue
    
    # Add full path
    header_path="$OCCT_INCLUDE/$line"
    if [[ -f "$header_path" ]]; then
        HEADERS+=("$header_path")
    else
        echo "Warning: Header not found: $header_path"
    fi
done < "$HEADERS_FILE"

if [[ ${#HEADERS[@]} -eq 0 ]]; then
    echo "Error: No valid headers found"
    exit 1
fi

echo "Generating bindings for ${#HEADERS[@]} headers..."

# Clean generated directory
echo "Cleaning $OUTPUT_DIR..."
rm -f "$OUTPUT_DIR"/*.rs "$OUTPUT_DIR"/*.hxx

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

# Run the generator
# --resolve-deps: automatically include headers that our explicit headers depend on
#                 so that all required base classes and types are parsed
"$REPO_ROOT/target/release/occt-bindgen" \
    --resolve-deps \
    -I "$OCCT_INCLUDE" \
    -o "$OUTPUT_DIR" \
    "$@" \
    "${HEADERS[@]}"

echo ""
echo "Bindings generated in $OUTPUT_DIR"
