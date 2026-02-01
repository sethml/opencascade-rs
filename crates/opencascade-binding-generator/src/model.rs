//! Internal representation (IR) for parsed C++ declarations
//!
//! These types represent the parsed information from OCCT headers
//! in a form suitable for code generation.

#![allow(dead_code)] // Some fields/methods are reserved for future use

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

/// A parsed C++ class or struct
#[derive(Debug, Clone)]
pub struct ParsedClass {
    /// Full class name (e.g., "gp_Pnt", "BRepPrimAPI_MakeBox")
    pub name: String,
    /// Module name extracted from prefix (e.g., "gp", "BRepPrimAPI")
    pub module: String,
    /// Documentation comment from the header
    pub comment: Option<String>,
    /// Constructors
    pub constructors: Vec<Constructor>,
    /// Instance methods
    pub methods: Vec<Method>,
    /// Static methods
    pub static_methods: Vec<StaticMethod>,
    /// Whether this type has DEFINE_STANDARD_HANDLE (is a Handle type)
    pub is_handle_type: bool,
}

impl ParsedClass {
    /// Get the class name without the module prefix (e.g., "Pnt" from "gp_Pnt")
    pub fn short_name(&self) -> &str {
        if let Some(underscore_pos) = self.name.find('_') {
            &self.name[underscore_pos + 1..]
        } else {
            &self.name
        }
    }

    /// Get a safe Rust name for this class, escaping CXX reserved names
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
}

impl Constructor {
    /// Generate a suffix for distinguishing overloaded constructors
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (f64, f64, f64) -> "_real3", (Pnt, Pnt) -> "_pnt2"
    pub fn overload_suffix(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }

        let type_names: Vec<String> = self
            .params
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

    /// Check if this constructor has any unbindable types (C strings, streams, void pointers, etc.)
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable())
    }
}

/// An instance method declaration
#[derive(Debug, Clone)]
pub struct Method {
    /// Method name (e.g., "X", "SetX", "Mirrored")
    pub name: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Whether the method is const (determines &self vs Pin<&mut self>)
    pub is_const: bool,
    /// Parameters (excluding implicit this)
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
}

impl Method {
    /// Check if this method returns by value (needs wrapper)
    pub fn returns_by_value(&self) -> bool {
        match &self.return_type {
            Some(Type::Class(_)) => true,
            Some(Type::Handle(_)) => true,
            _ => false,
        }
    }

    /// Check if this method has any unbindable types (streams, void pointers, etc.)
    /// in parameters or return type
    pub fn has_unbindable_types(&self) -> bool {
        // Check params
        if self.params.iter().any(|p| p.ty.is_unbindable()) {
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
}

impl StaticMethod {
    /// Check if this method has any unbindable types (streams, void pointers, etc.)
    /// in parameters or return type
    pub fn has_unbindable_types(&self) -> bool {
        // Check params
        if self.params.iter().any(|p| p.ty.is_unbindable()) {
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

    /// Generate a suffix for distinguishing overloaded static methods
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (f64, f64, f64) -> "_real3", (Shape, Builder) -> "_shape_builder"
    pub fn overload_suffix(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }

        let type_names: Vec<String> = self
            .params
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
}

/// A function parameter
#[derive(Debug, Clone)]
pub struct Param {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub ty: Type,
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
    /// long
    I64,
    /// unsigned long
    U64,
    /// size_t / Standard_Size - platform-dependent size
    Usize,
    /// float
    F32,
    /// double / Standard_Real
    F64,
    /// const T&
    ConstRef(Box<Type>),
    /// T& (mutable reference)
    MutRef(Box<Type>),
    /// const T*
    ConstPtr(Box<Type>),
    /// T* (mutable pointer)
    MutPtr(Box<Type>),
    /// Handle<T> / opencascade::handle<T>
    Handle(String),
    /// An OCCT class type (e.g., "gp_Pnt", "TopoDS_Shape")
    Class(String),
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
    pub fn short_name(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "uint".to_string(),
            Type::I64 => "long".to_string(),
            Type::U64 => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) => inner.short_name(),
            Type::ConstPtr(inner) | Type::MutPtr(inner) => format!("{}ptr", inner.short_name()),
            Type::Handle(name) => format!("handle{}", extract_short_name(name)),
            Type::Class(name) => extract_short_name(name),
        }
    }

    /// Check if this is a primitive type that can be passed by value in CXX
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Void
                | Type::Bool
                | Type::I32
                | Type::U32
                | Type::I64
                | Type::U64
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    /// Check if this is an OCCT class type (not primitive, not reference/pointer)
    pub fn is_class(&self) -> bool {
        matches!(self, Type::Class(_))
    }

    /// Check if this is a Handle type
    pub fn is_handle(&self) -> bool {
        matches!(self, Type::Handle(_))
    }

    /// Check if this is a reference type (const ref or mutable ref)
    pub fn is_reference(&self) -> bool {
        matches!(self, Type::ConstRef(_) | Type::MutRef(_))
    }

    /// Check if this is a const char* type (C string pointer)
    pub fn is_c_string(&self) -> bool {
        match self {
            Type::ConstPtr(inner) => matches!(inner.as_ref(), Type::Class(name) if name == "char"),
            _ => false,
        }
    }

    /// Check if this is a C++ stream type (Standard_OStream, Standard_IStream, etc.)
    /// These can't be bound through CXX
    pub fn is_stream(&self) -> bool {
        match self {
            Type::Class(name) => {
                name.contains("OStream")
                    || name.contains("IStream")
                    || name.contains("ostream")
                    || name.contains("istream")
            }
            Type::ConstRef(inner) | Type::MutRef(inner) => inner.is_stream(),
            _ => false,
        }
    }

    /// Check if this is a Standard_Address (void*) type
    /// These can't be bound through CXX
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
    }

    /// Check if this type is a raw pointer (requires unsafe in CXX)
    /// Note: const char* is NOT considered a raw pointer here because we handle it specially
    /// with rust::Str conversion wrappers.
    pub fn is_raw_ptr(&self) -> bool {
        match self {
            // const char* is bindable - we generate wrappers
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            // References to raw pointers also count as problematic
            Type::ConstRef(inner) | Type::MutRef(inner) => inner.is_raw_ptr(),
            _ => false,
        }
    }

    /// Check if this type is a nested/qualified type (e.g., SomeClass::value_type) or template type
    /// that couldn't be resolved to a simple type name.
    pub fn is_nested_type(&self) -> bool {
        match self {
            Type::Class(name) => name.contains("::") || name.contains('<') || name.contains('>'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_nested_type()
            }
            _ => false,
        }
    }

    /// Check if this type is unbindable through CXX.
    /// Note: const char* (C strings) ARE bindable - we generate wrappers that convert rust::Str.
    /// Nested types are still included here as a fallback - if canonical type resolution
    /// in the parser couldn't resolve them, they remain unbindable.
    pub fn is_unbindable(&self) -> bool {
        self.is_stream() || self.is_void_ptr() || self.is_array() || self.is_raw_ptr() || self.is_nested_type()
    }

    /// Get the module this type belongs to (if it's an OCCT class)
    pub fn module(&self) -> Option<String> {
        match self {
            Type::Class(name) | Type::Handle(name) => {
                if let Some(underscore_pos) = name.find('_') {
                    Some(name[..underscore_pos].to_string())
                } else {
                    None
                }
            }
            Type::ConstRef(inner) | Type::MutRef(inner) => inner.module(),
            _ => None,
        }
    }

    /// Convert this type to a Rust type string for use in method signatures
    pub fn to_rust_type_string(&self) -> String {
        match self {
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Usize => "usize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::ConstRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&{}", inner_str)
            }
            Type::MutRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&mut {}", inner_str)
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
                // Extract short name from full OCCT name
                let short = if let Some(underscore_pos) = name.find('_') {
                    &name[underscore_pos + 1..]
                } else {
                    name.as_str()
                };
                format!("Handle{}", short)
            }
            Type::Class(name) => {
                // Extract short name from full OCCT name (e.g., "gp_Pnt" -> "Pnt")
                if let Some(underscore_pos) = name.find('_') {
                    name[underscore_pos + 1..].to_string()
                } else {
                    name.clone()
                }
            }
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
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Usize => "usize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::ConstRef(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("&{}", inner_str)
            }
            Type::MutRef(inner) => {
                let inner_str = inner.to_rust_ffi_type_string();
                format!("&mut {}", inner_str)
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
                // Extract short name and prefix with ffi::
                let short = if let Some(underscore_pos) = name.find('_') {
                    &name[underscore_pos + 1..]
                } else {
                    name.as_str()
                };
                format!("ffi::Handle{}", short)
            }
            Type::Class(name) => {
                // Extract short name from full OCCT name (e.g., "gp_Pnt" -> "Pnt")
                let short_name = if let Some(underscore_pos) = name.find('_') {
                    &name[underscore_pos + 1..]
                } else {
                    name.as_str()
                };
                // Handle CXX reserved names (Vec, Box, String, etc.)
                let safe_name = match short_name {
                    "Vec" | "Box" | "String" | "Result" | "Option" | "Error" => {
                        format!("{}_", short_name)
                    }
                    _ => short_name.to_string(),
                };
                format!("ffi::{}", safe_name)
            }
        }
    }
}

/// Extract short name from a class name (e.g., "gp_Pnt" -> "pnt")
fn extract_short_name(name: &str) -> String {
    if let Some(underscore_pos) = name.find('_') {
        name[underscore_pos + 1..].to_lowercase()
    } else {
        name.to_lowercase()
    }
}
