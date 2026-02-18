//! OCCT Binding Generator Library
//!
//! This library parses OCCT C++ headers and generates extern "C" FFI bindings
//! Generates FFI bindings with a single ffi.rs module and per-module re-exports.

pub mod codegen;
pub mod config;
pub mod header_deps;
pub mod model;
pub mod module_graph;
pub mod parser;
pub mod resolver;
pub mod type_mapping;
