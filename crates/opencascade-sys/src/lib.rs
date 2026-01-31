//! OpenCascade-sys: Auto-generated Rust bindings to the OpenCascade CAD Kernel
//!
//! This crate provides low-level FFI bindings to OCCT (OpenCascade Technology).
//! The bindings are automatically generated using the opencascade-binding-generator crate.
//!
//! To regenerate the bindings, run:
//! ```bash
//! DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib cargo run -p opencascade-binding-generator -- \
//!   -I target/OCCT/include \
//!   -o crates/opencascade-sys/generated \
//!   $(cat crates/opencascade-sys/headers.txt | grep -v '^#' | grep -v '^$' | sed 's|^|target/OCCT/include/|')
//! ```

// Include the generated lib.rs which declares all modules
include!("../generated/lib.rs");
