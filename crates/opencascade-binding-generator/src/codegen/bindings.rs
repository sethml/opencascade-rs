//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, StaticMethod, Type};
use crate::module_graph;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

/// Rust keywords that need suffix escaping (CXX doesn't support raw identifiers).
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    pub cpp_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
}

/// A constructor that will have a C++ wrapper (std::make_unique).
#[derive(Debug, Clone)]
pub struct ConstructorBinding {
    /// FFI function name, e.g. "gp_Pnt_ctor_real3"
    pub ffi_fn_name: String,
    /// Impl method name in re-export, e.g. "new_real3"
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// C++ argument expressions for calling the constructor
    pub cpp_arg_exprs: Vec<String>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
}

/// A method bound directly by CXX (self receiver, no wrapper needed).
#[derive(Debug, Clone)]
pub struct DirectMethodBinding {
    /// Rust method name (snake_case, possibly with overload suffix)
    pub rust_name: String,
    /// Original C++ method name (for #[cxx_name])
    pub cxx_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// Return type (None for void)
    pub return_type: Option<ReturnTypeBinding>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
}

/// What kind of C++ wrapper is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrapperKind {
    /// Returns a class or handle by value → std::make_unique wrapper
    ByValueReturn,
    /// Has const char* parameters → rust::Str conversion wrapper
    CStringParam,
    /// Returns const char* → rust::String conversion wrapper
    CStringReturn,
}

/// A method that needs a C++ wrapper function.
#[derive(Debug, Clone)]
pub struct WrapperMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_mirrored_pnt")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ from ffi base if CXX conflict)
    pub impl_method_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters (excluding self)
    pub params: Vec<ParamBinding>,
    /// Return type
    pub return_type: Option<ReturnTypeBinding>,
    /// What kind of wrapper is needed
    pub wrapper_kind: WrapperKind,
    /// Original C++ method name
    pub cpp_method_name: String,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
}

/// A static method binding.
#[derive(Debug, Clone)]
pub struct StaticMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_origin_static")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ for instance/static conflicts)
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// Return type
    pub return_type: Option<ReturnTypeBinding>,
    /// Original C++ method name
    pub cpp_method_name: String,
    /// Whether reference returns need 'static lifetime
    pub needs_static_lifetime: bool,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
}

/// An upcast binding (Derived → Base).
#[derive(Debug, Clone)]
pub struct UpcastBinding {
    /// Base class C++ name, e.g. "Geom_Curve"
    pub base_class: String,
    /// Base class short name, e.g. "Curve"
    pub base_short_name: String,
    /// Base class module, e.g. "Geom"
    pub base_module: String,
    /// FFI function name for const upcast, e.g. "Geom_BSplineCurve_as_Geom_Curve"
    pub ffi_fn_name: String,
    /// FFI function name for mutable upcast
    pub ffi_fn_name_mut: String,
    /// Impl method name in re-export, e.g. "as_geom_curve" or "as_curve"
    pub impl_method_name: String,
}

/// A Handle upcast binding (Handle<Derived> → Handle<Base>).
#[derive(Debug, Clone)]
pub struct HandleUpcastBinding {
    /// Base handle type name, e.g. "HandleGeomCurve"
    pub base_handle_name: String,
    /// Base class C++ name, e.g. "Geom_Curve"
    pub base_class: String,
    /// FFI function name
    pub ffi_fn_name: String,
    /// Derived handle type name, e.g. "HandleGeomBSplineCurve"
    pub derived_handle_name: String,
}

/// An inherited method from an ancestor class.
#[derive(Debug, Clone)]
pub struct InheritedMethodBinding {
    /// FFI function name, e.g. "Geom_BSplineCurve_inherited_Continuity"
    pub ffi_fn_name: String,
    /// Impl method name in re-export
    pub impl_method_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters (resolved types from ancestor)
    pub params: Vec<ResolvedParamBinding>,
    /// Return type (resolved from ancestor)
    pub return_type: Option<ResolvedReturnTypeBinding>,
    /// Original C++ method name
    pub cpp_method_name: String,
    /// Which ancestor class this came from
    pub source_class: String,
}

/// A parameter binding with info for all three output targets.
#[derive(Debug, Clone)]
pub struct ParamBinding {
    /// Original C++ parameter name (for use in C++ wrapper declarations)
    pub cpp_name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    /// Type as it appears in ffi.rs (e.g. "f64", "&gp_Pnt", "Pin<&mut gp_Pnt>")
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt")
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx parameter (e.g. "Standard_Real", "const gp_Pnt&")
    pub cpp_type: String,
    /// C++ argument expression when calling OCCT (e.g. param name, or "std::string(x).c_str()")
    pub cpp_arg_expr: String,
}

/// A return type binding with info for all three output targets.
#[derive(Debug, Clone)]
pub struct ReturnTypeBinding {
    /// Type as it appears in ffi.rs
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx
    pub cpp_type: String,
    /// Whether the C++ return needs std::unique_ptr wrapping
    pub needs_unique_ptr: bool,
}

/// A resolved parameter binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedParamBinding {
    pub name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt")
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub cpp_arg_expr: String,
}

/// A resolved return type binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedReturnTypeBinding {
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub needs_unique_ptr: bool,
}

// ── Helper functions ────────────────────────────────────────────────────────

/// Convert a Type to Rust FFI type string using full C++ names.
///
/// Unlike `to_rust_type_string()` which uses short names for same-module types,
/// this always uses the full C++ name (e.g. `gp_Pnt` not `Pnt`). This is
/// needed for inherited methods which are declared in the derived class's FFI
/// block but reference types from the ancestor's module.
fn type_to_ffi_full_name(ty: &Type) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::os::raw::c_char".to_string()
            } else {
                name.clone() // Full C++ name like gp_Pnt, TopLoc_Location
            }
        }
        Type::Handle(name) => format!("Handle{}", name.replace("_", "")),
        Type::ConstRef(inner) => format!("&{}", type_to_ffi_full_name(inner)),
        Type::MutRef(inner) => {
            if inner.is_primitive() {
                format!("&mut {}", type_to_ffi_full_name(inner))
            } else {
                format!("Pin<&mut {}>", type_to_ffi_full_name(inner))
            }
        }
        Type::RValueRef(_) => "()".to_string(),
        Type::ConstPtr(inner) => {
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "&str".to_string()
            } else {
                format!("*const {}", type_to_ffi_full_name(inner))
            }
        }
        Type::MutPtr(inner) => format!("*mut {}", type_to_ffi_full_name(inner)),
    }
}

/// Convert a return Type to Rust FFI type string using full C++ names.
fn return_type_to_ffi_full_name(ty: &Type) -> String {
    match ty {
        Type::Class(name) if name != "char" => {
            format!("UniquePtr<{}>", name) // Full C++ name
        }
        Type::Handle(name) => {
            format!("UniquePtr<Handle{}>", name.replace("_", ""))
        }
        _ => type_to_ffi_full_name(ty),
    }
}

fn safe_method_name(name: &str) -> String {
    let snake_name = name.to_snake_case();
    if RUST_KEYWORDS.contains(&snake_name.as_str()) {
        format!("{}_", snake_name)
    } else {
        snake_name
    }
}

fn safe_param_name(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

/// Check if a type is or contains `const char*`
fn type_is_cstring(ty: &Type) -> bool {
    match ty {
        Type::ConstPtr(inner) => matches!(inner.as_ref(), Type::Class(name) if name == "char"),
        Type::ConstRef(inner) | Type::MutRef(inner) => type_is_cstring(inner),
        _ => false,
    }
}

/// Check if a parameter type uses an unknown Handle
fn param_uses_unknown_handle(ty: &Type, handle_able_classes: &HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            param_uses_unknown_handle(inner, handle_able_classes)
        }
        _ => false,
    }
}

/// Check if a type uses an unknown class/handle given the TypeContext
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Determine if a method needs a C++ wrapper function
fn needs_wrapper_function(method: &Method, all_enums: &HashSet<String>) -> bool {
    if method.params.iter().any(|p| p.ty.is_c_string()) {
        return true;
    }
    if method
        .return_type
        .as_ref()
        .map(|t| t.is_c_string())
        .unwrap_or(false)
    {
        return true;
    }
    method
        .return_type
        .as_ref()
        .map(|ty| {
            let is_class_or_handle = ty.is_class() || ty.is_handle();
            let is_enum = match ty {
                Type::Class(name) => all_enums.contains(name),
                _ => false,
            };
            is_class_or_handle && !is_enum
        })
        .unwrap_or(false)
}

/// Classify the wrapper kind for a method that needs_wrapper_function
fn classify_wrapper_kind(method: &Method, all_enums: &HashSet<String>) -> WrapperKind {
    let has_cstring_param = method.params.iter().any(|p| p.ty.is_c_string());
    let returns_cstring = method
        .return_type
        .as_ref()
        .map(|t| t.is_c_string())
        .unwrap_or(false);
    let returns_by_value = method.return_type.as_ref().map_or(false, |ty| {
        let is_class_or_handle = ty.is_class() || ty.is_handle();
        let is_enum = match ty {
            Type::Class(name) => all_enums.contains(name),
            _ => false,
        };
        is_class_or_handle && !is_enum
    });

    if returns_by_value {
        WrapperKind::ByValueReturn
    } else if has_cstring_param {
        // Has cstring param but doesn't return by value
        WrapperKind::CStringParam
    } else if returns_cstring {
        WrapperKind::CStringReturn
    } else {
        // Shouldn't happen if needs_wrapper_function returned true, but default
        WrapperKind::CStringReturn
    }
}

/// Convert a Type to C++ type string
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::I64 => "long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "Standard_Real".to_string(),
        Type::ConstRef(inner) => format!("const {}&", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}&", type_to_cpp(inner)),
        Type::RValueRef(inner) => format!("{}&&", type_to_cpp(inner)),
        Type::ConstPtr(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutPtr(inner) => format!("{}*", type_to_cpp(inner)),
        Type::Handle(name) => format!("opencascade::handle<{}>", name),
        Type::Class(name) => name.clone(),
    }
}

/// Convert a Type to C++ parameter type (const char* → rust::Str)
fn type_to_cpp_param(ty: &Type) -> String {
    match ty {
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "rust::Str".to_string()
        }
        _ => type_to_cpp(ty),
    }
}

/// Convert a parameter to C++ argument expression (rust::Str → std::string(...).c_str())
fn param_to_cpp_arg(param_name: &str, ty: &Type) -> String {
    match ty {
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            format!("std::string({}).c_str()", param_name)
        }
        _ => param_name.to_string(),
    }
}

/// Convert a Type to Rust type string for re-export files
fn unified_type_to_string(ty: &Type) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::os::raw::c_char".to_string()
            } else {
                format!("crate::ffi::{}", name)
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
        Type::ConstRef(inner) => format!("&{}", unified_type_to_string(inner)),
        Type::MutRef(inner) => {
            if inner.is_primitive() {
                format!("&mut {}", unified_type_to_string(inner))
            } else {
                format!(
                    "std::pin::Pin<&mut {}>",
                    unified_type_to_string(inner)
                )
            }
        }
        Type::RValueRef(_) => "()".to_string(),
        Type::ConstPtr(inner) => {
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "&str".to_string()
            } else {
                format!("*const {}", unified_type_to_string(inner))
            }
        }
        Type::MutPtr(inner) => format!("*mut {}", unified_type_to_string(inner)),
    }
}

/// Convert a return Type to Rust type string for re-export files
fn unified_return_type_to_string(ty: &Type) -> String {
    match ty {
        Type::Class(name) if name != "char" => {
            format!("cxx::UniquePtr<crate::ffi::{}>", name)
        }
        Type::Handle(name) => {
            format!(
                "cxx::UniquePtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        _ => unified_type_to_string(ty),
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────

/// Common filter for instance methods (both direct and wrapper)
fn is_method_bindable(method: &Method, ctx: &TypeContext) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    if resolver::method_has_unsupported_by_value_params(method).is_some() {
        return false;
    }
    if resolver::has_const_mut_return_mismatch(method) {
        return false;
    }
    if resolver::method_uses_enum(method, ctx.all_enums) {
        return false;
    }
    if resolver::method_needs_explicit_lifetimes(method) {
        return false;
    }
    if method
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return false;
        }
    }
    true
}

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> bool {
    if ctor
        .params
        .iter()
        .any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_)))
    {
        return false;
    }
    if ctor.has_unbindable_types() {
        return false;
    }
    if resolver::constructor_uses_enum(ctor, all_enum_names) {
        return false;
    }
    if ctor
        .params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    true
}

/// Filter for static methods
fn is_static_method_bindable(method: &StaticMethod, ctx: &TypeContext) -> bool {
    if method.has_unbindable_types() {
        return false;
    }
    if resolver::static_method_uses_enum(method, ctx.all_enums) {
        return false;
    }
    if method
        .params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return false;
        }
        if type_is_cstring(ret) {
            return false;
        }
    }
    true
}

// ── Building ParamBinding / ReturnTypeBinding ───────────────────────────────

fn build_param_binding(name: &str, ty: &Type, ffi_ctx: &TypeContext) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);
    let mapped = map_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = unified_type_to_string(ty);
    let cpp_type = type_to_cpp_param(ty);
    let cpp_arg_expr = param_to_cpp_arg(name, ty);

    ParamBinding {
        cpp_name,
        rust_name,
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        cpp_arg_expr,
    }
}

fn build_return_type_binding(ty: &Type, ffi_ctx: &TypeContext) -> ReturnTypeBinding {
    let mapped = map_return_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = unified_return_type_to_string(ty);
    let cpp_type = type_to_cpp(ty);
    let needs_unique_ptr = ty.is_class() || ty.is_handle();

    ReturnTypeBinding {
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        needs_unique_ptr,
    }
}

// ── Overload suffix computation ─────────────────────────────────────────────

/// Compute overload suffix with const/mut disambiguation for direct methods.
/// Returns (rust_name, suffix_used) for each method in the list.
fn compute_direct_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    let mut seen_names: HashMap<String, usize> = HashMap::new();

    methods
        .iter()
        .map(|method| {
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            let base_suffix = if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = methods.iter().any(|m| {
                    m.name == method.name
                        && m.overload_suffix() == base_suffix
                        && m.is_const != method.is_const
                });
                if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                }
            } else {
                String::new()
            };

            let base_rust_name = safe_method_name(&method.name);
            let candidate_name = if base_suffix.is_empty() {
                base_rust_name.clone()
            } else {
                format!("{}{}", base_rust_name, base_suffix)
            };

            let count = seen_names.entry(candidate_name.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                let suffix = format!("{}_{}", base_suffix, count);
                if suffix.is_empty() {
                    base_rust_name
                } else {
                    format!("{}{}", base_rust_name, suffix)
                }
            } else if base_suffix.is_empty() {
                base_rust_name
            } else {
                format!("{}{}", base_rust_name, base_suffix)
            }
        })
        .collect()
}

/// Compute overload suffix with const/mut disambiguation for wrapper methods.
/// Returns the base fn_name (without class prefix) for each method.
fn compute_wrapper_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = methods.iter().any(|m| {
                    m.name == method.name
                        && m.overload_suffix() == base_suffix
                        && m.is_const != method.is_const
                });
                let suffix = if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                };
                format!("{}{}", base_name, suffix)
            } else {
                base_name
            }
        })
        .collect()
}

/// Compute static method names with 3-level conflict resolution.
/// Returns (ffi_fn_name_base, impl_method_name) for each method.
fn compute_static_method_names(
    cpp_name: &str,
    methods: &[&StaticMethod],
    reserved_names: &HashSet<String>,
    all_instance_method_names: &HashSet<String>,
) -> Vec<(String, String)> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let has_internal_conflict =
                name_counts.get(&method.name).copied().unwrap_or(0) > 1;

            // Level 1: Internal overload suffix
            let candidate_fn_name = if has_internal_conflict {
                let suffix = method.overload_suffix();
                format!("{}{}", base_name, suffix)
            } else {
                base_name.clone()
            };

            // Level 2: Conflict with wrapper reserved names
            let candidate_full = format!("{}_{}", cpp_name, candidate_fn_name);
            let ffi_fn_name_base = if reserved_names.contains(&candidate_full) {
                let suffix = method.overload_suffix();
                if suffix.is_empty() {
                    format!("{}_static", base_name)
                } else {
                    format!("{}{}", base_name, suffix)
                }
            } else {
                candidate_fn_name
            };

            // Level 3: Conflict with instance method names (for re-export impl)
            let impl_method_name =
                if all_instance_method_names.contains(&ffi_fn_name_base) {
                    let suffix = method.overload_suffix();
                    if suffix.is_empty() {
                        format!("{}_static", ffi_fn_name_base)
                    } else {
                        format!("{}{}", base_name, suffix)
                    }
                } else {
                    ffi_fn_name_base.clone()
                };

            (ffi_fn_name_base, impl_method_name)
        })
        .collect()
}

// ── Abstract class detection ────────────────────────────────────────────────

/// Check if a class is effectively abstract by walking the inheritance chain.
///
/// A class is effectively abstract if:
/// 1. It declares pure virtual methods itself (`is_abstract` flag), OR
/// 2. It inherits pure virtual methods from ancestors that are not overridden
///    by any class in the inheritance chain (including itself).
fn is_effectively_abstract(
    class: &ParsedClass,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    symbol_table: &SymbolTable,
) -> bool {
    if class.is_abstract {
        return true;
    }

    // Collect ALL pure virtual methods from all ancestors
    let mut all_pvms: HashSet<String> = HashSet::new();
    // Collect ALL concrete methods from all ancestors + this class
    let mut all_concrete: HashSet<String> = HashSet::new();

    for ancestor_name in symbol_table.get_all_ancestors_by_name(&class.name) {
        if let Some(ancestor) = all_classes_by_name.get(&ancestor_name) {
            all_pvms.extend(ancestor.pure_virtual_methods.iter().cloned());
            // Concrete = all methods minus pure virtual declarations
            for m in &ancestor.all_method_names {
                if !ancestor.pure_virtual_methods.contains(m) {
                    all_concrete.insert(m.clone());
                }
            }
        }
    }

    // This class's own methods are concrete (is_abstract is false)
    all_concrete.extend(class.all_method_names.iter().cloned());

    // If any pure virtual method is not overridden, the class is abstract
    all_pvms.iter().any(|pvm| !all_concrete.contains(pvm))
}

// ── Main compute function ───────────────────────────────────────────────────

/// Compute all binding decisions for a class.
///
/// This is the SINGLE place where filtering, naming, overload suffixes,
/// and used_names conflict resolution happen.
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
) -> ClassBindings {
    let cpp_name = &class.name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    // ── Constructors ────────────────────────────────────────────────────
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes);
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (CXX self-receiver, no wrapper) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx) && !needs_wrapper_function(m, all_enum_names))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, ffi_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx));

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Wrapper methods (by-value return, const char*) ──────────────────
    let wrapper_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| is_method_bindable(m, ffi_ctx) && needs_wrapper_function(m, all_enum_names))
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = HashSet::new();
    for fn_name in &wrapper_fn_names {
        reserved_names.insert(format!("{}_{}", cpp_name, fn_name));
    }

    // Build CXX method names set (for re-export conflict detection)
    let cxx_method_names: HashSet<String> = direct_methods_raw
        .iter()
        .map(|m| safe_method_name(&m.name))
        .collect();

    // Build all_instance_method_names (CXX + wrapper impl names)
    let mut all_instance_method_names: HashSet<String> = cxx_method_names.clone();

    let wrapper_methods: Vec<WrapperMethodBinding> = wrapper_methods_raw
        .iter()
        .zip(wrapper_fn_names.iter())
        .map(|(method, fn_name)| {
            let ffi_fn_name = format!("{}_{}", cpp_name, fn_name);

            // Compute impl_method_name: may differ if fn_name conflicts with a CXX method
            let impl_method_name = if cxx_method_names.contains(fn_name) {
                let suffix = method.overload_suffix();
                if suffix.is_empty() {
                    format!("{}_wrapper", fn_name)
                } else {
                    let base_name = safe_method_name(&method.name);
                    format!("{}{}", base_name, suffix)
                }
            } else {
                fn_name.clone()
            };

            all_instance_method_names.insert(impl_method_name.clone());

            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, ffi_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx));
            let wrapper_kind = classify_wrapper_kind(method, all_enum_names);

            WrapperMethodBinding {
                ffi_fn_name,
                impl_method_name,
                is_const: method.is_const,
                params,
                return_type,
                wrapper_kind,
                cpp_method_name: method.name.clone(),
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Static methods ──────────────────────────────────────────────────
    let static_methods_raw: Vec<&StaticMethod> = class
        .static_methods
        .iter()
        .filter(|m| is_static_method_bindable(m, ffi_ctx))
        .collect();

    let static_method_names = compute_static_method_names(
        cpp_name,
        &static_methods_raw,
        &reserved_names,
        &all_instance_method_names,
    );

    let static_methods: Vec<StaticMethodBinding> = static_methods_raw
        .iter()
        .zip(static_method_names.iter())
        .map(|(method, (ffi_fn_name_base, impl_method_name))| {
            let ffi_fn_name = format!("{}_{}", cpp_name, ffi_fn_name_base);

            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, ffi_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx));

            let needs_static_lifetime = method
                .return_type
                .as_ref()
                .map(|ty| ty.is_reference())
                .unwrap_or(false);

            StaticMethodBinding {
                ffi_fn_name,
                impl_method_name: impl_method_name.clone(),
                params,
                return_type,
                cpp_method_name: method.name.clone(),
                needs_static_lifetime,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Upcasts ─────────────────────────────────────────────────────────
    let upcasts = compute_upcast_bindings(class, symbol_table);

    // ── to_owned ────────────────────────────────────────────────────────
    let copyable_modules = ["TopoDS", "gp", "TopLoc", "Bnd", "GProp"];
    let has_to_owned = copyable_modules.contains(&class.module.as_str())
        && !class.has_protected_destructor
        && !effectively_abstract;

    // ── to_handle ───────────────────────────────────────────────────────
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_to_handle {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };

    // ── Inherited methods ───────────────────────────────────────────────
    let inherited_methods =
        compute_inherited_method_bindings(class, symbol_table, handle_able_classes, ffi_ctx.all_classes, ffi_ctx.all_enums);

    ClassBindings {
        cpp_name: cpp_name.clone(),
        short_name: class.short_name().to_string(),
        module: class.module.clone(),
        is_abstract: effectively_abstract,
        is_handle_type: class.is_handle_type,
        has_protected_destructor: class.has_protected_destructor,
        doc_comment: class.comment.clone(),
        source_header: class.source_header.clone(),
        source_line: class.source_line,
        constructors,
        direct_methods,
        wrapper_methods,
        static_methods,
        upcasts,
        has_to_owned,
        has_to_handle,
        handle_upcasts,
        inherited_methods,
    }
}

// ── Constructor bindings ────────────────────────────────────────────────────

/// A constructor, possibly with trailing defaulted params trimmed.
struct TrimmedConstructor<'a> {
    original: &'a Constructor,
    /// How many params to include (may be less than original.params.len())
    trimmed_param_count: usize,
}

/// Check if a slice of params passes all bindability filters.
fn is_params_bindable(
    params: &[Param],
    all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> bool {
    if params
        .iter()
        .any(|p| matches!(&p.ty, Type::Class(_) | Type::Handle(_)))
    {
        return false;
    }
    if params.iter().any(|p| p.ty.is_unbindable()) {
        return false;
    }
    if resolver::params_use_enum(params, all_enum_names) {
        return false;
    }
    if params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    true
}

/// Compute overload suffix for a param slice (used for trimmed constructors).
fn overload_suffix_for_params(params: &[Param]) -> String {
    if params.is_empty() {
        return String::new();
    }

    let type_names: Vec<String> = params
        .iter()
        .map(|p| p.ty.short_name().to_lowercase())
        .collect();

    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < type_names.len() {
        let current = &type_names[i];
        let mut count = 1;
        while i + count < type_names.len() && &type_names[i + count] == current {
            count += 1;
        }
        if count > 1 {
            parts.push(format!("{}{}", current, count));
        } else {
            parts.push(current.clone());
        }
        i += count;
    }

    format!("_{}", parts.join("_"))
}

fn compute_constructor_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    handle_able_classes: &HashSet<String>,
) -> Vec<ConstructorBinding> {
    let cpp_name = &class.name;
    let all_enum_names = ffi_ctx.all_enums;

    // Collect directly bindable constructors
    let mut bindable_ctors: Vec<TrimmedConstructor> = class
        .constructors
        .iter()
        .filter(|c| is_constructor_bindable(c, all_enum_names, handle_able_classes))
        .map(|c| TrimmedConstructor {
            original: c,
            trimmed_param_count: c.params.len(),
        })
        .collect();

    // For constructors that failed binding, try trimming defaulted trailing params
    // that are unbindable (enums, by-value classes/handles). C++ requires defaults
    // contiguous from the right, so we strip from the end until the remaining
    // params pass the filter.
    for ctor in &class.constructors {
        if is_constructor_bindable(ctor, all_enum_names, handle_able_classes) {
            continue; // Already included
        }
        if ctor.has_unbindable_types() {
            continue; // Can't fix by trimming
        }

        // Try trimming from the end: find the rightmost non-default param
        // that still has issues, and see if trimming past it helps.
        let mut trim_to = ctor.params.len();
        while trim_to > 0 {
            let last_param = &ctor.params[trim_to - 1];
            if !last_param.has_default {
                break; // Can't trim non-default params
            }
            trim_to -= 1;

            // Check if the trimmed constructor would be bindable
            let trimmed_params = &ctor.params[..trim_to];
            if is_params_bindable(trimmed_params, all_enum_names, handle_able_classes) {
                // Check it's not a duplicate of an existing binding
                let already_exists = bindable_ctors.iter().any(|existing| {
                    existing.trimmed_param_count == trim_to
                        && existing
                            .original
                            .params
                            .iter()
                            .take(trim_to)
                            .zip(trimmed_params.iter())
                            .all(|(a, b)| a.ty == b.ty)
                });
                if !already_exists {
                    bindable_ctors.push(TrimmedConstructor {
                        original: ctor,
                        trimmed_param_count: trim_to,
                    });
                }
                break;
            }
        }
    }

    let mut ctor_names: HashMap<String, usize> = HashMap::new();

    bindable_ctors
        .iter()
        .map(|trimmed| {
            let params_slice = &trimmed.original.params[..trimmed.trimmed_param_count];
            let base_suffix = overload_suffix_for_params(params_slice);
            let method_name = if base_suffix.is_empty() {
                "new".to_string()
            } else {
                format!("new{}", base_suffix)
            };

            let count = ctor_names.entry(method_name.clone()).or_insert(0);
            *count += 1;
            let final_method_name = if *count > 1 {
                format!("{}_{}", method_name, count)
            } else {
                method_name
            };
            let impl_method_name = final_method_name.to_snake_case();

            let ffi_suffix = if base_suffix.is_empty() {
                "ctor".to_string()
            } else {
                format!("ctor{}", base_suffix)
            };
            let ffi_fn_name = format!("{}_{}", cpp_name, ffi_suffix);

            let params: Vec<ParamBinding> = params_slice
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, ffi_ctx))
                .collect();

            let cpp_arg_exprs: Vec<String> = params_slice
                .iter()
                .map(|p| param_to_cpp_arg(&p.name, &p.ty))
                .collect();

            ConstructorBinding {
                ffi_fn_name,
                impl_method_name,
                params,
                cpp_arg_exprs,
                doc_comment: trimmed.original.comment.clone(),
                source_line: trimmed.original.source_line,
            }
        })
        .collect()
}

// ── Upcast bindings ─────────────────────────────────────────────────────────

fn compute_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
) -> Vec<UpcastBinding> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = &class.name;

    all_ancestors
        .iter()
        .filter(|base| {
            !protected_destructor_classes.contains(*base)
                && symbol_table.all_class_names.contains(*base)
        })
        .map(|base_class| {
            let ffi_fn_name = format!("{}_as_{}", cpp_name, base_class);
            let ffi_fn_name_mut = format!("{}_mut", ffi_fn_name);

            let base_short_name = if let Some(underscore_pos) = base_class.find('_') {
                type_mapping::safe_short_name(&base_class[underscore_pos + 1..])
            } else {
                type_mapping::safe_short_name(base_class)
            };

            let base_module = if let Some(underscore_pos) = base_class.find('_') {
                base_class[..underscore_pos].to_string()
            } else {
                base_class.clone()
            };

            let impl_method_name = if base_module == class.module {
                format!("as_{}", heck::AsSnakeCase(&base_short_name))
            } else {
                format!("as_{}", heck::AsSnakeCase(base_class.as_str()))
            };

            UpcastBinding {
                base_class: base_class.clone(),
                base_short_name,
                base_module,
                ffi_fn_name,
                ffi_fn_name_mut,
                impl_method_name,
            }
        })
        .collect()
}

// ── Handle upcast bindings ──────────────────────────────────────────────────

fn compute_handle_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
) -> Vec<HandleUpcastBinding> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = &class.name;

    let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));

    all_ancestors
        .iter()
        .filter(|base| {
            if protected_destructor_classes.contains(*base) {
                return false;
            }
            if !handle_able_classes.contains(*base) {
                return false;
            }
            if let Some(base_class) = symbol_table.class_by_name(base) {
                base_class.is_handle_type
            } else {
                false
            }
        })
        .map(|base_class| {
            let base_handle_name = format!("Handle{}", base_class.replace("_", ""));
            let ffi_fn_name =
                format!("{}_to_{}", handle_type_name, base_handle_name);

            HandleUpcastBinding {
                base_handle_name,
                base_class: base_class.clone(),
                ffi_fn_name,
                derived_handle_name: handle_type_name.clone(),
            }
        })
        .collect()
}

// ── Inherited method bindings ───────────────────────────────────────────────

fn compute_inherited_method_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
) -> Vec<InheritedMethodBinding> {
    if class.has_protected_destructor {
        return Vec::new();
    }

    let existing_method_names: HashSet<String> =
        class.methods.iter().map(|m| m.name.clone()).collect();
    let mut seen_methods: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    let ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    for ancestor_name in &ancestors {
        if let Some(ancestor_class) = symbol_table.class_by_name(ancestor_name) {
            let ancestor_methods = symbol_table.included_methods(ancestor_class);

            for resolved_method in ancestor_methods {
                if existing_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if class.all_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if seen_methods.contains(&resolved_method.cpp_name) {
                    continue;
                }

                seen_methods.insert(resolved_method.cpp_name.clone());

                // Skip methods with raw pointers
                let uses_raw_pointers = resolved_method.params.iter().any(|p| {
                    p.ty.rust_ffi_type.contains("*const")
                        || p.ty.rust_ffi_type.contains("*mut")
                })
                    || resolved_method
                        .return_type
                        .as_ref()
                        .map(|rt| {
                            rt.rust_ffi_type.contains("*const")
                                || rt.rust_ffi_type.contains("*mut")
                        })
                        .unwrap_or(false);

                if uses_raw_pointers {
                    continue;
                }

                // Skip methods that reference unknown Handle types or unknown classes
                let uses_unknown_type = resolved_method.params.iter().any(|p| {
                    type_mapping::type_uses_unknown_handle(
                        &p.ty.original,
                        all_class_names,
                        handle_able_classes,
                    )
                }) || resolved_method
                    .return_type
                    .as_ref()
                    .map(|rt| {
                        type_mapping::type_uses_unknown_handle(
                            &rt.original,
                            all_class_names,
                            handle_able_classes,
                        )
                    })
                    .unwrap_or(false);

                if uses_unknown_type {
                    continue;
                }

                // Skip methods that use enum types (not yet handled for inherited methods)
                let uses_enum = resolved_method.params.iter().any(|p| {
                    matches!(&p.ty.original, Type::Class(name) if all_enum_names.contains(name))
                        || matches!(&p.ty.original, Type::ConstRef(inner) if matches!(inner.as_ref(), Type::Class(name) if all_enum_names.contains(name)))
                }) || resolved_method
                    .return_type
                    .as_ref()
                    .map(|rt| {
                        matches!(&rt.original, Type::Class(name) if all_enum_names.contains(name))
                    })
                    .unwrap_or(false);

                if uses_enum {
                    continue;
                }

                let ffi_fn_name = format!(
                    "{}_inherited_{}",
                    class.name, resolved_method.cpp_name
                );
                let impl_method_name =
                    safe_method_name(&resolved_method.cpp_name);

                let params: Vec<ResolvedParamBinding> = resolved_method
                    .params
                    .iter()
                    .map(|p| {
                        let cpp_arg_expr = if p.ty.cpp_type == "const char*" {
                            format!("std::string({}).c_str()", p.name)
                        } else {
                            p.name.clone()
                        };
                        let cpp_param_type = if p.ty.cpp_type == "const char*" {
                            "rust::Str".to_string()
                        } else {
                            p.ty.cpp_type.clone()
                        };
                        ResolvedParamBinding {
                            name: p.name.clone(),
                            rust_name: p.rust_name.clone(),
                            rust_ffi_type: type_to_ffi_full_name(&p.ty.original),
                            rust_reexport_type: unified_type_to_string(&p.ty.original),
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                        }
                    })
                    .collect();

                let return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        ResolvedReturnTypeBinding {
                            rust_ffi_type: return_type_to_ffi_full_name(&rt.original),
                            rust_reexport_type: unified_return_type_to_string(&rt.original),
                            cpp_type: rt.cpp_type.clone(),
                            needs_unique_ptr: rt.needs_unique_ptr,
                        }
                    });

                result.push(InheritedMethodBinding {
                    ffi_fn_name,
                    impl_method_name,
                    is_const: resolved_method.is_const,
                    params,
                    return_type,
                    cpp_method_name: resolved_method.cpp_name.clone(),
                    source_class: ancestor_name.clone(),
                });
            }
        }
    }

    result
}

// ── Top-level function ──────────────────────────────────────────────────────

/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
) -> Vec<ClassBindings> {
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are declared as opaque types in ffi.rs, so they're
    // "known types" for method filtering purposes
    all_class_names.extend(collection_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "unified",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();

    all_classes
        .iter()
        .map(|class| {
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name)
        })
        .collect()
}

// ── Emit functions ──────────────────────────────────────────────────────────

/// Emit C++ wrapper code for a single class from pre-computed ClassBindings.
///
/// Produces the same output as the old generate_unified_class_wrappers()
/// and its 10+ sub-functions, but consumes the pre-computed IR instead
/// of re-deriving decisions.
pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    let cn = &bindings.cpp_name;

    writeln!(output, "// ========================").unwrap();
    writeln!(output, "// {} wrappers", cn).unwrap();
    writeln!(output, "// ========================").unwrap();
    writeln!(output).unwrap();

    // 1. Constructor wrappers
    for ctor in &bindings.constructors {
        let params_cpp: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let params_str = params_cpp.join(", ");
        let args_str = ctor.cpp_arg_exprs.join(", ");

        writeln!(
            output,
            "inline std::unique_ptr<{cn}> {fn_name}({params_str}) {{",
            fn_name = ctor.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return std::make_unique<{cn}>({args_str});"
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 2. ByValueReturn wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueReturn)
    {
        let self_param = if wm.is_const {
            format!("const {cn}& self_")
        } else {
            format!("{cn}& self_")
        };

        let other_params: Vec<String> = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let all_params = std::iter::once(self_param)
            .chain(other_params)
            .collect::<Vec<_>>()
            .join(", ");
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;

        writeln!(
            output,
            "inline std::unique_ptr<{ret_cpp}> {fn_name}({all_params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return std::make_unique<{ret_cpp}>(self_.{method}({args_str}));",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 3. Static method wrappers
    // Note: In the old code, static methods were emitted between by-value and cstring wrappers
    // when you look at the call order in generate_unified_class_wrappers. Actually, the order is:
    // by-value → cstring-param → cstring-return → static. Let me re-check...
    // The actual call order in generate_unified_class_wrappers is:
    //   1. constructor
    //   2. return_by_value
    //   3. c_string_param
    //   4. c_string_return
    //   5. static_method
    //   6. upcast
    //   7. to_owned
    //   8. to_handle
    //   9. handle_upcast
    //   10. inherited_method

    // 3. CStringParam wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}& self")
        } else {
            format!("{cn}& self")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // Determine return behaviour
        let returns_cstring = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type == "const char*")
            .unwrap_or(false);
        let returns_reference = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type.contains('&'))
            .unwrap_or(false);

        if returns_cstring {
            writeln!(
                output,
                "inline rust::String {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return rust::String(self.{method}({args_str}));",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if returns_reference {
            let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;
            writeln!(
                output,
                "inline {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self.{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if wm.return_type.is_none() {
            writeln!(
                output,
                "inline void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self.{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;
            writeln!(
                output,
                "inline {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self.{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4. CStringReturn wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringReturn)
    {
        let self_param = if wm.is_const {
            format!("const {cn}& self")
        } else {
            format!("{cn}& self")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            output,
            "inline rust::String {fn_name}({params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return rust::String(self.{method}({args_str}));",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 5. Static method wrappers
    for sm in &bindings.static_methods {
        let params_str = sm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let args_str = sm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let (ret_type, needs_up) = match &sm.return_type {
            Some(rt) => (rt.cpp_type.clone(), rt.needs_unique_ptr),
            None => ("void".to_string(), false),
        };

        if needs_up {
            writeln!(
                output,
                "inline std::unique_ptr<{ret_type}> {fn_name}({params_str}) {{",
                fn_name = sm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return std::make_unique<{ret_type}>({cn}::{method}({args_str}));",
                method = sm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "inline {ret_type} {fn_name}({params_str}) {{",
                fn_name = sm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return {cn}::{method}({args_str});",
                method = sm.cpp_method_name
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 6. Upcast wrappers
    for up in &bindings.upcasts {
        // Const upcast
        writeln!(
            output,
            "inline const {base}& {fn_name}(const {cn}& self_) {{ return static_cast<const {base}&>(self_); }}",
            base = up.base_class,
            fn_name = up.ffi_fn_name
        )
        .unwrap();
        // Mutable upcast
        writeln!(
            output,
            "inline {base}& {fn_name_mut}({cn}& self_) {{ return static_cast<{base}&>(self_); }}",
            base = up.base_class,
            fn_name_mut = up.ffi_fn_name_mut
        )
        .unwrap();
    }

    // 7. to_owned wrapper
    if bindings.has_to_owned {
        let fn_name = format!("{cn}_to_owned");
        writeln!(
            output,
            "inline std::unique_ptr<{cn}> {fn_name}(const {cn}& self_) {{ return std::make_unique<{cn}>(self_); }}"
        )
        .unwrap();
    }

    // 8. to_handle wrapper
    if bindings.has_to_handle {
        let handle_type = format!("Handle{}", cn.replace("_", ""));
        let fn_name = format!("{cn}_to_handle");
        writeln!(
            output,
            "inline std::unique_ptr<{handle_type}> {fn_name}(std::unique_ptr<{cn}> obj) {{"
        )
        .unwrap();
        writeln!(
            output,
            "    return std::make_unique<{handle_type}>(obj.release());"
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 9. Handle upcast wrappers
    for hup in &bindings.handle_upcasts {
        writeln!(
            output,
            "inline std::unique_ptr<{base_handle}> {fn_name}(const {derived_handle}& self_) {{",
            base_handle = hup.base_handle_name,
            fn_name = hup.ffi_fn_name,
            derived_handle = hup.derived_handle_name
        )
        .unwrap();
        writeln!(
            output,
            "    return std::make_unique<{base_handle}>(self_);",
            base_handle = hup.base_handle_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 10. Inherited method wrappers
    for im in &bindings.inherited_methods {
        let self_param = if im.is_const {
            format!("const {cn}& self")
        } else {
            format!("{cn}& self")
        };
        let other_params: Vec<String> = im
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.name))
            .collect();
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params.join(", "))
        };
        let args_str = im
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let (ret_type_cpp, needs_up) = match &im.return_type {
            Some(rt) => {
                if rt.needs_unique_ptr {
                    (format!("std::unique_ptr<{}>", rt.cpp_type), true)
                } else {
                    (rt.cpp_type.clone(), false)
                }
            }
            None => ("void".to_string(), false),
        };

        writeln!(
            output,
            "inline {ret_type_cpp} {fn_name}({params}) {{",
            fn_name = im.ffi_fn_name
        )
        .unwrap();

        if needs_up {
            writeln!(
                output,
                "    return std::make_unique<{inner_type}>(self.{method}({args_str}));",
                inner_type = im.return_type.as_ref().unwrap().cpp_type,
                method = im.cpp_method_name
            )
            .unwrap();
        } else if im.return_type.is_some() {
            writeln!(
                output,
                "    return self.{method}({args_str});",
                method = im.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    self.{method}({args_str});",
                method = im.cpp_method_name
            )
            .unwrap();
        }

        writeln!(output, "}}").unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
pub fn emit_reexport_class(bindings: &ClassBindings, module_name: &str) -> String {
    let cn = &bindings.cpp_name;
    let short_name = &bindings.short_name;

    let mut output = String::new();

    // Doc comment
    if let Some(ref comment) = bindings.doc_comment {
        for line in comment.lines() {
            output.push_str(&format!("/// {}\n", line.trim()));
        }
    }

    // Type alias re-export
    output.push_str(&format!(
        "pub use crate::ffi::{} as {};\n\n",
        cn, short_name
    ));

    // Build impl methods
    let mut impl_methods: Vec<String> = Vec::new();

    // 1. Constructors
    for ctor in &bindings.constructors {
        let params: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = ctor.params.iter().map(|p| p.rust_name.clone()).collect();

        let doc = format_reexport_doc(&ctor.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}({})\n    }}\n",
            doc,
            ctor.impl_method_name,
            params.join(", "),
            ctor.ffi_fn_name,
            args.join(", ")
        ));
    }

    // 2. Wrapper methods (impl delegates to ffi free functions)
    for wm in &bindings.wrapper_methods {
        let self_param = if wm.is_const {
            "&self".to_string()
        } else {
            "self: std::pin::Pin<&mut Self>".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                wm.params
                    .iter()
                    .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type)),
            )
            .collect();
        let args: Vec<String> = std::iter::once("self".to_string())
            .chain(wm.params.iter().map(|p| p.rust_name.clone()))
            .collect();

        let return_type = wm
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        let doc = format_reexport_doc(&wm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n        crate::ffi::{}({})\n    }}\n",
            doc,
            wm.impl_method_name,
            params.join(", "),
            return_type,
            wm.ffi_fn_name,
            args.join(", ")
        ));
    }

    // 3. Static methods
    for sm in &bindings.static_methods {
        let params: Vec<String> = sm
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = sm.params.iter().map(|p| p.rust_name.clone()).collect();

        let return_type = sm
            .return_type
            .as_ref()
            .map(|rt| {
                let mut ty_str = rt.rust_reexport_type.clone();
                if sm.needs_static_lifetime
                    && ty_str.starts_with('&')
                    && !ty_str.contains("'static")
                {
                    ty_str = ty_str.replacen('&', "&'static ", 1);
                }
                format!(" -> {}", ty_str)
            })
            .unwrap_or_default();

        let doc = format_reexport_doc(&sm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n        crate::ffi::{}({})\n    }}\n",
            doc,
            sm.impl_method_name,
            params.join(", "),
            return_type,
            sm.ffi_fn_name,
            args.join(", ")
        ));
    }

    // 4. Upcast methods
    for up in &bindings.upcasts {
        let ret_type = if up.base_module == module_name {
            up.base_short_name.clone()
        } else {
            let rust_mod = module_graph::module_to_rust_name(&up.base_module);
            format!("crate::{}::{}", rust_mod, up.base_short_name)
        };

        impl_methods.push(format!(
            "    /// Upcast to {}\n    pub fn {}(&self) -> &{} {{\n        crate::ffi::{}(self)\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name
        ));

        impl_methods.push(format!(
            "    /// Upcast to {} (mutable)\n    pub fn {}_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut {}> {{\n        crate::ffi::{}(self)\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name_mut
        ));
    }

    // 5. to_owned
    if bindings.has_to_owned {
        let ffi_fn_name = format!("{}_to_owned", cn);
        impl_methods.push(format!(
            "    /// Clone into a new UniquePtr via copy constructor\n    pub fn to_owned(&self) -> cxx::UniquePtr<Self> {{\n        crate::ffi::{}(self)\n    }}\n",
            ffi_fn_name
        ));
    }

    // 6. to_handle
    if bindings.has_to_handle {
        let ffi_fn_name = format!("{}_to_handle", cn);
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
        impl_methods.push(format!(
            "    /// Wrap in a Handle (reference-counted smart pointer)\n    pub fn to_handle(obj: cxx::UniquePtr<Self>) -> cxx::UniquePtr<crate::ffi::{}> {{\n        crate::ffi::{}(obj)\n    }}\n",
            handle_type_name, ffi_fn_name
        ));
    }

    // 7. Inherited methods (delegates to inherited wrapper free functions)
    for im in &bindings.inherited_methods {
        let self_param = if im.is_const {
            "&self".to_string()
        } else {
            "self: std::pin::Pin<&mut Self>".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                im.params
                    .iter()
                    .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_reexport_type)),
            )
            .collect();
        let args: Vec<String> = std::iter::once("self".to_string())
            .chain(im.params.iter().map(|p| safe_param_name(&p.rust_name)))
            .collect();

        let return_type = im
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        impl_methods.push(format!(
            "    /// Inherited from {source}: {method}()\n    pub fn {}({}){} {{\n        crate::ffi::{}({})\n    }}\n",
            im.impl_method_name,
            params.join(", "),
            return_type,
            im.ffi_fn_name,
            args.join(", "),
            source = im.source_class,
            method = im.cpp_method_name,
        ));
    }

    // Generate the impl block
    if !impl_methods.is_empty() {
        output.push_str(&format!("impl {} {{\n", short_name));
        for method in impl_methods {
            output.push_str(&method);
        }
        output.push_str("}\n\n");
    }

    // 7. Handle type re-export and handle upcast methods
    if bindings.has_to_handle {
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
        // Re-export the handle type so external crates can name it
        output.push_str(&format!(
            "pub use crate::ffi::{};\n\n",
            handle_type_name
        ));

        // Generate handle upcast methods
        if !bindings.handle_upcasts.is_empty() {
            output.push_str(&format!("impl {} {{\n", handle_type_name));
            for hu in &bindings.handle_upcasts {
                // Extract the short name from the base class (e.g. "Geom_Curve" -> "Curve")
                // and snake_case it for the method name
                let base_short = hu.base_class.split('_').skip(1).collect::<Vec<_>>().join("_");
                let method_name = format!("to_handle_{}", base_short.to_snake_case());
                output.push_str(&format!(
                    "    /// Upcast Handle<{cn}> to Handle<{base}>\n    pub fn {method}(&self) -> cxx::UniquePtr<crate::ffi::{base_handle}> {{\n        crate::ffi::{ffi_fn}(self)\n    }}\n",
                    cn = cn,
                    base = hu.base_class,
                    method = method_name,
                    base_handle = hu.base_handle_name,
                    ffi_fn = hu.ffi_fn_name,
                ));
            }
            output.push_str("}\n\n");
        }
    }

    output
}

/// Format an optional doc comment for re-export impl methods (indented with 4 spaces).
fn format_reexport_doc(doc: &Option<String>) -> String {
    match doc {
        Some(comment) => {
            let formatted = comment
                .lines()
                .map(|line| format!("    /// {}", line.trim()))
                .collect::<Vec<_>>()
                .join("\n");
            format!("{}\n", formatted)
        }
        None => String::new(),
    }
}

// ── FFI TokenStream emit ────────────────────────────────────────────────────

/// Format source attribution for doc comments (same as rust.rs format_source_attribution).
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Emit ffi.rs code for a single class from pre-computed ClassBindings.
///
/// Returns a string fragment to be inserted inside `unsafe extern "C++" { ... }`.
/// All declarations are indented with 8 spaces to match the ffi.rs layout.
pub fn emit_ffi_class(bindings: &ClassBindings) -> String {
    let cn = &bindings.cpp_name;
    let mut out = String::new();

    // Section header
    writeln!(out, "        /// ======================== {} ========================", cn).unwrap();

    // Type declaration with doc comment
    let source_attr = format_source_attribution(&bindings.source_header, bindings.source_line, cn);
    emit_ffi_doc(&mut out, &source_attr, &bindings.doc_comment);
    writeln!(out, "        type {};", cn).unwrap();

    // ── Constructors ────────────────────────────────────────────────────
    for ctor in &bindings.constructors {
        let source = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        emit_ffi_doc(&mut out, &source, &ctor.doc_comment);

        let params_str = format_params(&ctor.params);
        writeln!(out, "        fn {}({}) -> UniquePtr<{}>;", ctor.ffi_fn_name, params_str, cn).unwrap();
    }

    // ── Direct methods (CXX self-receiver) ──────────────────────────────
    for dm in &bindings.direct_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        emit_ffi_doc(&mut out, &source, &dm.doc_comment);
        writeln!(out, "        #[cxx_name = \"{}\"]", dm.cxx_name).unwrap();

        let receiver = if dm.is_const {
            format!("self: &{}", cn)
        } else {
            format!("self: Pin<&mut {}>", cn)
        };
        let params_str = format_params(&dm.params);
        let all_params = if params_str.is_empty() {
            receiver
        } else {
            format!("{}, {}", receiver, params_str)
        };
        let ret = format_return_type(&dm.return_type);
        writeln!(out, "        fn {}({}){};", dm.rust_name, all_params, ret).unwrap();
    }

    // ── Wrapper methods (free functions with self_ parameter) ────────────
    for wm in &bindings.wrapper_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        emit_ffi_doc(&mut out, &source, &wm.doc_comment);

        let self_param = if wm.is_const {
            format!("self_: &{}", cn)
        } else {
            format!("self_: Pin<&mut {}>", cn)
        };
        let params_str = format_params(&wm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&wm.return_type);
        writeln!(out, "        fn {}({}){};", wm.ffi_fn_name, all_params, ret).unwrap();
    }

    // ── Static methods ──────────────────────────────────────────────────
    for sm in &bindings.static_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            sm.source_line,
            &format!("{}::{}()", cn, sm.cpp_method_name),
        );
        emit_ffi_doc(&mut out, &source, &sm.doc_comment);

        let params_str = format_params(&sm.params);
        let ret = if let Some(ref rt) = sm.return_type {
            let mut ty_str = rt.rust_ffi_type.clone();
            // Static methods returning references need 'static lifetime
            if sm.needs_static_lifetime
                && ty_str.starts_with('&')
                && !ty_str.contains("'static")
            {
                ty_str = ty_str.replacen('&', "&'static ", 1);
            }
            format!(" -> {}", ty_str)
        } else {
            String::new()
        };
        writeln!(out, "        fn {}({}){};", sm.ffi_fn_name, params_str, ret).unwrap();
    }

    // ── Upcasts ─────────────────────────────────────────────────────────
    for up in &bindings.upcasts {
        writeln!(out, "        /// Upcast {} to {}", cn, up.base_class).unwrap();
        writeln!(out, "        fn {}(self_: &{}) -> &{};", up.ffi_fn_name, cn, up.base_class).unwrap();
        writeln!(out, "        /// Upcast {} to {} (mutable)", cn, up.base_class).unwrap();
        writeln!(out, "        fn {}(self_: Pin<&mut {}>) -> Pin<&mut {}>;", up.ffi_fn_name_mut, cn, up.base_class).unwrap();
    }

    // ── to_owned ────────────────────────────────────────────────────────
    if bindings.has_to_owned {
        writeln!(out, "        /// Clone into a new UniquePtr via copy constructor").unwrap();
        writeln!(out, "        fn {}_to_owned(self_: &{}) -> UniquePtr<{}>;", cn, cn, cn).unwrap();
    }

    // ── to_handle ───────────────────────────────────────────────────────
    if bindings.has_to_handle {
        let handle_type_name = format!("Handle{}", cn.replace('_', ""));
        writeln!(out, "        /// Wrap {} in a Handle", cn).unwrap();
        writeln!(out, "        fn {}_to_handle(obj: UniquePtr<{}>) -> UniquePtr<{}>;", cn, cn, handle_type_name).unwrap();
    }

    // ── Handle upcasts ──────────────────────────────────────────────────
    for hu in &bindings.handle_upcasts {
        writeln!(out, "        /// Upcast Handle<{}> to Handle<{}>", cn, hu.base_class).unwrap();
        writeln!(out, "        fn {}(self_: &{}) -> UniquePtr<{}>;", hu.ffi_fn_name, hu.derived_handle_name, hu.base_handle_name).unwrap();
    }

    // ── Inherited methods (free functions with self_ parameter) ─────────
    for im in &bindings.inherited_methods {
        writeln!(out, "        /// Inherited from {}: {}()", im.source_class, im.cpp_method_name).unwrap();

        // Detect if we need explicit lifetime annotations.
        // Inherited methods are emitted as free functions (not using `self:` syntax),
        // so Rust's lifetime elision fails when there are 2+ input reference lifetimes
        // and the return type is a reference. We tie the return lifetime to self_.
        let returns_ref = im.return_type.as_ref()
            .map(|rt| rt.rust_ffi_type.starts_with('&') || rt.rust_ffi_type.starts_with("Pin<&"))
            .unwrap_or(false);
        let has_ref_params = im.params.iter().any(|p| {
            p.rust_ffi_type.starts_with('&') || p.rust_ffi_type.starts_with("Pin<&")
        });
        let needs_lifetime = returns_ref && has_ref_params;

        let lifetime_generic = if needs_lifetime { "<'a>" } else { "" };

        let self_param = if im.is_const {
            if needs_lifetime {
                format!("self_: &'a {}", cn)
            } else {
                format!("self_: &{}", cn)
            }
        } else {
            if needs_lifetime {
                format!("self_: Pin<&'a mut {}>", cn)
            } else {
                format!("self_: Pin<&mut {}>", cn)
            }
        };
        let params_str: String = im
            .params
            .iter()
            .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_ffi_type))
            .collect::<Vec<_>>()
            .join(", ");
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = match &im.return_type {
            Some(rt) => {
                if needs_lifetime {
                    // Insert 'a after the leading & in the return type
                    let annotated = if rt.rust_ffi_type.starts_with("Pin<&mut ") {
                        rt.rust_ffi_type.replacen("Pin<&mut ", "Pin<&'a mut ", 1)
                    } else if rt.rust_ffi_type.starts_with("Pin<&") {
                        rt.rust_ffi_type.replacen("Pin<&", "Pin<&'a ", 1)
                    } else if rt.rust_ffi_type.starts_with("&mut ") {
                        rt.rust_ffi_type.replacen("&mut ", "&'a mut ", 1)
                    } else if rt.rust_ffi_type.starts_with('&') {
                        rt.rust_ffi_type.replacen('&', "&'a ", 1)
                    } else {
                        rt.rust_ffi_type.clone()
                    };
                    format!(" -> {}", annotated)
                } else {
                    format!(" -> {}", rt.rust_ffi_type)
                }
            }
            None => String::new(),
        };
        writeln!(out, "        fn {}{}({}){};", im.ffi_fn_name, lifetime_generic, all_params, ret).unwrap();
    }

    out
}

/// Format parameter list for ffi.rs declarations.
fn format_params(params: &[ParamBinding]) -> String {
    params
        .iter()
        .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_ffi_type))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Format optional return type for ffi.rs declarations.
fn format_return_type(rt: &Option<ReturnTypeBinding>) -> String {
    match rt {
        Some(rt) => format!(" -> {}", rt.rust_ffi_type),
        None => String::new(),
    }
}

/// Emit a doc comment block for ffi.rs (indented 8 spaces).
fn emit_ffi_doc(out: &mut String, source: &str, comment: &Option<String>) {
    writeln!(out, "        /// {}", source).unwrap();
    if let Some(ref comment) = comment {
        writeln!(out, "        ///").unwrap();
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(out, "        ///").unwrap();
            } else {
                writeln!(out, "        /// {}", trimmed).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: compute_class_bindings shouldn't panic on a minimal ParsedClass
    #[test]
    fn test_compute_bindings_empty_class() {
        let class = ParsedClass {
            name: "gp_Pnt".to_string(),
            module: "gp".to_string(),
            comment: None,
            source_header: "gp_Pnt.hxx".to_string(),
            source_line: Some(1),
            constructors: Vec::new(),
            methods: Vec::new(),
            static_methods: Vec::new(),
            all_method_names: HashSet::new(),
            is_handle_type: false,
            base_classes: Vec::new(),
            has_protected_destructor: false,
            is_abstract: false,
            pure_virtual_methods: HashSet::new(),
            has_explicit_constructors: false,
        };

        let all_class_names: HashSet<String> = ["gp_Pnt".to_string()].into();
        let all_enum_names: HashSet<String> = HashSet::new();
        let handle_able_classes: HashSet<String> = HashSet::new();

        let ffi_ctx = TypeContext {
            current_module: "unified",
            module_classes: &all_class_names,
            all_enums: &all_enum_names,
            all_classes: &all_class_names,
            handle_able_classes: Some(&handle_able_classes),
        };

        // Create a minimal SymbolTable
        let symbol_table = SymbolTable {
            classes: HashMap::new(),
            constructors: HashMap::new(),
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            functions: HashMap::new(),
            enums: HashMap::new(),
            classes_by_module: HashMap::new(),
            functions_by_module: HashMap::new(),
            enums_by_module: HashMap::new(),
            all_enum_names: HashSet::new(),
            all_class_names: ["gp_Pnt".to_string()].into(),
            cross_module_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("gp_Pnt".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
        );

        assert_eq!(bindings.cpp_name, "gp_Pnt");
        assert_eq!(bindings.short_name, "Pnt");
        assert_eq!(bindings.module, "gp");
        // Non-abstract class with no explicit constructors gets a synthetic default constructor
        assert_eq!(bindings.constructors.len(), 1);
        assert_eq!(bindings.constructors[0].impl_method_name, "new");
        assert!(bindings.direct_methods.is_empty());
        assert!(bindings.wrapper_methods.is_empty());
        assert!(bindings.static_methods.is_empty());
        assert!(!bindings.has_to_handle);
        // gp module is copyable, and class is not abstract/protected
        assert!(bindings.has_to_owned);
    }

    /// Test that abstract classes don't get constructors
    #[test]
    fn test_abstract_class_no_constructors() {
        let class = ParsedClass {
            name: "Geom_Curve".to_string(),
            module: "Geom".to_string(),
            comment: None,
            source_header: "Geom_Curve.hxx".to_string(),
            source_line: Some(1),
            constructors: vec![Constructor {
                comment: None,
                params: Vec::new(),
                source_line: Some(10),
            }],
            methods: Vec::new(),
            static_methods: Vec::new(),
            all_method_names: HashSet::new(),
            is_handle_type: true,
            base_classes: Vec::new(),
            has_protected_destructor: false,
            is_abstract: true,
            pure_virtual_methods: HashSet::new(),
            has_explicit_constructors: true,
        };

        let all_class_names: HashSet<String> =
            ["Geom_Curve".to_string()].into();
        let all_enum_names: HashSet<String> = HashSet::new();
        let handle_able_classes: HashSet<String> =
            ["Geom_Curve".to_string()].into();

        let ffi_ctx = TypeContext {
            current_module: "unified",
            module_classes: &all_class_names,
            all_enums: &all_enum_names,
            all_classes: &all_class_names,
            handle_able_classes: Some(&handle_able_classes),
        };

        let symbol_table = SymbolTable {
            classes: HashMap::new(),
            constructors: HashMap::new(),
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            functions: HashMap::new(),
            enums: HashMap::new(),
            classes_by_module: HashMap::new(),
            functions_by_module: HashMap::new(),
            enums_by_module: HashMap::new(),
            all_enum_names: HashSet::new(),
            all_class_names: ["Geom_Curve".to_string()].into(),
            cross_module_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("Geom_Curve".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
        );

        assert!(bindings.constructors.is_empty());
        assert!(!bindings.has_to_owned);
        assert!(!bindings.has_to_handle); // abstract
    }

    /// Test overload suffix computation for wrapper methods
    #[test]
    fn test_wrapper_method_overload_suffix() {
        use crate::model::{Method, Param, Type};

        let methods = vec![
            Method {
                name: "Mirror".to_string(),
                comment: None,
                is_const: false,
                params: vec![Param {
                    name: "P".to_string(),
                    ty: Type::ConstRef(Box::new(Type::Class("gp_Pnt".to_string()))),
                    has_default: false,
                }],
                return_type: None,
                source_line: Some(10),
            },
            Method {
                name: "Mirror".to_string(),
                comment: None,
                is_const: false,
                params: vec![Param {
                    name: "A1".to_string(),
                    ty: Type::ConstRef(Box::new(Type::Class("gp_Ax1".to_string()))),
                    has_default: false,
                }],
                return_type: None,
                source_line: Some(20),
            },
        ];

        let method_refs: Vec<&Method> = methods.iter().collect();
        let names = compute_wrapper_method_names(&method_refs);

        // Should get different suffixes based on param types
        assert_ne!(names[0], names[1]);
        assert!(names[0].starts_with("mirror"));
        assert!(names[1].starts_with("mirror"));
    }

    /// Test static method 3-level conflict resolution
    #[test]
    fn test_static_method_conflict_resolution() {
        use crate::model::{StaticMethod, Type};

        let methods = vec![StaticMethod {
            name: "Origin".to_string(),
            comment: None,
            params: Vec::new(),
            return_type: Some(Type::Class("gp_Pnt".to_string())),
            source_line: Some(10),
        }];

        let method_refs: Vec<&StaticMethod> = methods.iter().collect();

        // Case 1: No conflicts
        let reserved: HashSet<String> = HashSet::new();
        let instance: HashSet<String> = HashSet::new();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin"); // ffi base
        assert_eq!(names[0].1, "origin"); // impl name

        // Case 2: Conflict with wrapper reserved name
        let reserved: HashSet<String> = ["gp_Pnt_origin".to_string()].into();
        let instance: HashSet<String> = HashSet::new();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin_static"); // ffi base adds _static
        assert_eq!(names[0].1, "origin_static"); // impl follows

        // Case 3: Conflict with instance method names
        let reserved: HashSet<String> = HashSet::new();
        let instance: HashSet<String> = ["origin".to_string()].into();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin"); // ffi base stays
        assert_eq!(names[0].1, "origin_static"); // impl adds _static
    }
}
