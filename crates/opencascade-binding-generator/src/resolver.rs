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
    /// Method uses an enum type that can't be bound (enum class requires integer conversion at FFI boundary)
    UsesEnum { enum_name: String },
    /// Class is abstract (has pure virtual methods)
    AbstractClass,
    /// Method needs explicit lifetimes (&mut self return with reference params)
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
    /// Function references Handle types for classes without Handle declarations
    UnknownHandleType,
    /// Method has const char*& or const char* const& parameter (needs manual binding)
    StringRefParam { param_name: String, type_name: String },
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
    /// All method names declared in this class (public AND protected/private).
    /// Used to detect when an intermediate class has overridden a public ancestor
    /// method as protected (access narrowing), preventing binding generation.
    pub all_method_names: HashSet<String>,
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
    /// Source line number in the header file
    pub source_line: Option<u32>,
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
    /// C++ short name without namespace (e.g., "Edge")
    pub short_name: String,
    /// Namespace (e.g., "TopoDS")
    pub namespace: String,
    /// Rust module
    pub rust_module: String,
    /// Rust function name (base, before dedup)
    pub rust_name: String,
    /// Deduplicated Rust FFI function name (unique across the entire FFI module)
    pub rust_ffi_name: String,
    /// C++ wrapper function name (used in both #[cxx_name] and wrappers.hxx)
    pub cpp_wrapper_name: String,
    /// Parameters
    pub params: Vec<ResolvedParam>,
    /// Return type
    pub return_type: Option<ResolvedType>,
    /// Binding status
    pub status: BindingStatus,
    /// Source header
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
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
    /// Binding status (enums are currently excluded due to FFI limitations)
    pub status: BindingStatus,
    /// Documentation comment
    pub doc_comment: Option<String>,
    /// Whether this enum is a bitset (values are powers of 2, used as flags)
    /// Bitset enums stay as i32 at the Rust API level; value enums get typed Rust enums.
    pub is_bitset: bool,
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
    /// Whether this parameter has a default value in C++
    pub has_default: bool,
}

impl ResolvedParam {
    /// Check if this parameter is a nullable pointer (T* = NULL or const T* = NULL).
    pub fn is_nullable_ptr(&self) -> bool {
        if !self.has_default {
            return false;
        }
        match &self.ty.original {
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }
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
    /// Whether this type needs new allocation (pointer return)
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin for mutable references
    pub needs_pin: bool,
    /// Module this type comes from (for cross-module references)
    pub source_module: Option<String>,
    /// If this is an enum type, the original C++ enum name (for static_cast in wrappers)
    pub enum_cpp_name: Option<String>,
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
    /// All class names (including collection typedef names)
    pub all_class_names: HashSet<String>,
    /// Classes that can have Handle<T> declarations (transitive closure + signature scanning)
    pub handle_able_classes: HashSet<String>,
    /// Cross-module type references by module
    pub cross_module_types: HashMap<String, Vec<CrossModuleType>>,
    /// Authoritative mapping from C++ type name to module name (built from parsed headers)
    /// This is the single source of truth for "which module does type X belong to?"
    pub type_to_module: HashMap<String, String>,
    /// Mapping from C++ enum name to qualified Rust enum type path (e.g., "crate::top_abs::Orientation")
    /// Only contains value enums (not bitset enums). These get typed Rust enum params/returns.
    pub enum_rust_types: HashMap<String, String>,
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

    /// Get all included functions for a module
    pub fn included_functions_for_module(&self, module: &str) -> Vec<&ResolvedFunction> {
        self.functions_for_module(module)
            .into_iter()
            .filter(|f| f.status.is_included())
            .collect()
    }

    /// Get all included functions across all modules, in stable order
    pub fn all_included_functions(&self) -> Vec<&ResolvedFunction> {
        let mut modules: Vec<&String> = self.functions_by_module.keys().collect();
        modules.sort();
        let mut result = Vec::new();
        for module in modules {
            if let Some(ids) = self.functions_by_module.get(module.as_str()) {
                for id in ids {
                    if let Some(f) = self.functions.get(id) {
                        if f.status.is_included() {
                            result.push(f);
                        }
                    }
                }
            }
        }
        result
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
        let mut to_process = std::collections::VecDeque::new();
        if let Some(class) = self.class_by_name(cpp_name) {
            for base in &class.base_classes {
                to_process.push_back(base.clone());
            }
        } else {
            return ancestors;
        };
        
        // BFS: process closest ancestors first so that `seen_methods` in
        // `compute_inherited_method_bindings` correctly lets the closest
        // ancestor's method shadow more-distant ancestors (C++ name-hiding).
        while let Some(base) = to_process.pop_front() {
            if visited.contains(&base) {
                continue;
            }
            visited.insert(base.clone());
            ancestors.push(base.clone());
            
            if let Some(base_class) = self.class_by_name(&base) {
                for parent in &base_class.base_classes {
                    if !visited.contains(parent) {
                        to_process.push_back(parent.clone());
                    }
                }
            }
        }
        
        ancestors
    }

    /// Get all descendants of a class by C++ name (classes that directly or indirectly inherit from it)
    pub fn get_all_descendants_by_name(&self, cpp_name: &str) -> Vec<String> {
        let mut descendants = Vec::new();
        for class in self.classes.values() {
            if class.cpp_name == cpp_name {
                continue;
            }
            let ancestors = self.get_all_ancestors_by_name(&class.cpp_name);
            if ancestors.contains(&cpp_name.to_string()) {
                descendants.push(class.cpp_name.clone());
            }
        }
        descendants.sort();
        descendants
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
        || method.return_type.as_ref().is_some_and(|t| type_uses_enum(t, all_enums))
}

/// Check if a constructor uses any enum types
pub fn constructor_uses_enum(ctor: &Constructor, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&ctor.params, all_enums)
}

/// Check if a static method uses any enum types
pub fn static_method_uses_enum(method: &StaticMethod, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&method.params, all_enums)
        || method.return_type.as_ref().is_some_and(|t| type_uses_enum(t, all_enums))
}

/// Check if a free function uses any enum types
pub fn function_uses_enum(func: &ParsedFunction, all_enums: &HashSet<String>) -> bool {
    params_use_enum(&func.params, all_enums)
        || func.return_type.as_ref().is_some_and(|t| type_uses_enum(t, all_enums))
}

/// Check if a method needs explicit lifetimes (FFI limitation)
/// Returns true if the method returns a mutable reference and has other reference parameters.
/// Rust can't infer lifetimes when there are multiple potential sources.
pub fn method_needs_explicit_lifetimes(method: &Method) -> bool {
    // Check if return type is a mutable reference (&mut Self or MutRef)
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

/// Check if a const method returns a mutable reference (not allowed at FFI boundary)
/// The FFI requires &mut self when returning &mut, but C++ allows const methods to return non-const refs
pub fn has_const_mut_return_mismatch(method: &Method) -> bool {
    if !method.is_const {
        return false;
    }
    // Check if return type is a mutable reference
    method.return_type.as_ref().map(|ty| {
        matches!(ty, Type::MutRef(_))
    }).unwrap_or(false)
}

/// Check if a method has unsupported by-value parameters.
/// By-value enums (Type::Class) are supported (mapped to i32).
/// By-value classes and Handles are now supported (C++ wrappers accept const T&).
/// MutRef to enums are NOT supported (output params need local variable + writeback).
pub fn method_has_unsupported_by_value_params(_method: &Method, _all_enum_names: &HashSet<String>) -> Option<(String, String)> {
    // MutRef to enum params are now handled via C++ wrapper with local variable + writeback.
    // No remaining unsupported param types.
    None
}

/// Check if a static method has unsupported by-value parameters.
/// By-value enums (Type::Class) are supported (mapped to i32).
/// By-value classes and Handles are now supported (C++ wrappers accept const T&).
/// MutRef to enums are now supported (C++ wrappers with local variable + writeback).
pub fn static_method_has_unsupported_by_value_params(_method: &StaticMethod, _all_enum_names: &HashSet<String>) -> Option<(String, String)> {
    // MutRef to enum params are now handled via C++ wrapper with local variable + writeback.
    // No remaining unsupported param types.
    None
}

/// Check if a method has const char*& or const char* const& parameters.
/// These require manual bindings because:
/// - const char*& (output param): Rust's &str is immutable, can't write back to C++
/// - const char* const&: Generator converts to &str but C++ expects &const char*
pub fn method_has_string_ref_param(method: &Method) -> Option<(String, String)> {
    for param in &method.params {
        let param_type = &param.ty;
        // Check for Standard_CString& (const char*&)
        if let Type::MutRef(inner) = param_type {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    if name == "char" {
                        return Some((param.name.clone(), "const char*&".to_string()));
                    }
                }
            }
        }
        // Check for const Standard_CString& (const char* const&)
        if let Type::ConstRef(inner) = param_type {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    if name == "char" {
                        return Some((param.name.clone(), "const char* const&".to_string()));
                    }
                }
            }
        }
    }
    None
}

/// Check if a static method has const char*& or const char* const& parameters.
pub fn static_method_has_string_ref_param(method: &StaticMethod) -> Option<(String, String)> {
    for param in &method.params {
        let param_type = &param.ty;
        // Check for Standard_CString& (const char*&)
        if let Type::MutRef(inner) = param_type {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    if name == "char" {
                        return Some((param.name.clone(), "const char*&".to_string()));
                    }
                }
            }
        }
        // Check for const Standard_CString& (const char* const&)
        if let Type::ConstRef(inner) = param_type {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    if name == "char" {
                        return Some((param.name.clone(), "const char* const&".to_string()));
                    }
                }
            }
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
    matches!(&method.return_type, Some(Type::Class(_)) | Some(Type::Handle(_)))
}

/// Check if a static method needs a C++ wrapper
fn static_method_needs_wrapper(method: &StaticMethod) -> bool {
    matches!(&method.return_type, Some(Type::Class(_)) | Some(Type::Handle(_)))
}

/// Determine if an enum is a bitset (flag-style) enum.
///
/// Bitset enums have values that are powers of 2 and are meant to be OR'd together.
/// These stay as i32 at the Rust API level. Value enums (the common case) get
/// typed Rust enum params/returns.
///
/// Heuristic: an enum is a bitset if:
/// - Its name contains "Flag", "Flags", or "Mask", OR
/// - All non-zero variant values are powers of 2, there are at least 3 such
///   powers, and the maximum value is >= 4 (to avoid false positives like
///   sequential 0, 1, 2 enums)
fn is_bitset_enum(parsed: &ParsedEnum) -> bool {
    let name = &parsed.name;
    // Check naming convention (covers combination-value flag enums like MaskFlags)
    if name.contains("Flag") || name.contains("Mask") {
        return true;
    }

    // Compute actual values (auto-increment when None)
    let mut values = Vec::new();
    let mut next_val: i64 = 0;
    for v in &parsed.variants {
        let val = v.value.unwrap_or(next_val);
        values.push(val);
        next_val = val + 1;
    }

    // Check if all non-zero values are powers of 2
    let nonzero: Vec<i64> = values.iter().copied().filter(|&v| v > 0).collect();
    if nonzero.len() < 3 {
        return false;
    }

    let all_powers_of_2 = nonzero.iter().all(|&v| (v & (v - 1)) == 0);
    let max_val = nonzero.iter().copied().max().unwrap_or(0);

    all_powers_of_2 && max_val >= 4
}

/// Build the symbol table from parsed headers and module graph
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Start from the pre-computed handle-able classes (transitive closure of inheritance graph)
    let mut handle_able_classes = handle_able_classes.clone();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
                    set.insert(name.clone());
                }
            }
            crate::model::Type::ConstRef(inner)
            | crate::model::Type::MutRef(inner)
            | crate::model::Type::ConstPtr(inner)
            | crate::model::Type::MutPtr(inner)
            | crate::model::Type::RValueRef(inner) => {
                collect_handle_types(inner, set);
            }
            _ => {}
        }
    }
    for class in all_classes {
        for method in &class.methods {
            for param in &method.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
            if let Some(ret) = &method.return_type {
                collect_handle_types(ret, &mut handle_able_classes);
            }
        }
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
        }
    }
    // Also scan standalone functions
    for func in all_functions {
        for param in &func.params {
            collect_handle_types(&param.ty, &mut handle_able_classes);
        }
        if let Some(ret) = &func.return_type {
            collect_handle_types(ret, &mut handle_able_classes);
        }
    }
    // These referenced Handle types also need to be known class names
    all_class_names.extend(handle_able_classes.iter().cloned());
    
    // Build authoritative type→module mapping from parsed header data.
    // This is the single source of truth for module membership.
    let type_to_module: HashMap<String, String> = all_classes
        .iter()
        .map(|c| (c.name.clone(), c.module.clone()))
        .chain(all_enums.iter().map(|e| (e.name.clone(), e.module.clone())))
        .collect();
    // Keep a reference copy for resolve_* functions (avoids borrow conflicts with table)
    let type_to_module_ref = type_to_module.clone();
    
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
        handle_able_classes: handle_able_classes.clone(),
        cross_module_types: HashMap::new(),
        type_to_module,
        enum_rust_types: HashMap::new(),
    };
    
    // Build cross-module types map
    for module in modules {
        let cross_types = graph.get_cross_module_types(&module.name);
        table.cross_module_types.insert(module.rust_name.clone(), cross_types);
    }
    
    // Resolve all enums (currently all excluded due to FFI limitations — integer conversion needed)
    for enum_decl in all_enums {
        let id = SymbolId::new(format!("enum::{}", enum_decl.name));
        
        let resolved = ResolvedEnum {
            id: id.clone(),
            cpp_name: enum_decl.name.clone(),
            rust_module: crate::module_graph::module_to_rust_name(&enum_decl.module),
            rust_name: safe_short_name(&crate::type_mapping::short_name_for_module(&enum_decl.name, &enum_decl.module)),
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
            // Enums are included via integer pass-through (i32 at FFI boundary,
            // C++ wrappers static_cast between int32_t and the OCCT enum type)
            status: BindingStatus::Included,
            doc_comment: enum_decl.comment.clone(),
            is_bitset: is_bitset_enum(enum_decl),
        };
        
        table.enums_by_module
            .entry(resolved.rust_module.clone())
            .or_default()
            .push(id.clone());
        table.enums.insert(id, resolved);
    }
    
    // Build enum_rust_types map: C++ enum name → qualified Rust type path
    // Only includes value enums (not bitset enums)
    for resolved in table.enums.values() {
        if !resolved.is_bitset && resolved.status.is_included() {
            let rust_type = format!("crate::{}::{}", resolved.rust_module, resolved.rust_name);
            table.enum_rust_types.insert(resolved.cpp_name.clone(), rust_type);
        }
    }
    
    // Resolve all classes
    for class in all_classes {
        resolve_class(&mut table, class, &all_enum_names, &type_to_module_ref);
    }
    
    // Resolve all free functions
    for func in all_functions {
        resolve_function(&mut table, func, &all_enum_names, &all_class_names, &handle_able_classes, &type_to_module_ref);
    }
    
    // Note: Function naming (rust_ffi_name, cpp_wrapper_name) is now handled by
    // compute_all_function_bindings() in bindings.rs. The placeholder names set
    // during resolve_function() are no longer used by emitters.
    
    table
}

/// Resolve a single class and its members
fn resolve_class(
    table: &mut SymbolTable,
    class: &ParsedClass,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) {
    let class_id = SymbolId::new(format!("class::{}", class.name));
    let rust_module = crate::module_graph::module_to_rust_name(&class.module);
    let short_name = crate::type_mapping::short_name_for_module(&class.name, &class.module);
    let rust_ffi_name = safe_short_name(&short_name);
    
    // Determine class binding status
    // Protected-destructor classes are now included (methods, statics, handles)
    // with only ctor/dtor generation skipped.
    let class_status = BindingStatus::Included;
    
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
            type_to_module,
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
            type_to_module,
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
            type_to_module,
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
        is_abstract: class.is_abstract,
        has_protected_destructor: class.has_protected_destructor,
        base_classes: class.base_classes.clone(),
        constructors: constructor_ids,
        methods: method_ids,
        static_methods: static_method_ids,
        all_method_names: class.all_method_names.clone(),
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
    type_to_module: &HashMap<String, String>,
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
            ty: resolve_type(&p.ty, all_enum_names, type_to_module),
            has_default: p.has_default,
        }
    }).collect();
    
    // Determine status
    let status = if is_abstract {
        BindingStatus::Excluded(ExclusionReason::AbstractClass)
    } else if ctor.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableConstructor)
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
    type_to_module: &HashMap<String, String>,
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
            ty: resolve_type(&p.ty, all_enum_names, type_to_module),
            has_default: p.has_default,
        }
    }).collect();
    
    // Resolve return type
    let return_type = method.return_type.as_ref().map(|t| resolve_type(t, all_enum_names, type_to_module));
    
    // Determine status
    let status = if method.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableType {
            description: "method has unbindable types".to_string(),
        })
    } else if method_needs_explicit_lifetimes(method) {
        BindingStatus::Excluded(ExclusionReason::NeedsExplicitLifetimes)
    } else if let Some((param_name, type_name)) = method_has_unsupported_by_value_params(method, all_enum_names) {
        BindingStatus::Excluded(ExclusionReason::UnsupportedByValueParam { param_name, type_name })
    } else if let Some((param_name, type_name)) = method_has_string_ref_param(method) {
        BindingStatus::Excluded(ExclusionReason::StringRefParam { param_name, type_name })
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
        source_line: method.source_line,
    }
}

/// Resolve a static method
fn resolve_static_method(
    id: &SymbolId,
    class_id: &SymbolId,
    class_name: &str,
    method: &StaticMethod,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
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
            ty: resolve_type(&p.ty, all_enum_names, type_to_module),
            has_default: p.has_default,
        }
    }).collect();
    
    // Resolve return type
    let return_type = method.return_type.as_ref().map(|t| resolve_type(t, all_enum_names, type_to_module));
    
    // Determine status
    let status = if method.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableStaticMethod)
    } else if let Some((param_name, type_name)) = static_method_has_unsupported_by_value_params(method, all_enum_names) {
        BindingStatus::Excluded(ExclusionReason::UnsupportedByValueParam { param_name, type_name })
    } else if let Some((param_name, type_name)) = static_method_has_string_ref_param(method) {
        BindingStatus::Excluded(ExclusionReason::StringRefParam { param_name, type_name })
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
    all_class_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) {
    // Build a unique ID that distinguishes overloads by parameter types
    let param_sig: String = func.params.iter()
        .map(|p| format!("{:?}", p.ty))
        .collect::<Vec<_>>()
        .join(",");
    let base_id = format!("func::{}({})", func.name, param_sig);
    
    // Handle the (rare) case where even the param signature isn't unique
    let id = if table.functions.contains_key(&SymbolId::new(base_id.clone())) {
        let mut counter = 2;
        loop {
            let candidate = SymbolId::new(format!("{}#{}", base_id, counter));
            if !table.functions.contains_key(&candidate) {
                break candidate;
            }
            counter += 1;
        }
    } else {
        SymbolId::new(base_id)
    };
    let rust_module = crate::module_graph::module_to_rust_name(&func.module);
    
    // Resolve parameters
    let params: Vec<ResolvedParam> = func.params.iter().map(|p| {
        ResolvedParam {
            name: p.name.clone(),
            rust_name: safe_param_name(&p.name),
            ty: resolve_type(&p.ty, all_enum_names, type_to_module),
            has_default: p.has_default,
        }
    }).collect();
    
    // Resolve return type
    let return_type = func.return_type.as_ref().map(|t| resolve_type(t, all_enum_names, type_to_module));
    
    // Determine status — check unbindable types and unknown handle types.
    // C string returns (const char*) are handled by C++ wrappers that return const char* directly.
    let status = if func.has_unbindable_types() {
        BindingStatus::Excluded(ExclusionReason::UnbindableFunction)
    } else if function_uses_unknown_handle(func, all_class_names, all_enum_names, handle_able_classes) {
        BindingStatus::Excluded(ExclusionReason::UnknownHandleType)
    } else {
        BindingStatus::Included
    };
    
    let base_rust_name = func.short_name.to_snake_case();
    
    let resolved = ResolvedFunction {
        id: id.clone(),
        cpp_name: func.name.clone(),
        short_name: func.short_name.clone(),
        namespace: func.namespace.clone(),
        rust_module: rust_module.clone(),
        rust_name: base_rust_name.clone(),
        // Placeholder names — will be assigned by assign_function_names()
        rust_ffi_name: base_rust_name,
        cpp_wrapper_name: String::new(),
        params,
        return_type,
        status,
        source_header: func.source_header.clone(),
        source_line: func.source_line,
        doc_comment: func.comment.clone(),
    };
    
    table.functions_by_module
        .entry(rust_module)
        .or_default()
        .push(id.clone());
    table.functions.insert(id, resolved);
}

/// Check if a function references unknown Handle/class types.
/// Enum types (Type::Class that are in all_enum_names) are known — they map to i32.
/// MutRef to enum is NOT skipped here — those are output parameters that
/// need special handling (local variable + writeback), not supported yet.
fn function_uses_unknown_handle(
    func: &ParsedFunction,
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> bool {
    let check = |ty: &Type| -> bool {
        // Enum types by value are known (mapped to i32), so skip them.
        // ConstRef to enum is also fine (maps to const int32_t&).
        // MutRef to enum is NOT skipped — extern "C" can't bind int32_t& ↔ EnumType&.
        match ty {
            Type::Class(name) if all_enum_names.contains(name) => return false,
            Type::ConstRef(inner) => {
                if let Type::Class(name) = inner.as_ref() {
                    if all_enum_names.contains(name) {
                        return false;
                    }
                }
            }
            _ => {}
        }
        crate::type_mapping::type_uses_unknown_handle(ty, all_class_names, handle_able_classes)
    };
    if func.params.iter().any(|p| check(&p.ty)) {
        return true;
    }
    if let Some(ref ret) = func.return_type {
        if check(ret) {
            return true;
        }
    }
    false
}

/// Resolve a type to its code generation form
fn resolve_type(ty: &Type, all_enum_names: &HashSet<String>, type_to_module: &HashMap<String, String>) -> ResolvedType {
    // Check if this type is an enum (possibly wrapped in const ref)
    let enum_name = extract_enum_name_from_type(ty, all_enum_names);
    if let Some(ref name) = enum_name {
        return ResolvedType {
            original: ty.clone(),
            rust_ffi_type: "i32".to_string(),
            cpp_type: "int32_t".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
            enum_cpp_name: Some(name.clone()),
        };
    }

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
        source_module: lookup_type_module(ty, type_to_module),
        enum_cpp_name: None,
    }
}

/// Look up the module for a Type from the authoritative type→module mapping
fn lookup_type_module(ty: &Type, type_to_module: &HashMap<String, String>) -> Option<String> {
    match ty {
        Type::Class(name) | Type::Handle(name) => type_to_module.get(name).cloned(),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            lookup_type_module(inner, type_to_module)
        }
        _ => None,
    }
}

/// Extract the enum C++ name from a type, unwrapping references
fn extract_enum_name_from_type(ty: &Type, all_enums: &HashSet<String>) -> Option<String> {
    match ty {
        Type::Class(name) if all_enums.contains(name) => Some(name.clone()),
        // Only unwrap const refs and rvalue refs, NOT MutRef (output params need special handling)
        Type::ConstRef(inner) | Type::RValueRef(inner) => {
            extract_enum_name_from_type(inner, all_enums)
        }
        _ => None,
    }
}

/// Convert a Type to C++ type string
fn type_to_cpp_string(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
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


