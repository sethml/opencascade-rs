//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

/// Convert a C++ class name to its Rust Handle type name.
///
/// Strips underscores and `::` (from nested classes) to produce a valid
/// Rust identifier.  e.g. `ShapePersistent_BRep::CurveRepresentation`
/// → `HandleShapePersistentBRepCurveRepresentation`.
pub fn handle_type_name(cpp_name: &str) -> String {
    format!("Handle{}", cpp_name.replace("::", "").replace('_', ""))
}

/// Lowercase variant used for parameter names.
pub fn handle_param_name(cpp_name: &str) -> String {
    format!("handle{}", cpp_name.to_lowercase().replace("::", "").replace('_', ""))
}

/// Result of mapping a C++ type to Rust
#[derive(Debug, Clone)]
pub struct RustTypeMapping {
    /// The Rust type string for use in extern "C" FFI declarations
    pub rust_type: String,
    /// Whether this type is returned as an owned pointer (*mut T) that the caller must free
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin<&mut T> for mutable self
    pub needs_pin: bool,
    /// The module this type comes from (if cross-module reference)
    pub source_module: Option<String>,
}

/// Map a parsed Type to its Rust representation for extern "C" FFI
pub fn map_type_to_rust(ty: &Type) -> RustTypeMapping {
    match ty {
        Type::Void => RustTypeMapping {
            rust_type: "()".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Bool => RustTypeMapping {
            rust_type: "bool".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I32 => RustTypeMapping {
            rust_type: "i32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U32 => RustTypeMapping {
            rust_type: "u32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U16 => RustTypeMapping {
            rust_type: "u16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I16 => RustTypeMapping {
            rust_type: "i16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I64 => RustTypeMapping {
            rust_type: "i64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U64 => RustTypeMapping {
            rust_type: "u64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Long => RustTypeMapping {
            rust_type: "std::ffi::c_long".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::ULong => RustTypeMapping {
            rust_type: "std::ffi::c_ulong".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Usize => RustTypeMapping {
            rust_type: "usize".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::F32 => RustTypeMapping {
            rust_type: "f32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::F64 => RustTypeMapping {
            rust_type: "f64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::CHAR16 => RustTypeMapping {
            rust_type: "u16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U8 => RustTypeMapping {
            rust_type: "u8".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I8 => RustTypeMapping {
            rust_type: "i8".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            if inner.is_primitive() {
                RustTypeMapping {
                    rust_type: format!("*mut {}", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: inner_mapping.source_module,
                }
            } else {
                RustTypeMapping {
                    rust_type: format!("*mut {}", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: inner_mapping.source_module,
                }
            }
        }
        Type::RValueRef(_) => {
            // RValueRef types should be filtered out before reaching here
            panic!("RValueRef types should not be mapped to Rust types - they are unbindable")
        }
        Type::ConstPtr(inner) => {
            // const char* stays as *const c_char for extern "C"
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutPtr(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*mut {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::FixedArray(inner, size) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("[{}; {}]", inner_mapping.rust_type, size),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::Handle(class_name) => {
            let source_module = extract_module_from_class(class_name);
            let handle_type = handle_type_name(class_name);
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true,
                needs_pin: false,
                source_module,
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "Aspect_RenderingContext" => {
            // Standard_Address is a typedef for void* — map to raw c_void pointer
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            // FFI supports c_char but not Rust's char (which is 4-byte Unicode)
            RustTypeMapping {
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if crate::model::std_bitmask_ffi_type(class_name).is_some() => {
            // Standard library bitmask types (e.g., std::ios_base::openmode)
            // mapped to their FFI integer type
            map_type_to_rust(&crate::model::std_bitmask_ffi_type(class_name).unwrap())
        }
        Type::Class(class_name) => {
            let source_module = extract_module_from_class(class_name);
            RustTypeMapping {
                rust_type: Type::ffi_safe_class_name(class_name),
                needs_unique_ptr: true, // C++ classes returned as *mut T, caller must free
                needs_pin: false,
                source_module,
            }
        }
    }
}

/// Map a type for use in return position (returns *mut T for owned objects)
pub fn map_return_type(ty: &Type) -> RustTypeMapping {
    let mut mapping = map_type_to_rust(ty);

    // Return-by-value C++ types are returned as *mut T (heap-allocated)
    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("*mut {}", mapping.rust_type);
    }

    mapping
}

/// Map a type for use in self position
pub fn map_self_type(ty: &Type, is_const: bool) -> RustTypeMapping {
    let inner_mapping = map_type_to_rust(ty);

    if is_const {
        RustTypeMapping {
            rust_type: format!("*const {}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        }
    } else {
        RustTypeMapping {
            rust_type: format!("*mut {}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        }
    }
}

/// Reserved names that would conflict with Rust standard library types.
/// Currently unused — we allow short names like `Vec`, `Box`, `Result` because
/// they live inside module namespaces and don't shadow prelude names.
const FFI_RESERVED_NAMES: &[&str] = &[];

/// Check if a short name is reserved and needs escaping
pub fn is_reserved_name(name: &str) -> bool {
    FFI_RESERVED_NAMES.contains(&name)
}

/// Get the safe Rust name for a short class name.
/// With an empty reserved list, this is effectively a no-op.
pub fn safe_short_name(short_name: &str) -> String {
    if is_reserved_name(short_name) {
        format!("{}_", short_name)
    } else {
        short_name.to_string()
    }
}

/// Compute the short type name by stripping the module prefix from a C++ type name.
///
/// When the type's name-based prefix differs from its header-based module,
/// the extra prefix text is preserved in the short name.
///
/// Examples:
///   - `("gp_Pnt", "gp")` → `"Pnt"`
///   - `("BRepOffset_Status", "BRepOffset")` → `"Status"`
///   - `("BRepOffsetSimple_Status", "BRepOffset")` → `"SimpleStatus"`
///   - `("TopoDS_Shape", "TopoDS")` → `"Shape"`
pub fn short_name_for_module(cpp_name: &str, module: &str) -> String {
    if let Some(rest) = cpp_name.strip_prefix(module) {
        // After stripping the module prefix, the remainder starts with either:
        // - "_Foo" (exact module match) → "Foo"
        // - "Simple_Foo" (longer prefix) → "SimpleFoo"  
        let rest = rest.strip_prefix('_').unwrap_or(rest);
        if rest.is_empty() {
            // Type name equals the module name (rare but possible)
            cpp_name.to_string()
        } else {
            rest.to_string()
        }
    } else {
        // Module prefix doesn't match at all — use the full C++ name as the short name.
        // This happens for types that don't follow OCCT naming conventions (e.g., Fortran
        // common blocks like `mdnombr_1_` in AdvApp2Var_Data.hxx).
        cpp_name.to_string()
    }
}

/// Look up module name for a type, using the authoritative map if available,
/// falling back to name-based derivation for context-free callers.
fn lookup_module_for_type(
    class_name: &str,
    type_to_module: Option<&std::collections::HashMap<String, String>>,
) -> Option<String> {
    if let Some(map) = type_to_module {
        map.get(class_name).map(|m| module_to_rust_name(m))
    } else {
        extract_module_from_class(class_name)
    }
}

/// Extract module name from class name (name-based fallback)
fn extract_module_from_class(class_name: &str) -> Option<String> {
    class_name.find('_').map(|underscore_pos| module_to_rust_name(&class_name[..underscore_pos]))
}

/// Extract short class name (without module prefix)
pub fn extract_short_class_name(class_name: &str) -> String {
    if let Some(underscore_pos) = class_name.find('_') {
        let module = &class_name[..underscore_pos];
        short_name_for_module(class_name, module)
    } else {
        class_name.to_string()
    }
}

fn short_name_for_type_in_context(class_name: &str, ctx: &TypeContext) -> String {
    if let Some(type_to_module) = ctx.type_to_module {
        if let Some(module) = type_to_module.get(class_name) {
            return short_name_for_module(class_name, module);
        }
    }
    extract_short_class_name(class_name)
}

/// Context for type mapping within a specific module
pub struct TypeContext<'a> {
    /// The current module name (e.g., "gp")
    pub current_module: &'a str,
    /// Classes defined in the current module (full C++ names like "gp_Pnt")
    pub module_classes: &'a std::collections::HashSet<String>,
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
/// Check if a class name is unknown (not in the known type set).
/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.
fn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {
    if all_classes.contains(class_name) {
        return false;
    }
    // Void pointer types — Standard_Address (void*) and literal "void" — are known
    if crate::model::is_void_type_name(class_name) {
        return false;
    }
    // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
    if crate::codegen::rust::is_primitive_type(class_name) {
        return false;
    }
    // Standard library bitmask types (e.g., std::ios_base::openmode) are known
    if crate::model::std_bitmask_ffi_type(class_name).is_some() {
        return false;
    }
    // Nested types (Parent::Nested) are known if the parent class is known
    if let Some(parent) = class_name.split("::").next() {
        if class_name.contains("::") && all_classes.contains(parent) {
            return false;
        }
    }
    true
}

pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
        Type::FixedArray(inner, _) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        Type::FixedArray(inner, _) => type_uses_unknown_handle(inner, all_classes, handle_able_classes),
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    match ty {
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            RustTypeMapping {
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "Aspect_RenderingContext" => {
            // Standard_Address is a typedef for void* — map to raw c_void pointer
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) => {
            // Enums are passed as i32 at the FFI boundary (integer pass-through)
            if ctx.all_enums.contains(class_name) {
                return RustTypeMapping {
                    rust_type: "i32".to_string(),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: None,
                };
            }
            // Standard library bitmask types (e.g., std::ios_base::openmode)
            // are mapped to their FFI integer type (e.g., u32)
            if let Some(ffi_ty) = crate::model::std_bitmask_ffi_type(class_name) {
                return map_type_to_rust(&ffi_ty);
            }
            
            let type_module = lookup_module_for_type(class_name, ctx.type_to_module);
            let short_name = short_name_for_type_in_context(class_name, ctx);
            
            // Check if this is a same-module reference
            if type_module.as_deref() == Some(ctx.current_module) 
                && ctx.module_classes.contains(class_name) 
            {
                // Use short name for same-module types
                let safe_name = safe_short_name(&short_name);
                RustTypeMapping {
                    rust_type: safe_name,
                    needs_unique_ptr: true,
                    needs_pin: false,
                    source_module: None, // Same module
                }
            } else {
                // Use full C++ name for cross-module types (will be aliased)
                // Flatten nested type names (Parent::Nested -> Parent_Nested)
                let ffi_name = Type::ffi_safe_class_name(class_name);
                RustTypeMapping {
                    rust_type: ffi_name,
                    needs_unique_ptr: true,
                    needs_pin: false,
                    source_module: type_module,
                }
            }
        }
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            RustTypeMapping {
                rust_type: format!("*mut {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::Handle(class_name) => {
            let source_module = lookup_module_for_type(class_name, ctx.type_to_module);
            let handle_type = handle_type_name(class_name);
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true,
                needs_pin: false,
                source_module,
            }
        }
        // Delegate primitives and other types to the context-free version
        _ => map_type_to_rust(ty),
    }
}

/// Map a return type in context
/// For const char* return types, maps to *const c_char
pub fn map_return_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    // const char* returns stay as *const c_char
    if ty.is_c_string() {
        return RustTypeMapping {
            rust_type: "*const std::ffi::c_char".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        };
    }
    
    let mut mapping = map_type_in_context(ty, ctx);

    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("*mut {}", mapping.rust_type);
    }

    mapping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_primitives() {
        assert_eq!(map_type_to_rust(&Type::F64).rust_type, "f64");
        assert_eq!(map_type_to_rust(&Type::I32).rust_type, "i32");
        assert_eq!(map_type_to_rust(&Type::Bool).rust_type, "bool");
    }

    #[test]
    fn test_map_const_ref() {
        let ty = Type::ConstRef(Box::new(Type::Class("gp_Pnt".to_string())));
        assert_eq!(map_type_to_rust(&ty).rust_type, "*const gp_Pnt");
    }

    #[test]
    fn test_map_class() {
        let mapping = map_type_to_rust(&Type::Class("gp_Pnt".to_string()));
        assert_eq!(mapping.rust_type, "gp_Pnt");
        assert!(mapping.needs_unique_ptr);
        assert_eq!(mapping.source_module, Some("gp".to_string()));
    }

    #[test]
    fn test_map_return_type() {
        let ty = Type::Class("TopoDS_Shape".to_string());
        let mapping = map_return_type(&ty);
        assert_eq!(mapping.rust_type, "*mut TopoDS_Shape");
    }
}
