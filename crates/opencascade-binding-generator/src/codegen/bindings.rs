//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type, is_void_type_name, is_opaque_class_name};
use crate::module_graph;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type, map_return_type_in_context, map_type_in_context, map_type_to_rust, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

/// Rust keywords that need suffix escaping (FFI doesn't support raw identifiers).
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
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
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
    /// Whether Handle_get/get_mut should be generated (true for all handle types, including abstract)
    pub has_handle_get: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub handle_downcasts: Vec<HandleDowncastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
    /// Whether this class is a POD struct that can be represented with real fields
    pub is_pod_struct: bool,
    /// Fields for POD structs (only populated when is_pod_struct is true)
    pub pod_fields: Vec<PodFieldBinding>,
    /// Symbols that were skipped during binding generation, with reasons
    pub skipped_symbols: Vec<SkippedSymbol>,
}

/// A symbol that was skipped during binding generation.
#[derive(Debug, Clone)]
pub struct SkippedSymbol {
    /// Kind of symbol ("constructor", "method", "static_method", "function")
    pub kind: &'static str,
    /// Rust module this symbol belongs to
    pub module: String,
    /// C++ name of the symbol
    pub cpp_name: String,
    /// Source header
    pub source_header: String,
    /// Source line number
    pub source_line: Option<u32>,
    /// Documentation comment from C++ header
    pub doc_comment: Option<String>,
    /// Human-readable reason why the symbol was skipped
    pub skip_reason: String,
    /// Best-guess Rust declaration (commented out in output)
    pub stub_rust_decl: String,
}

/// A single field in a POD struct.
#[derive(Debug, Clone)]
pub struct PodFieldBinding {
    /// Field name in Rust (snake_case)
    pub rust_name: String,
    /// Field name in C++ (original)
    pub cpp_name: String,
    /// Rust type string, e.g. "bool" or "f64"
    pub rust_type: String,
    /// Array size if this is a fixed-size array field
    pub array_size: Option<usize>,
    /// Byte offset for offsetof check
    pub offset_index: usize,
    /// Doc comment
    pub doc_comment: Option<String>,
}

/// A constructor that will have a C++ wrapper (std::make_unique),
/// or a Rust-only convenience wrapper that delegates to a full-argument constructor.
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
    /// If this is a convenience wrapper (fewer params with defaults filled in),
    /// contains info about the full-argument constructor it delegates to.
    /// When set, no ffi.rs or wrappers.hxx entry is generated — only a Rust-only
    /// method in the module re-export that calls the full-argument version.
    pub convenience_of: Option<ConvenienceInfo>,
    /// Whether this constructor should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// Info for a convenience constructor that delegates to a full-argument version.
#[derive(Debug, Clone)]
pub struct ConvenienceInfo {
    /// The impl_method_name of the full-argument constructor to call
    pub full_method_name: String,
    /// Rust expressions for the default values of the trimmed trailing params,
    /// in order. E.g. ["false", "false"] for two defaulted bool params.
    pub default_exprs: Vec<String>,
}

/// A method bound as a direct extern "C" wrapper (self receiver, no wrapper needed).
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
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
    /// Whether this returns a reference and has reference params (ambiguous lifetime)
    pub unsafe_lifetime: bool,
}

/// What kind of C++ wrapper is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrapperKind {
    /// Returns a class or handle by value → new T(...) wrapper
    ByValueReturn,
    /// Has const char* parameters → const char* pass-through
    CStringParam,
    /// Returns const char* → const char* pass-through
    CStringReturn,
    /// Uses enum types (params and/or return) → int32_t/static_cast wrapper
    EnumConversion,
    /// Has by-value class/handle parameters → pointer dereference wrapper
    ByValueParam,
    /// Const method returns &mut T — wrapper takes non-const self
    ConstMutReturnFix,
    /// Has &mut enum output parameters → local variable + writeback wrapper
    MutRefEnumParam,
    /// Simple pass-through wrapper (primitives, void, etc.)
    Simple,
}

/// A method that needs a C++ wrapper function.
#[derive(Debug, Clone)]
pub struct WrapperMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_mirrored_pnt")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ from ffi base if name conflict)
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
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
    /// Whether this returns a reference and has reference params (ambiguous lifetime)
    pub unsafe_lifetime: bool,
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
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// An upcast binding (Derived → Base).
#[derive(Debug, Clone)]
pub struct UpcastBinding {
    /// Base class FFI-safe name ("::" replaced with "_"), e.g. "Geom_Curve"
    pub base_class: String,
    /// Base class C++ qualified name (uses "::"), e.g. "Geom_Curve" or "Outer::Inner"
    pub base_class_cpp: String,
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
    /// Base class module, e.g. "Geom"
    pub base_module: String,
    /// FFI function name
    pub ffi_fn_name: String,
    /// Derived handle type name, e.g. "HandleGeomBSplineCurve"
    pub derived_handle_name: String,
}

/// A Handle downcast binding (Handle<Base> → Option<Handle<Derived>> via DownCast).
#[derive(Debug, Clone)]
pub struct HandleDowncastBinding {
    /// Derived handle type name, e.g. "HandleGeomPlane"
    pub derived_handle_name: String,
    /// Derived class C++ name, e.g. "Geom_Plane"
    pub derived_class: String,
    /// Derived class module, e.g. "Geom"
    pub derived_module: String,
    /// Base handle type name, e.g. "HandleGeomSurface"
    pub base_handle_name: String,
    /// FFI function name, e.g. "HandleGeomSurface_downcast_to_HandleGeomPlane"
    pub ffi_fn_name: String,
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
    /// Source header file for the ancestor method
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
    /// Whether this returns a reference and has reference params (ambiguous lifetime)
    pub unsafe_lifetime: bool,
}

/// A parameter binding with info for all three output targets.
#[derive(Debug, Clone)]
pub struct ParamBinding {
    /// Original C++ parameter name (for use in C++ wrapper declarations)
    pub cpp_name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    /// Type as it appears in ffi.rs (e.g. "f64", "&gp_Pnt", "*mut gp_Pnt")
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt" or enum type)
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx parameter (e.g. "Standard_Real", "const gp_Pnt&")
    pub cpp_type: String,
    /// C++ argument expression when calling OCCT (e.g. param name, or "std::string(x).c_str()")
    pub cpp_arg_expr: String,
    /// If this is a value enum param, the qualified Rust enum type (e.g. "crate::top_abs::Orientation")
    pub enum_rust_type: Option<String>,
    /// If this is a &mut enum output param, the C++ enum name for local var + writeback pattern
    pub mut_ref_enum_cpp_name: Option<String>,
    /// If this is a &mut enum output param, the qualified Rust enum type (e.g. "crate::geom_abs::Shape")
    pub mut_ref_enum_rust_type: Option<String>,
    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)
    pub is_nullable_ptr: bool,
    /// If this is a non-nullable class pointer param (const T* / T* where T is a known class)
    pub is_class_ptr: bool,
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
    /// If this is an enum return, the original C++ enum name (for static_cast)
    pub enum_cpp_name: Option<String>,
    /// If this is a value enum return, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a raw pointer return to a known class type (const T* / T*)
    pub is_class_ptr_return: bool,
}

/// A resolved parameter binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedParamBinding {
    pub name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt" or enum type)
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub cpp_arg_expr: String,
    /// If this is a value enum param, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a &mut enum output param, the C++ enum name (for preamble/postamble)
    pub mut_ref_enum_cpp_name: Option<String>,
    /// If this is a &mut enum output param, the qualified Rust enum type
    pub mut_ref_enum_rust_type: Option<String>,
    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)
    pub is_nullable_ptr: bool,
    /// If this is a non-nullable class pointer param (const T* / T* where T is a known class)
    pub is_class_ptr: bool,
}

/// A resolved return type binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedReturnTypeBinding {
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub needs_unique_ptr: bool,
    /// If this is an enum return, the original C++ enum name (for static_cast)
    pub enum_cpp_name: Option<String>,
    /// If this is a value enum return, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a raw pointer return to a known class type (const T* / T*)
    pub is_class_ptr_return: bool,
}

/// Pre-computed binding decisions for a single free function.
/// Parallel to `ClassBindings` — all naming, filtering, type mapping, and
/// conflict resolution happens once during `compute_all_function_bindings()`.
#[derive(Debug, Clone)]
pub struct FunctionBinding {
    /// Rust FFI name (short, used as module re-export alias, e.g. "precision_real")
    pub rust_ffi_name: String,
    /// C++ wrapper function name (the extern "C" symbol, e.g. "BRepBuilderAPI_precision_real")
    pub cpp_wrapper_name: String,
    /// C++ namespace (e.g. "BRepBuilderAPI")
    pub namespace: String,
    /// C++ short function name (e.g. "Precision")
    pub short_name: String,
    /// Rust module name (e.g. "b_rep_builder_api")
    pub module: String,
    /// Parameters with pre-computed type strings for ffi.rs, re-exports, and wrappers.hxx
    pub params: Vec<ParamBinding>,
    /// Return type with pre-computed type strings (None for void)
    pub return_type: Option<ReturnTypeBinding>,
    /// Source header file (e.g. "BRepBuilderAPI.hxx")
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Documentation comment
    pub doc_comment: Option<String>,
    /// C++ headers needed for this function's parameter and return types
    pub cpp_headers: Vec<String>,
    /// Whether this function should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
    /// Whether this function has ambiguous lifetime (returns ref with 2+ ref params)
    pub unsafe_lifetime: bool,
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
    // In Rust, function parameters are patterns, so tuple variant names from
    // the prelude (Ok, Err, Some, None) cannot be used as parameter names —
    // they conflict as enum variant patterns. Append '_' to avoid E0530.
    const RESERVED_PATTERNS: &[&str] = &["Ok", "Err", "Some", "None"];
    if RUST_KEYWORDS.contains(&name) || RESERVED_PATTERNS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────


fn param_uses_unknown_handle(ty: &Type, handle_able_classes: &HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            param_uses_unknown_handle(inner, handle_able_classes)
        }
        _ => false,
    }
}

/// Check if a type uses an unknown class/handle given the TypeContext.
/// Enum types (Type::Class that are in all_enums) are known — they map to i32.
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    // Enum types are known (mapped to i32), so skip them
    match ty {
        Type::Class(name) if ctx.all_enums.contains(name) => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
            }
        }
        _ => {}
    }
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a method has by-value class or handle parameters (not enums).
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && is_opaque_class_name(name),
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
fn has_mut_ref_enum_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| {
        if let Type::MutRef(inner) = &p.ty {
            if let Type::Class(name) = inner.as_ref() {
                return all_enums.contains(name);
            }
        }
        false
    })
}

/// Determine if a method needs a C++ wrapper function
fn needs_wrapper_function(_method: &Method, _all_enums: &HashSet<String>) -> bool {
    // With extern "C" FFI, all methods need C++ wrapper functions
    true
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

    if has_mut_ref_enum_params(&method.params, all_enums) {
        WrapperKind::MutRefEnumParam
    } else if returns_by_value {
        WrapperKind::ByValueReturn
    } else if has_cstring_param {
        WrapperKind::CStringParam
    } else if returns_cstring {
        WrapperKind::CStringReturn
    } else if resolver::has_const_mut_return_mismatch(method) {
        WrapperKind::ConstMutReturnFix
    } else if resolver::method_uses_enum(method, all_enums) {
        WrapperKind::EnumConversion
    } else if has_by_value_class_or_handle_params(&method.params, all_enums) {
        WrapperKind::ByValueParam
    } else {
        // Simple method wrapper (primitives, void, etc.)
        WrapperKind::Simple
    }
}


/// Convert a parameter to C++ argument expression for extern "C" wrappers.
/// Dereferences pointers to match C++ method signatures (which take references).
fn param_to_cpp_extern_c_arg(param_name: &str, ty: &Type) -> String {
    match ty {
        Type::ConstRef(_) | Type::MutRef(_) => format!("*{}", param_name),
        _ => param_name.to_string(),
    }
}

/// Context for resolving C++ class names to their public re-exported Rust type
/// paths instead of raw `crate::ffi::` paths.
pub struct ReexportTypeContext<'a> {
    /// Maps C++ class name (original, may contain ::) → (rust_module_name, short_name)
    pub class_public_info: &'a HashMap<String, (String, String)>,
    /// The Rust module name of the class/function currently being generated
    pub current_module_rust: String,
}

impl<'a> ReexportTypeContext<'a> {
    fn resolve_class(&self, cpp_name: &str) -> String {
        if let Some((module_rust, short)) = self.class_public_info.get(cpp_name) {
            if *module_rust == self.current_module_rust {
                short.clone()
            } else {
                format!("crate::{}::{}", module_rust, short)
            }
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(cpp_name))
        }
    }
}

/// Build the class_public_info map from a list of parsed classes.
/// Maps C++ class name → (rust_module_name, short_name).
pub(crate) fn build_class_public_info(all_classes: &[&ParsedClass]) -> HashMap<String, (String, String)> {
    all_classes
        .iter()
        .map(|c| {
            let ffi_name = c.name.replace("::", "_");
            let module_rust = crate::module_graph::module_to_rust_name(&c.module);
            let short = crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(&ffi_name, &c.module));
            (c.name.clone(), (module_rust, short))
        })
        .collect()
}

/// Convert a Type to Rust type string for re-export files
fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I16 => "i16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::CHAR16 => "u16".to_string(),
        Type::U8 => "u8".to_string(),
        Type::I8 => "i8".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::ConstRef(inner) => format!("&{}", type_to_rust_string(inner, reexport_ctx)),
        Type::MutRef(inner) => {
            format!("&mut {}", type_to_rust_string(inner, reexport_ctx))
        }
        Type::RValueRef(_) => "()".to_string(),
        Type::ConstPtr(inner) => {
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "&str".to_string()
            } else {
                format!("*const {}", type_to_rust_string(inner, reexport_ctx))
            }
        }
        Type::MutPtr(inner) => format!("*mut {}", type_to_rust_string(inner, reexport_ctx)),
    }
}

/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if is_opaque_class_name(name) => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "std::string::String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if !is_void_type_name(name)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
            }
        }
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if is_opaque_class_name(name)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&mut {}>", resolved)
            } else {
                unreachable!()
            }
        }
        _ => type_to_rust_string(ty, reexport_ctx),
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────

/// Describe which types in a method's params/return are unbindable.
fn describe_unbindable_types_method(method: &Method) -> String {
    let mut parts = Vec::new();
    for p in &method.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if let Some(ref ret) = method.return_type {
        if ret.is_unbindable() && ret.class_ptr_inner_name().is_none() {
            parts.push(format!("return: {}", describe_unbindable_reason(ret)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe which types in a constructor's params are unbindable.
fn describe_unbindable_types_ctor(ctor: &Constructor) -> String {
    let mut parts = Vec::new();
    for p in &ctor.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe which types in a static method's params/return are unbindable.
fn describe_unbindable_types_static(method: &StaticMethod) -> String {
    let mut parts = Vec::new();
    for p in &method.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if let Some(ref ret) = method.return_type {
        if ret.is_unbindable() {
            parts.push(format!("return: {}", describe_unbindable_reason(ret)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe why a specific type is unbindable.
fn describe_unbindable_reason(ty: &Type) -> String {
    if ty.is_stream() { return format!("stream type ({})", ty.to_cpp_string()); }
    if ty.is_void_ptr() { return format!("void pointer ({})", ty.to_cpp_string()); }
    if ty.is_array() { return format!("C-style array ({})", ty.to_cpp_string()); }
    if ty.is_raw_ptr() { return format!("raw pointer ({})", ty.to_cpp_string()); }
    if ty.is_rvalue_ref() { return format!("rvalue reference ({})", ty.to_cpp_string()); }
    format!("unresolved template type ({})", ty.to_cpp_string())
}

/// Generate a best-guess stub Rust declaration for a skipped method.
fn generate_method_stub(_class_name: &str, method: &Method) -> String {
    let self_param = if method.is_const { "&self" } else { "&mut self" };
    let params: Vec<String> = std::iter::once(self_param.to_string())
        .chain(method.params.iter().map(|p| format!("{}: {}", safe_param_name(&p.name), p.ty.to_rust_type_string_safe())))
        .collect();
    let ret = method.return_type.as_ref()
        .map(|ty| format!(" -> {}", stub_return_type_string(ty)))
        .unwrap_or_default();
    format!("pub fn {}({}){};", safe_method_name(&method.name), params.join(", "), ret)
}

/// Generate a best-guess stub Rust declaration for a skipped constructor.
fn generate_ctor_stub(_class_name: &str, ctor: &Constructor) -> String {
    let params: Vec<String> = ctor.params.iter()
        .map(|p| format!("{}: {}", safe_param_name(&p.name), p.ty.to_rust_type_string_safe()))
        .collect();
    let suffix = ctor.overload_suffix();
    let method_name = if suffix.is_empty() { "new".to_string() } else { format!("new{}", suffix) };
    format!("pub fn {}({}) -> OwnedPtr<Self>;", method_name, params.join(", "))
}

/// Generate a best-guess stub Rust declaration for a skipped static method.
fn generate_static_method_stub(_class_name: &str, method: &StaticMethod) -> String {
    let params: Vec<String> = method.params.iter()
        .map(|p| format!("{}: {}", safe_param_name(&p.name), p.ty.to_rust_type_string_safe()))
        .collect();
    let ret = method.return_type.as_ref()
        .map(|ty| format!(" -> {}", stub_return_type_string(ty)))
        .unwrap_or_default();
    format!("pub fn {}({}){};", safe_method_name(&method.name), params.join(", "), ret)
}

/// Generate a best-guess stub Rust declaration for a skipped free function.
fn generate_function_stub(func: &crate::resolver::ResolvedFunction) -> String {
    let params: Vec<String> = func.params.iter()
        .map(|p| format!("{}: {}", safe_param_name(&p.name), p.ty.original.to_rust_type_string_safe()))
        .collect();
    let ret = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", stub_return_type_string(&rt.original)))
        .unwrap_or_default();
    format!("pub fn {}({}){};", safe_method_name(&func.short_name), params.join(", "), ret)
}

/// Convert a return type to its best-guess Rust string for stub declarations.
/// Class/Handle types get wrapped in OwnedPtr; references stay as references.
fn stub_return_type_string(ty: &Type) -> String {
    match ty {
        Type::Class(name) if name != "char" => format!("OwnedPtr<{}>", name),
        Type::Handle(name) => format!("OwnedPtr<Handle<{}>>", name),
        _ => ty.to_rust_type_string_safe(),
    }
}

/// Common filter for instance methods (both direct and wrapper)
/// Methods that cause ambiguous overload errors due to multiple inheritance.
/// Format: (class_name, method_name)
/// TODO: Add to bindings.toml or fix in some other way.
const AMBIGUOUS_METHODS: &[(&str, &str)] = &[
    ("BOPAlgo_ParallelAlgo", "Perform"),
];

fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return Err("causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)".to_string());
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip methods where a class pointer param's inner type is unknown.
    // We check all_classes directly (not type_uses_unknown_type) because nested types
    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
    // is known, but they don't have their own FFI type declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Check class pointer returns for unknown inner types (same as params)
        if let Some(class_name) = ret.class_ptr_inner_name() {
            if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {
                return Err(format!("class pointer return inner type '{}' is unknown", ret.to_cpp_string()));
            }
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if !is_void_type_name(name) {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
}

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> Result<(), String> {
    // By-value class/handle params are now supported: C++ wrappers accept const T&
    // and the C++ compiler handles the copy.
    if ctor.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_ctor(ctor);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    if let Some(p) = ctor
        .params
        .iter()
        .find(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return Err(format!("param '{}' uses unknown Handle type", p.name));
    }
    // Also check for unknown class types in parameters.
    // This catches NCollection typedef types (e.g., TDF_LabelMap) that aren't
    // declared in the extern "C" FFI.
    if let Some(p) = ctor
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip constructors where a nullable pointer param's inner type is unknown
    if let Some(p) = ctor.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip constructors where a class pointer param's inner type is unknown.
    // Check all_classes directly — nested types don't have FFI declarations.
    if let Some(p) = ctor.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    Ok(())
}

/// Filter for static methods
fn is_static_method_bindable(method: &StaticMethod, ctx: &TypeContext) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_static(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip static methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::static_method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // &mut enum output params are now handled via C++ wrappers.
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip static methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip static methods where a class pointer param's inner type is unknown.
    // Check all_classes directly — nested types don't have FFI declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Same CppDeletable check as for instance methods (see is_method_bindable).
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if !is_void_type_name(name) {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // C-string returns (const char*) are handled via C++ wrappers returning const char*.
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
}

/// Check if a return type is a mutable reference to an enum.
/// Extern "C" can't handle these: Rust side has `&mut i32` but C++ has `EnumType&`.
fn return_type_is_mut_ref_enum(ty: &Type, all_enums: &HashSet<String>) -> bool {
    if let Type::MutRef(inner) = ty {
        if let Type::Class(name) = inner.as_ref() {
            return all_enums.contains(name);
        }
    }
    false
}

// ── Building ParamBinding / ReturnTypeBinding ───────────────────────────────

/// Extract the enum C++ name from a type, unwrapping const references.
/// MutRef to enums is NOT extracted — these are output parameters that need
/// special handling (local variable + writeback), not supported yet.
fn extract_enum_name(ty: &Type, all_enums: &HashSet<String>) -> Option<String> {
    match ty {
        Type::Class(name) if all_enums.contains(name) => Some(name.clone()),
        Type::ConstRef(inner) | Type::RValueRef(inner) => {
            extract_enum_name(inner, all_enums)
        }
        _ => None,
    }
}

fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                let enum_rust_type = ffi_ctx.enum_rust_types
                    .and_then(|map| map.get(enum_name))
                    .cloned();
                let reexport_type = enum_rust_type.as_ref()
                    .map(|t| format!("&mut {}", t))
                    .unwrap_or_else(|| "&mut i32".to_string());
                return ParamBinding {
                    cpp_name,
                    rust_name,
                    rust_ffi_type: "&mut i32".to_string(),
                    rust_reexport_type: reexport_type,
                    cpp_type: "int32_t&".to_string(),
                    // The arg expression uses the local variable name (preamble creates it)
                    cpp_arg_expr: format!("{}_enum_", name),
                    // No value enum conversion at Rust level
                    enum_rust_type: None,
                    mut_ref_enum_cpp_name: Some(enum_name.clone()),
                    mut_ref_enum_rust_type: enum_rust_type,
                    is_nullable_ptr: false,
                    is_class_ptr: false,
                };
            }
        }
    }

    // Check if this parameter is an enum type (by value or const ref)
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        // Look up the Rust enum type for value enums
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            cpp_arg_expr: format!("static_cast<{}>({})", enum_cpp_name, name),
            enum_rust_type,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: false,
            is_class_ptr: false,
        };
    }

    // Nullable pointer params: const T* = NULL -> Option<&T>, T* = NULL -> Option<&mut T>
    // In ffi.rs: *const T / *mut T (raw pointers, nullable)
    // In re-export: Option<&T> / Option<&mut T>
    // In C++: const T* / T* (passed through directly)
    if is_nullable && !ty.is_void_ptr() {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
                    format!("Option<&{}>", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*mut {}", inner_ffi),
                    format!("Option<&mut {}>", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: true,
            is_class_ptr: false,
        };
    }

    // Non-nullable class pointer params: const T* -> &T, T* -> &mut T
    // In ffi.rs: *const T / *mut T (raw pointers)
    // In re-export: &T / &mut T
    // In C++: const T* / T* (passed through directly)
    if let Some(_class_name) = ty.class_ptr_inner_name() {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
                    format!("&{}", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*mut {}", inner_ffi),
                    format!("&mut {}", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("class_ptr_inner_name() returned Some for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: false,
            is_class_ptr: true,
        };
    }

    // By-value class/handle params — opaque types
    // must be passed by reference. We convert them to const T& at the FFI
    // boundary; the C++ wrapper passes the reference to the original method
    // which accepts by value (C++ handles the implicit copy).
    let effective_ty = match ty {
        Type::Class(name) if is_opaque_class_name(name) && !ffi_ctx.all_enums.contains(name) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        Type::Handle(_) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        _ => ty.clone(),
    };

    let mapped = map_type_in_context(&effective_ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = type_to_rust_string(&effective_ty, reexport_ctx);
    let cpp_type = effective_ty.to_cpp_extern_c_param();
    let cpp_arg_expr = param_to_cpp_extern_c_arg(name, &effective_ty);

    ParamBinding {
        cpp_name,
        rust_name,
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        cpp_arg_expr,
        enum_rust_type: None,
        mut_ref_enum_cpp_name: None,
        mut_ref_enum_rust_type: None,
        is_nullable_ptr: false,
        is_class_ptr: false,
    }
}

fn build_return_type_binding(ty: &Type, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ReturnTypeBinding {
    // Check if this return type is an enum
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ReturnTypeBinding {
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            needs_unique_ptr: false,
            enum_cpp_name: Some(enum_cpp_name),
            enum_rust_type,
            is_class_ptr_return: false,
        };
    }

    // Check if this return type is a class pointer (const T* or T* where T is a known class)
    // These are bound as Option<&T> / Option<&mut T> since they may return null.
    if let Some(class_name) = ty.class_ptr_inner_name() {
        let mapped = map_return_type_in_context(ty, ffi_ctx);
        let rust_ffi_type = mapped.rust_type;
        let cpp_type = ty.to_cpp_string();
        // Build the reexport type as Option<&T> or Option<&mut T>
        let is_const = matches!(ty, Type::ConstPtr(_));
        let inner_rust_type = if let Some(ctx) = reexport_ctx {
            ctx.resolve_class(class_name)
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(class_name))
        };
        let rust_reexport_type = if is_const {
            format!("Option<&{}>", inner_rust_type)
        } else {
            format!("Option<&mut {}>", inner_rust_type)
        };
        return ReturnTypeBinding {
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            needs_unique_ptr: false,
            enum_cpp_name: None,
            enum_rust_type: None,
            is_class_ptr_return: true,
        };
    }

    let mapped = map_return_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = return_type_to_rust_string(ty, reexport_ctx);
    let cpp_type = ty.to_cpp_string();
    let needs_unique_ptr = (ty.is_class() && !ty.is_void_ptr()) || ty.is_handle();

    ReturnTypeBinding {
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        needs_unique_ptr,
        enum_cpp_name: None,
        enum_rust_type: None,
        is_class_ptr_return: false,
    }
}

// ── Overload suffix computation ─────────────────────────────────────────────

/// Compute overload suffix with const/mut disambiguation for direct methods.
/// Returns (rust_name, suffix_used) for each method in the list.
/// `constructor_names` contains the impl_method_names of constructors (e.g. "new", "new_2")
/// so that methods whose snake_case name collides with a constructor get a suffix.
fn compute_direct_method_names(methods: &[&Method], constructor_names: &HashSet<String>) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    let mut seen_names: HashMap<String, usize> = HashMap::new();
    // Pre-seed with constructor names so methods colliding with them get _2 suffix
    for name in constructor_names {
        seen_names.insert(name.clone(), 1);
    }

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
/// `constructor_names` contains the impl_method_names of constructors (e.g. "new", "new_2")
/// so that methods whose snake_case name collides with a constructor get a suffix.
fn compute_wrapper_method_names(methods: &[&Method], constructor_names: &HashSet<String>) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    // Pass 1: resolve C++ overloads (same C++ name, different params)
    let mut names: Vec<String> = methods
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
                combine_name_suffix(&base_name, &suffix)
            } else {
                base_name
            }
        })
        .collect();

    // Pass 2: resolve cross-name collisions (different C++ names that produce
    // the same snake_case name, e.g. SetInteger/setInteger → set_integer).
    // Append _2, _3, ... to later duplicates.
    // Pre-seed with constructor names so methods colliding with them get _2 suffix.
    let mut seen: HashMap<String, usize> = HashMap::new();
    for name in constructor_names {
        seen.insert(name.clone(), 1);
    }
    for name in &mut names {
        let count = seen.entry(name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            *name = format!("{}_{}", name, count);
        }
    }

    names
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

    let mut results: Vec<(String, String)> = methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let has_internal_conflict =
                name_counts.get(&method.name).copied().unwrap_or(0) > 1;

            // Level 1: Internal overload suffix
            let candidate_fn_name = if has_internal_conflict {
                let suffix = method.overload_suffix();
                combine_name_suffix(&base_name, &suffix)
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
                    combine_name_suffix(&base_name, &suffix)
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
                        combine_name_suffix(&base_name, &suffix)
                    }
                } else {
                    ffi_fn_name_base.clone()
                };

            (ffi_fn_name_base, impl_method_name)
        })
        .collect();

    // Pass 2: resolve cross-name collisions (different C++ names that produce
    // the same snake_case name). Append _2, _3, ... to later duplicates.
    let mut seen_ffi: HashMap<String, usize> = HashMap::new();
    let mut seen_impl: HashMap<String, usize> = HashMap::new();
    for (ffi_name, impl_name) in &mut results {
        let ffi_count = seen_ffi.entry(ffi_name.clone()).or_insert(0);
        *ffi_count += 1;
        if *ffi_count > 1 {
            *ffi_name = format!("{}_{}", ffi_name, ffi_count);
        }
        let impl_count = seen_impl.entry(impl_name.clone()).or_insert(0);
        *impl_count += 1;
        if *impl_count > 1 {
            *impl_name = format!("{}_{}", impl_name, impl_count);
        }
    }

    results
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
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    let mut skipped_symbols: Vec<SkippedSymbol> = Vec::new();

    // ── Constructors ────────────────────────────────────────────────────────────
    let exclude_ctors = exclude_methods.contains(&(class.name.clone(), class.name.clone()))
        || exclude_methods.contains(&(class.name.clone(), "*".to_string()));
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, reexport_ctx);
        if exclude_ctors {
            // Record excluded constructors from bindings.toml
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "excluded by bindings.toml".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
            ctors.clear();
        }
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
                convenience_of: None,
                is_unsafe: false,
            });
        }
        ctors
    } else {
        // Record skipped constructors for abstract/protected-destructor classes
        if effectively_abstract {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "class is abstract (has unimplemented pure virtual methods)".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        } else if class.has_protected_destructor {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "class has protected destructor".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        }
        Vec::new()
    };

    // Collect skipped constructors from bindability checks (in the pre-compute phase)
    if !effectively_abstract && !class.has_protected_destructor && !exclude_ctors {
        for ctor in &class.constructors {
            if let Err(reason) = is_constructor_bindable(ctor, all_enum_names, handle_able_classes, ffi_ctx) {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: reason,
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        }
    }

    // ── Instance methods (collect skipped, then partition into direct vs wrapper) ─────
    // First pass: categorize all methods as bindable or skipped
    let mut bindable_methods: Vec<&Method> = Vec::new();
    for method in &class.methods {
        if exclude_methods.contains(&(class.name.clone(), method.name.clone())) {
            skipped_symbols.push(SkippedSymbol {
                kind: "method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: "excluded by bindings.toml".to_string(),
                stub_rust_decl: generate_method_stub(cpp_name, method),
            });
            continue;
        }
        if let Err(reason) = is_method_bindable(method, ffi_ctx, cpp_name) {
            skipped_symbols.push(SkippedSymbol {
                kind: "method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_method_stub(cpp_name, method),
            });
            continue;
        }
        bindable_methods.push(method);
    }

    // Partition into direct vs wrapper
    let direct_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| !needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();
    let wrapper_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();

    // Build set of constructor impl_method_names so that method name disambiguation
    // can avoid collisions (e.g. C++ `New()` → `new` colliding with constructor `new()`).
    let constructor_names: HashSet<String> = constructors
        .iter()
        .map(|c| c.impl_method_name.clone())
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw, &constructor_names);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let mut return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            // If the method is const (&self) and returns a class pointer,
            // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
            if method.is_const {
                if let Some(ref mut rt) = return_type {
                    if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                        rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                    }
                }
            }

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types() || resolver::method_needs_explicit_lifetimes(method),
                unsafe_lifetime: resolver::method_needs_explicit_lifetimes(method),
            }
        })
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw, &constructor_names);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = HashSet::new();
    for fn_name in &wrapper_fn_names {
        reserved_names.insert(format!("{}_{}", cpp_name, fn_name));
    }

    // Build FFI method names set (for re-export conflict detection)
    let cxx_method_names: HashSet<String> = direct_methods_raw
        .iter()
        .map(|m| safe_method_name(&m.name))
        .collect();

    // Build all_instance_method_names (direct + wrapper impl names + constructor names)
    let mut all_instance_method_names: HashSet<String> = cxx_method_names.clone();
    // Include constructor impl_method_names so static methods don't collide with them
    for ctor in &constructors {
        all_instance_method_names.insert(ctor.impl_method_name.clone());
    }

    let wrapper_methods: Vec<WrapperMethodBinding> = wrapper_methods_raw
        .iter()
        .zip(wrapper_fn_names.iter())
        .map(|(method, fn_name)| {
            let ffi_fn_name = format!("{}_{}", cpp_name, fn_name);

            // Compute impl_method_name: may differ if fn_name conflicts with a direct method
            let impl_method_name = if cxx_method_names.contains(fn_name) {
                let suffix = method.overload_suffix();
                if suffix.is_empty() {
                    format!("{}_wrapper", fn_name)
                } else {
                    let base_name = safe_method_name(&method.name);
                    combine_name_suffix(&base_name, &suffix)
                }
            } else {
                fn_name.clone()
            };

            all_instance_method_names.insert(impl_method_name.clone());

            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let mut return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));
            let wrapper_kind = classify_wrapper_kind(method, all_enum_names);

            // For ConstMutReturnFix, the wrapper takes non-const self even though
            // the C++ method is const. This ensures methods returning &mut use &mut self.
            let effective_is_const = if wrapper_kind == WrapperKind::ConstMutReturnFix {
                false
            } else {
                method.is_const
            };

            // If the method is const (&self) and returns a class pointer,
            // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
            if effective_is_const {
                if let Some(ref mut rt) = return_type {
                    if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                        rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                    }
                }
            }

            WrapperMethodBinding {
                ffi_fn_name,
                impl_method_name,
                is_const: effective_is_const,
                params,
                return_type,
                wrapper_kind,
                cpp_method_name: method.name.clone(),
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types() || resolver::method_needs_explicit_lifetimes(method),
                unsafe_lifetime: resolver::method_needs_explicit_lifetimes(method),
            }
        })
        .collect();

    // ── Static methods ──────────────────────────────────────────────────────────
    let mut static_methods_raw: Vec<&StaticMethod> = Vec::new();
    for method in &class.static_methods {
        if exclude_methods.contains(&(class.name.clone(), method.name.clone())) {
            skipped_symbols.push(SkippedSymbol {
                kind: "static_method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: "excluded by bindings.toml".to_string(),
                stub_rust_decl: generate_static_method_stub(cpp_name, method),
            });
            continue;
        }
        if let Err(reason) = is_static_method_bindable(method, ffi_ctx) {
            skipped_symbols.push(SkippedSymbol {
                kind: "static_method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_static_method_stub(cpp_name, method),
            });
            continue;
        }
        static_methods_raw.push(method);
    }

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
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            let needs_static_lifetime = method
                .return_type
                .as_ref()
                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if is_opaque_class_name(name))))
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
                is_unsafe: method.has_unsafe_types(),
            }
        })
        .collect();

    // ── Upcasts ─────────────────────────────────────────────────────────
    let upcasts = compute_upcast_bindings(class, symbol_table);

    // ── to_owned ──────────────────────────────────────────────────────────────────────────────
    // Detect copyability using libclang copy/move constructor detection.
    // has_copy_constructor: Some(true) = explicit usable copy ctor,
    //                       Some(false) = explicitly deleted/private,
    //                       None = no explicit copy ctor (implicit may exist)
    //
    // Handle-able classes (inheriting from Standard_Transient) always use to_handle()
    // instead of to_owned(), even if they have explicit copy constructors.
    //
    // For None (no explicit copy ctor), we fall back to a conservative module
    // allowlist because implicit copy constructors can be silently deleted when
    // a class has non-copyable members (e.g., algorithm classes with Extrema solvers).
    // Clang does not enumerate implicitly-deleted copy constructors.
    let is_handle_type = handle_able_classes.contains(&class.name);
    let copyable_modules = ["TopoDS", "gp", "TopLoc", "Bnd", "GProp"];
    let is_copyable = if is_handle_type {
        false // Transient classes use handles, not copies
    } else {
        match class.has_copy_constructor {
            Some(true) => true,   // Explicit public non-deleted copy constructor
            Some(false) => false, // Explicitly deleted or non-public copy constructor
            None => {
                // No explicit copy ctor. Implicit one may or may not exist.
                // Move constructors suppress implicit copy ctors.
                // For remaining classes, fall back to known-copyable module list.
                !class.has_move_constructor
                    && copyable_modules.contains(&class.module.as_str())
            }
        }
    };
    let has_to_owned = is_copyable
        && !class.has_protected_destructor
        && !effectively_abstract;

    // ── to_handle ───────────────────────────────────────────────────────
    // Handle types with protected destructors can still use to_handle because
    // Handle<T> manages lifetime via reference counting, not direct delete.
    // However, to_handle requires constructability (it takes ownership of a raw pointer),
    // so skip for abstract classes and classes with protected destructors.
    let has_to_handle =
        is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle get/get_mut (works for abstract classes too) ─────────────
    // Also works for protected-destructor classes since we're just dereferencing the Handle.
    let has_handle_get = is_handle_type;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_handle_get {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };
    // ── Handle downcasts ─────────────────────────────────────────────
    let handle_downcasts = if has_handle_get {
        compute_handle_downcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };

    // ── Inherited methods ───────────────────────────────────────────────
    let inherited_methods_raw =
        compute_inherited_method_bindings(class, symbol_table, handle_able_classes, ffi_ctx.all_classes, ffi_ctx.all_enums, ffi_ctx.deletable_class_names, reexport_ctx, exclude_methods);
    // Filter out inherited methods whose Rust name conflicts with a constructor or direct method
    let ctor_and_method_names: std::collections::HashSet<&str> = constructors
        .iter()
        .map(|c| c.impl_method_name.as_str())
        .chain(direct_methods.iter().map(|m| m.rust_name.as_str()))
        .chain(wrapper_methods.iter().map(|m| m.impl_method_name.as_str()))
        .chain(static_methods.iter().map(|m| m.impl_method_name.as_str()))
        .collect();
    let mut inherited_methods: Vec<InheritedMethodBinding> = inherited_methods_raw
        .into_iter()
        .filter(|im| !ctor_and_method_names.contains(im.impl_method_name.as_str()))
        .collect();
    // Dedup inherited methods against each other (different C++ names that
    // produce the same snake_case, e.g. GetChildLabel/getChildLabel).
    // Also dedup the FFI function names (C++ wrappers) to avoid link-time
    // collisions.
    {
        let mut seen_impl: HashMap<String, usize> = HashMap::new();
        let mut seen_ffi: HashMap<String, usize> = HashMap::new();
        for im in &mut inherited_methods {
            let impl_count = seen_impl.entry(im.impl_method_name.clone()).or_insert(0);
            *impl_count += 1;
            if *impl_count > 1 {
                im.impl_method_name = format!("{}_{}", im.impl_method_name, impl_count);
            }
            let ffi_count = seen_ffi.entry(im.ffi_fn_name.clone()).or_insert(0);
            *ffi_count += 1;
            if *ffi_count > 1 {
                im.ffi_fn_name = format!("{}_{}", im.ffi_fn_name, ffi_count);
            }
        }
    }
    // ── POD struct fields ────────────────────────────────────────────────
    let pod_fields = if class.is_pod_struct {
        compute_pod_field_bindings(&class.fields)
    } else {
        Vec::new()
    };

    ClassBindings {
        cpp_name: cpp_name.clone(),
        cpp_qualified_name: class.name.clone(),
        short_name: crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(cpp_name, &class.module)),
        module: class.module.clone(),
        is_abstract: effectively_abstract,
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
        has_handle_get,
        handle_upcasts,
        handle_downcasts,
        inherited_methods,
        is_pod_struct: class.is_pod_struct,
        pod_fields,
        skipped_symbols,
    }
}

// ── POD struct field bindings ───────────────────────────────────────────────

/// Map a ParsedField's Type to the Rust type string for a POD struct field.
fn pod_field_rust_type(ty: &Type) -> Option<&'static str> {
    match ty {
        Type::Bool => Some("bool"),
        Type::I32 => Some("i32"),
        Type::U32 => Some("u32"),
        Type::U16 => Some("u16"),
        Type::I16 => Some("i16"),
        Type::I64 => Some("i64"),
        Type::U64 => Some("u64"),
        Type::Long => Some("std::os::raw::c_long"),
        Type::ULong => Some("std::os::raw::c_ulong"),
        Type::Usize => Some("usize"),
        Type::F32 => Some("f32"),
        Type::F64 => Some("f64"),
        _ => None,
    }
}

fn compute_pod_field_bindings(fields: &[ParsedField]) -> Vec<PodFieldBinding> {
    fields
        .iter()
        .enumerate()
        .filter_map(|(idx, field)| {
            let rust_type = pod_field_rust_type(&field.ty)?;
            Some(PodFieldBinding {
                rust_name: field.name.to_snake_case(),
                cpp_name: field.name.clone(),
                rust_type: rust_type.to_string(),
                array_size: field.array_size,
                offset_index: idx,
                doc_comment: field.comment.clone(),
            })
        })
        .collect()
}

// ── Constructor bindings ─────────────────────────────────────────────────


/// Adapt a C++ default value expression to be valid for the corresponding Rust type.
///
/// C++ allows implicit conversions (e.g., `0` for `double`, `0` for `nullptr`).
/// This function returns `None` if the default can't be properly expressed in Rust.
fn adapt_default_for_rust_type(default_expr: &str, param_type: &Type) -> Option<String> {
    // Unwrap references since the default applies to the underlying type
    let inner_type = match param_type {
        Type::ConstRef(inner) | Type::MutRef(inner) => inner.as_ref(),
        _ => param_type,
    };

    match inner_type {
        Type::Bool => {
            // Bool defaults should already be "true" or "false"
            match default_expr {
                "true" | "false" => Some(default_expr.to_string()),
                "0" => Some("false".to_string()),
                "1" => Some("true".to_string()),
                _ => None,
            }
        }
        Type::F64 | Type::F32 => {
            // C++ allows integer literals for floating types (e.g., `0` for `0.0`)
            if default_expr.contains('.') {
                Some(default_expr.to_string())
            } else if let Ok(_) = default_expr.parse::<i64>() {
                Some(format!("{}.0", default_expr))
            } else {
                None
            }
        }
        Type::I32 | Type::U32 | Type::U16 | Type::I16 | Type::I64 | Type::U64 | Type::Long | Type::ULong | Type::Usize => {
            // Integer literals should work directly
            if default_expr.parse::<i64>().is_ok() || default_expr.parse::<u64>().is_ok() {
                Some(default_expr.to_string())
            } else {
                None
            }
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            // const char* defaults — `0`/`nullptr` means null pointer, not expressible as &str
            None
        }
        _ => {
            // For other types (classes, handles, etc.), we can't express defaults
            None
        }
    }
}

/// A constructor, possibly with trailing defaulted params trimmed.
struct TrimmedConstructor<'a> {
    original: &'a Constructor,
    /// How many params to include (may be less than original.params.len())
    trimmed_param_count: usize,
    /// If this is a convenience wrapper, the index of the full-argument parent
    /// in the regular_ctors vec, plus that parent's trimmed_param_count.
    convenience_parent: Option<(usize, usize)>,
}

/// Check if a slice of params passes all bindability filters.
fn is_params_bindable(
    params: &[Param],
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> bool {
    // By-value class/handle params are now supported via C++ wrappers (const T& conversion).
    if params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
        return false;
    }
    if params
        .iter()
        .any(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return false;
    }
    // Check for unknown class types
    if params
        .iter()
        .any(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return false;
    }
    // Check for class pointer params whose inner type is unknown.
    // Check all_classes directly — nested types don't have FFI declarations.
    if params.iter().any(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return false;
    }
    true
}

/// Compute overload suffix for a param slice (used for trimmed constructors).
/// Combine a base name with an overload suffix, avoiding double underscores.
/// If base_name ends with '_' (e.g. keyword-escaped "type_") and suffix starts with '_',
/// we merge them to avoid "type__suffix" → "type_suffix" instead.
fn combine_name_suffix(base: &str, suffix: &str) -> String {
    if base.ends_with('_') && suffix.starts_with('_') {
        format!("{}{}", base, &suffix[1..])
    } else {
        format!("{}{}", base, suffix)
    }
}

fn overload_suffix_for_params(params: &[Param]) -> String {
    let types: Vec<Type> = params.iter().map(|p| p.ty.clone()).collect();
    overload_suffix_for_types(&types)
}

/// Compute an overload suffix from a slice of types.
/// Uses `Type::short_name()` to generate human-readable suffixes like
/// `_real`, `_pnt_dir`, `_real3`. Consecutive identical types are compressed:
/// `[f64, f64, f64]` → `_real3`.
fn overload_suffix_for_types(types: &[Type]) -> String {
    if types.is_empty() {
        return String::new();
    }

    let type_names: Vec<String> = types
        .iter()
        .map(|t| t.short_name().to_lowercase())
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

/// Strip const/mut ref qualifiers from a type, leaving inner type intact.
/// Used to detect const/mut pair overloads (e.g., `const TopoDS_Shape&` vs `TopoDS_Shape&`).
fn strip_ref_qualifiers(ty: &Type) -> Type {
    match ty {
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            strip_ref_qualifiers(inner)
        }
        other => other.clone(),
    }
}

fn compute_constructor_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    handle_able_classes: &HashSet<String>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> Vec<ConstructorBinding> {
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    // Collect directly bindable constructors
    let mut bindable_ctors: Vec<TrimmedConstructor> = class
        .constructors
        .iter()
        .filter(|c| is_constructor_bindable(c, all_enum_names, handle_able_classes, ffi_ctx).is_ok())
        .map(|c| TrimmedConstructor {
            original: c,
            trimmed_param_count: c.params.len(),
            convenience_parent: None,
        })
        .collect();

    // For constructors that failed binding, try trimming defaulted trailing params
    // that are unbindable (enums, by-value classes/handles). C++ requires defaults
    // contiguous from the right, so we strip from the end until the remaining
    // params pass the filter.
    for ctor in &class.constructors {
        if is_constructor_bindable(ctor, all_enum_names, handle_able_classes, ffi_ctx).is_ok() {

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
            if is_params_bindable(trimmed_params, all_enum_names, handle_able_classes, ffi_ctx) {

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
                        convenience_parent: None,
                    });
                }
                break;
            }
        }
    }

    // For bindable constructors that have trailing default params, also generate
    // convenience wrappers with fewer params. These are Rust-only wrappers that
    // call the full-argument version with default values filled in.
    // E.g., BRepBuilderAPI_Transform(S, T, copy=false, copyMesh=false) generates:
    //   new_shape_trsf_bool2(S, T, copy, copyMesh)  — full version (C++ wrapper)
    //   new_shape_trsf_bool(S, T, copy)              — 3-param convenience (Rust-only)
    //   new_shape_trsf(S, T)                         — 2-param convenience (Rust-only)
    let regular_count = bindable_ctors.len();
    for i in 0..regular_count {
        let ctor = bindable_ctors[i].original;
        let full_count = bindable_ctors[i].trimmed_param_count;

        // Only process constructors with trailing default params
        if full_count == 0 {
            continue;
        }

        let mut trim_to = full_count;
        while trim_to > 0 {
            let last_param = &ctor.params[trim_to - 1];
            if !last_param.has_default {
                break; // Can't trim non-default params
            }
            trim_to -= 1;

            // Check that we can express all trimmed params' defaults as valid Rust
            let trimmed_range = &ctor.params[trim_to..full_count];
            let all_defaults_expressible = trimmed_range.iter().all(|p| {
                p.default_value.is_some()
                    && adapt_default_for_rust_type(p.default_value.as_deref().unwrap(), &p.ty).is_some()
            });
            if !all_defaults_expressible {
                // Can't generate a Rust-only convenience without valid defaults
                continue;
            }

            let trimmed_params = &ctor.params[..trim_to];
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
                    convenience_parent: Some((i, full_count)),
                });
            }
        }
    }

    // Now compute names and build ConstructorBindings.
    // Phase 1: Assign names to all constructors (regular and convenience alike).
    let mut ctor_names: HashMap<String, usize> = HashMap::new();

    let all_names: Vec<String> = bindable_ctors
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
            final_method_name.to_snake_case()
        })
        .collect();

    // Phase 2: Build ConstructorBindings using the computed names.
    bindable_ctors
        .iter()
        .enumerate()
        .map(|(idx, trimmed)| {
            let params_slice = &trimmed.original.params[..trimmed.trimmed_param_count];
            let impl_method_name = all_names[idx].clone();

            let params: Vec<ParamBinding> = params_slice
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();

            let convenience_of = trimmed.convenience_parent.map(|(parent_idx, parent_param_count)| {
                let full_method_name = all_names[parent_idx].clone();
                let default_exprs: Vec<String> = trimmed
                    .original
                    .params[trimmed.trimmed_param_count..parent_param_count]
                    .iter()
                    .map(|p| {
                        let raw = p.default_value.as_deref().unwrap_or("Default::default()");
                        adapt_default_for_rust_type(raw, &p.ty)
                            .unwrap_or_else(|| "Default::default()".to_string())
                    })
                    .collect();
                ConvenienceInfo {
                    full_method_name,
                    default_exprs,
                }
            });

            let (ffi_fn_name, cpp_arg_exprs) = if convenience_of.is_some() {
                // Convenience constructors don't need FFI entries
                (String::new(), Vec::new())
            } else {
                let base_suffix = overload_suffix_for_params(params_slice);
                let ffi_suffix = if base_suffix.is_empty() {
                    "ctor".to_string()
                } else {
                    format!("ctor{}", base_suffix)
                };
                let ffi_fn_name = format!("{}_{}", cpp_name, ffi_suffix);
                let cpp_arg_exprs: Vec<String> = params
                    .iter()
                    .map(|p| p.cpp_arg_expr.clone())
                    .collect();
                (ffi_fn_name, cpp_arg_exprs)
            };

            let is_unsafe = trimmed.original.has_unsafe_types();

            ConstructorBinding {
                ffi_fn_name,
                impl_method_name,
                params,
                cpp_arg_exprs,
                doc_comment: trimmed.original.comment.clone(),
                source_line: trimmed.original.source_line,
                convenience_of,
                is_unsafe,
            }
        })
        .collect()
}

// ── Upcast bindings ─────────────────────────────────────────────────────────

fn compute_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
) -> Vec<UpcastBinding> {
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    all_ancestors
        .iter()
        .filter(|base| {
            symbol_table.all_class_names.contains(*base)
        })
        .map(|base_class| {
            let base_ffi_name = base_class.replace("::", "_");
            let ffi_fn_name = format!("{}_as_{}", cpp_name, base_ffi_name);
            let ffi_fn_name_mut = format!("{}_mut", ffi_fn_name);

            let base_module = if let Some(underscore_pos) = base_ffi_name.find('_') {
                base_ffi_name[..underscore_pos].to_string()
            } else {
                base_ffi_name.clone()
            };

            let base_short_name = type_mapping::safe_short_name(&type_mapping::short_name_for_module(&base_ffi_name, &base_module));

            let impl_method_name = if base_module == class.module {
                format!("as_{}", heck::AsSnakeCase(&base_short_name))
            } else {
                format!("as_{}", heck::AsSnakeCase(base_ffi_name.as_str()))
            };

            UpcastBinding {
                base_class: base_ffi_name,
                base_class_cpp: base_class.clone(),
                base_short_name,
                base_module,
                ffi_fn_name,
                ffi_fn_name_mut,
                impl_method_name,
            }
        })
        .collect()
}

// ── Method name deduplication ──────────────────────────────────────────────────

/// Deduplicate method names. Each entry has (short_name, full_name).
/// Prefers short names when unique, upgrades to full names on collision,
/// and appends numeric suffixes if full names also collide.
fn deduplicate_method_names(candidates: &[(String, String)]) -> Vec<String> {
    use std::collections::HashMap;

    // Phase 1: count short name occurrences
    let mut short_counts: HashMap<&str, usize> = HashMap::new();
    for (short, _) in candidates {
        *short_counts.entry(short.as_str()).or_insert(0) += 1;
    }

    // Phase 2: pick short if unique, full otherwise
    let mut names: Vec<String> = candidates.iter().map(|(short, full)| {
        if short_counts.get(short.as_str()).copied().unwrap_or(0) > 1 {
            full.clone()
        } else {
            short.clone()
        }
    }).collect();

    // Phase 3: fix any remaining duplicates by appending numeric suffix
    let mut counts: HashMap<String, usize> = HashMap::new();
    for name in &names {
        *counts.entry(name.clone()).or_insert(0) += 1;
    }
    let mut seen: HashMap<String, usize> = HashMap::new();
    for name in names.iter_mut() {
        if counts.get(name.as_str()).copied().unwrap_or(0) > 1 {
            let idx = *seen.entry(name.clone()).or_insert(0);
            *seen.get_mut(name.as_str()).unwrap() += 1;
            if idx > 0 {
                *name = format!("{}_{}", name, idx);
            }
        }
    }

    names
}

// ── Handle upcast bindings ──────────────────────────────────────────────────

fn compute_handle_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
) -> Vec<HandleUpcastBinding> {
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    let handle_type_name = type_mapping::handle_type_name(cpp_name);

    all_ancestors
        .iter()
        .filter(|base| {
            // Base must be handle-able AND an actual parsed class (not just a
            // typedef name injected for transitive-closure purposes).
            handle_able_classes.contains(*base)
                && symbol_table.class_by_name(base).is_some()
        })
        .map(|base_class| {
            let base_handle_name = type_mapping::handle_type_name(base_class);
            let ffi_fn_name =
                format!("{}_to_{}", handle_type_name, base_handle_name);
            // Flatten nested class names for module extraction heuristic.
            let flattened = base_class.replace("::", "_");
            let base_module = if let Some(underscore_pos) = flattened.find('_') {
                flattened[..underscore_pos].to_string()
            } else {
                flattened.clone()
            };

            HandleUpcastBinding {
                base_handle_name,
                base_class: base_class.clone(),
                base_module,
                ffi_fn_name,
                derived_handle_name: handle_type_name.clone(),
            }
        })
        .collect()
}

// ── Handle downcast bindings ─────────────────────────────────────────────────────

fn compute_handle_downcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
) -> Vec<HandleDowncastBinding> {
    let all_descendants = symbol_table.get_all_descendants_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    let handle_type_name = type_mapping::handle_type_name(cpp_name);

    all_descendants
        .iter()
        .filter(|desc| {
            if !handle_able_classes.contains(*desc) {
                return false;
            }
            if let Some(desc_class) = symbol_table.class_by_name(desc) {
                // Only generate downcasts to concrete (non-abstract) descendants
                !desc_class.is_abstract
            } else {
                false
            }
        })
        .map(|derived_class| {
            let derived_handle_name = type_mapping::handle_type_name(derived_class);
            let ffi_fn_name =
                format!("{}_downcast_to_{}", handle_type_name, derived_handle_name);
            // Flatten nested class names (e.g., "Parent::Nested" -> "Parent_Nested")
            // for module extraction heuristic.
            let flattened = derived_class.replace("::", "_");
            let derived_module = if let Some(underscore_pos) = flattened.find('_') {
                flattened[..underscore_pos].to_string()
            } else {
                flattened.clone()
            };

            HandleDowncastBinding {
                derived_handle_name,
                derived_class: derived_class.clone(),
                derived_module,
                base_handle_name: handle_type_name.clone(),
                ffi_fn_name,
            }
        })
        .collect()
}

// ── Inherited method bindings ───────────────────────────────────────────────────
fn compute_inherited_method_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
    deletable_class_names: Option<&HashSet<String>>,
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> Vec<InheritedMethodBinding> {
    if class.has_protected_destructor {
        return Vec::new();
    }

    let existing_method_names: HashSet<String> =
        class.methods.iter().map(|m| m.name.clone()).collect();
    let mut seen_methods: HashSet<String> = HashSet::new();
    // Methods that an intermediate ancestor has re-declared as protected/private.
    // These must not be generated as inherited bindings even if a more-distant
    // ancestor exposes them publicly (e.g. BOPAlgo_PaveFiller narrowing Clear()).
    let mut protected_in_ancestors: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    let ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    for ancestor_name in &ancestors {
        if let Some(ancestor_class) = symbol_table.class_by_name(ancestor_name) {
            let ancestor_methods = symbol_table.included_methods(ancestor_class);

            // Collect public method names for this ancestor.
            let ancestor_public_names: HashSet<&str> =
                ancestor_methods.iter().map(|m| m.cpp_name.as_str()).collect();
            // Any method declared by this ancestor (in all_method_names) that is
            // NOT publicly exposed has been narrowed (protected/private override).
            // Block it from being inherited from further-up ancestors.
            for method_name in &ancestor_class.all_method_names {
                if !ancestor_public_names.contains(method_name.as_str()) {
                    protected_in_ancestors.insert(method_name.clone());
                }
            }

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
                // Skip methods narrowed to protected/private in an intermediate ancestor.
                // Example: BOPAlgo_PaveFiller overrides BOPAlgo_Options::Clear() as
                // protected; BOPAlgo_CheckerSI must not inherit Clear() from Options.
                if protected_in_ancestors.contains(&resolved_method.cpp_name) {
                    continue;
                }

                // Skip inherited methods that are explicitly excluded for the child
                // class or for the ancestor class (same method, misresolved type).
                if exclude_methods.contains(&(class.name.clone(), resolved_method.cpp_name.clone()))
                    || exclude_methods.contains(&(ancestor_name.clone(), resolved_method.cpp_name.clone()))
                {
                    continue;
                }

                seen_methods.insert(resolved_method.cpp_name.clone());

                // Skip methods with raw pointers (but allow nullable pointer params)
                let uses_raw_pointers = resolved_method.params.iter().any(|p| {
                    (p.ty.rust_ffi_type.contains("*const")
                        || p.ty.rust_ffi_type.contains("*mut"))
                        && !p.is_nullable_ptr()
                        && p.ty.original.class_ptr_inner_name().is_none()
                })
                    || resolved_method
                        .return_type
                        .as_ref()
                        .map(|rt| {
                            (rt.rust_ffi_type.contains("*const")
                                || rt.rust_ffi_type.contains("*mut"))
                                && rt.original.class_ptr_inner_name().is_none()
                        })
                        .unwrap_or(false);

                if uses_raw_pointers {
                    continue;
                }

                // Skip methods that reference unknown Handle types or unknown classes.
                // But skip this check for params/return types that are enums (they have
                // enum_cpp_name set and are mapped to i32, so they aren't "unknown").
                let uses_unknown_type = resolved_method.params.iter().any(|p| {
                    p.ty.enum_cpp_name.is_none()
                        && type_mapping::type_uses_unknown_handle(
                            &p.ty.original,
                            all_class_names,
                            handle_able_classes,
                        )
                }) || resolved_method
                    .return_type
                    .as_ref()
                    .map(|rt| {
                        rt.enum_cpp_name.is_none()
                            && type_mapping::type_uses_unknown_handle(
                                &rt.original,
                                all_class_names,
                                handle_able_classes,
                            )
                    })
                    .unwrap_or(false);

                if uses_unknown_type {
                    continue;
                }

                // Skip inherited methods whose return type is a class without a
                // generated destructor (OwnedPtr<T> requires CppDeletable for T)
                if let Some(ref rt) = resolved_method.return_type {
                    if let Type::Class(name) = &rt.original {
                        if let Some(deletable) = deletable_class_names {
                            if !deletable.contains(name.as_str()) && !all_enum_names.contains(name.as_str()) {
                                continue;
                            }
                        }
                    }
                }

                // Skip nullable pointer params whose inner type is unknown
                let nullable_uses_unknown = resolved_method.params.iter().any(|p| {
                    if p.is_nullable_ptr() {
                        match &p.ty.original {
                            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                                type_mapping::type_uses_unknown_handle(inner, all_class_names, handle_able_classes)
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                });
                if nullable_uses_unknown {
                    continue;
                }

                // Skip class pointer params whose inner type is unknown.
                // Check all_class_names directly — nested types don't have FFI declarations.
                let class_ptr_uses_unknown = resolved_method.params.iter().any(|p| {
                    if let Some(class_name) = p.ty.original.class_ptr_inner_name() {
                        !all_class_names.contains(class_name) && !all_enum_names.contains(class_name)
                    } else {
                        false
                    }
                });
                if class_ptr_uses_unknown {
                    continue;
                }

                // Skip class pointer returns whose inner type is unknown.
                if let Some(ref rt) = resolved_method.return_type {
                    if let Some(class_name) = rt.original.class_ptr_inner_name() {
                        if !all_class_names.contains(class_name) && !all_enum_names.contains(class_name) {
                            continue;
                        }
                    }
                }

                let ffi_fn_name = format!(
                    "{}_inherited_{}",
                    class.name.replace("::", "_"), resolved_method.cpp_name
                );
                let impl_method_name =
                    safe_method_name(&resolved_method.cpp_name);

                let params: Vec<ResolvedParamBinding> = resolved_method
                    .params
                    .iter()
                    .map(|p| {
                        let is_nullable = p.is_nullable_ptr();

                        // Nullable pointer params: pass through as raw pointers
                        if is_nullable {
                            let (rust_ffi_type, rust_reexport_type, cpp_type) = match &p.ty.original {
                                Type::ConstPtr(inner) => {
                                    let inner_ffi = map_type_to_rust(inner).rust_type;
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = inner.to_cpp_string();
                                    (
                                        format!("*const {}", inner_ffi),
                                        format!("Option<&{}>", inner_rust),
                                        format!("const {}*", inner_cpp),
                                    )
                                }
                                Type::MutPtr(inner) => {
                                    let inner_ffi = map_type_to_rust(inner).rust_type;
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = inner.to_cpp_string();
                                    (
                                        format!("*mut {}", inner_ffi),
                                        format!("Option<&mut {}>", inner_rust),
                                        format!("{}*", inner_cpp),
                                    )
                                }
                                _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
                            };
                            return ResolvedParamBinding {
                                name: p.name.clone(),
                                rust_name: p.rust_name.clone(),
                                rust_ffi_type,
                                rust_reexport_type,
                                cpp_type,
                                cpp_arg_expr: p.name.clone(),
                                enum_rust_type: None,
                                mut_ref_enum_cpp_name: None,
                                mut_ref_enum_rust_type: None,
                                is_nullable_ptr: true,
                                is_class_ptr: false,
                            };
                        }

                        // Non-nullable class pointer params: const T* -> &T, T* -> &mut T
                        if let Some(_class_name) = p.ty.original.class_ptr_inner_name() {
                            let (rust_ffi_type, rust_reexport_type, cpp_type) = match &p.ty.original {
                                Type::ConstPtr(inner) => {
                                    let inner_ffi = map_type_to_rust(inner).rust_type;
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = inner.to_cpp_string();
                                    (
                                        format!("*const {}", inner_ffi),
                                        format!("&{}", inner_rust),
                                        format!("const {}*", inner_cpp),
                                    )
                                }
                                Type::MutPtr(inner) => {
                                    let inner_ffi = map_type_to_rust(inner).rust_type;
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = inner.to_cpp_string();
                                    (
                                        format!("*mut {}", inner_ffi),
                                        format!("&mut {}", inner_rust),
                                        format!("{}*", inner_cpp),
                                    )
                                }
                                _ => unreachable!("class_ptr_inner_name() returned Some for non-pointer type"),
                            };
                            return ResolvedParamBinding {
                                name: p.name.clone(),
                                rust_name: p.rust_name.clone(),
                                rust_ffi_type,
                                rust_reexport_type,
                                cpp_type,
                                cpp_arg_expr: p.name.clone(),
                                enum_rust_type: None,
                                mut_ref_enum_cpp_name: None,
                                mut_ref_enum_rust_type: None,
                                is_nullable_ptr: false,
                                is_class_ptr: true,
                            };
                        }

                        // Check for &mut enum output params — same as build_param_binding
                        if let Type::MutRef(inner) = &p.ty.original {
                            if let Type::Class(enum_name) = inner.as_ref() {
                                if all_enum_names.contains(enum_name) {
                                    let enum_rust_type = symbol_table.enum_rust_types.get(enum_name).cloned();
                                    let reexport_type = enum_rust_type.as_ref()
                                        .map(|t| format!("&mut {}", t))
                                        .unwrap_or_else(|| "&mut i32".to_string());
                                    return ResolvedParamBinding {
                                        name: p.name.clone(),
                                        rust_name: p.rust_name.clone(),
                                        rust_ffi_type: "&mut i32".to_string(),
                                        rust_reexport_type: reexport_type,
                                        cpp_type: "int32_t&".to_string(),
                                        cpp_arg_expr: format!("{}_enum_", p.name),
                                        enum_rust_type: None,
                                        mut_ref_enum_cpp_name: Some(enum_name.clone()),
                                        mut_ref_enum_rust_type: enum_rust_type,
                                        is_nullable_ptr: false,
                                        is_class_ptr: false,
                                    };
                                }
                            }
                        }

                        // Convert by-value class/handle params to const ref (same as build_param_binding)
                        let effective_ty = match &p.ty.original {
                            Type::Class(name) if is_opaque_class_name(name) && p.ty.enum_cpp_name.is_none() => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            Type::Handle(_) => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            _ => p.ty.original.clone(),
                        };
                        let cpp_arg_expr = if let Some(ref enum_name) = p.ty.enum_cpp_name {
                            format!("static_cast<{}>({})", enum_name, p.name)
                        } else {
                            p.name.clone()
                        };
                        let cpp_param_type = if p.ty.enum_cpp_name.is_some() {
                            // Enum params are passed as int32_t at the extern "C" boundary;
                            // the static_cast in cpp_arg_expr converts to the actual enum type.
                            "int32_t".to_string()
                        } else {
                            effective_ty.to_cpp_string()
                        };
                        ResolvedParamBinding {
                            name: p.name.clone(),
                            rust_name: p.rust_name.clone(),
                            rust_ffi_type: if p.ty.enum_cpp_name.is_some() { "i32".to_string() } else { map_type_to_rust(&effective_ty).rust_type },
                            rust_reexport_type: if let Some(ref enum_name) = p.ty.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
                                type_to_rust_string(&effective_ty, reexport_ctx)
                            },
                            cpp_type: cpp_param_type,
                            cpp_arg_expr,
                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),
                            mut_ref_enum_cpp_name: None,
                            mut_ref_enum_rust_type: None,
                            is_nullable_ptr: false,
                            is_class_ptr: false,
                        }
                    })
                    .collect();

                let mut return_type =
                    resolved_method.return_type.as_ref().map(|rt| {
                        let enum_rust_type = rt.enum_cpp_name.as_ref()
                            .and_then(|n| symbol_table.enum_rust_types.get(n))
                            .cloned();
                        ResolvedReturnTypeBinding {
                            rust_ffi_type: if rt.enum_cpp_name.is_some() { "i32".to_string() } else { map_return_type(&rt.original).rust_type },
                            rust_reexport_type: if let Some(ref enum_name) = rt.enum_cpp_name {
                                symbol_table.enum_rust_types.get(enum_name).cloned().unwrap_or_else(|| "i32".to_string())
                            } else {
                                return_type_to_rust_string(&rt.original, reexport_ctx)
                            },
                            cpp_type: rt.cpp_type.clone(),
                            needs_unique_ptr: rt.needs_unique_ptr,
                            enum_cpp_name: rt.enum_cpp_name.clone(),
                            enum_rust_type,
                            is_class_ptr_return: rt.original.class_ptr_inner_name().is_some(),
                        }
                    });

                // If the method is const (&self) and returns a class pointer,
                // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
                if resolved_method.is_const {
                    if let Some(ref mut rt) = return_type {
                        if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                            rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                        }
                    }
                }

                // Check if inherited method has unsafe types (raw pointers / void pointers)
                let has_unsafe_types = resolved_method.params.iter().any(|p| {
                    p.ty.original.needs_unsafe_fn()
                        && !p.is_nullable_ptr()
                        && p.ty.original.class_ptr_inner_name().is_none()
                }) || resolved_method.return_type.as_ref().map_or(false, |rt| {
                    rt.original.needs_unsafe_fn() && rt.original.class_ptr_inner_name().is_none()
                });

                // Check if inherited method returns a reference with reference params (ambiguous lifetime)
                let unsafe_lifetime = {
                    let returns_ref = resolved_method.return_type.as_ref()
                        .map_or(false, |rt| rt.original.is_reference());
                    returns_ref && resolved_method.params.iter()
                        .any(|p| p.ty.original.is_lifetime_source())
                };

                let is_unsafe = has_unsafe_types || unsafe_lifetime;

                result.push(InheritedMethodBinding {
                    ffi_fn_name,
                    impl_method_name,
                    is_const: resolved_method.is_const,
                    params,
                    return_type,
                    cpp_method_name: resolved_method.cpp_name.clone(),
                    source_class: ancestor_name.clone(),
                    source_header: ancestor_class.source_header.clone(),
                    source_line: resolved_method.source_line,
                    is_unsafe,
                    unsafe_lifetime,
                });
            }
        }
    }

    result
}


/// Compute the set of classes that can be wrapped in `Handle<T>`.
///
/// A class is handle-able if it IS `Standard_Transient` or transitively inherits
/// from `Standard_Transient` through the known class graph. The inheritance graph
/// now includes `Standard_*` base classes, so the transitive closure naturally
/// discovers all handle-able classes from just the `Standard_Transient` seed.
///
/// When the inheritance chain passes through typedef'd template specializations
/// (e.g. `BVH_PrimitiveSet3d`) that aren't in `all_classes`, the transitive
/// closure can't reach classes like `BRepExtrema_TriangleSet`. As a fallback,
/// the presence of `DynamicType`/`get_type_descriptor` methods (generated by
/// `DEFINE_STANDARD_RTTIEXT`) is used as definitive proof of handle-ability.
pub fn compute_handle_able_classes(all_classes: &[&ParsedClass]) -> HashSet<String> {
    let mut handle_able = HashSet::new();

    // Seed: Standard_Transient is the root of the Handle hierarchy
    handle_able.insert("Standard_Transient".to_string());

    // Transitive closure: any class with a handle-able base is handle-able
    loop {
        let mut changed = false;
        for class in all_classes {
            if handle_able.contains(&class.name) {
                continue;
            }
            for base in &class.base_classes {
                if handle_able.contains(base) {
                    handle_able.insert(class.name.clone());
                    changed = true;
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Fallback: classes with RTTI methods (DynamicType, get_type_descriptor)
    // from DEFINE_STANDARD_RTTIEXT are definitively in the Standard_Transient
    // hierarchy, even if the transitive closure couldn't reach them through
    // intermediate typedef'd template bases (e.g. BVH_PrimitiveSet3d).
    let mut rtti_added = false;
    for class in all_classes {
        if handle_able.contains(&class.name) {
            continue;
        }
        let has_rtti = class.methods.iter().any(|m| m.name == "DynamicType")
            || class.static_methods.iter().any(|m| m.name == "get_type_descriptor");
        if has_rtti {
            handle_able.insert(class.name.clone());
            rtti_added = true;
            // Also mark intermediate base class names as handle-able so that
            // other classes sharing the same typedef'd base get discovered
            // in the next round of closure.
            for base in &class.base_classes {
                handle_able.insert(base.clone());
            }
        }
    }

    // If RTTI fallback added new entries, run transitive closure again
    if rtti_added {
        loop {
            let mut changed = false;
            for class in all_classes {
                if handle_able.contains(&class.name) {
                    continue;
                }
                for base in &class.base_classes {
                    if handle_able.contains(base) {
                        handle_able.insert(class.name.clone());
                        changed = true;
                        break;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }

    // Only return names that correspond to actual parsed classes.
    // Intermediate typedef names (e.g. BVH_PrimitiveSet3d) were added to the
    // working set for transitive-closure purposes but should not leak into the
    // public handle_able set — they are not real ParsedClass entries and would
    // cause spurious upcast/typedef generation.
    let parsed_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    handle_able.retain(|name| parsed_class_names.contains(name));

    handle_able
}

// ── Top-level function ──────────────────────────────────────────────────────

/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    manual_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the manually-specified known collections (which get generated destructors) +
    // NCollection typedef names from extra_typedef_names (e.g. gp_Vec3f, gp_Pnt2f).
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .chain(extra_typedef_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
        for method in &class.methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    all_class_names.extend(manual_type_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();


    let class_public_info = build_class_public_info(all_classes);

    all_classes
        .iter()
        .map(|class| {
            let reexport_ctx = ReexportTypeContext {
                class_public_info: &class_public_info,
                current_module_rust: crate::module_graph::module_to_rust_name(&class.module),
            };
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, Some(&reexport_ctx), exclude_methods)
        })
        .collect()
}

// ── Free function bindings ──────────────────────────────────────────────────

/// Collect nested type names (Parent::Nested) that should be considered deletable.
/// These get destructors generated via the nested type destructor mechanism.
fn collect_nested_deletable_names(ty: &Type, known_classes: &HashSet<&str>, out: &mut HashSet<String>) {
    match ty {
        Type::Class(name) if name.contains("::") => {
            if let Some(parent) = name.split("::").next() {
                if known_classes.contains(parent) {
                    out.insert(name.clone());
                }
            }
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) |
        Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            collect_nested_deletable_names(inner, known_classes, out);
        }
        _ => {}
    }
}

/// Collect C++ headers needed for a type (for #include directives in wrappers.hxx).
fn collect_headers_for_type(ty: &Type, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if ty.is_unbindable() {
        return;
    }
    match ty {
        Type::Class(name) => {
            if matches!(name.as_str(),
                "bool" | "char" | "int" | "unsigned" | "float" | "double" |
                "void" | "size_t" | "Standard_Address"
            ) {
                return;
            }
            // For nested types (Parent::Nested), include the parent class header
            if name.contains("::") {
                if let Some(parent) = name.split("::").next() {
                    if parent.contains('_') || parent.starts_with("Standard") {
                        let header = format!("{}.hxx", parent);
                        if known_headers.is_empty() || known_headers.contains(&header) {
                            headers.insert(header);
                        }
                    }
                }
                return;
            }
            if !name.contains('_') && !name.starts_with("Standard") {
                return;
            }
            let header = format!("{}.hxx", name);
            if known_headers.is_empty() || known_headers.contains(&header) {
                headers.insert(header);
            }
        }
        Type::Handle(name) => {
            let header = format!("{}.hxx", name);
            if known_headers.is_empty() || known_headers.contains(&header) {
                headers.insert(header);
            }
            headers.insert("Standard_Handle.hxx".to_string());
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            collect_headers_for_type(inner, headers, known_headers);
        }
        _ => {}
    }
}

/// Compute all binding decisions for every free function.
///
/// This is the SINGLE place where naming (overload suffixes, dedup) happens
/// for free functions, using the same `overload_suffix_for_types` / `combine_name_suffix`
/// logic as class methods. The result is shared by all three output generators.
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
    manual_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> (Vec<FunctionBinding>, Vec<SkippedSymbol>) {
    let all_functions = symbol_table.all_included_functions();
    if all_functions.is_empty() {
        return (Vec::new(), Vec::new());
    }

    // Build TypeContext
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types as deletable (they get destructor generation)
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for func in &all_functions {
        if let Some(ref ret) = func.return_type {
            collect_nested_deletable_names(&ret.original, &known_class_names, &mut deletable_class_names);
        }
        for param in &func.params {
            collect_nested_deletable_names(&param.ty.original, &known_class_names, &mut deletable_class_names);
        }
    }

    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    all_class_names.extend(manual_type_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    // Group by base rust_name to detect overloads
    let mut name_groups: HashMap<String, usize> = HashMap::new();
    for func in &all_functions {
        *name_groups.entry(func.rust_name.clone()).or_insert(0) += 1;
    }

    // Pre-pass: identify "const/mut pair" overload groups.
    // If ALL overloads of a name differ only in ref qualifiers (const vs mutable),
    // the const variant keeps the base name and the mut variant gets `_mut`.
    // This handles common patterns like TopoDS::Wire(const Shape&) / Wire(Shape&).
    let mut const_mut_pair_names: HashSet<String> = HashSet::new();
    for (base_name, &count) in &name_groups {
        if count <= 1 {
            continue;
        }
        let members: Vec<_> = all_functions
            .iter()
            .filter(|f| f.rust_name == *base_name)
            .collect();
        // Check if all members have the same canonical types (ignoring const/mut ref)
        let canonical_types = |f: &crate::resolver::ResolvedFunction| -> Vec<Type> {
            f.params
                .iter()
                .map(|p| strip_ref_qualifiers(&p.ty.original))
                .collect()
        };
        let first_canonical = canonical_types(members[0]);
        let all_same_canonical = members.iter().all(|m| canonical_types(m) == first_canonical);
        if all_same_canonical {
            const_mut_pair_names.insert(base_name.clone());
        }
    }

    let class_public_info = build_class_public_info(all_classes);

    let mut used_names: HashSet<String> = HashSet::new();
    let mut result = Vec::new();
    let mut skipped = Vec::new();

    for func in &all_functions {
        // Skip functions with unbindable types
        let unbindable_param = func.params.iter().find(|p| {
            p.ty.original.is_unbindable() || type_uses_unknown_type(&p.ty.original, &ffi_ctx)
        });
        if let Some(p) = unbindable_param {
            let reason = if p.ty.original.is_unbindable() {
                format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty.original))
            } else {
                format!("param '{}' uses unknown type '{}'", p.name, p.ty.original.to_cpp_string())
            };
            skipped.push(SkippedSymbol {
                kind: "function",
                module: func.rust_module.clone(),
                cpp_name: format!("{}::{}", func.namespace, func.short_name),
                source_header: func.source_header.clone(),
                source_line: func.source_line,
                doc_comment: func.doc_comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_function_stub(func),
            });
            continue;
        }
        if let Some(ref ret) = func.return_type {
            if ret.original.is_unbindable() {
                skipped.push(SkippedSymbol {
                    kind: "function",
                    module: func.rust_module.clone(),
                    cpp_name: format!("{}::{}", func.namespace, func.short_name),
                    source_header: func.source_header.clone(),
                    source_line: func.source_line,
                    doc_comment: func.doc_comment.clone(),
                    skip_reason: format!("return type: {}", describe_unbindable_reason(&ret.original)),
                    stub_rust_decl: generate_function_stub(func),
                });
                continue;
            }
            if type_uses_unknown_type(&ret.original, &ffi_ctx) {
                skipped.push(SkippedSymbol {
                    kind: "function",
                    module: func.rust_module.clone(),
                    cpp_name: format!("{}::{}", func.namespace, func.short_name),
                    source_header: func.source_header.clone(),
                    source_line: func.source_line,
                    doc_comment: func.doc_comment.clone(),
                    skip_reason: format!("return type '{}' is unknown", ret.original.to_cpp_string()),
                    stub_rust_decl: generate_function_stub(func),
                });
                continue;
            }
            // CppDeletable check for return types (same as class methods)
            if let Type::Class(name) = &ret.original {
                if !is_void_type_name(name) {
                    if let Some(ref deletable) = ffi_ctx.deletable_class_names {
                        if !deletable.contains(name.as_str()) && !ffi_ctx.all_enums.contains(name.as_str()) {
                            skipped.push(SkippedSymbol {
                                kind: "function",
                                module: func.rust_module.clone(),
                                cpp_name: format!("{}::{}", func.namespace, func.short_name),
                                source_header: func.source_header.clone(),
                                source_line: func.source_line,
                                doc_comment: func.doc_comment.clone(),
                                skip_reason: format!("return type '{}' is not CppDeletable", name),
                                stub_rust_decl: generate_function_stub(func),
                            });
                            continue;
                        }
                    }
                }
            }
        }

        // Ambiguous lifetime check for free functions:
        // If the function returns a reference and has 2+ reference params, Rust can't infer
        // which param the return borrows from. We mark it unsafe instead of skipping.
        // Threshold is 2 (not 1 as for methods) because free functions have no &self.
        let unsafe_lifetime = func.return_type.as_ref()
            .map_or(false, |rt| rt.original.is_reference())
            && func.params.iter().filter(|p| p.ty.original.is_lifetime_source()).count() >= 2;

        let base_rust_name = &func.rust_name;
        let is_overloaded = name_groups.get(base_rust_name).copied().unwrap_or(0) > 1;

        // Compute overload suffix using the same algorithm as class methods
        let rust_ffi_name = if !is_overloaded {
            base_rust_name.clone()
        } else if const_mut_pair_names.contains(base_rust_name) {
            // Const/mut pair: const variant keeps base name, mut variant gets _mut
            let has_mut_ref = func
                .params
                .iter()
                .any(|p| matches!(&p.ty.original, Type::MutRef(_)));
            if has_mut_ref {
                format!("{}_mut", base_rust_name)
            } else {
                base_rust_name.clone()
            }
        } else {
            let param_types: Vec<Type> = func.params.iter()
                .map(|p| p.ty.original.clone())
                .collect();
            let suffix = overload_suffix_for_types(&param_types);
            let candidate = if suffix.is_empty() {
                base_rust_name.clone()
            } else {
                combine_name_suffix(base_rust_name, &suffix)
            };
            // If collision (two overloads with identical param type short names),
            // try _mut suffix for mutable-ref variants before numeric fallback
            if used_names.contains(&candidate) {
                let has_mut_ref = func
                    .params
                    .iter()
                    .any(|p| matches!(&p.ty.original, Type::MutRef(_)));
                if has_mut_ref {
                    let mut_candidate = format!("{}_mut", base_rust_name);
                    if !used_names.contains(&mut_candidate) {
                        mut_candidate
                    } else {
                        let mut counter = 2;
                        loop {
                            let numbered = format!("{}_{}", candidate, counter);
                            if !used_names.contains(&numbered) {
                                break numbered;
                            }
                            counter += 1;
                        }
                    }
                } else {
                    let mut counter = 2;
                    loop {
                        let numbered = format!("{}_{}", candidate, counter);
                        if !used_names.contains(&numbered) {
                            break numbered;
                        }
                        counter += 1;
                    }
                }
            } else {
                candidate
            }
        };

        used_names.insert(rust_ffi_name.clone());
        let cpp_wrapper_name = format!("{}_{}", func.namespace, rust_ffi_name);

        let reexport_ctx = ReexportTypeContext {
            class_public_info: &class_public_info,
            current_module_rust: crate::module_graph::module_to_rust_name(&func.namespace),
        };

        // Build ParamBindings using the shared build_param_binding()
        let params: Vec<ParamBinding> = func.params.iter()
            .map(|p| build_param_binding(&p.name, &p.ty.original, p.is_nullable_ptr(), &ffi_ctx, Some(&reexport_ctx)))
            .collect();

        // Build ReturnTypeBinding
        let return_type = func.return_type.as_ref()
            .map(|rt| build_return_type_binding(&rt.original, &ffi_ctx, Some(&reexport_ctx)));

        // Collect C++ headers needed for this function's types
        let mut headers: HashSet<String> = HashSet::new();
        headers.insert(format!("{}.hxx", func.namespace));
        for p in &func.params {
            collect_headers_for_type(&p.ty.original, &mut headers, known_headers);
        }
        if let Some(ref rt) = func.return_type {
            collect_headers_for_type(&rt.original, &mut headers, known_headers);
        }
        let mut cpp_headers: Vec<String> = headers.into_iter().collect();
        cpp_headers.sort();

        result.push(FunctionBinding {
            rust_ffi_name,
            cpp_wrapper_name,
            namespace: func.namespace.clone(),
            short_name: func.short_name.clone(),
            module: func.rust_module.clone(),
            params,
            return_type,
            source_header: func.source_header.clone(),
            source_line: func.source_line,
            doc_comment: func.doc_comment.clone(),
            cpp_headers,
            is_unsafe: func.params.iter().any(|p| p.ty.original.needs_unsafe_fn())
                || func.return_type.as_ref().map_or(false, |rt| rt.original.needs_unsafe_fn())
                || unsafe_lifetime,
            unsafe_lifetime,
        });
    }

    (result, skipped)
}

// ── Emit functions ──────────────────────────────────────────────────────────

/// Emit C++ wrapper code for a single class from pre-computed ClassBindings.
///
/// Produces C++ wrapper code for a class
/// and its 10+ sub-functions, but consumes the pre-computed IR instead
/// of re-deriving decisions.
pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    let ffi_cn = &bindings.cpp_name;  // Rust-safe flattened name (for FFI function names)
    let cn = &bindings.cpp_qualified_name;  // C++ qualified name (for C++ type expressions)

    // POD structs don't need C++ wrappers, but we generate a sizeof helper
    // so Rust tests can verify layout compatibility at runtime,
    // and a destructor so CppDeletable can be implemented (needed when returned by pointer).
    if bindings.is_pod_struct {
        writeln!(output, "// sizeof helper for POD struct {}", cn).unwrap();
        writeln!(output, "extern \"C\" size_t {}_sizeof() {{ return sizeof({}); }}", ffi_cn, cn).unwrap();
        writeln!(output, "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}", ffi_cn, cn).unwrap();
        writeln!(output).unwrap();
        return output;
    }

    writeln!(output, "// ========================").unwrap();
    writeln!(output, "// {} wrappers", cn).unwrap();
    writeln!(output, "// ========================").unwrap();
    writeln!(output).unwrap();

    // 1. Constructor wrappers (skip convenience — they are Rust-only)
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let params_cpp: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let params_str = params_cpp.join(", ");
        let args_str = ctor.cpp_arg_exprs.join(", ");

        writeln!(
            output,
            "extern \"C\" {cn}* {fn_name}({params_str}) {{",
            fn_name = ctor.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {cn}({args_str});"
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
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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
            "extern \"C\" {ret_cpp}* {fn_name}({all_params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {ret_cpp}(self_->{method}({args_str}));",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 3. Static method wrappers
    // Note: In the old code, static methods were emitted between by-value and cstring wrappers
    // when you look at the call order in generate_class_wrappers. Actually, the order is:
    // by-value → cstring-param → cstring-return → static. Let me re-check...
    // The actual call order in generate_class_wrappers is:
    //   1. constructor
    //   2. return_by_value
    //   3. c_string_param
    //   4. c_string_return
    //   5. static_method
    //   6. upcast
    //   7. to_owned
    //   8. to_handle
    //   9. handle_upcast
    //   9b. handle_downcast
    //   10. inherited_method

    // 3. CStringParam wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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
                "extern \"C\" const char* {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if returns_reference {
            let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;
            writeln!(
                output,
                "extern \"C\" {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else if wm.return_type.is_none() {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            let rt = wm.return_type.as_ref().unwrap();
            let ret_cpp = &rt.cpp_type;
            writeln!(
                output,
                "extern \"C\" {ret_cpp} {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            if rt.enum_cpp_name.is_some() {
                writeln!(
                    output,
                    "    return static_cast<int32_t>(self_->{method}({args_str}));",
                    method = wm.cpp_method_name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    return self_->{method}({args_str});",
                    method = wm.cpp_method_name
                )
                .unwrap();
            }
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
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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
            "extern \"C\" const char* {fn_name}({params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return self_->{method}({args_str});",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 4b. EnumConversion wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::EnumConversion)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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

        let call_expr = format!("self_->{}({})", wm.cpp_method_name, args_str);

        if let Some(ref rt) = wm.return_type {
            if let Some(ref _enum_name) = rt.enum_cpp_name {
                // Enum return: cast to int32_t
                writeln!(
                    output,
                    "extern \"C\" int32_t {fn_name}({params}) {{",
                    fn_name = wm.ffi_fn_name
                )
                .unwrap();
                writeln!(
                    output,
                    "    return static_cast<int32_t>({call_expr});"
                )
                .unwrap();
            } else {
                // Non-enum return (rare for EnumConversion kind, but handle it)
                writeln!(
                    output,
                    "extern \"C\" {} {fn_name}({params}) {{",
                    rt.cpp_type,
                    fn_name = wm.ffi_fn_name
                )
                .unwrap();
                writeln!(output, "    return {call_expr};").unwrap();
            }
        } else {
            // Void return, enum params only
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(output, "    {call_expr};").unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4c. ByValueParam wrapper methods
    // These take const T& at the FFI boundary; the C++ method receives by value (implicit copy).
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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

        if let Some(ref rt) = wm.return_type {
            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params}) {{",
                rt.cpp_type,
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4d. ConstMutReturnFix wrapper methods
    // These are const methods returning &mut T — the wrapper takes non-const self
    // to ensure &mut self is used when returning &mut T.
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ConstMutReturnFix)
    {
        // Always non-const self (that's the fix)
        let self_param = format!("{cn}* self_");

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

        if let Some(ref rt) = wm.return_type {
            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params}) {{",
                rt.cpp_type,
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4e. MutRefEnumParam wrapper methods
    // These have &mut enum output parameters. The wrapper:
    // 1. Takes int32_t& at the FFI boundary
    // 2. Creates local enum variables from the int32_t values
    // 3. Calls the original method
    // 4. Writes back the enum values as int32_t
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::MutRefEnumParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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

        // Determine return type
        let ret_type_cpp = match &wm.return_type {
            Some(rt) if rt.needs_unique_ptr => format!("{}*", rt.cpp_type),
            Some(rt) if rt.enum_cpp_name.is_some() => "int32_t".to_string(),
            Some(rt) => rt.cpp_type.clone(),
            None => "void".to_string(),
        };

        writeln!(
            output,
            "extern \"C\" {} {fn_name}({params}) {{",
            ret_type_cpp,
            fn_name = wm.ffi_fn_name
        )
        .unwrap();

        // Emit preamble: create local enum variables from int32_t input values
        for p in &wm.params {
            if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    auto {local} = static_cast<{enum_name}>({param});",
                    local = p.cpp_arg_expr,
                    param = p.cpp_name,
                )
                .unwrap();
            }
        }

        // Emit the call
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let has_enum_return = wm.return_type.as_ref()
            .and_then(|rt| rt.enum_cpp_name.as_ref())
            .is_some();

        if let Some(ref rt) = wm.return_type {
            if rt.needs_unique_ptr {
                writeln!(
                    output,
                    "    auto result_ = new {cpp_type}(self_->{method}({args_str}));",
                    cpp_type = rt.cpp_type,
                    method = wm.cpp_method_name,
                )
                .unwrap();
            } else if has_enum_return {
                writeln!(
                    output,
                    "    auto result_ = static_cast<int32_t>(self_->{method}({args_str}));",
                    method = wm.cpp_method_name,
                )
                .unwrap();
            } else {
                let auto_kw = if rt.cpp_type.ends_with('&') { "auto&" } else { "auto" };
                writeln!(
                    output,
                    "    {auto_kw} result_ = self_->{method}({args_str});",
                    auto_kw = auto_kw,
                    method = wm.cpp_method_name,
                )
                .unwrap();
            }
        } else {
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name,
            )
            .unwrap();
        }

        // Emit postamble: write back enum values to int32_t& output params
        for p in &wm.params {
            if let Some(ref _enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    {param} = static_cast<int32_t>({local});",
                    param = p.cpp_name,
                    local = p.cpp_arg_expr,
                )
                .unwrap();
            }
        }

        // Emit return
        if wm.return_type.is_some() {
            writeln!(output, "    return result_;").unwrap();
        }

        writeln!(output, "}}").unwrap();
    }

    // 4f. Simple wrapper methods (primitives, void, references, etc.)
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::Simple)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
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

        if let Some(ref rt) = wm.return_type {
            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params}) {{",
                rt.cpp_type,
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        }
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

        let has_enum_return = sm
            .return_type
            .as_ref()
            .and_then(|rt| rt.enum_cpp_name.as_ref())
            .is_some();

        let has_mut_ref_enum = sm.params.iter().any(|p| p.mut_ref_enum_cpp_name.is_some());

        // Check for c_string return (const char* -> const char*)
        let returns_cstring = sm.return_type.as_ref()
            .map(|rt| rt.cpp_type == "const char*")
            .unwrap_or(false);

        if has_mut_ref_enum {
            // Static methods with &mut enum output params need preamble/postamble
            let ret_type_cpp = if needs_up {
                format!("{}*", ret_type)
            } else if has_enum_return {
                "int32_t".to_string()
            } else {
                ret_type.clone()
            };

            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params_str}) {{",
                ret_type_cpp,
                fn_name = sm.ffi_fn_name
            )
            .unwrap();

            // Preamble: create local enum vars
            for p in &sm.params {
                if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                    writeln!(
                        output,
                        "    auto {local} = static_cast<{enum_name}>({param});",
                        local = p.cpp_arg_expr,
                        param = p.cpp_name,
                    )
                    .unwrap();
                }
            }

            // Call
            if let Some(ref rt) = sm.return_type {
                if rt.needs_unique_ptr {
                    writeln!(
                        output,
                        "    auto result_ = new {cpp_type}({cn}::{method}({args_str}));",
                        cpp_type = rt.cpp_type,
                        method = sm.cpp_method_name,
                    )
                    .unwrap();
                } else if has_enum_return {
                    writeln!(
                        output,
                        "    auto result_ = static_cast<int32_t>({cn}::{method}({args_str}));",
                        method = sm.cpp_method_name,
                    )
                    .unwrap();
                } else {
                    let auto_kw = if rt.cpp_type.ends_with('&') { "auto&" } else { "auto" };
                    writeln!(
                        output,
                        "    {auto_kw} result_ = {cn}::{method}({args_str});",
                        auto_kw = auto_kw,
                        method = sm.cpp_method_name,
                    )
                    .unwrap();
                }
            } else {
                writeln!(
                    output,
                    "    {cn}::{method}({args_str});",
                    method = sm.cpp_method_name,
                )
                .unwrap();
            }

            // Postamble: write back enum values
            for p in &sm.params {
                if let Some(ref _enum_name) = p.mut_ref_enum_cpp_name {
                    writeln!(
                        output,
                        "    {param} = static_cast<int32_t>({local});",
                        param = p.cpp_name,
                        local = p.cpp_arg_expr,
                    )
                    .unwrap();
                }
            }

            // Return
            if sm.return_type.is_some() {
                writeln!(output, "    return result_;").unwrap();
            }
        } else if returns_cstring {
            writeln!(
                output,
                "extern \"C\" const char* {fn_name}({params_str}) {{",
                fn_name = sm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return {cn}::{method}({args_str});",
                method = sm.cpp_method_name
            )
            .unwrap();
        } else if needs_up {
            writeln!(
                output,
                "extern \"C\" {ret_type}* {fn_name}({params_str}) {{",
                fn_name = sm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return new {ret_type}({cn}::{method}({args_str}));",
                method = sm.cpp_method_name
            )
            .unwrap();
        } else if has_enum_return {
            writeln!(
                output,
                "extern \"C\" int32_t {fn_name}({params_str}) {{",
                fn_name = sm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return static_cast<int32_t>({cn}::{method}({args_str}));",
                method = sm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "extern \"C\" {ret_type} {fn_name}({params_str}) {{",
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
            "extern \"C\" const {base}* {fn_name}(const {cn}* self_) {{ return static_cast<const {base}*>(self_); }}",
            base = up.base_class_cpp,
            fn_name = up.ffi_fn_name
        )
        .unwrap();
        // Mutable upcast
        writeln!(
            output,
            "extern \"C\" {base}* {fn_name_mut}({cn}* self_) {{ return static_cast<{base}*>(self_); }}",
            base = up.base_class_cpp,
            fn_name_mut = up.ffi_fn_name_mut
        )
        .unwrap();
    }

    // 7. to_owned wrapper
    if bindings.has_to_owned {
        let fn_name = format!("{ffi_cn}_to_owned");
        writeln!(
            output,
            "extern \"C\" {cn}* {fn_name}(const {cn}* self_) {{ return new {cn}(*self_); }}"
        )
        .unwrap();
    }

    // 8. to_handle wrapper
    if bindings.has_to_handle {
        let handle_type = type_mapping::handle_type_name(ffi_cn);
        let fn_name = format!("{ffi_cn}_to_handle");
        writeln!(
            output,
            "extern \"C\" {handle_type}* {fn_name}({cn}* obj) {{"
        )
        .unwrap();
        writeln!(
            output,
            "    return new {handle_type}(obj);"
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 8b. Handle get (dereference) wrapper
    if bindings.has_handle_get {
        let handle_type = type_mapping::handle_type_name(ffi_cn);
        writeln!(
            output,
            "extern \"C\" const {cn}* {handle_type}_get(const {handle_type}* handle) {{ return (*handle).get(); }}"
        )
        .unwrap();
        writeln!(
            output,
            "extern \"C\" {cn}* {handle_type}_get_mut({handle_type}* handle) {{ return (*handle).get(); }}"
        )
        .unwrap();
    }

    // 9. Handle upcast wrappers
    for hup in &bindings.handle_upcasts {
        writeln!(
            output,
            "extern \"C\" {base_handle}* {fn_name}(const {derived_handle}* self_) {{",
            base_handle = hup.base_handle_name,
            fn_name = hup.ffi_fn_name,
            derived_handle = hup.derived_handle_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {base_handle}(*self_);",
            base_handle = hup.base_handle_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 9b. Handle downcast wrappers
    for hdown in &bindings.handle_downcasts {
        writeln!(
            output,
            "extern \"C\" {derived_handle}* {fn_name}(const {base_handle}* self_) {{",
            derived_handle = hdown.derived_handle_name,
            fn_name = hdown.ffi_fn_name,
            base_handle = hdown.base_handle_name
        )
        .unwrap();
        writeln!(
            output,
            "    opencascade::handle<{derived_class}> result = opencascade::handle<{derived_class}>::DownCast(*self_);",
            derived_class = hdown.derived_class
        )
        .unwrap();
        writeln!(output, "    if (result.IsNull()) return nullptr;").unwrap();
        writeln!(
            output,
            "    return new {derived_handle}(result);",
            derived_handle = hdown.derived_handle_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 10. Inherited method wrappers
    for im in &bindings.inherited_methods {
        let self_param = if im.is_const {
            format!("const {cn}* self")
        } else {
            format!("{cn}* self")
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

        let has_mut_ref_enum = im.params.iter().any(|p| p.mut_ref_enum_cpp_name.is_some());

        let (ret_type_cpp, needs_up) = match &im.return_type {
            Some(rt) => {
                if rt.needs_unique_ptr {
                    (format!("{}*", rt.cpp_type), true)
                } else if rt.enum_cpp_name.is_some() {
                    ("int32_t".to_string(), false)
                } else {
                    (rt.cpp_type.clone(), false)
                }
            }
            None => ("void".to_string(), false),
        };

        writeln!(
            output,
            "extern \"C\" {ret_type_cpp} {fn_name}({params}) {{",
            fn_name = im.ffi_fn_name
        )
        .unwrap();

        // Preamble: create local enum variables from int32_t for &mut enum params
        for p in &im.params {
            if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    auto {local} = static_cast<{enum_name}>({param});",
                    local = p.cpp_arg_expr,
                    param = p.name,
                )
                .unwrap();
            }
        }

        let has_enum_return = im
            .return_type
            .as_ref()
            .and_then(|rt| rt.enum_cpp_name.as_ref())
            .is_some();

        if has_mut_ref_enum {
            // Multi-statement pattern: call, postamble, return
            if let Some(ref rt) = im.return_type {
                if needs_up {
                    writeln!(
                        output,
                        "    auto result_ = new {inner_type}(self->{method}({args_str}));",
                        inner_type = rt.cpp_type,
                        method = im.cpp_method_name
                    )
                    .unwrap();
                } else if has_enum_return {
                    writeln!(
                        output,
                        "    auto result_ = static_cast<int32_t>(self->{method}({args_str}));",
                        method = im.cpp_method_name
                    )
                    .unwrap();
                } else {
                    let auto_kw = if rt.cpp_type.ends_with('&') { "auto&" } else { "auto" };
                    writeln!(
                        output,
                        "    {auto_kw} result_ = self->{method}({args_str});",
                        auto_kw = auto_kw,
                        method = im.cpp_method_name
                    )
                    .unwrap();
                }
            } else {
                writeln!(
                    output,
                    "    self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            }

            // Postamble: write back enum to int32_t&
            for p in &im.params {
                if let Some(ref _enum_name) = p.mut_ref_enum_cpp_name {
                    writeln!(
                        output,
                        "    {param} = static_cast<int32_t>({local});",
                        param = p.name,
                        local = p.cpp_arg_expr,
                    )
                    .unwrap();
                }
            }

            if im.return_type.is_some() {
                writeln!(output, "    return result_;").unwrap();
            }
        } else {
            // Simple single-statement pattern (no &mut enum params)
            if needs_up {
                writeln!(
                    output,
                    "    return new {inner_type}(self->{method}({args_str}));",
                    inner_type = im.return_type.as_ref().unwrap().cpp_type,
                    method = im.cpp_method_name
                )
                .unwrap();
            } else if has_enum_return {
                writeln!(
                    output,
                    "    return static_cast<int32_t>(self->{method}({args_str}));",
                    method = im.cpp_method_name
                )
                .unwrap();
            } else if im.return_type.is_some() {
                writeln!(
                    output,
                    "    return self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            }
        }

        writeln!(output, "}}").unwrap();
    }

    // 11. Destructor wrapper
    if !bindings.has_protected_destructor {
        writeln!(
            output,
            "extern \"C\" void {ffi_cn}_destructor({cn}* self_) {{ delete self_; }}"
        )
        .unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", p.rust_name)
        }
    } else if p.is_class_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{} as *const _", p.rust_name)
        } else {
            format!("{} as *mut _", p.rust_name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", p.rust_name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", p.rust_name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", p.rust_name)
    } else {
        p.rust_name.clone()
    }
}

fn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", name)
        }
    } else if p.is_class_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{} as *const _", name)
        } else {
            format!("{} as *mut _", name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", name)
    } else {
        name.to_string()
    }
}

/// Generate let-bindings for CString (&str) params and &mut enum params.
/// These must appear before the unsafe block so the temporaries live long enough.
fn cstr_prelude_params(params: &[ParamBinding]) -> String {
    let mut result = String::new();
    for p in params {
        if p.rust_reexport_type == "&str" {
            result.push_str(&format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name));
        }
        if p.mut_ref_enum_rust_type.is_some() {
            result.push_str(&format!("        let mut {}_i32_: i32 = (*{}).into();\n", p.rust_name, p.rust_name));
        }
    }
    result
}

fn cstr_prelude_resolved(params: &[ResolvedParamBinding], names: &[String]) -> String {
    let mut result = String::new();
    for (p, name) in params.iter().zip(names.iter()) {
        if p.rust_reexport_type == "&str" {
            result.push_str(&format!("        let c_{} = std::ffi::CString::new({}).unwrap();\n", name, name));
        }
        if p.mut_ref_enum_rust_type.is_some() {
            result.push_str(&format!("        let mut {}_i32_: i32 = (*{}).into();\n", name, name));
        }
    }
    result
}

/// Generate the postamble for &mut enum params: write back from i32 to typed enum.
fn mut_ref_enum_postamble_params(params: &[ParamBinding], indent: &str) -> String {
    let mut result = String::new();
    for p in params {
        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {
            result.push_str(&format!("{}*{} = {}::try_from({}_i32_).unwrap();\n", indent, p.rust_name, enum_type, p.rust_name));
        }
    }
    result
}

fn mut_ref_enum_postamble_resolved(params: &[ResolvedParamBinding], names: &[String], indent: &str) -> String {
    let mut result = String::new();
    for (p, name) in params.iter().zip(names.iter()) {
        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {
            result.push_str(&format!("{}*{} = {}::try_from({}_i32_).unwrap();\n", indent, name, enum_type, name));
        }
    }
    result
}

/// Wrap a reexport body expression with &mut enum writeback postamble.
/// When postamble is non-empty, splits into multi-statement body:
///   let result_ = <body>;
///   <postamble>
///   result_
/// For void returns (has_return=false):
///   <body>;
///   <postamble trimmed>
fn wrap_body_with_postamble(body: &str, postamble: &str, has_return: bool) -> String {
    if postamble.is_empty() {
        return body.to_string();
    }
    if has_return {
        format!("let result_ = {};\n{}        result_", body, postamble)
    } else {
        // Void return: body as statement, then postamble (trim trailing newline for last line)
        let trimmed_postamble = postamble.trim_end_matches('\n');
        format!("{};\n{}", body, trimmed_postamble)
    }
}

/// Build the body expression for a re-export method call.
/// Handles the conversion from FFI raw pointer returns to Rust references/OwnedPtr.
fn build_reexport_body(raw_call: &str, reexport_type: Option<&str>, is_enum: Option<&str>, needs_owned_ptr: bool, is_class_ptr_return: bool) -> String {
    if is_class_ptr_return {
        // Class pointer returns are bound as Option<&T> / Option<&mut T>.
        // The FFI returns a raw pointer; we null-check and convert.
        if let Some(rt) = reexport_type {
            if rt.starts_with("Option<&mut ") {
                return format!("{{ let ptr = unsafe {{ {} }}; if ptr.is_null() {{ None }} else {{ Some(unsafe {{ &mut *ptr }}) }} }}", raw_call);
            }
        }
        return format!("{{ let ptr = unsafe {{ {} }}; if ptr.is_null() {{ None }} else {{ Some(unsafe {{ &*ptr }}) }} }}", raw_call);
    }
    if let Some(enum_type) = is_enum {
        format!("unsafe {{ {}::try_from({}).unwrap() }}", enum_type, raw_call)
    } else if needs_owned_ptr {
        format!("unsafe {{ crate::OwnedPtr::from_raw({}) }}", raw_call)
    } else if let Some(rt) = reexport_type {
        if rt == "std::string::String" {
            format!("unsafe {{ std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned() }}", raw_call)
        } else if rt.starts_with("&mut ") {
            format!("unsafe {{ &mut *({}) }}", raw_call)
        } else if rt.starts_with('&') {
            format!("unsafe {{ &*({}) }}", raw_call)
        } else {
            format!("unsafe {{ {} }}", raw_call)
        }
    } else {
        format!("unsafe {{ {} }}", raw_call)
    }
}

pub fn emit_reexport_class(bindings: &ClassBindings, module_name: &str) -> String {
    let cn = &bindings.cpp_name;
    let short_name = &bindings.short_name;

    let mut output = String::new();

    // Source attribution + doc comment for the class
    let class_source = format_source_attribution(
        &bindings.source_header,
        bindings.source_line,
        cn,
    );
    output.push_str(&format!("/// {}\n", class_source));
    if let Some(ref comment) = bindings.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                output.push_str("///\n");
            } else {
                output.push_str(&format!("/// {}\n", trimmed));
            }
        }
    }

    // Type alias re-export
    output.push_str(&format!(
        "pub use crate::ffi::{} as {};\n\n",
        cn, short_name
    ));

    // POD structs are Copy types with real fields.
    // They still need CppDeletable because other classes may return them by pointer.
    if bindings.is_pod_struct {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
        return output;
    }

    // CppDeletable impl (unless protected destructor)
    if !bindings.has_protected_destructor {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
    }

    // Build impl methods
    let mut impl_methods: Vec<String> = Vec::new();

    // 1. Constructors
    for ctor in &bindings.constructors {
        let params: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = ctor.params.iter().map(|p| convert_arg(p)).collect();

        let source_attr = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        let doc = format_reexport_doc(&source_attr, &ctor.doc_comment);

        if let Some(ref conv) = ctor.convenience_of {
            // Convenience constructor: Rust-only wrapper that delegates to full-arg version
            // Use raw param names (no CString conversion) since the target method handles it
            let convenience_args: Vec<String> = ctor.params.iter().map(|p| p.rust_name.clone()).collect();
            let mut all_args = convenience_args;
            all_args.extend(conv.default_exprs.iter().cloned());
            let unsafe_kw = if ctor.is_unsafe { "unsafe " } else { "" };
            impl_methods.push(format!(
                "{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                unsafe_kw,
                ctor.impl_method_name,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            let unsafe_kw = if ctor.is_unsafe { "unsafe " } else { "" };
            impl_methods.push(format!(
                "{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
                unsafe_kw,
                ctor.impl_method_name,
                params.join(", "),
                prelude,
                ctor.ffi_fn_name,
                args.join(", ")
            ));
        }
    }

    // 2. Wrapper methods (impl delegates to ffi free functions)
    for wm in &bindings.wrapper_methods {
        let self_param = if wm.is_const {
            "&self".to_string()
        } else {
            "&mut self".to_string()
        };

        let self_arg = if wm.is_const {
            "self as *const Self".to_string()
        } else {
            "self as *mut Self".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                wm.params
                    .iter()
                    .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type)),
            )
            .collect();
        let args: Vec<String> = std::iter::once(self_arg)
            .chain(wm.params.iter().map(|p| convert_arg(p)))
            .collect();

        let return_type = wm
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        let raw_call = format!("crate::ffi::{}({})", wm.ffi_fn_name, args.join(", "));
        let is_enum_return = wm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = wm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = wm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&wm.params);

        let is_class_ptr_ret = wm.return_type.as_ref().map_or(false, |rt| rt.is_class_ptr_return);
        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr, is_class_ptr_ret);
        let postamble = mut_ref_enum_postamble_params(&wm.params, "        ");
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        let source_attr = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        let mut doc = format_reexport_doc(&source_attr, &wm.doc_comment);
        if wm.unsafe_lifetime {
            doc.push_str(format_lifetime_safety_doc());
        }
        let unsafe_kw = if wm.is_unsafe { "unsafe " } else { "" };
        impl_methods.push(format!(
            "{}    pub {}fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            unsafe_kw,
            wm.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // 2b. Direct methods (also delegates to ffi free functions, same pattern as wrappers)
    for dm in &bindings.direct_methods {
        let self_param = if dm.is_const {
            "&self".to_string()
        } else {
            "&mut self".to_string()
        };

        let self_arg = if dm.is_const {
            "self as *const Self".to_string()
        } else {
            "self as *mut Self".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                dm.params
                    .iter()
                    .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type)),
            )
            .collect();
        let args: Vec<String> = std::iter::once(self_arg)
            .chain(dm.params.iter().map(|p| convert_arg(p)))
            .collect();

        let return_type = dm
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        let ffi_fn_name = format!("{}_{}", cn, dm.rust_name);
        let raw_call = format!("crate::ffi::{}({})", ffi_fn_name, args.join(", "));
        let is_enum_return = dm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = dm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = dm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&dm.params);

        let is_class_ptr_ret = dm.return_type.as_ref().map_or(false, |rt| rt.is_class_ptr_return);
        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr, is_class_ptr_ret);
        let postamble = mut_ref_enum_postamble_params(&dm.params, "        ");
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        let source_attr = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        let mut doc = format_reexport_doc(&source_attr, &dm.doc_comment);
        if dm.unsafe_lifetime {
            doc.push_str(format_lifetime_safety_doc());
        }
        let unsafe_kw = if dm.is_unsafe { "unsafe " } else { "" };
        impl_methods.push(format!(
            "{}    pub {}fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            unsafe_kw,
            dm.rust_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

    // 3. Static methods
    for sm in &bindings.static_methods {
        let params: Vec<String> = sm
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = sm.params.iter().map(|p| convert_arg(p)).collect();

        let return_type = sm
            .return_type
            .as_ref()
            .map(|rt| {
                let mut ty_str = rt.rust_reexport_type.clone();
                if sm.needs_static_lifetime {
                    if ty_str.starts_with('&') && !ty_str.contains("'static") {
                        ty_str = ty_str.replacen('&', "&'static ", 1);
                    } else if ty_str.starts_with("Option<&") && !ty_str.contains("'static") {
                        ty_str = ty_str.replacen("Option<&", "Option<&'static ", 1);
                    }
                }
                format!(" -> {}", ty_str)
            })
            .unwrap_or_default();

        let source_attr = format_source_attribution(
            &bindings.source_header,
            sm.source_line,
            &format!("{}::{}()", cn, sm.cpp_method_name),
        );
        let doc = format_reexport_doc(&source_attr, &sm.doc_comment);
        let raw_call = format!("crate::ffi::{}({})", sm.ffi_fn_name, args.join(", "));
        let is_enum_return = sm.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = sm.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = sm.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_params(&sm.params);

        let is_class_ptr_ret = sm.return_type.as_ref().map_or(false, |rt| rt.is_class_ptr_return);
        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr, is_class_ptr_ret);
        let postamble = mut_ref_enum_postamble_params(&sm.params, "        ");
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        let unsafe_kw = if sm.is_unsafe { "unsafe " } else { "" };
        impl_methods.push(format!(
            "{}    pub {}fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            unsafe_kw,
            sm.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
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
            "    /// Upcast to {}\n    pub fn {}(&self) -> &{} {{\n        unsafe {{ &*(crate::ffi::{}(self as *const Self)) }}\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name
        ));

        impl_methods.push(format!(
            "    /// Upcast to {} (mutable)\n    pub fn {}_mut(&mut self) -> &mut {} {{\n        unsafe {{ &mut *(crate::ffi::{}(self as *mut Self)) }}\n    }}\n",
            up.base_class, up.impl_method_name, ret_type, up.ffi_fn_name_mut
        ));
    }

    // 5. to_owned
    if bindings.has_to_owned {
        let ffi_fn_name = format!("{}_to_owned", cn);
        impl_methods.push(format!(
            "    /// Clone into a new OwnedPtr via copy constructor\n    pub fn to_owned(&self) -> crate::OwnedPtr<Self> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}(self as *const Self)) }}\n    }}\n",
            ffi_fn_name
        ));
    }

    // 6. to_handle
    if bindings.has_to_handle {
        let ffi_fn_name = format!("{}_to_handle", cn);
        let handle_type_name = type_mapping::handle_type_name(cn);
        impl_methods.push(format!(
            "    /// Wrap in a Handle (reference-counted smart pointer)\n    pub fn to_handle(obj: crate::OwnedPtr<Self>) -> crate::OwnedPtr<crate::ffi::{}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}(obj.into_raw())) }}\n    }}\n",
            handle_type_name, ffi_fn_name
        ));
    }

    // 7. Inherited methods (delegates to inherited wrapper free functions)
    for im in &bindings.inherited_methods {
        let self_param = if im.is_const {
            "&self".to_string()
        } else {
            "&mut self".to_string()
        };

        let self_arg = if im.is_const {
            "self as *const Self".to_string()
        } else {
            "self as *mut Self".to_string()
        };

        let params: Vec<String> = std::iter::once(self_param)
            .chain(
                im.params
                    .iter()
                    .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_reexport_type)),
            )
            .collect();
        let param_names: Vec<String> = im.params.iter().map(|p| safe_param_name(&p.rust_name)).collect();
        let args: Vec<String> = std::iter::once(self_arg)
            .chain(im.params.iter().zip(param_names.iter()).map(|(p, name)| {
                convert_arg_resolved(name, p)
            }))
            .collect();

        let return_type = im
            .return_type
            .as_ref()
            .map(|rt| format!(" -> {}", rt.rust_reexport_type))
            .unwrap_or_default();

        let raw_call = format!("crate::ffi::{}({})", im.ffi_fn_name, args.join(", "));
        let is_enum_return = im.return_type.as_ref().and_then(|rt| rt.enum_rust_type.as_ref());
        let needs_owned_ptr = im.return_type.as_ref().map_or(false, |rt| rt.needs_unique_ptr);
        let reexport_rt = im.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());

        let prelude = cstr_prelude_resolved(&im.params, &param_names);

        let is_class_ptr_ret = im.return_type.as_ref().map_or(false, |rt| rt.is_class_ptr_return);
        let body = build_reexport_body(&raw_call, reexport_rt, is_enum_return.map(|s| s.as_str()), needs_owned_ptr, is_class_ptr_ret);
        let postamble = mut_ref_enum_postamble_resolved(&im.params, &param_names, "        ");
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        let no_doc: Option<String> = None;
        let mut inherited_doc = format_reexport_doc(
            &format!("Inherited: {}", format_source_attribution(
                &im.source_header,
                im.source_line,
                &format!("{}::{}()", im.source_class, im.cpp_method_name),
            )),
            &no_doc,
        );
        if im.unsafe_lifetime {
            inherited_doc.push_str(format_lifetime_safety_doc());
        }
        let unsafe_kw = if im.is_unsafe { "unsafe " } else { "" };
        impl_methods.push(format!(
            "{}    pub {}fn {}({}){} {{\n{}        {}\n    }}\n",
            inherited_doc,
            unsafe_kw,
            im.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
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

    // Handle type re-export, CppDeletable, get method, and handle upcast methods
    if bindings.has_handle_get {
        let handle_type_name = type_mapping::handle_type_name(cn);
        // Re-export the handle type so external crates can name it
        output.push_str(&format!(
            "pub use crate::ffi::{};\n\n",
            handle_type_name
        ));

        // CppDeletable for handle type
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            handle_type_name, handle_type_name
        ));

        // Generate impl block with get(), get_mut(), and upcast methods
        output.push_str(&format!("impl {} {{\n", handle_type_name));
        // get() - dereference handle to &T
        output.push_str(&format!(
            "    /// Dereference this Handle to access the underlying {}\n    pub fn get(&self) -> &crate::ffi::{} {{\n        unsafe {{ &*(crate::ffi::{}_get(self as *const Self)) }}\n    }}\n",
            cn, cn, handle_type_name
        ));
        // get_mut() - dereference handle to &mut T
        output.push_str(&format!(
            "    /// Dereference this Handle to mutably access the underlying {}\n    pub fn get_mut(&mut self) -> &mut crate::ffi::{} {{\n        unsafe {{ &mut *(crate::ffi::{}_get_mut(self as *mut Self)) }}\n    }}\n",
            cn, cn, handle_type_name
        ));
        // Build upcast method names with robust deduplication.
        let upcast_method_names = deduplicate_method_names(
            &bindings.handle_upcasts.iter().map(|hu| {
                let flattened = hu.base_class.replace("::", "_");
                let short = crate::type_mapping::short_name_for_module(&flattened, &hu.base_module).to_snake_case();
                let full = flattened.to_snake_case();
                (format!("to_handle_{}", short), format!("to_handle_{}", full))
            }).collect::<Vec<_>>()
        );
        for (i, hu) in bindings.handle_upcasts.iter().enumerate() {
            output.push_str(&format!(
                "    /// Upcast Handle<{cn}> to Handle<{base}>\n    pub fn {method}(&self) -> crate::OwnedPtr<crate::ffi::{base_handle}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{ffi_fn}(self as *const Self)) }}\n    }}\n",
                cn = cn,
                base = hu.base_class,
                method = upcast_method_names[i],
                base_handle = hu.base_handle_name,
                ffi_fn = hu.ffi_fn_name,
            ));
        }
        // Build downcast method names with robust deduplication.
        let downcast_method_names = deduplicate_method_names(
            &bindings.handle_downcasts.iter().map(|hd| {
                let flattened = hd.derived_class.replace("::", "_");
                let short = crate::type_mapping::short_name_for_module(&flattened, &hd.derived_module).to_snake_case();
                let full = flattened.to_snake_case();
                (format!("downcast_to_{}", short), format!("downcast_to_{}", full))
            }).collect::<Vec<_>>()
        );
        for (i, hd) in bindings.handle_downcasts.iter().enumerate() {
            output.push_str(&format!(
                "    /// Downcast Handle<{cn}> to Handle<{derived}>\n    ///\n    /// Returns `None` if the handle does not point to a `{derived}` (or subclass).\n    pub fn {method}(&self) -> Option<crate::OwnedPtr<crate::ffi::{derived_handle}>> {{\n        let ptr = unsafe {{ crate::ffi::{ffi_fn}(self as *const Self) }};\n        if ptr.is_null() {{ None }} else {{ Some(unsafe {{ crate::OwnedPtr::from_raw(ptr) }}) }}\n    }}\n",
                cn = cn,
                derived = hd.derived_class,
                method = downcast_method_names[i],
                derived_handle = hd.derived_handle_name,
                ffi_fn = hd.ffi_fn_name,
            ));
        }
        output.push_str("}\n\n");
    }

    // Emit skipped symbols as comments
    if !bindings.skipped_symbols.is_empty() {
        output.push_str(&format!("// ── Skipped symbols for {} ({} total) ──\n", short_name, bindings.skipped_symbols.len()));
        for skip in &bindings.skipped_symbols {
            let source_attr = format_source_attribution(
                &skip.source_header,
                skip.source_line,
                &skip.cpp_name,
            );
            output.push_str(&format!("// SKIPPED: {}\n", source_attr));
            if let Some(ref doc) = skip.doc_comment {
                for line in doc.lines().take(3) {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        output.push_str(&format!("//   {}: {}\n", skip.kind, trimmed));
                    }
                }
            }
            output.push_str(&format!("//   Reason: {}\n", skip.skip_reason));
            output.push_str(&format!("//   // {}\n", skip.stub_rust_decl));
            output.push_str("//\n");
        }
        output.push('\n');
    }

    output
}

/// Emit comments for skipped free functions in a module's re-export file.
pub fn emit_skipped_functions(skipped: &[SkippedSymbol]) -> String {
    if skipped.is_empty() {
        return String::new();
    }
    let mut output = String::new();
    output.push_str(&format!("// ── Skipped free functions ({} total) ──\n", skipped.len()));
    for skip in skipped {
        let source_attr = format_source_attribution(
            &skip.source_header,
            skip.source_line,
            &skip.cpp_name,
        );
        output.push_str(&format!("// SKIPPED: {}\n", source_attr));
        if let Some(ref doc) = skip.doc_comment {
            for line in doc.lines().take(3) {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    output.push_str(&format!("//   {}: {}\n", skip.kind, trimmed));
                }
            }
        }
        output.push_str(&format!("//   Reason: {}\n", skip.skip_reason));
        output.push_str(&format!("//   // {}\n", skip.stub_rust_decl));
        output.push_str("//\n");
    }
    output.push('\n');
    output
}

/// Format source attribution + optional doc comment for re-export impl methods (indented with 4 spaces).
fn format_reexport_doc(source_attr: &str, doc: &Option<String>) -> String {
    let mut out = format!("    /// {}\n", source_attr);
    if let Some(comment) = doc {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                out.push_str("    ///\n");
            } else {
                out.push_str(&format!("    /// {}\n", trimmed));
            }
        }
    }
    out
}

/// Format a `# Safety` doc comment section for methods with ambiguous return lifetimes.
fn format_lifetime_safety_doc() -> &'static str {
    "    ///\n    /// # Safety\n    ///\n    /// It is not known whether the returned reference borrows from `self` or from one\n    /// of the reference parameters. The caller must ensure the returned reference does\n    /// not outlive whichever source it actually borrows from.\n"
}

/// Format a `# Safety` doc comment section for free functions with ambiguous return lifetimes.
pub fn format_lifetime_safety_doc_free_fn() -> &'static str {
    "///\n/// # Safety\n///\n/// It is not known which reference parameter the returned reference borrows from.\n/// The caller must ensure the returned reference does not outlive whichever source\n/// it actually borrows from.\n"
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
/// Returns a string fragment to be inserted inside `extern "C" { ... }`.
/// All declarations are indented with 4 spaces.
pub fn emit_ffi_class(bindings: &ClassBindings) -> String {
    // POD structs are defined as #[repr(C)] with real fields — they only
    // need a sizeof helper for layout verification.
    if bindings.is_pod_struct {
        let cn = &bindings.cpp_name;
        let mut out = String::new();
        writeln!(out, "    // ======================== {} (POD) ========================", cn).unwrap();
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
        writeln!(out, "    pub fn {}_sizeof() -> usize;", cn).unwrap();
        return out;
    }

    let cn = &bindings.cpp_name;
    let mut out = String::new();

    // Section header
    writeln!(out, "    // ======================== {} ========================", cn).unwrap();

    // ── Destructor ──────────────────────────────────────────────────────
    if !bindings.has_protected_destructor {
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
    }

    // ── Constructors (skip convenience wrappers — they are Rust-only) ──
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let source = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        emit_ffi_doc_4(&mut out, &source, &ctor.doc_comment);

        let params_str = format_params(&ctor.params);
        writeln!(out, "    pub fn {}({}) -> *mut {};", ctor.ffi_fn_name, params_str, cn).unwrap();
    }

    // ── Direct methods — with extern "C", these become wrapper functions too ──
    for dm in &bindings.direct_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        emit_ffi_doc_4(&mut out, &source, &dm.doc_comment);

        let self_param = if dm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&dm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&dm.return_type);
        writeln!(out, "    pub fn {}_{}({}){};", cn, dm.rust_name, all_params, ret).unwrap();
    }

    // ── Wrapper methods (free functions with self_ parameter) ────────────
    for wm in &bindings.wrapper_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        emit_ffi_doc_4(&mut out, &source, &wm.doc_comment);

        let self_param = if wm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&wm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&wm.return_type);
        writeln!(out, "    pub fn {}({}){};", wm.ffi_fn_name, all_params, ret).unwrap();
    }

    // ── Static methods ──────────────────────────────────────────────────
    for sm in &bindings.static_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            sm.source_line,
            &format!("{}::{}()", cn, sm.cpp_method_name),
        );
        emit_ffi_doc_4(&mut out, &source, &sm.doc_comment);

        let params_str = format_params(&sm.params);
        let ret = if let Some(ref rt) = sm.return_type {
            format!(" -> {}", rt.rust_ffi_type)
        } else {
            String::new()
        };
        writeln!(out, "    pub fn {}({}){};", sm.ffi_fn_name, params_str, ret).unwrap();
    }

    // ── Upcasts ─────────────────────────────────────────────────────────
    for up in &bindings.upcasts {
        writeln!(out, "    /// Upcast {} to {}", cn, up.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *const {};", up.ffi_fn_name, cn, up.base_class).unwrap();
        writeln!(out, "    /// Upcast {} to {} (mutable)", cn, up.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *mut {}) -> *mut {};", up.ffi_fn_name_mut, cn, up.base_class).unwrap();
    }

    // ── to_owned ────────────────────────────────────────────────────────
    if bindings.has_to_owned {
        writeln!(out, "    /// Clone via copy constructor").unwrap();
        writeln!(out, "    pub fn {}_to_owned(self_: *const {}) -> *mut {};", cn, cn, cn).unwrap();
    }

    // ── to_handle ───────────────────────────────────────────────────────
    if bindings.has_to_handle {
        let handle_type_name = type_mapping::handle_type_name(cn);
        writeln!(out, "    /// Wrap {} in a Handle", cn).unwrap();
        writeln!(out, "    pub fn {}_to_handle(obj: *mut {}) -> *mut {};", cn, cn, handle_type_name).unwrap();
    }

    // ── Handle get (dereference) ─────────────────────────────────────────
    if bindings.has_handle_get {
        let handle_type_name = type_mapping::handle_type_name(cn);
        writeln!(out, "    /// Destroy Handle<{}>", cn).unwrap();
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", handle_type_name, handle_type_name).unwrap();
        writeln!(out, "    /// Dereference Handle to get *const {}", cn).unwrap();
        writeln!(out, "    pub fn {}_get(handle: *const {}) -> *const {};", handle_type_name, handle_type_name, cn).unwrap();
        writeln!(out, "    /// Dereference Handle to get *mut {}", cn).unwrap();
        writeln!(out, "    pub fn {}_get_mut(handle: *mut {}) -> *mut {};", handle_type_name, handle_type_name, cn).unwrap();
    }

    // ── Handle upcasts ──────────────────────────────────────────────────
    for hu in &bindings.handle_upcasts {
        writeln!(out, "    /// Upcast Handle<{}> to Handle<{}>", cn, hu.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *mut {};", hu.ffi_fn_name, hu.derived_handle_name, hu.base_handle_name).unwrap();
    }

    // ── Handle downcasts ─────────────────────────────────────────────────────
    for hd in &bindings.handle_downcasts {
        writeln!(out, "    /// Downcast Handle<{}> to Handle<{}> (returns null on failure)", cn, hd.derived_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *mut {};", hd.ffi_fn_name, hd.base_handle_name, hd.derived_handle_name).unwrap();
    }

    // ── Inherited methods (free functions with self_ parameter) ─────────
    for im in &bindings.inherited_methods {
        let source = format_source_attribution(
            &im.source_header,
            im.source_line,
            &format!("{}::{}()", im.source_class, im.cpp_method_name),
        );
        writeln!(out, "    /// Inherited: {}", source).unwrap();

        let self_param = if im.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
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
        let ret = im.return_type.as_ref()
            .map(|rt| format!(" -> {}", rt.rust_ffi_type))
            .unwrap_or_default();
        writeln!(out, "    pub fn {}({}){};", im.ffi_fn_name, all_params, ret).unwrap();
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


/// Emit source attribution only for ffi.rs (indented 4 spaces, no doc comments).
fn emit_ffi_doc_4(out: &mut String, source: &str, _comment: &Option<String>) {
    writeln!(out, "    /// {}", source).unwrap();
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
            base_classes: Vec::new(),
            has_protected_destructor: false,
            is_abstract: false,
            pure_virtual_methods: HashSet::new(),
            has_explicit_constructors: false,
            fields: Vec::new(),
            is_pod_struct: false,
            has_copy_constructor: None,
            has_move_constructor: false,
        };

        let all_class_names: HashSet<String> = ["gp_Pnt".to_string()].into();
        let all_enum_names: HashSet<String> = HashSet::new();
        let handle_able_classes: HashSet<String> = HashSet::new();

        let ffi_ctx = TypeContext {
            current_module: "ffi",
            module_classes: &all_class_names,
            all_enums: &all_enum_names,
            all_classes: &all_class_names,
            handle_able_classes: Some(&handle_able_classes),
            type_to_module: None,
            enum_rust_types: None,
            deletable_class_names: None,
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
            handle_able_classes: HashSet::new(),
            cross_module_types: HashMap::new(),
            type_to_module: HashMap::new(),
            enum_rust_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("gp_Pnt".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            None,
            &HashSet::new(),
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
            base_classes: vec!["Standard_Transient".to_string()],
            has_protected_destructor: false,
            is_abstract: true,
            pure_virtual_methods: HashSet::new(),
            has_explicit_constructors: true,
            fields: Vec::new(),
            is_pod_struct: false,
            has_copy_constructor: None,
            has_move_constructor: false,
        };

        let all_class_names: HashSet<String> =
            ["Geom_Curve".to_string()].into();
        let all_enum_names: HashSet<String> = HashSet::new();
        let handle_able_classes: HashSet<String> =
            ["Geom_Curve".to_string()].into();

        let ffi_ctx = TypeContext {
            current_module: "ffi",
            module_classes: &all_class_names,
            all_enums: &all_enum_names,
            all_classes: &all_class_names,
            handle_able_classes: Some(&handle_able_classes),
            type_to_module: None,
            enum_rust_types: None,
            deletable_class_names: None,
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
            handle_able_classes: ["Geom_Curve".to_string()].into(),
            cross_module_types: HashMap::new(),
            type_to_module: HashMap::new(),
            enum_rust_types: HashMap::new(),
        };

        let all_classes_by_name: HashMap<String, &ParsedClass> =
            [("Geom_Curve".to_string(), &class)].into();

        let bindings = compute_class_bindings(
            &class,
            &ffi_ctx,
            &symbol_table,
            &handle_able_classes,
            &all_classes_by_name,
            None,
            &HashSet::new(),
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
                    default_value: None,
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
                    default_value: None,
                }],
                return_type: None,
                source_line: Some(20),
            },
        ];

        let method_refs: Vec<&Method> = methods.iter().collect();
        let names = compute_wrapper_method_names(&method_refs, &HashSet::new());

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
