//! OCCT Binding Generator Library
//!
//! This library parses OCCT C++ headers and generates CXX bridge code
//! with a unified FFI module and per-module re-exports.

pub mod codegen;
pub mod header_deps;
pub mod model;
pub mod module_graph;
pub mod parser;
pub mod resolver;
pub mod type_mapping;
