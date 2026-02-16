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
