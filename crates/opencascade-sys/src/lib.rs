//! OpenCASCADE FFI bindings
//!
//! This crate provides low-level FFI bindings to the OpenCASCADE geometry kernel.
//! The bindings are auto-generated using the `opencascade-binding-generator` crate.
//!
//! # Structure
//!
//! The generated code is organized by OCCT module:
//! - `gp` - Basic geometry types (points, vectors, directions, transforms)
//! - `topo_ds` - Topology data structure (shapes, vertices, edges, faces, etc.)
//! - `collections` - Iterator wrappers for OCCT collection types (ListOfShape, etc.)
//!
//! # Re-generation
//!
//! To regenerate the bindings, run:
//! ```bash
//! ./scripts/regenerate-bindings.sh
//! ```

// Support types for extern "C" FFI
mod owned_ptr;
pub use owned_ptr::*;

// Include the generated modules (including collections)
#[path = "../generated/lib.rs"]
mod generated;

// Re-export all generated modules
pub use generated::*;
