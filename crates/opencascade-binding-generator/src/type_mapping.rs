//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for CXX bridge generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

/// Result of mapping a C++ type to Rust
#[derive(Debug, Clone)]
pub struct RustTypeMapping {
    /// The Rust type string for use in CXX bridge
    pub rust_type: String,
    /// Whether this type needs to be behind UniquePtr in return position
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin<&mut T> for mutable self
    pub needs_pin: bool,
    /// The module this type comes from (if cross-module reference)
    pub source_module: Option<String>,
}

/// Map a parsed Type to its Rust representation for CXX
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
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("&{}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            // Mutable references to C++ types need Pin
            let needs_pin = !inner.is_primitive();
            if needs_pin {
                RustTypeMapping {
                    rust_type: format!("Pin<&mut {}>", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: true,
                    source_module: inner_mapping.source_module,
                }
            } else {
                RustTypeMapping {
                    rust_type: format!("&mut {}", inner_mapping.rust_type),
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
            // Special case: const char* -> &str for parameters (C string input)
            // Note: For return types, use map_c_string_return_type() instead, which returns String
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                return RustTypeMapping {
                    rust_type: "&str".to_string(),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: None,
                };
            }
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
        Type::Handle(class_name) => {
            let source_module = extract_module_from_class(class_name);
            // Handles are typedef'd in the bridge
            // Use full class name to avoid collisions (e.g., Geom_Curve vs Geom2d_Curve)
            let handle_type = format!("Handle{}", class_name.replace("_", ""));
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true, // CXX requires UniquePtr for any opaque C++ type returned by value
                needs_pin: false,
                source_module,
            }
        }
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            // CXX supports c_char but not Rust's char (which is 4-byte Unicode)
            RustTypeMapping {
                rust_type: "c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) => {
            let source_module = extract_module_from_class(class_name);
            // Use full C++ name in CXX bridge (will be aliased if cross-module)
            RustTypeMapping {
                rust_type: class_name.clone(),
                needs_unique_ptr: true, // C++ classes need UniquePtr in return position
                needs_pin: false,
                source_module,
            }
        }
    }
}

/// Map a type for use in return position (wraps in UniquePtr if needed)
pub fn map_return_type(ty: &Type) -> RustTypeMapping {
    let mut mapping = map_type_to_rust(ty);

    // Return-by-value C++ types need to be wrapped in UniquePtr
    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("UniquePtr<{}>", mapping.rust_type);
    }

    mapping
}

/// Map a type for use in self position
pub fn map_self_type(ty: &Type, is_const: bool) -> RustTypeMapping {
    let inner_mapping = map_type_to_rust(ty);

    if is_const {
        RustTypeMapping {
            rust_type: format!("&{}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        }
    } else {
        RustTypeMapping {
            rust_type: format!("Pin<&mut {}>", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: true,
            source_module: inner_mapping.source_module,
        }
    }
}

/// CXX reserved names that can't be used as type names
const CXX_RESERVED_NAMES: &[&str] = &["Vec", "Box", "String", "Result", "Option"];

/// Check if a short name is reserved in CXX and needs escaping
pub fn is_reserved_name(name: &str) -> bool {
    CXX_RESERVED_NAMES.contains(&name)
}

/// Get the safe Rust name for a short class name, escaping reserved names with trailing underscore
pub fn safe_short_name(short_name: &str) -> String {
    if is_reserved_name(short_name) {
        // Add trailing underscore for reserved names (will be re-exported with correct name)
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
            // Remove any remaining underscore separators to get a PascalCase name
            // e.g., "Simple_Status" → "SimpleStatus"
            rest.replace('_', "")
        }
    } else {
        // Module prefix doesn't match at all — fall back to first-underscore split
        extract_short_class_name(cpp_name)
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
        class_name[underscore_pos + 1..].to_string()
    } else {
        class_name.to_string()
    }
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
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => !all_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type && !has_protected_destructor)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => !all_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    match ty {
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            // CXX supports c_char but not Rust's char (which is 4-byte Unicode)
            RustTypeMapping {
                rust_type: "c_char".to_string(),
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
            
            let type_module = lookup_module_for_type(class_name, ctx.type_to_module);
            let short_name = extract_short_class_name(class_name);
            
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
                RustTypeMapping {
                    rust_type: class_name.clone(),
                    needs_unique_ptr: true,
                    needs_pin: false,
                    source_module: type_module,
                }
            }
        }
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            RustTypeMapping {
                rust_type: format!("&{}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            let needs_pin = !inner.is_primitive();
            if needs_pin {
                RustTypeMapping {
                    rust_type: format!("Pin<&mut {}>", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: true,
                    source_module: inner_mapping.source_module,
                }
            } else {
                RustTypeMapping {
                    rust_type: format!("&mut {}", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: inner_mapping.source_module,
                }
            }
        }
        Type::Handle(class_name) => {
            let source_module = lookup_module_for_type(class_name, ctx.type_to_module);
            // Use full class name to avoid collisions (e.g., Geom_Curve vs Geom2d_Curve)
            let handle_type = format!("Handle{}", class_name.replace("_", ""));
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true, // CXX requires UniquePtr for any opaque C++ type returned by value
                needs_pin: false,
                source_module,
            }
        }
        // Delegate primitives and other types to the context-free version
        _ => map_type_to_rust(ty),
    }
}

/// Map a return type in context
/// For const char* return types, maps to String (owned) rather than &str (borrowed)
pub fn map_return_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    // Special case: const char* return -> String (CXX supports rust::String as return type)
    if ty.is_c_string() {
        return RustTypeMapping {
            rust_type: "String".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        };
    }
    
    let mut mapping = map_type_in_context(ty, ctx);

    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("UniquePtr<{}>", mapping.rust_type);
    }

    mapping
}

/// Map a C++ type string directly (for cases where we only have the string)
pub fn map_cpp_type_string(cpp_type: &str) -> RustTypeMapping {
    let cpp_type = cpp_type.trim();

    // Handle primitives
    match cpp_type {
        "void" => return map_type_to_rust(&Type::Void),
        "bool" | "Standard_Boolean" => return map_type_to_rust(&Type::Bool),
        "int" | "Standard_Integer" => return map_type_to_rust(&Type::I32),
        "unsigned int" => return map_type_to_rust(&Type::U32),
        "long" => return map_type_to_rust(&Type::I64),
        "unsigned long" => return map_type_to_rust(&Type::U64),
        "float" => return map_type_to_rust(&Type::F32),
        "double" | "Standard_Real" => return map_type_to_rust(&Type::F64),
        _ => {}
    }

    // Handle const references
    if cpp_type.starts_with("const ") && cpp_type.ends_with('&') {
        let inner = cpp_type[6..cpp_type.len() - 1].trim();
        let inner_mapping = map_cpp_type_string(inner);
        return RustTypeMapping {
            rust_type: format!("&{}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        };
    }

    // Handle mutable references
    if let Some(inner) = cpp_type.strip_suffix('&') {
        let inner = inner.trim();
        let inner_mapping = map_cpp_type_string(inner);
        return RustTypeMapping {
            rust_type: format!("Pin<&mut {}>", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: true,
            source_module: inner_mapping.source_module,
        };
    }

    // Handle Handle types
    if cpp_type.starts_with("Handle(") && cpp_type.ends_with(')') {
        let inner = &cpp_type[7..cpp_type.len() - 1];
        return map_type_to_rust(&Type::Handle(inner.to_string()));
    }

    if cpp_type.starts_with("opencascade::handle<") && cpp_type.ends_with('>') {
        let inner = &cpp_type[20..cpp_type.len() - 1];
        return map_type_to_rust(&Type::Handle(inner.to_string()));
    }

    // Default: treat as C++ class type
    map_type_to_rust(&Type::Class(cpp_type.to_string()))
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
        assert_eq!(map_type_to_rust(&ty).rust_type, "&gp_Pnt");
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
        assert_eq!(mapping.rust_type, "UniquePtr<TopoDS_Shape>");
    }
}
