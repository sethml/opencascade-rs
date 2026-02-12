//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

#![allow(dead_code)] // Some functions reserved for Phase B2 emit functions

use crate::model::{Constructor, Method, ParsedClass, StaticMethod, Type};
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
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
    pub rust_ffi_type: String,
    pub cpp_type: String,
    pub cpp_arg_expr: String,
}

/// A resolved return type binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedReturnTypeBinding {
    pub rust_ffi_type: String,
    pub cpp_type: String,
    pub needs_unique_ptr: bool,
}

// ── Helper functions ────────────────────────────────────────────────────────

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

/// Check if a method returns by value AND is NOT already handled as a by-value wrapper
/// (i.e., it's a c_string param method whose return is also by-value class/handle,
/// meaning it's already covered by the by-value wrapper)
fn is_cstring_already_wrapped_by_value(method: &Method, all_enums: &HashSet<String>) -> bool {
    if method.returns_by_value() {
        let return_type_name = match &method.return_type {
            Some(Type::Class(name)) => Some(name.as_str()),
            _ => None,
        };
        let is_enum = return_type_name
            .map(|n| all_enums.contains(n))
            .unwrap_or(false);
        !is_enum
    } else {
        false
    }
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
    let rust_name = safe_param_name(name);
    let mapped = map_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = unified_type_to_string(ty);
    let cpp_type = type_to_cpp_param(ty);
    let cpp_arg_expr = param_to_cpp_arg(name, ty);

    ParamBinding {
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
) -> ClassBindings {
    let cpp_name = &class.name;
    let all_enum_names = ffi_ctx.all_enums;

    // ── Constructors ────────────────────────────────────────────────────
    let constructors = if !class.is_abstract && !class.has_protected_destructor {
        compute_constructor_bindings(class, ffi_ctx, handle_able_classes)
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
        && !class.is_abstract;

    // ── to_handle ───────────────────────────────────────────────────────
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !class.is_abstract;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_to_handle {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };

    // ── Inherited methods ───────────────────────────────────────────────
    let inherited_methods =
        compute_inherited_method_bindings(class, symbol_table);

    ClassBindings {
        cpp_name: cpp_name.clone(),
        short_name: class.short_name().to_string(),
        module: class.module.clone(),
        is_abstract: class.is_abstract,
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

fn compute_constructor_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    handle_able_classes: &HashSet<String>,
) -> Vec<ConstructorBinding> {
    let cpp_name = &class.name;
    let all_enum_names = ffi_ctx.all_enums;

    let bindable_ctors: Vec<&Constructor> = class
        .constructors
        .iter()
        .filter(|c| is_constructor_bindable(c, all_enum_names, handle_able_classes))
        .collect();

    let mut ctor_names: HashMap<String, usize> = HashMap::new();

    bindable_ctors
        .iter()
        .map(|ctor| {
            let base_suffix = ctor.overload_suffix();
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

            let params: Vec<ParamBinding> = ctor
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, ffi_ctx))
                .collect();

            let cpp_arg_exprs: Vec<String> = ctor
                .params
                .iter()
                .map(|p| param_to_cpp_arg(&p.name, &p.ty))
                .collect();

            ConstructorBinding {
                ffi_fn_name,
                impl_method_name,
                params,
                cpp_arg_exprs,
                doc_comment: ctor.comment.clone(),
                source_line: ctor.source_line,
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
                            rust_ffi_type: p.ty.rust_ffi_type.clone(),
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                        }
                    })
                    .collect();

                let return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        ResolvedReturnTypeBinding {
                            rust_ffi_type: rt.rust_ffi_type.clone(),
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
) -> Vec<ClassBindings> {
    let all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
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

    all_classes
        .iter()
        .map(|class| {
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes)
        })
        .collect()
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

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
        );

        assert_eq!(bindings.cpp_name, "gp_Pnt");
        assert_eq!(bindings.short_name, "Pnt");
        assert_eq!(bindings.module, "gp");
        assert!(bindings.constructors.is_empty());
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

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
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
