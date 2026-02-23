//! Internal representation (IR) for parsed C++ declarations
//!
//! These types represent the parsed information from OCCT headers
//! in a form suitable for code generation.

#![allow(dead_code)] // Some fields/methods are reserved for future use

use std::collections::HashSet;
use std::path::PathBuf;

/// A parsed header file containing class declarations
#[derive(Debug, Clone)]
pub struct ParsedHeader {
    /// Path to the header file
    pub path: PathBuf,
    /// Classes defined in this header
    pub classes: Vec<ParsedClass>,
    /// Enums defined in this header
    pub enums: Vec<ParsedEnum>,
    /// Free functions (namespace-level) defined in this header
    pub functions: Vec<ParsedFunction>,
}

/// A parsed free function (namespace-level function like TopoDS::Edge)
#[derive(Debug, Clone)]
pub struct ParsedFunction {
    /// Full function name (e.g., "TopoDS::Edge")
    pub name: String,
    /// Namespace name (e.g., "TopoDS")
    pub namespace: String,
    /// Simple function name without namespace (e.g., "Edge")
    pub short_name: String,
    /// Module name derived from namespace
    pub module: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Source header file name (e.g., "TopoDS.hxx")
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Parameters
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
}

impl ParsedFunction {
    /// Check if this function has any unbindable types
    pub fn has_unbindable_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            if ret.is_unbindable() {
                return true;
            }
        }
        false
    }

    /// Check if this function has any unsafe raw pointer types that require
    /// the function to be marked `unsafe fn`.
    pub fn has_unsafe_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            if ret.needs_unsafe_fn() {
                return true;
            }
        }
        false
    }
}

/// A parsed C++ enum
#[derive(Debug, Clone)]
pub struct ParsedEnum {
    /// Full enum name (e.g., "TopAbs_ShapeEnum")
    pub name: String,
    /// Module name extracted from prefix
    pub module: String,
    /// Documentation comment from the header
    pub comment: Option<String>,
    /// Source header file name (e.g., "TopAbs_ShapeEnum.hxx")
    pub source_header: String,
    /// Enum variants
    pub variants: Vec<EnumVariant>,
}

/// A single enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    /// Variant name (e.g., "TopAbs_COMPOUND")
    pub name: String,
    /// Explicit value if specified
    pub value: Option<i64>,
    /// Documentation comment
    pub comment: Option<String>,
}

/// A public data member (field) of a class or struct
#[derive(Debug, Clone)]
pub struct ParsedField {
    /// Field name (e.g., "myPeriodic")
    pub name: String,
    /// Field type
    pub ty: Type,
    /// Array size if this is a fixed-size array (e.g., 3 for `bool myPeriodic[3]`)
    pub array_size: Option<usize>,
    /// Documentation comment
    pub comment: Option<String>,
}


/// A parsed C++ class or struct
///
/// When `is_pod_struct` is true, the class has only public primitive/array fields,
/// no virtual methods, no non-trivial base classes, and can be represented as a
/// `#[repr(C)]` Rust struct with real fields instead of an opaque type.
#[derive(Debug, Clone)]
pub struct ParsedClass {
    /// Full class name (e.g., "gp_Pnt", "BRepPrimAPI_MakeBox")
    pub name: String,
    /// Module name extracted from prefix (e.g., "gp", "BRepPrimAPI")
    pub module: String,
    /// Documentation comment from the header
    pub comment: Option<String>,
    /// Source header file name (e.g., "gp_Pnt.hxx")
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Constructors
    pub constructors: Vec<Constructor>,
    /// Instance methods (public only)
    pub methods: Vec<Method>,
    /// Static methods (public only)
    pub static_methods: Vec<StaticMethod>,
    /// All method names in this class (including protected/private) - used for filtering inherited methods
    pub all_method_names: std::collections::HashSet<String>,
    /// Direct base classes (for generating upcast helpers)
    pub base_classes: Vec<String>,
    /// Whether this class has a protected/private destructor (non-instantiable abstract base)
    pub has_protected_destructor: bool,
    /// Whether this class is abstract (has pure virtual methods)
    pub is_abstract: bool,
    /// Names of pure virtual methods declared in this class
    pub pure_virtual_methods: HashSet<String>,
    /// Whether this class has any explicit constructor declarations (public or not).
    /// If true, C++ won't generate an implicit default constructor.
    pub has_explicit_constructors: bool,
    /// Public data members (fields)
    pub fields: Vec<ParsedField>,
    /// Whether this class is a POD struct (all public fields, no virtuals, trivially copyable)
    pub is_pod_struct: bool,
    /// Whether this class has a usable (public, non-deleted) copy constructor.
    /// - `Some(true)`: explicit public, non-deleted copy constructor found
    /// - `Some(false)`: explicit copy constructor found but deleted or non-public
    /// - `None`: no explicit copy constructor (implicit may exist based on C++ rules)
    pub has_copy_constructor: Option<bool>,
    /// Whether this class has an explicit move constructor.
    /// A move constructor suppresses the implicit copy constructor in C++.
    pub has_move_constructor: bool,
}

impl ParsedClass {
    /// Get the class name without the module prefix (e.g., "Pnt" from "gp_Pnt")
    /// For nested classes (e.g., "Poly_CoherentTriangulation::TwoIntegers"),
    /// returns the name after the first underscore, which may contain "::". 
    /// Callers that need correct short names for nested types should use
    /// type_mapping::short_name_for_module() on the flattened name instead.
    pub fn short_name(&self) -> &str {
        if let Some(underscore_pos) = self.name.find('_') {
            &self.name[underscore_pos + 1..]
        } else {
            &self.name
        }
    }

    /// Get a safe Rust name for this class, escaping FFI reserved names
    pub fn safe_short_name(&self) -> String {
        crate::type_mapping::safe_short_name(self.short_name())
    }
}

/// A constructor declaration
#[derive(Debug, Clone)]
pub struct Constructor {
    /// Documentation comment
    pub comment: Option<String>,
    /// Parameters
    pub params: Vec<Param>,
    /// Source line number in the header file
    pub source_line: Option<u32>,
}

impl Constructor {
    /// Generate a suffix for distinguishing overloaded constructors
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (f64, f64, f64) -> "_real3", (Pnt, Pnt) -> "_pnt2"
    pub fn overload_suffix(&self) -> String {
        overload_suffix_from_params(&self.params)
    }

    /// Check if this constructor has any unbindable types (C strings, streams, void pointers, etc.)
    /// Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
    }

    /// Check if this constructor has any unsafe raw pointer types.
    pub fn has_unsafe_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
    }
}

/// An instance method declaration
#[derive(Debug, Clone)]
pub struct Method {
    /// Method name (e.g., "X", "SetX", "Mirrored")
    pub name: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Whether the method is const (determines &self vs &mut self)
    pub is_const: bool,
    /// Parameters (excluding implicit this)
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
    /// Source line number in the header file
    pub source_line: Option<u32>,
}

impl Method {
    /// Check if this method returns by value (needs wrapper)
    pub fn returns_by_value(&self) -> bool {
        matches!(&self.return_type, Some(Type::Class(_)) | Some(Type::Handle(_)))
    }

    /// Check if this method has any unbindable types (streams, void pointers, etc.)
    /// in parameters or return type. Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        // Check params (skip nullable pointer params — they're handled as Option<&T>)
        // Also skip class raw pointer params — they're handled as &T / &mut T
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        // Check return type
        if let Some(ref ret) = self.return_type {
            if ret.is_unbindable() && ret.class_ptr_inner_name().is_none() {
                return true;
            }
        }
        false
    }

    /// Check if this method has any unsafe raw pointer types that require
    /// the function to be marked `unsafe fn`.
    pub fn has_unsafe_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            // Instance method class pointer returns are handled safely as Option<&T>
            if ret.needs_unsafe_fn() && ret.class_ptr_inner_name().is_none() {
                return true;
            }
        }
        false
    }

    /// Generate a suffix for distinguishing overloaded methods
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (Pnt) -> "_pnt", (Box, Trsf) -> "_box_trsf", (f64, f64, f64) -> "_real3"
    pub fn overload_suffix(&self) -> String {
        overload_suffix_from_params(&self.params)
    }
}

/// A static method declaration
#[derive(Debug, Clone)]
pub struct StaticMethod {
    /// Method name
    pub name: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Parameters
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
    /// Source line number in the header file
    pub source_line: Option<u32>,
}

impl StaticMethod {
    /// Check if this method has any unbindable types (streams, void pointers, etc.)
    /// in parameters or return type. Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        // Check params (skip nullable pointer params — they're handled as Option<&T>)
        // Also skip class raw pointer params — they're handled as &T / &mut T
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        // Check return type
        if let Some(ref ret) = self.return_type {
            if ret.is_unbindable() {
                return true;
            }
        }
        false
    }

    /// Check if this static method has any unsafe raw pointer types.
    pub fn has_unsafe_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            if ret.needs_unsafe_fn() {
                return true;
            }
        }
        false
    }

    /// Generate a suffix for distinguishing overloaded static methods
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (f64, f64, f64) -> "_real3", (Shape, Builder) -> "_shape_builder"
    pub fn overload_suffix(&self) -> String {
        overload_suffix_from_params(&self.params)
    }
}

/// A function parameter
#[derive(Debug, Clone)]
pub struct Param {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub ty: Type,
    /// Whether this parameter has a default value in C++
    pub has_default: bool,
    /// The default value as a Rust expression (e.g. "false", "0", "0.0")
    /// Only populated for types we can represent as Rust literals.
    pub default_value: Option<String>,
}

impl Param {
    /// Check if this parameter is a nullable pointer (T* param = NULL or const T* param = NULL).
    /// These are optional parameters that can be bound as Option<&T> / Option<&mut T>.
    pub fn is_nullable_ptr(&self) -> bool {
        if !self.has_default {
            return false;
        }
        match &self.ty {
            // const char* is handled separately (string conversion)
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }
}

/// Representation of C++ types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// void
    Void,
    /// bool / Standard_Boolean
    Bool,
    /// int / Standard_Integer
    I32,
    /// unsigned int
    U32,
    /// unsigned short / uint16_t
    U16,
    /// short / int16_t
    I16,
    /// long long / int64_t
    I64,
    /// unsigned long long / uint64_t
    U64,
    /// long (platform-dependent: 32-bit on Windows LLP64, 64-bit on LP64)
    Long,
    /// unsigned long (platform-dependent size)
    ULong,
    /// size_t / Standard_Size - platform-dependent size
    Usize,
    /// float
    F32,
    /// double / Standard_Real
    F64,
    /// char16_t / Standard_ExtCharacter
    CHAR16,
    /// unsigned char / Standard_Byte / uint8_t
    U8,
    /// signed char / int8_t
    I8,
    /// const T&
    ConstRef(Box<Type>),
    /// T& (mutable reference)
    MutRef(Box<Type>),
    /// T&& (rvalue reference) - not bindable through the FFI
    RValueRef(Box<Type>),
    /// const T*
    ConstPtr(Box<Type>),
    /// T* (mutable pointer)
    MutPtr(Box<Type>),
    /// Handle<T> / opencascade::handle<T>
    Handle(String),
    /// Fixed-size C array type (e.g., `Standard_Integer[3]`)
    FixedArray(Box<Type>, usize),
    /// An OCCT class type (e.g., "gp_Pnt", "TopoDS_Shape")
    Class(String),
}

/// Check if a class name represents a void pointer type.
/// Standard_Address is a typedef for void*, and "void" is the parsed
/// form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
pub fn std_bitmask_ffi_type(name: &str) -> Option<Type> {
    match name {
        "std::ios_base::openmode" => Some(Type::U32),
        _ => None,
    }
}

/// Check if a class name is a real opaque C++ class (not a primitive
/// mapped to a special Rust type like char or void pointer types,
/// and not a standard library bitmask type).
pub fn is_opaque_class_name(name: &str) -> bool {
    name != "char" && !is_void_type_name(name) && std_bitmask_ffi_type(name).is_none()
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
    pub fn short_name(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "uint".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.short_name(),
            Type::ConstPtr(inner) | Type::MutPtr(inner) => format!("{}ptr", inner.short_name()),
            Type::Handle(name) => crate::type_mapping::handle_param_name(name),
            Type::FixedArray(inner, size) => format!("{}{}", inner.short_name(), size),
            Type::Class(name) => extract_short_name(name),
            Type::CHAR16 => "char16".to_string(),
            Type::U8 => "u8".to_string(),
            Type::I8 => "i8".to_string(),
        }
    }

    /// Check if this is a primitive type that can be passed by value in FFI
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Void
                | Type::Bool
                | Type::I32
                | Type::U32
                | Type::U16
                | Type::I16
                | Type::I64
                | Type::U64
                | Type::Long
                | Type::ULong
                | Type::Usize
                | Type::F32
                | Type::F64
                | Type::CHAR16
                | Type::U8
                | Type::I8
        )
    }

    /// Check if this type is suitable as a field in a POD struct.
    /// Only primitive numeric types (bool, integers, floats) are POD-safe.
    pub fn is_pod_field_type(&self) -> bool {
        matches!(
            self,
            Type::Bool | Type::I32 | Type::U32 | Type::U16 | Type::I16 | Type::I64 | Type::U64
                | Type::Long | Type::ULong | Type::Usize | Type::F32 | Type::F64 | Type::CHAR16
                | Type::U8 | Type::I8
        )
    }


    /// Check if this is an OCCT class type (not primitive, not reference/pointer)
    pub fn is_class(&self) -> bool {
        matches!(self, Type::Class(_))
    }

    /// Check if this is a Type::Class with a primitive type name (e.g., "char" from Standard_Character).
    /// These are resolved from C++ typedefs to primitive names but remain as Type::Class in the AST.
    pub fn is_primitive_class(&self) -> bool {
        if let Type::Class(name) = self {
            crate::codegen::rust::is_primitive_type(name)
        } else {
            false
        }
    }

    /// Check if this is a Handle type
    pub fn is_handle(&self) -> bool {
        matches!(self, Type::Handle(_))
    }

    /// Check if this is a reference type (const ref or mutable ref)
    pub fn is_reference(&self) -> bool {
        matches!(self, Type::ConstRef(_) | Type::MutRef(_))
    }

    /// Check if this type counts as a "lifetime source" for elision purposes.
    /// References (&T, &mut T) and C-string pointers (const char*) both introduce
    /// an input lifetime that could be tied to a returned reference.
    pub fn is_lifetime_source(&self) -> bool {
        self.is_reference() || self.is_c_string()
    }

    /// Check if this is a const char* type (C string pointer)
    pub fn is_c_string(&self) -> bool {
        match self {
            Type::ConstPtr(inner) => matches!(inner.as_ref(), Type::Class(name) if name == "char"),
            _ => false,
        }
    }

    /// Check if this is a C++ stream type (Standard_OStream, Standard_IStream, etc.)
    /// These can't be bound through the FFI
    pub fn is_stream(&self) -> bool {
        match self {
            Type::Class(name) => {
                name.contains("OStream")
                    || name.contains("IStream")
                    || name.contains("ostream")
                    || name.contains("istream")
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_stream(),
            _ => false,
        }
    }

    /// Check if this type is a mutable string ref output param: `const char*&` (MutRef(ConstPtr(Class("char")))).
    /// These are Standard_CString& output parameters in C++.
    pub fn is_string_ref_output(&self) -> bool {
        if let Type::MutRef(inner) = self {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    return name == "char";
                }
            }
        }
        false
    }

    /// Check if this type is a const string ref input param: `const char* const&` (ConstRef(ConstPtr(Class("char")))).
    /// These are const Standard_CString& parameters in C++.
    pub fn is_string_ref_input(&self) -> bool {
        if let Type::ConstRef(inner) = self {
            if let Type::ConstPtr(inner2) = inner.as_ref() {
                if let Type::Class(name) = inner2.as_ref() {
                    return name == "char";
                }
            }
        }
        false
    }

    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => is_void_type_name(name),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8] or fixed-size array refs)
    pub fn is_array(&self) -> bool {
        match self {
            Type::FixedArray(_, _) => true,
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
    }

    /// Check if this is a bindable fixed-array reference parameter (&[T; N] / &mut [T; N]).
    pub fn is_fixed_array_ref(&self) -> bool {
        match self {
            Type::ConstRef(inner) | Type::MutRef(inner) => matches!(inner.as_ref(), Type::FixedArray(_, _)),
            _ => false,
        }
    }

    /// Check if this is a direct fixed-size array parameter (`T[N]`).
    pub fn is_fixed_array_param(&self) -> bool {
        matches!(self, Type::FixedArray(_, _))
    }

    /// Check if this type is a raw pointer (requires unsafe in FFI)
    /// Note: const char* is NOT considered a raw pointer here because we handle it specially
    /// with const char* pass-through wrappers.
    pub fn is_raw_ptr(&self) -> bool {
        match self {
            // const char* is bindable - we generate wrappers
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            // References to raw pointers also count as problematic
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_raw_ptr(),
            _ => false,
        }
    }

    /// Check if this type is a raw pointer to a class type (e.g., `const SomeClass*` or `SomeClass*`).
    /// Returns the inner class name if so. Excludes:
    /// - `const char*` (handled as strings)
    /// - Pointer-to-pointer (`T**`)
    /// - Reference-to-pointer (`T*&`)
    /// - Primitive type pointers (`int*`, `double*`, etc.)
    pub fn class_ptr_inner_name(&self) -> Option<&str> {
        match self {
            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                match inner.as_ref() {
                    Type::Class(name) if is_opaque_class_name(name) => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }


    /// Similar to `Param::is_nullable_ptr()` but operates on a bare `Type` without
    /// requiring a `has_default` check. Excludes `const char*` (handled as strings).
    pub fn is_nullable_ptr(&self) -> bool {
        match self {
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }

    /// Check if this type is a nested/qualified type (e.g., SomeClass::value_type) or template type
    /// that couldn't be resolved to a simple type name.
    pub fn is_nested_type(&self) -> bool {
        match self {
            Type::Class(name) => {
                // Explicit nested type indicators
                if name.contains("::") || name.contains('<') || name.contains('>') {
                    return true;
                }
                // OCCT classes follow Module_ClassName pattern (e.g., gp_Pnt, TopoDS_Shape)
                // Types without underscore that aren't known primitive-like names are likely
                // nested types whose qualified name was resolved by clang to a simple name
                // (e.g., Message_Messenger::StreamBuffer -> StreamBuffer)
                if !name.contains('_') {
                    // Allow known types that don't have underscore
                    if matches!(name.as_str(), "bool" | "char" | "int" | "unsigned" | "float" | "double" | "void" | "size_t") {
                        return false;
                    }
                    return true;
                }
                false
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_nested_type()
            }
            _ => false,
        }
    }

    /// Convert a nested C++ type name to a valid Rust FFI identifier.
    /// `Parent::Nested` becomes `Parent_Nested`. Non-nested names pass through.
    pub fn ffi_safe_class_name(name: &str) -> String {
        name.replace("::", "_")
    }

    /// Check if this type is an rvalue reference (T&&)
    /// Rvalue references are not bindable through the FFI
    pub fn is_rvalue_ref(&self) -> bool {
        matches!(self, Type::RValueRef(_))
    }

    /// Check if this type is unbindable through the FFI.
    /// Note: const char* (C strings) ARE bindable - we generate wrappers that pass const char* directly.
    /// Note: Raw pointers (void*, int*, T*) are NOT unbindable — they are bound as unsafe raw pointer types.
    /// Nested types (Parent::Nested) are supported via name flattening
    /// (Parent::Nested → Parent_Nested in Rust FFI), BUT unresolved template types
    /// and unqualified names without underscore remain unbindable.
    pub fn is_unbindable(&self) -> bool {
        (self.is_array() && !self.is_fixed_array_ref() && !self.is_fixed_array_param())
            || self.is_rvalue_ref()
            || self.is_unresolved_template_type()
    }

    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr()
    }

    /// Convert this type to a C++ parameter type for extern "C" wrapper functions.
    /// References become pointers (const T& → const T*, T& → T*).
    pub fn to_cpp_extern_c_param(&self) -> String {
        match self {
            // Use postfix const ("T const*") rather than prefix ("const T*") so that
            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class("X")))),
            // the const correctly qualifies the pointer level, not the pointee:
            //   ConstRef(MutPtr(X)) → "X* const*" (correct: pointer to const-pointer-to-X)
            //   vs. "const X**" (wrong: pointer to pointer-to-const-X)
            // For simple types, "T const*" and "const T*" are equivalent in C/C++.
            Type::ConstRef(inner) => {
                if let Type::FixedArray(elem, _) = inner.as_ref() {
                    format!("{} const*", elem.to_cpp_string())
                } else {
                    format!("{} const*", inner.to_cpp_string())
                }
            }
            Type::MutRef(inner) => {
                if let Type::FixedArray(elem, _) = inner.as_ref() {
                    format!("{}*", elem.to_cpp_string())
                } else {
                    format!("{}*", inner.to_cpp_string())
                }
            }
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
                "const char*".to_string()
            }
            _ => self.to_cpp_string(),
        }
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int32_t".to_string(),
            Type::U32 => "uint32_t".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I16 => "int16_t".to_string(),
            Type::I64 => "int64_t".to_string(),
            Type::U64 => "uint64_t".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::CHAR16 => "char16_t".to_string(),
            Type::U8 => "uint8_t".to_string(),
            Type::I8 => "int8_t".to_string(),
            // Use postfix const ("T const&") rather than prefix ("const T&") so that
            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class("X")))),
            // the const correctly qualifies the pointer level, not the pointee:
            //   ConstRef(MutPtr(X)) → "X* const&" (correct: const-ref to pointer-to-X)
            //   vs. "const X*&" (wrong: ref to pointer-to-const-X)
            // For simple types, "T const&" and "const T&" are equivalent in C/C++.
            Type::ConstRef(inner) => format!("{} const&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            // Use postfix const for same reason as ConstRef above.
            Type::ConstPtr(inner) => format!("{} const*", inner.to_cpp_string()),
            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
            Type::Handle(name) => format!("Handle({})", name),
            Type::FixedArray(inner, size) => format!("{}[{}]", inner.to_cpp_string(), size),
            Type::Class(name) => name.clone(),
        }
    }

    /// Check if this type is an unresolved template instantiation that can't be
    /// represented in Rust FFI. Only catches template types with `<>`.
    /// Non-underscore class names (e.g., `LDOMString`) are NOT caught here —
    /// they are handled by `type_uses_unknown_class()` in the binding layer
    /// which checks against the symbol table.
    fn is_unresolved_template_type(&self) -> bool {
        match self {
            Type::Class(name) => {
                // Template types with angle brackets are not representable
                name.contains('<') || name.contains('>')
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_unresolved_template_type()
            }
            Type::FixedArray(inner, _) => inner.is_unresolved_template_type(),
            _ => false,
        }
    }

    /// Convert this type to a Rust type string for use in method signatures
    pub fn to_rust_type_string(&self) -> String {
        match self {
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
            Type::CHAR16 => "u16".to_string(), // Rust doesn't have char16, so we use u16 and rely on callers to convert
            Type::U8 => "u8".to_string(),
            Type::I8 => "i8".to_string(),
            Type::ConstRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&{}", inner_str)
            }
            Type::MutRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&mut {}", inner_str)
            }
            Type::RValueRef(_) => {
                panic!("RValueRef types should not be converted to Rust type strings - they are unbindable")
            }
            Type::ConstPtr(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("*const {}", inner_str)
            }
            Type::MutPtr(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("*mut {}", inner_str)
            }
            Type::Handle(name) => {
                let flat_name = name.replace("::", "_");
                let short = short_name_for_leaf(&flat_name);
                format!("Handle{}", short)
            }
            Type::FixedArray(inner, size) => {
                let inner_str = inner.to_rust_type_string();
                format!("[{}; {}]", inner_str, size)
            }
            Type::Class(name) => {
                let flat = Type::ffi_safe_class_name(name);
                short_name_for_leaf(&flat)
            }
        }
    }

    /// Safe version of `to_rust_type_string()` that returns a placeholder
    /// for unbindable types instead of panicking. Used for diagnostic stubs.
    pub fn to_rust_type_string_safe(&self) -> String {
        if self.is_unbindable() {
            format!("/* {} */", self.to_cpp_string())
        } else {
            self.to_rust_type_string()
        }
    }

    /// Convert this type to a Rust type string for use outside the ffi module.
    /// Class and Handle types are prefixed with `ffi::` since they live in the ffi module.
    /// Uses safe short names (e.g., "gp_Vec" -> "ffi::Vec_" because Vec is reserved).
    pub fn to_rust_ffi_type_string(&self) -> String {
        match self {
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
            Type::CHAR16 => "u16".to_string(), // Rust doesn't have char16, so we use u16 and rely on callers to convert
            Type::U8 => "u8".to_string(),
            Type::I8 => "i8".to_string(),
            Type::ConstRef(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("&{}", inner_str)
            }
            Type::MutRef(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("&mut {}", inner_str)
            }
            Type::RValueRef(_) => {
                panic!("RValueRef types should not be converted to Rust type strings - they are unbindable")
            }
            Type::ConstPtr(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("*const {}", inner_str)
            }
            Type::MutPtr(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("*mut {}", inner_str)
            }
            Type::Handle(name) => {
                let flat_name = name.replace("::", "_");
                let short = short_name_for_leaf(&flat_name);
                format!("ffi::Handle{}", short)
            }
            Type::FixedArray(inner, size) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("[{}; {}]", inner_str, size)
            }
            Type::Class(name) => {
                let flat = Type::ffi_safe_class_name(name);
                let short_name = short_name_for_leaf(&flat);
                let safe_name = match short_name.as_str() {
                    "Vec" | "Box" | "String" | "Result" | "Option" | "Error" => {
                        format!("{}_", short_name)
                    }
                    _ => short_name,
                };
                format!("ffi::{}", safe_name)
            }
        }
    }
}

/// Extract short name from a class name (e.g., "gp_Pnt" -> "pnt")
/// For nested types like "Parent::Nested", uses only the leaf name.
fn extract_short_name(name: &str) -> String {
    // Strip parent class qualifier for nested types
    let leaf = if let Some(pos) = name.rfind("::") {
        &name[pos + 2..]
    } else {
        name
    };
    short_name_for_leaf(leaf).to_lowercase()
}

fn short_name_for_leaf(leaf: &str) -> String {
    if let Some(underscore_pos) = leaf.find('_') {
        let module = &leaf[..underscore_pos];
        crate::type_mapping::short_name_for_module(leaf, module)
    } else {
        leaf.to_string()
    }
}

// =============================================================================
// Type visitor helpers
// =============================================================================

impl Type {
    /// Recursively unwrap reference/pointer wrappers and call `f` on each
    /// innermost (leaf) type. Unwraps ConstRef, MutRef, RValueRef, ConstPtr,
    /// and MutPtr.
    pub fn visit_inner(&self, f: &mut impl FnMut(&Type)) {
        match self {
            Type::ConstRef(inner)
            | Type::MutRef(inner)
            | Type::RValueRef(inner)
            | Type::ConstPtr(inner)
            | Type::MutPtr(inner)
            | Type::FixedArray(inner, _) => inner.visit_inner(f),
            _ => f(self),
        }
    }

    /// Mutable version of `visit_inner`.
    pub fn visit_inner_mut(&mut self, f: &mut impl FnMut(&mut Type)) {
        match self {
            Type::ConstRef(inner)
            | Type::MutRef(inner)
            | Type::RValueRef(inner)
            | Type::ConstPtr(inner)
            | Type::MutPtr(inner)
            | Type::FixedArray(inner, _) => inner.visit_inner_mut(f),
            _ => f(self),
        }
    }
}

/// Visit every type in a class's methods, static methods, and constructors.
/// Calls `f` on each parameter type and return type.
pub fn for_each_type_in_class(class: &ParsedClass, f: &mut impl FnMut(&Type)) {
    for method in &class.methods {
        for param in &method.params {
            f(&param.ty);
        }
        if let Some(ref ret) = method.return_type {
            f(ret);
        }
    }
    for method in &class.static_methods {
        for param in &method.params {
            f(&param.ty);
        }
        if let Some(ref ret) = method.return_type {
            f(ret);
        }
    }
    for ctor in &class.constructors {
        for param in &ctor.params {
            f(&param.ty);
        }
    }
}

/// Mutable version of `for_each_type_in_class`.
pub fn for_each_type_in_class_mut(class: &mut ParsedClass, f: &mut impl FnMut(&mut Type)) {
    for method in &mut class.methods {
        for param in &mut method.params {
            f(&mut param.ty);
        }
        if let Some(ref mut ret) = method.return_type {
            f(ret);
        }
    }
    for method in &mut class.static_methods {
        for param in &mut method.params {
            f(&mut param.ty);
        }
        if let Some(ref mut ret) = method.return_type {
            f(ret);
        }
    }
    for ctor in &mut class.constructors {
        for param in &mut ctor.params {
            f(&mut param.ty);
        }
    }
}

/// Visit every type in a function's parameters and return type.
pub fn for_each_type_in_function(func: &ParsedFunction, f: &mut impl FnMut(&Type)) {
    for param in &func.params {
        f(&param.ty);
    }
    if let Some(ref ret) = func.return_type {
        f(ret);
    }
}

/// Mutable version of `for_each_type_in_function`.
pub fn for_each_type_in_function_mut(func: &mut ParsedFunction, f: &mut impl FnMut(&mut Type)) {
    for param in &mut func.params {
        f(&mut param.ty);
    }
    if let Some(ref mut ret) = func.return_type {
        f(ret);
    }
}

/// Visit every type in a parsed header (all classes and all free functions).
pub fn for_each_type_in_header(header: &ParsedHeader, f: &mut impl FnMut(&Type)) {
    for class in &header.classes {
        for_each_type_in_class(class, f);
    }
    for func in &header.functions {
        for_each_type_in_function(func, f);
    }
}

/// Mutable version of `for_each_type_in_header`.
pub fn for_each_type_in_header_mut(header: &mut ParsedHeader, f: &mut impl FnMut(&mut Type)) {
    for class in &mut header.classes {
        for_each_type_in_class_mut(class, f);
    }
    for func in &mut header.functions {
        for_each_type_in_function_mut(func, f);
    }
}

// =============================================================================
// Shared helpers for duplicated logic
// =============================================================================

/// Generate an overload suffix from parameter types.
/// Consecutive identical types are compressed: ["real", "real", "real"] -> "_real3".
/// Used by Constructor, Method, and StaticMethod.
pub fn overload_suffix_from_params(params: &[Param]) -> String {
    if params.is_empty() {
        return String::new();
    }

    let type_names: Vec<String> = params
        .iter()
        .map(|p| p.ty.short_name().to_lowercase())
        .collect();

    // Compress consecutive identical types: ["real", "real", "real"] -> ["real3"]
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
