//! Module dependency graph analysis
//!
//! Analyzes which types each class references to determine module dependencies
//! and generate proper cross-module type aliases.

use crate::model::{ParsedClass, ParsedHeader, Type};
use heck::ToSnakeCase;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// A graph of module dependencies
#[derive(Debug, Default)]
pub struct ModuleGraph {
    /// All modules, keyed by module name
    pub modules: BTreeMap<String, Module>,
}

/// A single module containing types and their dependencies
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name (e.g., "gp", "TopoDS", "BRepPrimAPI")
    pub name: String,
    /// Rust module name (snake_case, e.g., "gp", "topo_ds", "brep_prim_api")
    pub rust_name: String,
    /// Types defined in this module
    pub types: Vec<String>,
    /// Enum types defined in this module (subset of types)
    pub enum_types: BTreeSet<String>,
    /// Other modules this module depends on (references types from)
    pub dependencies: BTreeSet<String>,
}

impl Module {
    /// Create a new module with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rust_name: module_to_rust_name(name),
            types: Vec::new(),
            enum_types: BTreeSet::new(),
            dependencies: BTreeSet::new(),
        }
    }

    /// Add a type to this module
    pub fn add_type(&mut self, type_name: &str) {
        if !self.types.contains(&type_name.to_string()) {
            self.types.push(type_name.to_string());
        }
    }

    /// Add an enum type to this module
    pub fn add_enum_type(&mut self, type_name: &str) {
        self.add_type(type_name);
        self.enum_types.insert(type_name.to_string());
    }

    /// Check if a type is an enum
    pub fn is_enum(&self, type_name: &str) -> bool {
        self.enum_types.contains(type_name)
    }

    /// Add a dependency on another module
    pub fn add_dependency(&mut self, module_name: &str) {
        if module_name != self.name {
            self.dependencies.insert(module_name.to_string());
        }
    }
}

impl ModuleGraph {
    /// Build a module graph from parsed headers
    pub fn from_headers(headers: &[ParsedHeader]) -> Self {
        let mut graph = ModuleGraph::default();

        // First pass: register all types in their modules
        // Include all classes (even those with protected destructors) because they may be
        // referenced by upcast methods or as cross-module type aliases
        for header in headers {
            for class in &header.classes {
                let module = graph
                    .modules
                    .entry(class.module.clone())
                    .or_insert_with(|| Module::new(&class.module));
                module.add_type(&class.name);
            }

            // Also register enums (use add_enum_type to track them separately)
            // Note: enums are currently disabled in codegen, so don't add them to types
            // This prevents cross-module type aliases for enums that won't exist
            for enum_decl in &header.enums {
                let module = graph
                    .modules
                    .entry(enum_decl.module.clone())
                    .or_insert_with(|| Module::new(&enum_decl.module));
                module.add_enum_type(&enum_decl.name);
            }
        }

        // Build reverse lookup: type_name -> module_name from first pass data
        let type_to_module: HashMap<String, String> = graph
            .modules
            .iter()
            .flat_map(|(module_name, module)| {
                module.types.iter().map(move |type_name| (type_name.clone(), module_name.clone()))
            })
            .collect();

        // Second pass: analyze dependencies using lookup from first pass
        for header in headers {
            for class in &header.classes {
                let dependencies = collect_type_dependencies(class);

                if let Some(module) = graph.modules.get_mut(&class.module) {
                    for dep_type in dependencies {
                        if let Some(dep_module) = type_to_module.get(&dep_type) {
                            module.add_dependency(dep_module);
                        }
                    }
                }
            }
        }

        graph
    }

    /// Get a module by name
    #[allow(dead_code)]
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    /// Get all modules in dependency order (modules with no dependencies first)
    pub fn modules_in_order(&self) -> Vec<&Module> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut in_progress = HashSet::new();

        for module_name in self.modules.keys() {
            self.visit_module(module_name, &mut visited, &mut in_progress, &mut result);
        }

        result
    }

    fn visit_module<'a>(
        &'a self,
        name: &str,
        visited: &mut HashSet<String>,
        in_progress: &mut HashSet<String>,
        result: &mut Vec<&'a Module>,
    ) {
        if visited.contains(name) {
            return;
        }

        // Cycle detection
        if in_progress.contains(name) {
            // Cyclic dependency - just continue
            return;
        }

        in_progress.insert(name.to_string());

        if let Some(module) = self.modules.get(name) {
            // Visit dependencies first
            for dep in &module.dependencies {
                self.visit_module(dep, visited, in_progress, result);
            }

            visited.insert(name.to_string());
            in_progress.remove(name);
            result.push(module);
        }
    }

    /// Get all cross-module type references needed for a module
    pub fn get_cross_module_types(&self, module_name: &str) -> Vec<CrossModuleType> {
        let mut result = Vec::new();

        if let Some(module) = self.modules.get(module_name) {
            for dep_module_name in &module.dependencies {
                if let Some(dep_module) = self.modules.get(dep_module_name) {
                    for type_name in &dep_module.types {
                        result.push(CrossModuleType {
                            cpp_name: type_name.clone(),
                            rust_name: crate::type_mapping::short_name_for_module(type_name, &dep_module.name),
                            source_module: dep_module.rust_name.clone(),
                            is_enum: dep_module.is_enum(type_name),
                        });
                    }
                }
            }
        }

        // Sort for deterministic ordering
        result.sort_by(|a, b| a.cpp_name.cmp(&b.cpp_name));
        result
    }
}

/// A type from another module that needs to be aliased
#[derive(Debug, Clone)]
pub struct CrossModuleType {
    /// Full C++ type name (e.g., "gp_Pnt")
    pub cpp_name: String,
    /// Rust type name without module prefix (e.g., "Pnt")
    pub rust_name: String,
    /// Source module's Rust name (e.g., "gp")
    pub source_module: String,
    /// Whether this is an enum (enums use full C++ names, classes use short names)
    pub is_enum: bool,
}

/// Collect all type dependencies from a class
fn collect_type_dependencies(class: &ParsedClass) -> HashSet<String> {
    let mut deps = HashSet::new();

    // Collect from base classes (for upcasts and inherited methods)
    for base_class in &class.base_classes {
        deps.insert(base_class.clone());
    }

    // Collect from constructors
    for ctor in &class.constructors {
        for param in &ctor.params {
            collect_types_from_type(&param.ty, &mut deps);
        }
    }

    // Collect from methods
    for method in &class.methods {
        for param in &method.params {
            collect_types_from_type(&param.ty, &mut deps);
        }
        if let Some(ref ret) = method.return_type {
            collect_types_from_type(ret, &mut deps);
        }
    }

    // Collect from static methods
    for method in &class.static_methods {
        for param in &method.params {
            collect_types_from_type(&param.ty, &mut deps);
        }
        if let Some(ref ret) = method.return_type {
            collect_types_from_type(ret, &mut deps);
        }
    }

    deps
}

/// Recursively collect OCCT class types from a type
fn collect_types_from_type(ty: &Type, deps: &mut HashSet<String>) {
    match ty {
        Type::Class(name) => {
            // Skip types that don't have a module prefix (e.g., "ReadMode_ProductContext")
            // These are likely nested types that aren't accessible at global scope
            if name.contains('_') || name.starts_with("std::") || name.starts_with("opencascade::") {
                // Has module prefix or is a known namespace type
                deps.insert(name.clone());
            }
            // Otherwise skip - it's likely a nested type without proper scoping
        }
        Type::Handle(name) => {
            deps.insert(name.clone());
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            collect_types_from_type(inner, deps);
        }
        _ => {}
    }
}

/// Convert C++ module name to Rust module name (snake_case)
pub fn module_to_rust_name(name: &str) -> String {
    // Handle special cases
    match name {
        "gp" => "gp".to_string(),
        _ => name.to_snake_case(),
    }
}

/// Extract Rust type name from C++ class name (remove module prefix)
#[cfg(test)]
fn extract_rust_type_name(cpp_name: &str) -> String {
    if let Some(underscore_pos) = cpp_name.find('_') {
        cpp_name[underscore_pos + 1..].to_string()
    } else {
        cpp_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_to_rust_name() {
        assert_eq!(module_to_rust_name("gp"), "gp");
        assert_eq!(module_to_rust_name("TopoDS"), "topo_ds");
        assert_eq!(module_to_rust_name("BRepPrimAPI"), "b_rep_prim_api");
    }

    #[test]
    fn test_extract_rust_type_name() {
        assert_eq!(extract_rust_type_name("gp_Pnt"), "Pnt");
        assert_eq!(extract_rust_type_name("TopoDS_Shape"), "Shape");
        assert_eq!(extract_rust_type_name("BRepPrimAPI_MakeBox"), "MakeBox");
    }
}
