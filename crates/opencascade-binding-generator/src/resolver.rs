//! Symbol resolution and binding decision making
//!
//! This module implements Pass 1 of the two-pass architecture:
//! Parse all headers and build a complete `SymbolTable` containing every symbol
//! we'll wrap, with all derived information pre-computed.
//!
//! Benefits:
//! - Single source of truth for filtering (computed once, used by both rust.rs and cpp.rs)
//! - Pre-computed naming (rust_ffi_name, rust_public_name, cpp_name)
//! - Better cross-module support (all symbols known before code generation)
//! - Debuggability (can dump symbol table to inspect what will be generated)

use crate::model::{Constructor, Method, ParsedClass, ParsedEnum, ParsedFunction, Param, StaticMethod, Type};
use crate::module_graph::{CrossModuleType, Module, ModuleGraph};
use crate::type_mapping::safe_short_name;
use heck::ToSnakeCase;
use std::collections::{HashMap, HashSet};

/// Unique identifier for a symbol in the symbol table
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub String);

impl SymbolId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The kind of symbol being bound
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// A C++ class
    Class,
    /// A class constructor
    Constructor,
    /// An instance method
    Method,
    /// A static method  
    StaticMethod,
    /// A free function (namespace-level)
    Function,
    /// An enum type
    Enum,
    /// A Handle<T> type
    HandleType,
}

/// Why a symbol is excluded from binding generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExclusionReason {
    /// Method uses an enum type that can't be bound (CXX requires enum class)
    UsesEnum { enum_name: String },
    /// Class is abstract (has pure virtual methods)
    AbstractClass,
    /// Class has protected/private destructor
    ProtectedDestructor,
    /// Method needs explicit lifetimes (Pin<&mut Self> return with reference params)
    NeedsExplicitLifetimes,
    /// Method has unsupported by-value parameter (class or handle type)
    UnsupportedByValueParam { param_name: String, type_name: String },
    /// Method has const/mut return mismatch
    ConstMutReturnMismatch,
    /// Type is unbindable (streams, void pointers, arrays, etc.)
    UnbindableType { description: String },
    /// Constructor has unbindable types
    UnbindableConstructor,
    /// Static method has unbindable types
    UnbindableStaticMethod,
    /// Function has unbindable types
    UnbindableFunction,
}

/// Binding status for a symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingStatus {
    /// Will be generated
    Included,
    /// Skipped with reason
    Excluded(ExclusionReason),
}

impl BindingStatus {
    pub fn is_included(&self) -> bool {
        matches!(self, BindingStatus::Included)
    }
    
    pub fn is_excluded(&self) -> bool {
        !self.is_included()
    }
}

/// Information about a resolved class
#[derive(Debug, Clone)]
pub struct ResolvedClass {
    /// Symbol ID for this class
    pub id: SymbolId,
    /// C++ fully qualified name (e.g., "gp_Pnt")
    pub cpp_name: String,
    /// Rust module this belongs to (e.g., "gp")
    pub rust_module: String,
    /// Rust FFI type name with escaping (e.g., "Pnt", "Vec_")
    pub rust_ffi_name: String,
    /// Rust public name for re-exports (e.g., "Vec" when ffi name is "Vec_")
    pub rust_public_name: String,
    /// Source header file
    pub source_header: String,
    /// Documentation comment
    pub doc_comment: Option<String>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this is a Handle type
    pub is_handle_type: bool,
    /// Whether this class is abstract
    pub is_abstract: bool,
    /// Whether this class has a protected destructor
    pub has_protected_destructor: bool,
    /// Base classes (C++ names)
    pub base_classes: Vec<String>,
    /// Constructor symbol IDs
    pub constructors: Vec<SymbolId>,
    /// Method symbol IDs
    pub methods: Vec<SymbolId>,
    /// Static method symbol IDs
    pub static_methods: Vec<SymbolId>,
}

/// Information about a resolved constructor
#[derive(Debug, Clone)]
pub struct ResolvedConstructor {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Rust function name (e.g., "new", "new_real3")
    pub rust_name: String,
    /// C++ wrapper function name
    pub cpp_wrapper_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Binding status
    pub status: BindingStatus,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved method
#[derive(Debug, Clone)]
pub struct ResolvedMethod {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Original C++ method name
    pub cpp_name: String,
    /// Rust method name (snake_case)
    pub rust_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this method needs a C++ wrapper (returns by value)
    pub needs_wrapper: bool,
    /// C++ wrapper function name (if needs_wrapper)
    pub cpp_wrapper_name: Option<String>,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved static method
#[derive(Debug, Clone)]
pub struct ResolvedStaticMethod {
    /// Symbol ID
    pub id: SymbolId,
    /// Parent class ID
    pub class_id: SymbolId,
    /// Original C++ method name
    pub cpp_name: String,
    /// Rust method name (snake_case)
    pub rust_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Whether this method needs a C++ wrapper
    pub needs_wrapper: bool,
    /// C++ wrapper function name (if needs_wrapper)
    pub cpp_wrapper_name: Option<String>,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved free function
#[derive(Debug, Clone)]
pub struct ResolvedFunction {
    /// Symbol ID
    pub id: SymbolId,
    /// C++ fully qualified name (e.g., "TopoDS::Edge")
    pub cpp_name: String,
    /// Namespace (e.g., "TopoDS")
    pub namespace: String,
    /// Rust module
    pub rust_module: String,
    /// Rust function name
    pub rust_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Source header
    pub source_header: String,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// Information about a resolved enum
#[derive(Debug, Clone)]
pub struct ResolvedEnum {
    /// Symbol ID
    pub id: SymbolId,
    /// C++ enum name
    pub cpp_name: String,
    /// Rust module
    pub rust_module: String,
    /// Rust type name
    pub rust_name: String,
    /// Source header
    pub source_header: String,
    /// Variants
    pub variants: Vec<ResolvedEnumVariant>,
    /// Binding status (enums are currently excluded due to CXX limitations)
    pub status: BindingStatus,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// A resolved enum variant
#[derive(Debug, Clone)]
pub struct ResolvedEnumVariant {
    /// C++ variant name
    pub cpp_name: String,
    /// Rust variant name (PascalCase)
    pub rust_name: String,
    /// Explicit value if specified
    pub value: Option<i64>,
    /// Documentation comment
    pub doc_comment: Option<String>,
}

/// A resolved parameter
#[derive(Debug, Clone)]
pub struct ResolvedParam {
    /// Parameter name
    pub name: String,
    /// Safe Rust name (keywords escaped)
    pub rust_name: String,
    /// Parameter type
    pub ty: ResolvedType,
}

/// A resolved type with all information needed for code generation
#[derive(Debug, Clone)]
pub struct ResolvedType {
    /// The original Type from parsing
    pub original: Type,
    /// Rust type string for FFI
    pub rust_ffi_type: String,
    /// C++ type string
    pub cpp_type: String,
    /// Whether this type needs UniquePtr in return position
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin for mutable references
    pub needs_pin: bool,
    /// Module this type comes from (for cross-module references)
    pub source_module: Option<String>,
}

/// Complete symbol table for all modules
#[derive(Debug)]
pub struct SymbolTable {
    /// All classes indexed by ID
    pub classes: HashMap<SymbolId, ResolvedClass>,
    /// All constructors indexed by ID
    pub constructors: HashMap<SymbolId, ResolvedConstructor>,
    /// All methods indexed by ID
    pub methods: HashMap<SymbolId, ResolvedMethod>,
    /// All static methods indexed by ID
    pub static_methods: HashMap<SymbolId, ResolvedStaticMethod>,
    /// All functions indexed by ID
    pub functions: HashMap<SymbolId, ResolvedFunction>,
    /// All enums indexed by ID
    pub enums: HashMap<SymbolId, ResolvedEnum>,
    /// Classes grouped by module
    pub classes_by_module: HashMap<String, Vec<SymbolId>>,
    /// Functions grouped by module
    pub functions_by_module: HashMap<String, Vec<SymbolId>>,
    /// Enums grouped by module
    pub enums_by_module: HashMap<String, Vec<SymbolId>>,
    /// All enum names (for filtering methods that use enums)
    pub all_enum_names: HashSet<String>,
    /// All class names
    pub all_class_names: HashSet<String>,
    /// Cross-module type references by module
    pub cross_module_types: HashMap<String, Vec<CrossModuleType>>,
}

impl SymbolTable {
    /// Get all classes for a module
    pub fn classes_for_module(&self, module: &str) -> Vec<&ResolvedClass> {
        self.classes_by_module
            .get(module)
            .map(|ids| ids.iter().filter_map(|id| self.classes.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get all included classes for a module
    pub fn included_classes_for_module(&self, module: &str) -> Vec<&ResolvedClass> {
        self.classes_for_module(module)
            .into_iter()
            .filter(|c| c.status.is_included())
            .collect()
    }
    
    /// Get all functions for a module
    pub fn functions_for_module(&self, module: &str) -> Vec<&ResolvedFunction> {
        self.functions_by_module
            .get(module)
            .map(|ids| ids.iter().filter_map(|id| self.functions.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get all enums for a module
    pub fn enums_for_module(&self, module: &str) -> Vec<&ResolvedEnum> {
        self.enums_by_module
            .get(module)
            .map(|ids| ids.iter().filter_map(|id| self.enums.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get cross-module types for a module
    pub fn cross_module_types_for_module(&self, module: &str) -> &[CrossModuleType] {
        self.cross_module_types
            .get(module)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    
    /// Get all included constructors for a class
    pub fn included_constructors(&self, class: &ResolvedClass) -> Vec<&ResolvedConstructor> {
        class.constructors
            .iter()
            .filter_map(|id| self.constructors.get(id))
            .filter(|c| c.status.is_included())
            .collect()
    }
    
    /// Get all included methods for a class
    pub fn included_methods(&self, class: &ResolvedClass) -> Vec<&ResolvedMethod> {
        class.methods
            .iter()
            .filter_map(|id| self.methods.get(id))
            .filter(|m| m.status.is_included())
            .collect()
    }
    
    /// Get all included static methods for a class
    pub fn included_static_methods(&self, class: &ResolvedClass) -> Vec<&ResolvedStaticMethod> {
        class.static_methods
            .iter()
            .filter_map(|id| self.static_methods.get(id))
            .filter(|m| m.status.is_included())
            .collect()
    }
    
    /// Get class by C++ name
    pub fn class_by_name(&self, cpp_name: &str) -> Option<&ResolvedClass> {
        let id = SymbolId::new(format!("class::{}", cpp_name));
        self.classes.get(&id)
    }
    
    /// Get set of all C++ class names that have protected destructors
    pub fn protected_destructor_class_names(&self) -> HashSet<String> {
        self.classes
            .values()
            .filter(|c| c.has_protected_destructor)
            .map(|c| c.cpp_name.clone())
            .collect()
    }
    
    /// Get base classes for a class (recursively collecting all ancestors)
    pub fn get_all_ancestors(&self, class: &ResolvedClass) -> Vec<String> {
        self.get_all_ancestors_by_name(&class.cpp_name)
    }
    
    /// Get all ancestors by C++ class name (recursively collecting all base classes)
    pub fn get_all_ancestors_by_name(&self, cpp_name: &str) -> Vec<String> {
        let mut ancestors = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        
        // Start with the direct base classes of the given class
        let mut to_process = if let Some(class) = self.class_by_name(cpp_name) {
            class.base_classes.clone()
        } else {
            return ancestors;
        };
        
        while let Some(base) = to_process.pop() {
            if visited.contains(&base) {
                continue;
            }
            visited.insert(base.clone());
            ancestors.push(base.clone());
            
            if let Some(base_class) = self.class_by_name(&base) {
                for parent in &base_class.base_classes {
                    if !visited.contains(parent) {
                        to_process.push(parent.clone());
                    }
                }
            }
        }
        
        // Sort for deterministic output
        ancestors.sort();
        ancestors
    }
}

/// Rust keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Check if a type uses an enum
pub fn type_uses_enum(ty: &Type, all_enums: &HashSet<String>) -> bool {
    match ty {
        Type::Class(name) => all_enums.contains(name),
        Type::Handle(name) => all_enums.contains(name),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) |
        Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_enum(inner, all_enums),
        _ => false,
    }
}

/// Check if parameters use any enum types
pub fn params_use_enum(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| type_uses_enum(&p.ty, all_enums))
}

/// Check if a method uses any enum types (params or return type)
pub fn method_uses_enum(method: &Method, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&method.params, all_enums) 
        || method.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enums))
}

/// Check if a constructor uses any enum types
pub fn constructor_uses_enum(ctor: &Constructor, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&ctor.params, all_enums)
}

/// Check if a static method uses any enum types
pub fn static_method_uses_enum(method: &StaticMethod, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&method.params, all_enums)
        || method.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enums))
}

/// Check if a free function uses any enum types
pub fn function_uses_enum(func: &ParsedFunction, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&func.params, all_enums)
        || func.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enums))
}

/// Check if a method needs explicit lifetimes (CXX limitation)
/// Returns true if the method returns a mutable reference and has other reference parameters.
/// Rust can't infer lifetimes when there are multiple potential sources.
pub fn method_needs_explicit_lifetimes(method: &Method) -> bool {
    // Check if return type is a mutable reference (Pin<&mut Self> or MutRef)
    let returns_mut_ref = method.return_type.as_ref().map(|ty| {
        matches!(ty, Type::MutRef(_))
    }).unwrap_or(false);
    
    if !returns_mut_ref {
        return false;
    }
    
    // Check if any parameter is a reference (other than self which is handled separately)
    // Also treat const char* as a reference parameter
    method.params.iter().any(|p| {
        matches!(&p.ty, Type::ConstRef(_) | Type::MutRef(_)) || p.ty.is_c_string()
    })
}

/// Check if a const method returns a mutable reference (not allowed by CXX)
/// CXX requires &mut self when returning &mut, but C++ allows const methods to return non-const refs
pub fn has_const_mut_return_mismatch(method: &Method) -> bool {
    if !method.is_const {
        return false;
    }
    // Check if return type is a mutable reference
    method.return_type.as_ref().map(|ty| {
        matches!(ty, Type::MutRef(_))
    }).unwrap_or(false)
}

/// Check if a method has unsupported by-value parameters
pub fn method_has_unsupported_by_value_params(method: &Method) -> Option<(String, String)> {
    for param in &method.params {
        match &param.ty {
            Type::Class(name) => {
                return Some((param.name.clone(), name.clone()));
            }
            Type::Handle(name) => {
                return Some((param.name.clone(), format!("Handle<{}>", name)));
            }
            _ => {}
        }
    }
    None
}

/// Check if a static method has unsupported by-value parameters
pub fn static_method_has_unsupported_by_value_params(method: &StaticMethod) -> Option<(String, String)> {
    for param in &method.params {
        match &param.ty {
            Type::Class(name) => {
                return Some((param.name.clone(), name.clone()));
            }
            Type::Handle(name) => {
                return Some((param.name.clone(), format!("Handle<{}>", name)));
            }
            _ => {}
        }
    }
    None
}

/// Convert a method name to safe Rust identifier
fn safe_method_name(name: &str) -> String {
    let snake_name = name.to_snake_case();
    if RUST_KEYWORDS.contains(&snake_name.as_str()) {
        format!("{}_", snake_name)
    } else {
        snake_name
    }
}

/// Convert a parameter name to safe Rust identifier
fn safe_param_name(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

/// Check if a method needs a C++ wrapper (returns class by value)
fn method_needs_wrapper(method: &Method) -> bool {
    match &method.return_type {
        Some(Type::Class(_)) => true,
        Some(Type::Handle(_)) => true,
        _ => false,
    }
}

/// Check if a static method needs a C++ wrapper
fn static_method_needs_wrapper(method: &StaticMethod) -> bool {
    match &method.return_type {
        Some(Type::Class(_)) => true,
        Some(Type::Handle(_)) => true,
        _ => false,
    }
}

/// Build the symbol table from parsed headers and module graph
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    
    let mut table = SymbolTable {
        classes: HashMap::new(),
        constructors: HashMap::new(),
        methods: HashMap::new(),
        static_methods: HashMap::new(),
        functions: HashMap::new(),
        enums: HashMap::new(),
        classes_by_module: HashMap::new(),
        functions_by_module: HashMap::new(),
        enums_by_module: HashMap::new(),
        all_enum_names: all_enum_names.clone(),
        all_class_names: all_class_names.clone(),
        cross_module_types: HashMap::new(),
    };
    
    // Build cross-module types map
    for module in modules {
        let cross_types = graph.get_cross_module_types(&module.name);
        table.cross_module_types.insert(module.rust_name.clone(), cross_types);
    }
    
    // Resolve all enums (currently all excluded due to CXX limitations)
    for enum_decl in all_enums {
        let id = SymbolId::new(format!("enum::{}", enum_decl.name));
        
        let resolved = ResolvedEnum {
            id: id.clone(),
            cpp_name: enum_decl.name.clone(),
            rust_module: crate::module_graph::module_to_rust_name(&enum_decl.module),
            rust_name: safe_short_name(enum_decl.name.split('_').last().unwrap_or(&enum_decl.name)),
            source_header: enum_decl.source_header.clone(),
            variants: enum_decl.variants.iter().map(|v| {
                // Convert SCREAMING_SNAKE to PascalCase for Rust
                let rust_name = v.name
                    .split('_')
                    .skip(1) // Skip module prefix
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().chain(chars.map(|c| c.to_ascii_lowercase())).collect(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                
                ResolvedEnumVariant {
                    cpp_name: v.name.clone(),
                    rust_name: if rust_name.is_empty() { v.name.clone() } else { rust_name },
                    value: v.value,
                    doc_comment: v.comment.clone(),
                }
            }).collect(),
            // Enums are excluded because CXX requires enum class but OCCT uses unscoped enums
            status: BindingStatus::Excluded(ExclusionReason::UsesEnum { 
                enum_name: "CXX requires enum class".to_string() 
            }),
            doc_comment: enum_decl.comment.clone(),
        };
        
        table.enums_by_module
            .entry(resolved.rust_module.clone())
            .or_default()
            .push(id.clone());
        table.enums.insert(id, resolved);
    }
    
    // Resolve all classes
    for class in all_classes {
        resolve_class(&mut table, class, &all_enum_names);
    }
    
    // Resolve all free functions
    for func in all_functions {
        resolve_function(&mut table, func, &all_enum_names);
    }
    
    table
}

/// Resolve a single class and its members
fn resolve_class(
    table: &mut SymbolTable,
    class: &ParsedClass,
    all_enum_names: &HashSet<String>,
) {
    let class_id = SymbolId::new(format!("class::{}", class.name));
    let rust_module = crate::module_graph::module_to_rust_name(&class.module);
    let short_name = class.short_name();
    let rust_ffi_name = safe_short_name(short_name);
    
    // Determine class binding status
    let class_status = if class.has_protected_destructor {
        BindingStatus::Excluded(ExclusionReason::ProtectedDestructor)
    } else {
        BindingStatus::Included
    };
    
    // Resolve constructors
    let mut constructor_ids = Vec::new();
    for (idx, ctor) in class.constructors.iter().enumerate() {
        let ctor_id = SymbolId::new(format!("ctor::{}::{}", class.name, idx));
        let resolved_ctor = resolve_constructor(
            &ctor_id,
            &class_id,
            &class.name,
            ctor,
            idx,
            class.is_abstract,
            all_enum_names,
        );
        constructor_ids.push(ctor_id.clone());
        table.constructors.insert(ctor_id, resolved_ctor);
    }
    
    // Resolve methods
    let mut method_ids = Vec::new();
    for (idx, method) in class.methods.iter().enumerate() {
        let method_id = SymbolId::new(format!("method::{}::{}::{}", class.name, method.name, idx));
        let resolved_method = resolve_method(
            &method_id,
            &class_id,
            &class.name,
            method,
            all_enum_names,
        );
        method_ids.push(method_id.clone());
        table.methods.insert(method_id, resolved_method);
    }
    
    // Resolve static methods
    let mut static_method_ids = Vec::new();
    for (idx, method) in class.static_methods.iter().enumerate() {
        let method_id = SymbolId::new(format!("static::{}::{}::{}", class.name, method.name, idx));
        let resolved_method = resolve_static_method(
            &method_id,
            &class_id,
            &class.name,
            method,
            all_enum_names,
        );
        static_method_ids.push(method_id.clone());
        table.static_methods.insert(method_id, resolved_method);
    }
    
    let resolved = ResolvedClass {
        id: class_id.clone(),
        cpp_name: class.name.clone(),
        rust_module: rust_module.clone(),
        rust_ffi_name,
        rust_public_name: short_name.to_string(),
        source_header: class.source_header.clone(),
        doc_comment: class.comment.clone(),
        status: class_status,
        is_handle_type: class.is_handle_type,
        is_abstract: class.is_abstract,
        has_protected_destructor: class.has_protected_destructor,
        base_classes: class.base_classes.clone(),
        constructors: constructor_ids,
        methods: method_ids,
        static_methods: static_method_ids,
    };
    
    table.classes_by_module
        .entry(rust_module)
        .or_default()
        .push(class_id.clone());
    table.classes.insert(class_id, resolved);
}

/// Resolve a constructor
fn resolve_constructor(
    id: &SymbolId,
    class_id: &SymbolId,
    class_name: &str,
    ctor: &Constructor,
    _idx: usize,
    is_abstract: bool,
    all_enum_names: &HashSet<String>,
) -> ResolvedConstructor {
    // Determine constructor name (new, new_real3, etc.)
    let suffix = ctor.overload_suffix();
    let rust_name = format!("new{}", suffix);
    
    // C++ wrapper name
    let cpp_wrapper_name = format!("{}_ctor{}", class_name, suffix);
    
    // Resolve parameters
    let params: Vec<ResolvedParam> = ctor.params.iter().map(|p| {
        ResolvedParam {
            name: p.name.clone(),
            rust_name: safe_param_name(&p.name),
            ty: resolve_type(&p.ty),
        }
    }).collect();
    
    // Determine status
    let status = if is_abstract {
        BindingStatus::Excluded(ExclusionReason::AbstractClass)
    } else if ctor.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableConstructor)
    } else if params_use_enum(&ctor.params, all_enum_names) {
        let enum_name = ctor.params.iter()
            .find(|p| type_uses_enum(&p.ty, all_enum_names))
            .map(|p| format!("{:?}", p.ty))
            .unwrap_or_default();
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else {
        BindingStatus::Included
    };
    
    ResolvedConstructor {
        id: id.clone(),
        class_id: class_id.clone(),
        rust_name,
        cpp_wrapper_name,
        params,
        status,
        doc_comment: ctor.comment.clone(),
    }
}

/// Resolve a method
fn resolve_method(
    id: &SymbolId,
    class_id: &SymbolId,
    class_name: &str,
    method: &Method,
    all_enum_names: &HashSet<String>,
) -> ResolvedMethod {
    let rust_name = safe_method_name(&method.name);
    let needs_wrapper = method_needs_wrapper(method);
    
    // C++ wrapper name (if needed)
    let cpp_wrapper_name = if needs_wrapper {
        Some(format!("{}_{}", class_name, method.name))
    } else {
        None
    };
    
    // Resolve parameters
    let params: Vec<ResolvedParam> = method.params.iter().map(|p| {
        ResolvedParam {
            name: p.name.clone(),
            rust_name: safe_param_name(&p.name),
            ty: resolve_type(&p.ty),
        }
    }).collect();
    
    // Resolve return type
    let return_type = method.return_type.as_ref().map(resolve_type);
    
    // Determine status
    let status = if method.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableType {
            description: "method has unbindable types".to_string(),
        })
    } else if params_use_enum(&method.params, all_enum_names) {
        let enum_name = method.params.iter()
            .find(|p| type_uses_enum(&p.ty, all_enum_names))
            .map(|p| format!("{:?}", p.ty))
            .unwrap_or_default();
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else if method.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enum_names)) {
        let enum_name = format!("{:?}", method.return_type);
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else if method_needs_explicit_lifetimes(method) {
        BindingStatus::Excluded(ExclusionReason::NeedsExplicitLifetimes)
    } else if let Some((param_name, type_name)) = method_has_unsupported_by_value_params(method) {
        BindingStatus::Excluded(ExclusionReason::UnsupportedByValueParam { param_name, type_name })
    } else {
        BindingStatus::Included
    };
    
    ResolvedMethod {
        id: id.clone(),
        class_id: class_id.clone(),
        cpp_name: method.name.clone(),
        rust_name,
        is_const: method.is_const,
        params,
        return_type,
        status,
        needs_wrapper,
        cpp_wrapper_name,
        doc_comment: method.comment.clone(),
    }
}

/// Resolve a static method
fn resolve_static_method(
    id: &SymbolId,
    class_id: &SymbolId,
    class_name: &str,
    method: &StaticMethod,
    all_enum_names: &HashSet<String>,
) -> ResolvedStaticMethod {
    let rust_name = safe_method_name(&method.name);
    let needs_wrapper = static_method_needs_wrapper(method);
    
    // C++ wrapper name (if needed)
    let cpp_wrapper_name = if needs_wrapper {
        Some(format!("{}_{}", class_name, method.name))
    } else {
        None
    };
    
    // Resolve parameters
    let params: Vec<ResolvedParam> = method.params.iter().map(|p| {
        ResolvedParam {
            name: p.name.clone(),
            rust_name: safe_param_name(&p.name),
            ty: resolve_type(&p.ty),
        }
    }).collect();
    
    // Resolve return type
    let return_type = method.return_type.as_ref().map(resolve_type);
    
    // Determine status
    let status = if method.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableStaticMethod)
    } else if params_use_enum(&method.params, all_enum_names) {
        let enum_name = method.params.iter()
            .find(|p| type_uses_enum(&p.ty, all_enum_names))
            .map(|p| format!("{:?}", p.ty))
            .unwrap_or_default();
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else if method.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enum_names)) {
        let enum_name = format!("{:?}", method.return_type);
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else if let Some((param_name, type_name)) = static_method_has_unsupported_by_value_params(method) {
        BindingStatus::Excluded(ExclusionReason::UnsupportedByValueParam { param_name, type_name })
    } else {
        BindingStatus::Included
    };
    
    ResolvedStaticMethod {
        id: id.clone(),
        class_id: class_id.clone(),
        cpp_name: method.name.clone(),
        rust_name,
        params,
        return_type,
        status,
        needs_wrapper,
        cpp_wrapper_name,
        doc_comment: method.comment.clone(),
    }
}

/// Resolve a free function
fn resolve_function(
    table: &mut SymbolTable,
    func: &ParsedFunction,
    all_enum_names: &HashSet<String>,
) {
    let id = SymbolId::new(format!("func::{}", func.name));
    let rust_module = crate::module_graph::module_to_rust_name(&func.module);
    
    // Resolve parameters
    let params: Vec<ResolvedParam> = func.params.iter().map(|p| {
        ResolvedParam {
            name: p.name.clone(),
            rust_name: safe_param_name(&p.name),
            ty: resolve_type(&p.ty),
        }
    }).collect();
    
    // Resolve return type
    let return_type = func.return_type.as_ref().map(resolve_type);
    
    // Determine status
    let status = if func.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableFunction)
    } else if params_use_enum(&func.params, all_enum_names) {
        let enum_name = func.params.iter()
            .find(|p| type_uses_enum(&p.ty, all_enum_names))
            .map(|p| format!("{:?}", p.ty))
            .unwrap_or_default();
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else if func.return_type.as_ref().map_or(false, |t| type_uses_enum(t, all_enum_names)) {
        let enum_name = format!("{:?}", func.return_type);
        BindingStatus::Excluded(ExclusionReason::UsesEnum { enum_name })
    } else {
        BindingStatus::Included
    };
    
    let resolved = ResolvedFunction {
        id: id.clone(),
        cpp_name: func.name.clone(),
        namespace: func.namespace.clone(),
        rust_module: rust_module.clone(),
        rust_name: safe_method_name(&func.short_name),
        params,
        return_type,
        status,
        source_header: func.source_header.clone(),
        doc_comment: func.comment.clone(),
    };
    
    table.functions_by_module
        .entry(rust_module)
        .or_default()
        .push(id.clone());
    table.functions.insert(id, resolved);
}

/// Resolve a type to its code generation form
fn resolve_type(ty: &Type) -> ResolvedType {
    // For unbindable types, use a placeholder string
    // The binding status will ensure these don't get generated
    let rust_ffi_type = match ty {
        Type::RValueRef(_) => "<unbindable: rvalue-ref>".to_string(),
        _ => ty.to_rust_type_string(),
    };
    
    ResolvedType {
        original: ty.clone(),
        rust_ffi_type,
        cpp_type: type_to_cpp_string(ty),
        needs_unique_ptr: matches!(ty, Type::Class(_) | Type::Handle(_)),
        needs_pin: matches!(ty, Type::MutRef(inner) if !inner.is_primitive()),
        source_module: ty.module(),
    }
}

/// Convert a Type to C++ type string
fn type_to_cpp_string(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::I64 => "long".to_string(),
        Type::U64 => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "Standard_Real".to_string(),
        Type::ConstRef(inner) => format!("const {}&", type_to_cpp_string(inner)),
        Type::MutRef(inner) => format!("{}&", type_to_cpp_string(inner)),
        Type::RValueRef(inner) => format!("{}&&", type_to_cpp_string(inner)),
        Type::ConstPtr(inner) => format!("const {}*", type_to_cpp_string(inner)),
        Type::MutPtr(inner) => format!("{}*", type_to_cpp_string(inner)),
        Type::Handle(name) => format!("Handle({})", name),
        Type::Class(name) => name.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_method_name() {
        assert_eq!(safe_method_name("GetValue"), "get_value");
        assert_eq!(safe_method_name("Type"), "type_"); // keyword
        assert_eq!(safe_method_name("Move"), "move_"); // keyword
    }
    
    #[test]
    fn test_safe_param_name() {
        assert_eq!(safe_param_name("value"), "value");
        assert_eq!(safe_param_name("type"), "type_"); // keyword
        assert_eq!(safe_param_name("self"), "self_"); // keyword
    }
}


