# Converting Raw Pointers to Unsafe Functions

- **Date:** 2026-02-20 07:11 – 09:10
- **Models:** Claude Opus 4.6, Claude Sonnet 4.6
- **Turns:** 8
- **Tool calls:** 455
- **Thinking blocks:** 33
- **Input words:** 79,317
- **Output words:** 18,306
- **Prompt tokens (last round):** 361,510
- **API rounds:** 302
- **Total elapsed:** 4915s

## Table of Contents

1. [Lets ahve the binding generator turn functions involving unhandled raw pointers into unsafe funct...](#user-1) (Claude Opus 4.6)
2. [@agent Try Again](#user-2) (Claude Opus 4.6)
3. [No, fix the broader parser issue. Don&#x27;t just skip the work. Don&#x27;t make them unbindable. The goal ...](#user-3) (Claude Opus 4.6)
4. [Why is signed 16-bit int being treated the same as unsigned 16-bit int? Can&#x27;t we tell the differe...](#user-4) (Claude Opus 4.6)
5. [Figure out where the parser is converting Standard_ExtCharacter to U16, and figure out how to hav...](#user-5) (Claude Opus 4.6) ⚠️ canceled
6. [OK, stop. What version of Standard_ExtCharacter is the parser seeing - the uint16_t one or the ch...](#user-6) (Claude Opus 4.6)
7. [First, let&#x27;s have the binding generator use the same -std=c++14 as the build. Make that change.](#user-7) (Claude Opus 4.6)
8. [commit](#user-8) (Claude Sonnet 4.6)

---

<a id="user-1"></a>

## User (1)

> Lets ahve the binding generator turn functions involving unhandled raw pointers into unsafe functions. For example, convert void* arguments/returns into *mut std::ffi::c_void and const Foo* argumentsreturns into *const foo, and the function marked unsafe.

*2026-02-20 07:16*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to modify the binding generator so that instead of skipping functions with raw pointer types (void*, T*, const T*), it generates them as `unsafe` functions with the appropriate Rust pointer types. Let me break this down:
> 
> 1. **Void pointers** (`Standard_Address` / `void*`) → `*mut std::ffi::c_void` 
> 2. **Raw pointer returns** (static/free fn `T*`/`const T*`) → `*const T` / `*mut T`
> 3. **Primitive pointer params** (`int*`, `double*`) → `*mut i32` / `*mut f64`
> 
> 
> 4. **Unsafe marking** in the re-export layer
> 
> Now I need to examine the current implementation to see where raw pointers are being filtered out and how to modify the codegen to produce unsafe functions instead.

**Todo list:**
- 🔄 Research current raw pointer handling
- ⬜ Understand exclusion reasons for pointers
- ⬜ Plan changes to type_mapping/resolver/codegen
- ⬜ Implement void* type mapping
- ⬜ Implement raw pointer param/return binding
- ⬜ Mark generated functions as unsafe
- ⬜ Regenerate bindings and build
- ⬜ Run tests

Let me start by understanding the current raw pointer handling in the codebase.

Research raw pointer handling

<details>
<summary>Searched for regex <code>is_method_bindable|is_constructor_bindable|is_static_method_bindable|ExclusionReason|raw.pointer|void.pointer|RawPointer|Unbindable</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:333](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L333)
- [bindings.rs:372](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L372)
- [bindings.rs:759](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L759)
- [bindings.rs:760](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L760)
- [bindings.rs:763](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L763)
- [bindings.rs:764](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L764)
- [bindings.rs:768](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L768)
- [bindings.rs:769](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L769)
- [bindings.rs:775](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L775)
- [bindings.rs:776](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L776)
- [bindings.rs:779](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L779)
- [bindings.rs:780](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L780)
- [bindings.rs:786](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L786)
- [bindings.rs:787](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L787)
- [bindings.rs:790](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L790)
- [bindings.rs:791](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L791)
- [bindings.rs:795](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L795)
- [bindings.rs:796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L796)
- [bindings.rs:802](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L802)
- [bindings.rs:803](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L803)
</details>

<details>
<summary>Searched for regex <code>pointer|void|raw|Standard_Address|skip|exclude|filter</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 20 results</summary>

- [resolver.rs:8](../crates/opencascade-binding-generator/src/resolver.rs#L8)
- [resolver.rs:52](../crates/opencascade-binding-generator/src/resolver.rs#L52)
- [resolver.rs:65](../crates/opencascade-binding-generator/src/resolver.rs#L65)
- [resolver.rs:84](../crates/opencascade-binding-generator/src/resolver.rs#L84)
- [resolver.rs:85](../crates/opencascade-binding-generator/src/resolver.rs#L85)
- [resolver.rs:93](../crates/opencascade-binding-generator/src/resolver.rs#L93)
- [resolver.rs:256](../crates/opencascade-binding-generator/src/resolver.rs#L256)
- [resolver.rs:292](../crates/opencascade-binding-generator/src/resolver.rs#L292)
- [resolver.rs:314](../crates/opencascade-binding-generator/src/resolver.rs#L314)
- [resolver.rs:345](../crates/opencascade-binding-generator/src/resolver.rs#L345)
- [resolver.rs:366](../crates/opencascade-binding-generator/src/resolver.rs#L366)
- [resolver.rs:374](../crates/opencascade-binding-generator/src/resolver.rs#L374)
- [resolver.rs:382](../crates/opencascade-binding-generator/src/resolver.rs#L382)
- [resolver.rs:390](../crates/opencascade-binding-generator/src/resolver.rs#L390)
- [resolver.rs:417](../crates/opencascade-binding-generator/src/resolver.rs#L417)
- [resolver.rs:433](../crates/opencascade-binding-generator/src/resolver.rs#L433)
- [resolver.rs:434](../crates/opencascade-binding-generator/src/resolver.rs#L434)
- [resolver.rs:442](../crates/opencascade-binding-generator/src/resolver.rs#L442)
- [resolver.rs:443](../crates/opencascade-binding-generator/src/resolver.rs#L443)
</details>

Searched for regex `Standard_Address|void|pointer|int\*|double\*|raw|RawPointer|Pointer` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 4 results
- [type_mapping.rs:15](../crates/opencascade-binding-generator/src/type_mapping.rs#L15)
- [type_mapping.rs:26](../crates/opencascade-binding-generator/src/type_mapping.rs#L26)
- [type_mapping.rs:486](../crates/opencascade-binding-generator/src/type_mapping.rs#L486)

<details>
<summary>Searched for regex <code>unsafe|fn |impl |pub fn|signature|re.export</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 20 results</summary>

- [rust.rs:4](../crates/opencascade-binding-generator/src/codegen/rust.rs#L4)
- [rust.rs:11](../crates/opencascade-binding-generator/src/codegen/rust.rs#L11)
- [rust.rs:27](../crates/opencascade-binding-generator/src/codegen/rust.rs#L27)
- [rust.rs:76](../crates/opencascade-binding-generator/src/codegen/rust.rs#L76)
- [rust.rs:106](../crates/opencascade-binding-generator/src/codegen/rust.rs#L106)
- [rust.rs:124](../crates/opencascade-binding-generator/src/codegen/rust.rs#L124)
- [rust.rs:133](../crates/opencascade-binding-generator/src/codegen/rust.rs#L133)
- [rust.rs:190](../crates/opencascade-binding-generator/src/codegen/rust.rs#L190)
- [rust.rs:203](../crates/opencascade-binding-generator/src/codegen/rust.rs#L203)
- [rust.rs:204](../crates/opencascade-binding-generator/src/codegen/rust.rs#L204)
- [rust.rs:374](../crates/opencascade-binding-generator/src/codegen/rust.rs#L374)
- [rust.rs:394](../crates/opencascade-binding-generator/src/codegen/rust.rs#L394)
- [rust.rs:400](../crates/opencascade-binding-generator/src/codegen/rust.rs#L400)
- [rust.rs:417](../crates/opencascade-binding-generator/src/codegen/rust.rs#L417)
- [rust.rs:443](../crates/opencascade-binding-generator/src/codegen/rust.rs#L443)
- [rust.rs:449](../crates/opencascade-binding-generator/src/codegen/rust.rs#L449)
</details>

<details>
<summary>Searched for regex <code>ParamType|Pointer|RawPointer|TypeRef|ResolvedType|enum.*Type</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 20 results</summary>

- [model.rs:214](../crates/opencascade-binding-generator/src/model.rs#L214)
- [model.rs:215](../crates/opencascade-binding-generator/src/model.rs#L215)
- [model.rs:244](../crates/opencascade-binding-generator/src/model.rs#L244)
- [model.rs:245](../crates/opencascade-binding-generator/src/model.rs#L245)
- [model.rs:247](../crates/opencascade-binding-generator/src/model.rs#L247)
- [model.rs:248](../crates/opencascade-binding-generator/src/model.rs#L248)
- [model.rs:312](../crates/opencascade-binding-generator/src/model.rs#L312)
- [model.rs:313](../crates/opencascade-binding-generator/src/model.rs#L313)
- [model.rs:315](../crates/opencascade-binding-generator/src/model.rs#L315)
- [model.rs:316](../crates/opencascade-binding-generator/src/model.rs#L316)
- [model.rs:379](../crates/opencascade-binding-generator/src/model.rs#L379)
- [model.rs:396](../crates/opencascade-binding-generator/src/model.rs#L396)
- [model.rs:429](../crates/opencascade-binding-generator/src/model.rs#L429)
- [model.rs:490](../crates/opencascade-binding-generator/src/model.rs#L490)
- [model.rs:505](../crates/opencascade-binding-generator/src/model.rs#L505)
- [model.rs:551](../crates/opencascade-binding-generator/src/model.rs#L551)
- [model.rs:552](../crates/opencascade-binding-generator/src/model.rs#L552)
- [model.rs:559](../crates/opencascade-binding-generator/src/model.rs#L559)
- [model.rs:565](../crates/opencascade-binding-generator/src/model.rs#L565)
- [model.rs:568](../crates/opencascade-binding-generator/src/model.rs#L568)
</details>

<details>
<summary>Reading lines 390-610 of <a href="../crates/opencascade-binding-generator/src/model.rs#L390-L611">model.rs</a></summary>

```
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
    /// unsigned short / uint16_t / Standard_ExtCharacter
    U16,
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
            Type::U16 => "u16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.short_name(),
            Type::ConstPtr(inner) | Type::MutPtr(inner) => format!("{}ptr", inner.short_name()),
            Type::Handle(name) => format!("handle{}", name.to_lowercase().replace('_', "")),
            Type::Class(name) => extract_short_name(name),
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
                | Type::I64
                | Type::U64
                | Type::Long
                | Type::ULong
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    /// Check if this type is suitable as a field in a POD struct.
    /// Only primitive numeric types (bool, integers, floats) are POD-safe.
    pub fn is_pod_field_type(&self) -> bool {
        matches!(
            self,
            Type::Bool | Type::I32 | Type::U32 | Type::U16 | Type::I64 | Type::U64
                | Type::Long | Type::ULong | Type::Usize | Type::F32 | Type::F64
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
    /// These can't be bound through the FFI
    pub fn is_stream(&self) -> bool {
        match self {
... (truncated, 8170 chars)
```
</details>

<details>
<summary>Reading lines 210-330 of <a href="../crates/opencascade-binding-generator/src/model.rs#L210-L331">model.rs</a></summary>

```

        format!("_{}", parts.join("_"))
    }

    /// Check if this constructor has any unbindable types (C strings, streams, void pointers, etc.)
    /// Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
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

    /// Generate a suffix for distinguishing overloaded methods
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (Pnt) -> "_pnt", (Box, Trsf) -> "_box_trsf", (f64, f64, f64) -> "_real3"
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
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_n
... (truncated, 4393 chars)
```
</details>

<details>
<summary>Reading lines 320-400 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L320-L401">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 750-840 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L750-L841">bindings.rs</a></summary>

```
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
fn generate_static_method_stub(_class_name: &str, method: &StaticMethod) -> String 
... (truncated, 4275 chars)
```
</details>

<details>
<summary>Reading lines 40-120 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L40-L121">resolver.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 1-60 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L61">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

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
        Type::I64 => RustTypeMapping {
            rust_type: "i64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
```
</details>

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#390-390), lines 390 to 620

Read [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs#1-1), lines 1 to 200

<details>
<summary>Searched for regex <code>generate_reexport|generate_module_reexport|impl_block|generate_impl|re_export|reexport</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 17 results</summary>

- [rust.rs:640](../crates/opencascade-binding-generator/src/codegen/rust.rs#L640)
- [rust.rs:649](../crates/opencascade-binding-generator/src/codegen/rust.rs#L649)
- [rust.rs:663](../crates/opencascade-binding-generator/src/codegen/rust.rs#L663)
- [rust.rs:675](../crates/opencascade-binding-generator/src/codegen/rust.rs#L675)
- [rust.rs:682](../crates/opencascade-binding-generator/src/codegen/rust.rs#L682)
- [rust.rs:688](../crates/opencascade-binding-generator/src/codegen/rust.rs#L688)
- [rust.rs:740](../crates/opencascade-binding-generator/src/codegen/rust.rs#L740)
- [rust.rs:827](../crates/opencascade-binding-generator/src/codegen/rust.rs#L827)
- [rust.rs:831](../crates/opencascade-binding-generator/src/codegen/rust.rs#L831)
- [rust.rs:836](../crates/opencascade-binding-generator/src/codegen/rust.rs#L836)
- [rust.rs:842](../crates/opencascade-binding-generator/src/codegen/rust.rs#L842)
- [rust.rs:844](../crates/opencascade-binding-generator/src/codegen/rust.rs#L844)
- [rust.rs:857](../crates/opencascade-binding-generator/src/codegen/rust.rs#L857)
- [rust.rs:863](../crates/opencascade-binding-generator/src/codegen/rust.rs#L863)
- [rust.rs:867](../crates/opencascade-binding-generator/src/codegen/rust.rs#L867)
- [rust.rs:870](../crates/opencascade-binding-generator/src/codegen/rust.rs#L870)
</details>

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#620-620), lines 620 to 740

Searched for regex `emit_reexport_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4725](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4725)

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#620-620), lines 620 to 620

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4725-4725), lines 4725 to 5000

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#620-620), lines 620 to 835

<details>
<summary>Searched for regex <code>has_unbindable|UnbindableType|is_unbindable|unbindable</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 20 results</summary>

- [resolver.rs:65](../crates/opencascade-binding-generator/src/resolver.rs#L65)
- [resolver.rs:66](../crates/opencascade-binding-generator/src/resolver.rs#L66)
- [resolver.rs:67](../crates/opencascade-binding-generator/src/resolver.rs#L67)
- [resolver.rs:68](../crates/opencascade-binding-generator/src/resolver.rs#L68)
- [resolver.rs:69](../crates/opencascade-binding-generator/src/resolver.rs#L69)
- [resolver.rs:70](../crates/opencascade-binding-generator/src/resolver.rs#L70)
- [resolver.rs:71](../crates/opencascade-binding-generator/src/resolver.rs#L71)
- [resolver.rs:72](../crates/opencascade-binding-generator/src/resolver.rs#L72)
- [resolver.rs:1062](../crates/opencascade-binding-generator/src/resolver.rs#L1062)
- [resolver.rs:1063](../crates/opencascade-binding-generator/src/resolver.rs#L1063)
- [resolver.rs:1112](../crates/opencascade-binding-generator/src/resolver.rs#L1112)
- [resolver.rs:1113](../crates/opencascade-binding-generator/src/resolver.rs#L1113)
- [resolver.rs:1114](../crates/opencascade-binding-generator/src/resolver.rs#L1114)
- [resolver.rs:1175](../crates/opencascade-binding-generator/src/resolver.rs#L1175)
- [resolver.rs:1176](../crates/opencascade-binding-generator/src/resolver.rs#L1176)
- [resolver.rs:1243](../crates/opencascade-binding-generator/src/resolver.rs#L1243)
- [resolver.rs:1245](../crates/opencascade-binding-generator/src/resolver.rs#L1245)
- [resolver.rs:1246](../crates/opencascade-binding-generator/src/resolver.rs#L1246)
- [resolver.rs:1334](../crates/opencascade-binding-generator/src/resolver.rs#L1334)
- [resolver.rs:1337](../crates/opencascade-binding-generator/src/resolver.rs#L1337)
</details>

<details>
<summary>Searched for regex <code>is_unbindable|has_unbindable|unbindable_types</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:760](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L760)
- [bindings.rs:763](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L763)
- [bindings.rs:768](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L768)
- [bindings.rs:776](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L776)
- [bindings.rs:779](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L779)
- [bindings.rs:787](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L787)
- [bindings.rs:790](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L790)
- [bindings.rs:795](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L795)
- [bindings.rs:875](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L875)
- [bindings.rs:876](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L876)
- [bindings.rs:964](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L964)
- [bindings.rs:965](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L965)
- [bindings.rs:1014](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1014)
- [bindings.rs:1015](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1015)
- [bindings.rs:2162](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2162)
- [bindings.rs:2284](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2284)
- [bindings.rs:3127](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3127)
- [bindings.rs:3271](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3271)
- [bindings.rs:3274](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3274)
- [bindings.rs:3292](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3292)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#860-860), lines 860 to 930

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#950-950), lines 950 to 1050

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#1040-1040), lines 1040 to 1080

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#1100-1100), lines 1100 to 1260

<details>
<summary>Searched for regex <code>ParsedFunction|impl ParsedFunction|pub fn has_unbindable_types</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 7 results</summary>

- [model.rs:21](../crates/opencascade-binding-generator/src/model.rs#L21)
- [model.rs:26](../crates/opencascade-binding-generator/src/model.rs#L26)
- [model.rs:47](../crates/opencascade-binding-generator/src/model.rs#L47)
- [model.rs:49](../crates/opencascade-binding-generator/src/model.rs#L49)
- [model.rs:216](../crates/opencascade-binding-generator/src/model.rs#L216)
- [model.rs:246](../crates/opencascade-binding-generator/src/model.rs#L246)
- [model.rs:314](../crates/opencascade-binding-generator/src/model.rs#L314)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3100-3100), lines 3100 to 3320

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#26-26), lines 26 to 75

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#700-700), lines 700 to 730

Searched for regex `fn build_reexport_body` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4695](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4695)

Searched for regex `Standard_Address|void\s*\*` (`**/crates/opencascade-binding-generator/src/parser.rs`), 4 results
- [parser.rs:1605](../crates/opencascade-binding-generator/src/parser.rs#L1605)
- [parser.rs:1607](../crates/opencascade-binding-generator/src/parser.rs#L1607)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4695-4695), lines 4695 to 4725

Read [parser.rs](../crates/opencascade-binding-generator/src/parser.rs#1595-1595), lines 1595 to 1620

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Pj1LFWuM3sPKMCcYaq4nmn__vscode-1771312785663/content.txt)

**Todo list:**
- ✅ Research current raw pointer handling
- 🔄 Read key source files in detail
- ⬜ Plan changes to type_mapping/resolver/codegen
- ⬜ Implement void* type mapping
- ⬜ Implement raw pointer param/return binding
- ⬜ Mark generated functions as unsafe
- ⬜ Regenerate bindings and build
- ⬜ Run tests

Now let me read the actual source files to understand the details precisely.Good. Let me now read the key areas of the source files in detail.

<details>
<summary>Reading lines 1-80 of <a href="../crates/opencascade-binding-generator/src/model.rs#L1-L81">model.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 396-670 of <a href="../crates/opencascade-binding-generator/src/model.rs#L396-L671">model.rs</a></summary>

```
pub enum Type {
    /// void
    Void,
    /// bool / Standard_Boolean
    Bool,
    /// int / Standard_Integer
    I32,
    /// unsigned int
    U32,
    /// unsigned short / uint16_t / Standard_ExtCharacter
    U16,
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
            Type::U16 => "u16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.short_name(),
            Type::ConstPtr(inner) | Type::MutPtr(inner) => format!("{}ptr", inner.short_name()),
            Type::Handle(name) => format!("handle{}", name.to_lowercase().replace('_', "")),
            Type::Class(name) => extract_short_name(name),
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
                | Type::I64
                | Type::U64
                | Type::Long
                | Type::ULong
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    /// Check if this type is suitable as a field in a POD struct.
    /// Only primitive numeric types (bool, integers, floats) are POD-safe.
    pub fn is_pod_field_type(&self) -> bool {
        matches!(
            self,
            Type::Bool | Type::I32 | Type::U32 | Type::U16 | Type::I64 | Type::U64
                | Type::Long | Type::ULong | Type::Usize | Type::F32 | Type::F64
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
    /// These can't be bound through the FFI
    pub fn is_stream(&self) -> bool {
        match self {
            Type::Class(name) => {
                name.contains("OStream")
             
... (truncated, 10835 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_018o62iA4jbL1NpVVxUnT4kq__vscode-1771312785700/content.txt)

Now let me read the key sections of the other files I need to modify.

<details>
<summary>Reading lines 199-330 of <a href="../crates/opencascade-binding-generator/src/model.rs#L199-L331">model.rs</a></summary>

```
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
    /// Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
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

    /// Generate a suffix for distinguishing overloaded methods
    /// based on parameter types, with consecutive identical types compressed.
    /// E.g., (Pnt) -> "_pnt", (Box, Trsf) -> "_box_trsf", (f64, f64, f64) -> "_real3"
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
    /// in parameters or return type. Nullable pointer 
... (truncated, 4754 chars)
```
</details>

<details>
<summary>Reading lines 1-200 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L201">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

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
  
... (truncated, 7298 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01FQjUSMSbqY6CNd19AM7ym4__vscode-1771312785703/content.txt)

Now let me read the bindings.rs file to see how methods are checked and generated:

Read bindings.rs key sections

Searched for text `pub struct MethodBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for regex <code>pub struct (MethodBinding|ConstructorBinding|StaticMethodBinding|FunctionBinding|ParamBinding|ReturnTypeBinding)</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:104](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L104)
- [bindings.rs:199](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L199)
- [bindings.rs:293](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L293)
- [bindings.rs:320](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L320)
- [bindings.rs:380](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L380)
</details>

Searched for regex `struct MethodBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for regex <code>MethodBinding</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 17 results</summary>

- [bindings.rs:44](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L44)
- [bindings.rs:45](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L45)
- [bindings.rs:46](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L46)
- [bindings.rs:54](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L54)
- [bindings.rs:136](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L136)
- [bindings.rs:176](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L176)
- [bindings.rs:199](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L199)
- [bindings.rs:270](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L270)
- [bindings.rs:1727](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1727)
- [bindings.rs:1751](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1751)
- [bindings.rs:1784](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1784)
- [bindings.rs:1834](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1834)
- [bindings.rs:1887](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1887)
- [bindings.rs:1909](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1909)
- [bindings.rs:1995](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1995)
- [bindings.rs:2614](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2614)
- [bindings.rs:2962](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2962)
</details>

Searched for regex `fn is_method_bindable|fn is_constructor_bindable|fn is_static_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:874](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874)
- [bindings.rs:956](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L956)
- [bindings.rs:1013](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1013)

Searched for regex `fn compute_all_function_bindings|impl_methods.*pub fn|is_unsafe` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3177](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3177)

<details>
<summary>Searched for regex <code>impl_methods</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 13 results</summary>

- [bindings.rs:4774](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4774)
- [bindings.rs:4798](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4798)
- [bindings.rs:4809](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4809)
- [bindings.rs:4871](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4871)
- [bindings.rs:4933](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4933)
- [bindings.rs:4987](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4987)
- [bindings.rs:5007](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5007)
- [bindings.rs:5012](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5012)
- [bindings.rs:5021](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5021)
- [bindings.rs:5031](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5031)
- [bindings.rs:5085](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5085)
- [bindings.rs:5104](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5104)
- [bindings.rs:5106](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5106)
</details>

<details>
<summary>Reading lines 30-100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L30-L101">bindings.rs</a></summary>

```
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

```
</details>

<details>
<summary>Reading lines 100-300 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L100-L301">bindings.rs</a></summary>

```

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
}

/// A static method binding.
#[derive(Debug, Clone)]
pub struct StaticMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_origin_static")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ for instance/static conflicts)
    pub impl_method_name: String,
    /// Parameters
  
... (truncated, 7578 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Trsnsv29nefW6hLKmC4som__vscode-1771312785715/content.txt)

<details>
<summary>Reading lines 300-430 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L300-L431">bindings.rs</a></summary>

```
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
    pub cpp_wrapper_name
... (truncated, 5974 chars)
```
</details>

<details>
<summary>Reading lines 874-1080 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874-L1081">bindings.rs</a></summary>

```
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
    if resolver::method_needs_explicit_lifetimes(method) {
        return Err("returns &mut with reference params (ambiguous lifetimes)".to_string());
    }
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
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return Err(format!("return type '{}' is not CppDeletable", name));
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if ret
... (truncated, 9687 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011hXSuCDRBUPkWX5k4QSNAG__vscode-1771312785718/content.txt)

<details>
<summary>Reading lines 4770-5120 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4770-L5121">bindings.rs</a></summary>

```
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
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
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
            &format!("{}::{}(
... (truncated, 14175 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014BcH326RyMXvACquHvvNh8__vscode-1771312785720/content.txt)

<details>
<summary>Reading lines 3177-3350 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3177-L3351">bindings.rs</a></summary>

```
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
    manual_type_names: &HashSet<String>,
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

    let handle_able_classes = compute_handle_able_classes(all_classes);

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
        if let Some(p) = unbi
... (truncated, 8060 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01YLKLLiAsfgVexFusk1zqJj__vscode-1771312785722/content.txt)

<details>
<summary>Reading lines 3350-3550 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3350-L3551">bindings.rs</a></summary>

```
                        cpp_name: format!("{}::{}", func.namespace, func.short_name),
                        source_header: func.source_header.clone(),
                        source_line: func.source_line,
                        doc_comment: func.doc_comment.clone(),
                        skip_reason: "returns &mut with reference params \u{2014} ambiguous lifetime".to_string(),
                        stub_rust_decl: generate_function_stub(func),
                    });
                    continue;
                }
            }
        }

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

        // Collect C++ headers needed for 
... (truncated, 7967 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01DKuJ5AdTYpfj7bL5BEaGuk__vscode-1771312785724/content.txt#1-1), lines 1 to 200

Now let me read the critical sections I need to modify in detail:

<details>
<summary>Reading lines 30-135 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L30-L136">bindings.rs</a></summary>

```
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
}

/// Info for a convenience constructor that delegates to a full-argument version.
#[derive(Debug, Clone)]
pub struct ConvenienceInfo {
    /// The impl_method_name of the full-argument constructor to call
    pub full_method_name: String,
    /// Rust expressions for the default values of the trimmed trailing params,
    /// in order. E.g. ["false", "false"] for two d
... (truncated, 4170 chars)
```
</details>

<details>
<summary>Reading lines 136-420 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L136-L421">bindings.rs</a></summary>

```
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
    /// Der
... (truncated, 11496 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_018dWnc1sxBd3bePPzBjWv8Z__vscode-1771312785727/content.txt)

Now let me read the `is_method_bindable` etc. and the impl generation code:

<details>
<summary>Reading lines 800-1080 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800-L1081">bindings.rs</a></summary>

```
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
    // Skip methods with const ch
... (truncated, 13357 chars)
```
</details>

<details>
<summary>Reading lines 4770-5120 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4770-L5121">bindings.rs</a></summary>

```
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
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
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
            &format!("{}::{}(
... (truncated, 14175 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01CSTemZPm4pEnQtidg86sUJ__vscode-1771312785729/content.txt)

Now let me read the impl method generation code:

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Wn9oujYqpBa9KoyLCGE1KF__vscode-1771312785730/content.txt)

Now let me also look at how free functions are emitted in rust.rs:

<details>
<summary>Reading lines 680-780 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L680-L781">rust.rs</a></summary>

```

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
            call_expr
        }
    } else {
        call_expr
    };

    // Generate postamble for &mut enum writeback
    let postamble: String = func.params.iter()
        .filter_map(|p| {
            p.mut_ref_enum_rust_type.as_ref().map(|enum_type| {
                format!("    *{} = {}::try_from({}_i32_).unwrap();\n", p.rust_name, enum_type, p.rust_name)
            })
        })
        .collect();

    let has_return = !return_type_str.is_empty();

    writeln!(output, "pub fn {}({}){} {{", func.rust_ffi_name, params.join(", "), return_type_str).unwrap();
    write!(output, "{}", prelude).unwrap();

    if postamble.is_empty() {
        writeln!(output, "    unsafe {{ {} }}", body).unwrap();
    } else if has_return {
        writeln!(output, "    let result_ = unsafe {{ {} }};", body).unwrap();
        write!(output, "{}", postamble).unwrap();
        writeln!(output, "    result_").unwrap();
    } else {
        writeln!(output, "    unsafe {{ {} }};", body).unwrap();
        // Trim trailing newline from postamble for clean formatting
        write!(output, "{}", postamble.trim_end_matches('\n')).unwrap();
        writeln!(output).unwrap();
    }

    writeln!(output, "}}").unwrap();
}

/// Generate a module re-export file
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    module_fn_bindings: &[&super::bindings::FunctionBinding],
    module_skipped_fns: &[&super::bindings::SkippedSymbol],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");

    // Generate re-exports for free functions from pre-computed FunctionBindings.
    for func in module_fn_bindings {
        // All free functions become real wrapper functions (not pub use re-exports)
        // so IDE "go to definition" lands in the public module.
        emit_free_function_wrapper(&mut output, func);
    }

    if !module_fn_bindings.is_empty() {
        output.push('\n');
    }

    // Generate Rust enum definitions for enums in this module
    let rust_module = crate::module_graph::module_to_rust_name(module_name);
    if let Some(enum_ids) = symbol_table.enums_by_module.get(&rust_module) {
        for enum_id in enum_ids {
            if let Some(resolved_enum) = symbol_table.enums.get(enum_id) {

... (truncated, 4142 chars)
```
</details>

Now let me also look at how `compute_all_function_bindings()` handles things:

<details>
<summary>Reading lines 3177-3470 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3177-L3471">bindings.rs</a></summary>

```
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
    manual_type_names: &HashSet<String>,
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

    let handle_able_classes = compute_handle_able_classes(all_classes);

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
        if let Some(p) = unbi
... (truncated, 12981 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_0182ceYYnSgVnap6Xm6mz3Qr__vscode-1771312785734/content.txt)

Now let me also look at how the resolver handles unbindable types:

Searched for text `has_unbindable_types` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 4 results
- [resolver.rs:1062](../crates/opencascade-binding-generator/src/resolver.rs#L1062)
- [resolver.rs:1112](../crates/opencascade-binding-generator/src/resolver.rs#L1112)
- [resolver.rs:1175](../crates/opencascade-binding-generator/src/resolver.rs#L1175)
- [resolver.rs:1245](../crates/opencascade-binding-generator/src/resolver.rs#L1245)

<details>
<summary>Reading lines 1050-1270 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1050-L1271">resolver.rs</a></summary>

```
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
    let return_type = method.return_type.as_ref().map(|t| resolve_type(t, all_enu
... (truncated, 7732 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_018ZfuKkJzwqGgN7AvzeqXY9__vscode-1771312785737/content.txt#1-1), lines 1 to 220

**Todo list:**
- ✅ Research current raw pointer handling
- ✅ Read key source files in detail
- 🔄 Plan changes to type_mapping/resolver/codegen
- ⬜ Implement void* type mapping
- ⬜ Implement raw pointer param/return binding
- ⬜ Mark generated functions as unsafe
- ⬜ Regenerate bindings and build
- ⬜ Run tests

> 💭 Now I have a comprehensive understanding of the codebase. Let me plan the changes:
> 
> ## Plan
> 
> The key concept is that raw pointers (`void*`, `int*`, `double*`, `T*`, `const T*`) are currently treated as "unbindable" via `is_unbindable()` → `is_void_ptr() || is_raw_ptr()`. We want to instead:
> 
> 1. **Allow them through** the binding pipeline
> 2. **Map them to correct Rust types** (`*mut std::ffi::c_void` for void*, `*const T`/`*mut T` for class pointers, `*const f64`/`*mut i32` for primitive pointers)
> 
> 
> 3. **Mark the generated re-export functions as `pub unsafe fn`** instead of `pub fn`
> 
> ### Detailed changes needed:
> 
> #### 1. `model.rs` — Split `is_unbindable()` into two concepts
> I need to refactor the unbindable logic by keeping the detection methods but introducing a new `is_unsafe_ptr()` function that identifies pointers requiring unsafe marking. Then I'll update `is_unbindable()` to exclude void and raw pointers, and modify `has_unbindable_types()` across all struct types to let raw pointers pass through the binding pipeline.
> 
> Looking more closely at the current `has_unbindable_types()` implementation, I see it already has exceptions for nullable pointers and class pointer inner names. The cases that still get caught are void pointers, primitive pointer parameters where `class_ptr_inner_name()` returns None, raw pointer returns on static/free functions, and references to raw pointers. Let me trace through the logic in `ParsedFunction` more carefully to understand exactly when a parameter gets flagged as unbindable. So for Method, I need to handle both parameters and return types differently — parameters get the nullable pointer and class pointer exceptions, but return types only get the class pointer exception since instance methods can bind class pointer returns as Option references. Now I'm mapping out the actual changes needed in model.rs to remove raw and void pointers from the is_unbindable check itself. I'm realizing that if I remove `is_void_ptr()` and `is_raw_ptr()` from `is_unbindable()`, then `has_unbindable_types()` will automatically stop filtering them out since it relies on `is_unbindable()`, which means all raw pointers will pass through to the resolver and bindings logic without needing any additional changes.
> 
> Now I need to handle the `Standard_Address` type mapping in `type_mapping.rs` by adding a special case that converts it to a mutable void pointer before the general class type handling kicks in.
> 
> I'm realizing there's a subtlety with how const qualifiers interact with this typedef — `const Standard_Address` in C++ means a const pointer to void, but my current mapping would produce a const pointer to a mutable void pointer, which isn't semantically equivalent. Let me reconsider how these type qualifiers should compose with the underlying void pointer representation. `Standard_Address` maps to `*mut std::ffi::c_void`, and wrapping it in ConstPtr/MutPtr would create double pointers. But I need to reconsider what the `is_void_ptr()` check is actually catching—it's picking up `ConstRef(Standard_Address)` and `MutRef(Standard_Address)`, which are references to the void pointer itself, not references through the void pointer.
> 
> The real issue is that `map_type_to_rust` for `Standard_Address` currently returns a mapping that marks it as needing unique_ptr handling, but that's not quite right since it's already a pointer type.
> 
> If I change the mapping to return `*mut std::ffi::c_void` with `needs_unique_ptr: false`, then parameters and return types work correctly, but wrapping it in ConstRef creates `*const *mut c_void` which doesn't quite match the C++ semantics of `const Standard_Address&`.
> 
> Let me focus on the core cases: `Standard_Address` as a parameter or return should both map to `*mut c_void`. The ConstRef and MutRef wrappers will handle themselves through the existing type mapping logic, and whatever gets produced should work through FFI.
> 
> The real issue is that `Standard_Address` is currently parsed as a class type and gets the default class mapping with `needs_unique_ptr: true`, but it's actually just a void pointer, so I need to override that behavior specifically for this type.
> 
> Adding a special case that maps `Standard_Address` directly to `*mut c_void` with `needs_unique_ptr: false` prevents the return type from getting double-wrapped. For the C++ side, `Standard_Address` is already void pointer in the original code, so the wrappers just pass those values through unchanged. Primitive pointers like `int*` and `double*` already have the right mappings in place.
> 
> Now I need to track which functions require `unsafe` by adding an `is_unsafe` field to all the binding structs, then emit that in the codegen...
> 
> Actually, the cleanest approach is to set this flag whenever a parameter or return type contains raw pointers — checking for `is_void_ptr()` or `is_raw_ptr()` cases that aren't already handled as nullable or reference types. I see that for class method parameters, `nullable_ptr` and `class_ptr` are already converted to safe reference types, so they shouldn't trigger the unsafe requirement even though the underlying type is a raw pointer. I need to refine the unsafe check to exclude these cases for parameters, and similarly handle return types where class pointers are already bound as `Option<&T>`. This mirrors the logic from the old `has_unbindable_types()` check, so I'll add `needs_unsafe_fn()` methods to the model types that capture these same conditions.
> 
> The cleanest approach is to separate concerns: modify `is_unbindable()` to exclude raw and void pointers, then add a new `has_unsafe_types()` method to function and method types that specifically checks for raw pointer and void pointer cases. This way, bindability checks pass through while unsafe detection happens separately in codegen. I need to be careful about the existing patterns in `has_unbindable_types()` for Method parameters though, since there are specific cases already being handled. I'm checking the constructor's unbindable type logic—it only examines parameters since constructors always return Self. For static methods, the return type check is stricter and doesn't have the class pointer exception, so class pointer returns were being blocked there too. Let me trace through exactly what parameters will now pass through with these changes. Looking at the unbindable type checks in the static method logic, I see that class pointer returns like `const SomeClass*` are being marked as unbindable because `is_raw_ptr()` returns true, which cascades through the checks and causes these methods to be excluded entirely.
> 
> But the README actually documents that static and free function class pointer returns are meant to be excluded from binding. With this change, they'd start coming through, which creates a problem—unlike instance methods where we can bind class pointers as `Option<&T>` tied to the `self` lifetime, static functions have no such lifetime anchor, so they'd need to be exposed as raw pointers marked unsafe instead.
> 
> Let me map out what's currently safe versus what's changing. Instance method class pointer parameters and returns are already handled—parameters become `&T` or `&mut T`, and returns become `Option<&T>` or `Option<&mut T>` bound to `self`. The new unsafe category includes void pointers, primitive pointers, and class pointer returns from static methods and free functions, which would all need raw pointer bindings. I need to verify whether the type mapping logic will handle class pointer returns on static methods correctly.
> 
> Looking at how `ConstPtr(Class("gp_Pnt"))` gets mapped—it recursively maps the inner class type first, then wraps it as a raw pointer, which should work. The issue is that the reexport layer might be generating reference types instead of raw pointers for these cases. I need to trace through the binding construction functions to see where the type gets transformed.

Let me look at the `build_param_binding` and `build_return_type_binding` functions:

Searched for text `fn build_param_binding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1102](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1102)

Searched for text `fn build_return_type_binding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1282](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282)

<details>
<summary>Reading lines 1102-1400 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1102-L1401">bindings.rs</a></summary>

```
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
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = type_to_cpp(inner);
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
                let cpp_inner = type_to_cpp(inner);
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
         
... (truncated, 11948 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01STpn6tSwkdL2vaZ55oKWYA__vscode-1771312785742/content.txt)

> 💭 Now I have a good understanding. Let me also check the `type_to_rust_string` function used in the re-export layer, and the `return_type_to_rust_string` function, since these determine what the re-exported Rust types look like.

Searched for text `fn type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:668](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L668)

Searched for text `fn return_type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:709](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L709)

<details>
<summary>Reading lines 668-800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L668-L801">bindings.rs</a></summary>

```
fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
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
        Type::Class(name) if name != "char" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
            parts.push(format!("param '{}'
... (truncated, 5517 chars)
```
</details>

> 💭 Now I understand the full picture. Let me also check how the C++ wrapper code handles types — specifically, for the new types that will now pass through (void*, raw primitive pointers), does the C++ wrapper generation already handle them? Let me look at `type_to_cpp()` and `type_to_cpp_extern_c_param()`.

Searched for text `fn type_to_cpp(` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:584](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L584)

Searched for text `fn type_to_cpp_extern_c_param` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L610)

Searched for text `fn param_to_cpp_extern_c_arg` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:623](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L623)

<details>
<summary>Reading lines 584-668 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L584-L669">bindings.rs</a></summary>

```
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
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


/// Convert a Type to C++ parameter type for extern "C" wrapper functions (pointers instead of references)
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "const char*".to_string()
        }
        _ => type_to_cpp(ty),
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
```
</details>

> 💭 OK now I have a comprehensive understanding. Let me also check - does `type_to_rust_string` handle `Standard_Address`? Currently:
> 
> ```rust
> Type::Class(name) => {
>     if name == "char" {
>         "std::ffi::c_char".to_string()
>     } else if let Some(ctx) = reexport_ctx {
>         ctx.resolve_class(name)
>     } else {
>         format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
>     }
> }
> ```
> 
> So `Standard_Address` would map to `crate::ffi::Standard_Address` or whatever the reexport context resolves to. 
> 
> But since `Standard_Address` is a typedef rather than an actual FFI class declaration, this approach would fail. I need to add special handling for it. Similarly, `return_type_to_rust_string` would incorrectly try to wrap `ConstPtr(Class("Standard_Address"))` as `Option<&crate::ffi::Standard_Address>`, which is wrong. Let me map out all the changes I need to make comprehensively.
> 
> In `model.rs`, I'll first update `is_unbindable()` to remove the void pointer and raw pointer checks since those need different handling. Then I'll add a new `needs_unsafe_fn()` method that specifically identifies types involving unsafe raw pointers — void pointers like `Standard_Address` and raw `T*`/`const T*` pointers that aren't already handled as safe references. Finally, I'll add `has_unsafe_types()` methods to `ParsedFunction`, `Constructor`, `Method`, and `StaticMethod` to propagate this information...
> 
> Adding unsafe type checking to function signatures... I'm implementing the same `has_unsafe_types` method for `ParsedFunction`, checking if any parameters or return types need unsafe handling. Then I'm moving to `type_mapping.rs` to add a special case for `Standard_Address`...
> 
> Adding unsafe field to bindings...
> 
> I need to think through how to mark bindings as unsafe. The straightforward approach would be to check the generated Rust type strings for raw pointer patterns, but that's fragile. Instead, I should add flags like `is_raw_ptr` to `ParamBinding` and `is_raw_ptr_return` to `ReturnTypeBinding`, then derive `is_unsafe` from those. Though there's a complication with void pointers like `Standard_Address` which appear as regular class types at the parameter level, not raw pointers, so I need to handle that case specially. The real problem is that `Standard_Address` gets wrapped in `ConstRef` during param binding because it's recognized as a class type, resulting in `*const *mut void*` instead of just `*mut void*`. I need to either parse it as a raw pointer type from the start so it bypasses class handling entirely, or add special-case logic to exclude it from the const reference wrapping like I do for `char`.
> 
> Looking at the parser, changing `Standard_Address` to parse as `Type::MutPtr(Type::Void)` would fix the issue cleanly — it would then be recognized as a raw pointer and skip the problematic const reference wrapping, though I'd need to update the `is_void_ptr()` check since it currently looks for the class variant.
> 
> The real problem is that `Type::Void` maps to `()` in Rust, not `c_void`, so I need to handle that mapping specially when generating the final type signature.
> 
> I think the cleanest approach is to keep the parser as-is and instead handle `Standard_Address` properly in the binding generation functions. Specifically, I'd add a check in `build_param_binding` to recognize `Standard_Address` and treat it as a primitive opaque pointer rather than wrapping it in a reference type.
> 
> Adding special handling for Standard_Address... I'm realizing this approach keeps the parsing unchanged and only adjusts how Standard_Address maps to Rust types in the binding layer, which is cleaner. But I need to think through what happens with pointer and reference variants of Standard_Address — those would need their own handling rules.
> 
> Actually, there's a simpler way to think about this: Standard_Address is fundamentally a void pointer at the semantic level, so maybe I should reconsider how it's being treated from the start rather than patching it in multiple places. is_void_ptr()` to also check `is_raw_ptr()` for the `Standard_Address` case, and track pointer depth...
> 
> No, let me just go with the practical approach:
> 
> 1. Add special handling for `Standard_Address` in type_mapping, type_to_rust_string, return_type_to_rust_string, and build_param_binding (to avoid wrapping in ConstRef)
> 2. For primitive pointers like `int*`, `double*` — they're `MutPtr(I32)`, `ConstPtr(F64)` — these already map correctly in type_mapping. And they won't be wrapped in ConstRef because the by-value class wrapping only triggers for `Type::Class` and `Type::Handle`. So they'll pass through correctly.
> 3. For class pointer returns on static/free functions — `ConstPtr(Class("gp_Pnt"))` — the `return_type_to_rust_string` has class pointer match arms that produce `Option<&T>`. But for static/free functions, this is unsafe without a lifetime.
> 
> Actually, I'm realizing the issue: in `return_type_to_rust_string`, class pointers map to `Option<&T>`, and there's a check in `build_return_type_binding` that flags class pointer returns with `is_class_ptr_return: true`. For instance methods, this triggers null-check wrapping in `build_reexport_body`, but for static/free functions, those class pointer returns would've been caught by `is_unbindable()` previously.
> 
> Now they'll pass through, and `build_return_type_binding` will create `Option<&T>` without a clear lifetime — it defaults to the anonymous lifetime. For static methods with no `self`, the anonymous lifetime would need to come from a reference parameter or be `'static`. In Rust, `fn foo() -> Option<&SomeType>` requires a lifetime annotation, and `fn foo() -> &SomeType` requires `'static`.
> 
> I'm realizing the core issue: for static/free functions returning class pointers, I need to either mark them as unbindable or ensure the return type gets a proper lifetime. Let me think about what makes sense here...
> 
> Actually, I should keep class pointer returns on static methods as raw pointers `*const T` / `*mut T` to avoid lifetime issues. I need to trace through the existing flow more carefully — there's already a `needs_static_lifetime` flag in `StaticMethodBinding`, and there are additional raw pointer checks in `is_method_bindable()` that I should examine.
> 
> For instance methods, class pointer returns where the class exists in the known set would pass the validation checks and correctly generate `Option<&T>` returns. Static methods have their own separate validation path that I need to look at.
> 
> Looking at the static method generation code, there's a `needs_static_lifetime` flag that handles adding `'static` lifetime to reference returns, which should cover this case. But I'm wondering about the lifetime ownership when `is_class_ptr_return` goes through the null check in `build_reexport_body` — I need to verify that produces the right code for static methods.
> 
> Examining the generated code...
> 
> Actually, I think the cleaner approach is to handle class pointer returns differently for static and free functions versus instance methods. For static methods, they should...
> 
> Wait, I'm realizing that class pointer returns on static methods were never actually possible before — they got filtered out by the unbindable types check that caught raw pointers. So the `build_return_type_binding` logic with the class pointer handling was only ever triggered for instance methods, which means I don't need to worry about differentiating the behavior by context. I'm realizing the core issue: static methods with class pointer returns need raw pointers instead of `Option<&T>` since there's no lifetime context. The cleanest solution is to pass a flag through `build_return_type_binding` that distinguishes between instance methods (which can use `Option<&T>`) and static/free functions (which should use raw pointers).
> 
> But actually, re-reading the user's request clarifies everything—they want raw pointer types in the signature for all unhandled pointers, marked as `unsafe fn`. So I shouldn't be converting class pointer returns to `Option<&T>` at all; instead, `const Foo*` becomes `*const Foo`, `Foo*` becomes `*mut Foo`, and so on, with the function marked unsafe.
> 
> The strategy is to remove void_ptr and raw_ptr from the unbindable check, keep the existing safe handling for nullable and class pointer params/returns as-is, and let the new cases that now pass through use raw pointers with unsafe—void* becomes `*mut std::ffi::c_void`, primitive pointers become `*const` or `*mut` depending on const-ness, and class pointers also use raw pointer syntax instead of references.
> 
> For static methods returning class pointers, I need to bypass the `class_ptr_inner_name` path that would normally wrap them in `Option<&T>`, which I can do by passing a context flag to distinguish static from instance methods. I need to add a boolean parameter to `build_return_type_binding` that tracks whether we're in an instance method context, so class pointer returns convert to `Option<&T>` only for instance methods while static methods and free functions keep raw pointer types. I'll pass this flag through from the four call sites: wrapper methods, direct methods, static methods, and free functions.
> 
> Now I'm thinking through how `Standard_Address` (which is `void*`) gets handled when wrapped in const references or pointers, and whether the parser correctly distinguishes these cases or treats them all the same way.
> 
> I need to make sure `Standard_Address` doesn't get double-wrapped when it's already a pointer type — so I'll exclude it from the by-value-class conversion logic, similar to how `char` is handled. Then I should verify that `type_to_cpp` already handles it correctly.
> 
> For return types, `Standard_Address` shouldn't be wrapped in `OwnedPtr` since it's already a primitive pointer, not a managed class. I need to add exclusions in both `return_type_to_rust_string` and `build_param_binding` to prevent this incorrect wrapping.
> 
> Alternatively, I could restructure the parser to represent `Standard_Address` as a `MutPtr` variant wrapping `c_void`, which would naturally handle all these cases—it wouldn't be treated as a class, so no `ConstRef` wrapping for parameters, and no `OwnedPtr` wrapping for returns.
> 
> The downside is that this breaks the current `is_void_ptr()` check and any code pattern-matching on `Class("Standard_Address")`, but there are only a handful of those spots scattered throughout the codebase.
> 
> Rather than rearchitect the whole type system, I'll just add the special cases where needed—it's clearer and more localized. Let me map out the specific changes required across the files.
> 
> Adding methods to handle unsafe types... Adding unsafe type checking methods... I need to add the `is_unsafe` field to several binding structures, then handle `Standard_Address` type mappings in the type conversion functions to map it to a C void pointer, and finally exclude it from the standard by-value class conversion logic. Still writing unsafe function logic...
> 
> Now I need to think about what to preserve — the existing nullable pointer parameters that map to `Option<&T>` should stay safe, class pointer parameters and returns on instance methods should remain safe, and anything without raw pointers stays as a regular `pub fn`. The new behavior kicks in for void pointers in parameters and certain return types that require unsafe handling.
> 
> For the C++ wrapper generation side, void pointers and raw pointer types don't need special handling since they're already passed through directly from the C++ method signatures. ++ wrapper generation
> 
> For primitive pointers (int*, double*):
> - C++ wrapper param: `Standard_Integer* param` → passed through (already a pointer)
> - C++ wrapper return: same
> - These are already ConstPtr/MutPtr types, so `type_to_cpp` already handles them correctly
> 
> For class pointer returns on static methods:
> - C++ wrapper already returns the pointer (type_to_cpp handles ConstPtr/MutPtr)
> - No changes needed
> 
> So the C++ wrapper generation should work without changes for raw pointers. Let me verify that `type_to_cpp_extern_c_param` handles these correctly... For `Standard_Address` as a param, `type_to_cpp(Class("Standard_Address"))` → `"Standard_Address"` which is correct in C++. For `MutPtr(I32)`, `type_to_cpp` produces `"Standard_Integer*"` which is correct.
> 
> I think I have a complete plan now. Let me check the `build_reexport_body` function to see how raw pointer returns should be handled in the re-export layer. For raw pointer returns that aren't OwnedPtr, enum, or class_ptr_return, the body should just be the raw FFI call. Looking at the logic, when `needs_unique_ptr` is false, `is_enum_return` is None, and `is_class_ptr_ret` is false, the re-export body would just be the unsafe FFI call.
> 
> Examining the `build_reexport_body` function...
> 
> Actually, I've already reviewed much of this. For a plain pointer return without OwnedPtr, enum, or class_ptr handling, it's just the raw FFI call. That's correct — `unsafe { crate::ffi::method(...) }` returns the raw pointer directly. Now I'm wondering about the unsafe block itself: even though the wrapper function is `pub unsafe fn`, does the body still need `unsafe { ... }`? In Rust, calling FFI functions requires an unsafe block regardless of whether the containing function is unsafe.
> 
> Looking at the actual pattern, safe methods wrap the FFI call in `unsafe { OwnedPtr::from_raw(...) }`, while unsafe methods do `unsafe { ffi::method(...) }`. The outer `unsafe fn` signature tells callers they need to handle safety requirements, and the inner unsafe block is still necessary for the FFI call itself.
> 
> Now I need to think through how `convert_arg` handles raw pointer parameters.

Searched for text `fn convert_arg(` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4575](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4575)

<details>
<summary>Reading lines 4575-4650 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4575-L4651">bindings.rs</a></summary>

```
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

```
</details>

> 💭 Good. For raw pointer params (void*, int*, etc.), the `convert_arg` function falls through to the `else` branch: just `p.rust_name.clone()`. This is correct — the raw pointer is passed directly to the FFI function.
> 
> Now let me also check `build_reexport_body` to see how returns are handled:

Searched for text `fn build_reexport_body` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4695](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4695)

<details>
<summary>Reading lines 4695-4773 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4695-L4774">bindings.rs</a></summary>

```
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
        if rt == "String" {
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
```
</details>

> 💭 Now I have a thorough understanding. For raw pointer returns that aren't OwnedPtr/enum/class_ptr/String/&ref, the `build_reexport_body` function uses `format!("unsafe {{ {} }}", raw_call)` which just wraps the FFI call in unsafe. That's correct for raw pointer returns.
> 
> Now, one concern: for raw pointer returns, in the `build_reexport_body` function, if the reexport type starts with `*const` or `*mut`, it won't match any of the special cases (String, &mut, &) and will fall through to `format!("unsafe {{ {} }}", raw_call)`. 
> 
> This is correct. For the `is_class_ptr_return` path in `build_reexport_body`, this handles `Option<&T>` conversion with null checking, and I need to ensure that static/free function class pointer returns don't take this path. Now I'm looking at how `build_return_type_binding` should handle the distinction between class pointer returns that need Option wrapping versus those that don't.
> 
> The cleanest approach is to skip intercepting class pointer returns in `build_return_type_binding` altogether, letting the return type mapping produce raw pointer types instead, then handling the Option wrapping separately downstream.
> 
> Actually, that would require extracting the class pointer logic out of `build_return_type_binding` into the method binding code—too much refactoring. I'll just add a `safe_class_ptr_return` parameter that gates the class pointer check, falling through to normal type mapping when it's false.
> 
> Now I need to find all the call sites for `build_return_type_binding` to see where this parameter should be threaded through.

<details>
<summary>Searched for text <code>build_return_type_binding(</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:1282](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282)
- [bindings.rs:1739](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1739)
- [bindings.rs:1813](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1813)
- [bindings.rs:1901](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1901)
- [bindings.rs:3440](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3440)
</details>

Let me see the context of each call site to know which is for instance methods, which for static, etc.:

<details>
<summary>Reading lines 1720-1735 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1720-L1736">bindings.rs</a></summary>

```
        .collect();
    let wrapper_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
```
</details>

<details>
<summary>Reading lines 1800-1820 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1800-L1821">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 1890-1910 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1890-L1911">bindings.rs</a></summary>

```
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
                .map(|ty| ty.is_reference())
                .unwrap_or(false);

            StaticMethodBinding {
                ffi_fn_name,
```
</details>

> 💭 OK, so the call sites are:
> 1. Line 1739 — direct methods (instance methods that DON'T need a wrapper) — safe class_ptr_return
> 2. Line 1813 — wrapper methods (instance methods that DO need a wrapper) — safe class_ptr_return
> 3. Line 1901 — static methods — unsafe class_ptr_return
> 4. Line 3440 — free functions — unsafe class_ptr_return
> 
> I add a parameter `safe_class_ptr_return: bool` to `build_return_type_binding()`: true for calls 1 and 2, false for calls 3 and 4.
> 
> 
> Now I need to track where each binding struct gets constructed so I can propagate the `is_unsafe` flag. I'm looking at DirectMethodBinding around line 1730, WrapperMethodBinding at 1810, StaticMethodBinding at 1890, and FunctionBinding at 3454, plus I need to find where ConstructorBinding and InheritedMethodBinding are created.

Searched for text `ConstructorBinding {` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:104](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L104)
- [bindings.rs:1622](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1622)
- [bindings.rs:2458](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2458)

<details>
<summary>Reading lines 1612-1650 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1612-L1651">bindings.rs</a></summary>

```
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
```
</details>

Searched for text `InheritedMethodBinding {` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:270](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L270)
- [bindings.rs:2962](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2962)

Let me see the full ConstructorBinding creation with params:

<details>
<summary>Reading lines 1540-1615 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1540-L1616">bindings.rs</a></summary>

```
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
```
</details>

Searched for text `fn compute_constructor_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:2253](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2253)

<details>
<summary>Reading lines 2253-2340 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2253-L2341">bindings.rs</a></summary>

```
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
```
</details>

Now let me look at where ConstructorBindings are actually created with their field values:

<details>
<summary>Reading lines 2410-2500 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2410-L2501">bindings.rs</a></summary>

```
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

            ConstructorBinding {
                ffi_fn_name,
                impl_method_name,
                params,
                cpp_arg_exprs,
                doc_comment: trimmed.original.comment.clone(),
                source_line: trimmed.original.source_line,
                convenience_of,
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
```
</details>

Now let me also look at where Now let me also look at where DirectMethodBinding, WrapperMethodBinding, and InheritedMethodBinding are created, and the full creation of StaticMethodBinding:

<details>
<summary>Reading lines 1726-1770 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1726-L1771">bindings.rs</a></summary>

```
    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
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
            }
        })
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = HashSet::new();
    for fn_name in &wrapper_fn_names {
        reserved_names.insert(format!("{}_{}", cpp_name, fn_name));
    }

```
</details>

<details>
<summary>Reading lines 1770-1870 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1770-L1871">bindings.rs</a></summary>

```

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
                stub_rust_decl: generate_static_method_stu
... (truncated, 4455 chars)
```
</details>

<details>
<summary>Reading lines 1870-1930 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1870-L1931">bindings.rs</a></summary>

```
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

    // ── to_owned ──────────────────────────────────────────────────────────────────────────────
    // Detect copyability using libclang copy/move constructor detection.
    // has_copy_constructor: Some(true) = explicit usable copy ctor,
    //                       Some(false) = explicitly deleted/private,
    //                       None = no explicit copy ctor (implicit may exist)
    //
```
</details>

Now let me check InheritedMethodBinding creation around line 2962:

<details>
<summary>Reading lines 2920-3010 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2920-L3011">bindings.rs</a></summary>

```
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
                            rust_ffi_type: if rt.enum_cpp_name.is_some() { "i32".to_string() } else { return_type_to_ffi_full_name(&rt.original) },
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
         
... (truncated, 4146 chars)
```
</details>

**Todo list:**
- ✅ Research current raw pointer handling
- ✅ Read key source files in detail
- ✅ Plan changes to type_mapping/resolver/codegen
- 🔄 Implement changes in model.rs
- ⬜ Implement changes in type_mapping.rs
- ⬜ Implement changes in bindings.rs
- ⬜ Implement changes in rust.rs
- ⬜ Build binding generator
- ⬜ Regenerate bindings and build
- ⬜ Run tests

Now I have enough context. Let me implement all the changes systematically. Starting with `model.rs`:Now let me implement all the changes. First, `model.rs`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L635-L644">model.rs -8+9</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L644-L652">+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "635:uo,636:dc,637:hd,638:wx,639:is,640:km,641:gd,642:ax",
      "content": "    /// Check if this type is unbindable through the FFI.\n    /// Note: const char* (C strings) ARE bindable - we generate wrappers that pass const char* directly.\n    /// Note: Raw pointers (void*, int*, T*) are NOT unbindable — they are bound as unsafe raw pointer types.\n    /// Nested types (Parent::Nested) are supported via name flattening\n    /// (Parent::Nested → Parent_Nested in Rust FFI), BUT unresolved template types\n    /// and unqualified names without underscore remain unbindable.\n    pub fn is_unbindable(&self) -> bool {\n        self.is_array() || self.is_rvalue_ref() || self.is_unresolved_template_type()\n    }"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "642:ax",
      "insertAfter": true,
      "content": "\n    /// Check if this type involves raw pointers that require the containing\n    /// function to be marked `unsafe`. True for void pointers (Standard_Address)\n    /// and raw T*/const T* pointers (excluding const char* which is handled\n    /// as C strings).\n    pub fn needs_unsafe_fn(&self) -> bool {\n        self.is_void_ptr() || self.is_raw_ptr()\n    }"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 635,
      "endLine": 651
    }
  ]
}
```
</details>

Now add `has_unsafe_types()` to all four struct impl blocks. First ParsedFunction:

<details>
<summary>Reading lines 47-62 of <a href="../crates/opencascade-binding-generator/src/model.rs#L47-L63">model.rs</a></summary>

```
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
}

/// A parsed C++ enum
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L60-L74">model.rs +14</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    /// Check if this function has any unsafe raw pointer types that require\n    /// the function to be marked `unsafe fn`.\n    pub fn has_unsafe_types(&self) -> bool {\n        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {\n            return true;\n        }\n        if let Some(ref ret) = self.return_type {\n            if ret.needs_unsafe_fn() {\n                return true;\n            }\n        }\n        false\n    }",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "insertAfter": true,
      "lineHashes": "59:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":59,"endLine":73}
```
</details>

Now add to Constructor:

<details>
<summary>Reading lines 228-235 of <a href="../crates/opencascade-binding-generator/src/model.rs#L228-L236">model.rs</a></summary>

```
    /// Check if this constructor has any unbindable types (C strings, streams, void pointers, etc.)
    /// Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
    }
}

/// An instance method declaration
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L233-L238">model.rs +5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    /// Check if this constructor has any unsafe raw pointer types.\n    pub fn has_unsafe_types(&self) -> bool {\n        self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())\n    }",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "insertAfter": true,
      "lineHashes": "232:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":232,"endLine":237}
```
</details>

Now add to Method:

<details>
<summary>Reading lines 258-280 of <a href="../crates/opencascade-binding-generator/src/model.rs#L258-L281">model.rs</a></summary>

```
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

    /// Generate a suffix for distinguishing overloaded methods
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L279-L294">model.rs +15</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    /// Check if this method has any unsafe raw pointer types that require\n    /// the function to be marked `unsafe fn`.\n    pub fn has_unsafe_types(&self) -> bool {\n        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {\n            return true;\n        }\n        if let Some(ref ret) = self.return_type {\n            // Instance method class pointer returns are handled safely as Option<&T>\n            if ret.needs_unsafe_fn() && ret.class_ptr_inner_name().is_none() {\n                return true;\n            }\n        }\n        false\n    }",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "insertAfter": true,
      "lineHashes": "278:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":278,"endLine":293}
```
</details>

Now add to StaticMethod:

<details>
<summary>Reading lines 335-355 of <a href="../crates/opencascade-binding-generator/src/model.rs#L335-L356">model.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 355-375 of <a href="../crates/opencascade-binding-generator/src/model.rs#L355-L376">model.rs</a></summary>

```
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L362-L375">model.rs +13</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "\n    /// Check if this static method has any unsafe raw pointer types.\n    pub fn has_unsafe_types(&self) -> bool {\n        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {\n            return true;\n        }\n        if let Some(ref ret) = self.return_type {\n            if ret.needs_unsafe_fn() {\n                return true;\n            }\n        }\n        false\n    }",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "insertAfter": true,
      "lineHashes": "361:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":361,"endLine":374}
```
</details>

**Todo list:**
- ✅ Research current raw pointer handling
- ✅ Read key source files in detail
- ✅ Plan changes to type_mapping/resolver/codegen
- ✅ Implement changes in model.rs
- 🔄 Implement changes in type_mapping.rs
- ⬜ Implement changes in bindings.rs
- ⬜ Implement changes in rust.rs
- ⬜ Build binding generator
- ⬜ Regenerate bindings and build
- ⬜ Run tests

Now implement type_mapping.rs — add `Standard_Address` → `*mut std::ffi::c_void`:

<details>
<summary>Reading lines 155-178 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L155-L179">type_mapping.rs</a></summary>

```
                source_module,
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L158-L168">type_mapping.rs -1+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" => {\n            // Standard_Address is a typedef for void* — map to raw c_void pointer\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"char\" => {",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "158:hh"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":158,"endLine":167}
```
</details>

**Todo list:**
- ✅ Research current raw pointer handling
- ✅ Read key source files in detail
- ✅ Plan changes to type_mapping/resolver/codegen
- ✅ Implement changes in model.rs
- ✅ Implement changes in type_mapping.rs
- 🔄 Implement changes in bindings.rs
- ⬜ Implement changes in rust.rs
- ⬜ Build binding generator
- ⬜ Regenerate bindings and build
- ⬜ Run tests

Now the big changes in `crates/opencascade-binding-generator/src/codegen/bindings.rs`. Let me make the changes systematically:

1. Add `is_unsafe` to binding structs
2. Add `Standard_Address` handling in type conversion functions
3. Add `safe_class_ptr_return` parameter to 
4. Compute `is_unsafe` when creating bindings
5. Use `is_unsafe` in code generation

<details>
<summary>Reading lines 100-155 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L100-L156">bindings.rs</a></summary>

```

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
}

/// What kind of C++ wrapper is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrapperKind {
```
</details>

Add `is_unsafe` to all binding structs:

*2026-02-20 07:47 · 1883s · Words: 27,701 in · 5,313 out · 61 rounds*

---

<a id="user-2"></a>

## User (2)

> @agent Try Again

*2026-02-20 07:48*

### Assistant — Claude Opus 4.6

Now add `is_unsafe` to the binding structs:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L121-L125">bindings.rs -2+4</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L152-L156">-2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub convenience_of: Option<ConvenienceInfo>,\n    /// Whether this constructor should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "121:ns,122:io"
    },
    {
      "content": "    pub source_line: Option<u32>,\n    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "150:nv,151:io"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 121,
      "endLine": 155
    }
  ]
}
```
</details>

Now add to WrapperMethodBinding:

<details>
<summary>Reading lines 177-200 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L177-L201">bindings.rs</a></summary>

```

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
}

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L198-L202">bindings.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub source_line: Option<u32>,\n    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "198:nv,199:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":198,"endLine":201}
```
</details>

Now StaticMethodBinding:

<details>
<summary>Reading lines 201-225 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L201-L226">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L221-L225">bindings.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub source_line: Option<u32>,\n    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "221:nv,222:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":221,"endLine":224}
```
</details>

Now InheritedMethodBinding:

<details>
<summary>Reading lines 274-300 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L274-L301">bindings.rs</a></summary>

```


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
}

/// A parameter binding with info for all three output targets.
#[derive(Debug, Clone)]
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L296-L300">bindings.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub source_line: Option<u32>,\n    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "296:nv,297:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":296,"endLine":299}
```
</details>

Now add `is_unsafe` to FunctionBinding:

<details>
<summary>Reading lines 388-415 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L388-L416">bindings.rs</a></summary>

```
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
}

// ── Helper functions ────────────────────────────────────────────────────────
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L412-L416">bindings.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub cpp_headers: Vec<String>,\n    /// Whether this function should be marked `unsafe fn` (has raw pointer params/returns)\n    pub is_unsafe: bool,\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "412:dl,413:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":412,"endLine":415}
```
</details>

Now I need to handle `Standard_Address` in `type_to_cpp` and `type_to_rust_string` and `return_type_to_rust_string`. Let me also update `type_to_cpp` to handle `Standard_Address`:

<details>
<summary>Reading lines 590-620 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L590-L621">bindings.rs</a></summary>

```
        // Simple method wrapper (primitives, void, etc.)
        WrapperKind::Simple
    }
}

/// Convert a Type to C++ type string
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
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


```
</details>

The `type_to_cpp` is fine as-is — `Standard_Address` will just remain `Standard_Address` in C++ wrappers, which is correct (it's `void*` in C++).

Now update `type_to_rust_string` and `return_type_to_rust_string` to handle `Standard_Address`:

<details>
<summary>Reading lines 681-720 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L681-L721">bindings.rs</a></summary>

```
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
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
```
</details>

I need to add a `Standard_Address` → `*mut std::ffi::c_void` mapping in `type_to_rust_string`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L694-L705">bindings.rs -9+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) => {\n            if name == \"char\" {\n                \"std::ffi::c_char\".to_string()\n            } else if name == \"Standard_Address\" {\n                \"*mut std::ffi::c_void\".to_string()\n            } else if let Some(ctx) = reexport_ctx {\n                ctx.resolve_class(name)\n            } else {\n                format!(\"crate::ffi::{}\", Type::ffi_safe_class_name(name))\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "694:tm,695:kp,696:xq,697:sn,698:bx,699:ud,700:yn,701:uo,702:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":694,"endLine":704}
```
</details>

Now update `return_type_to_rust_string`:

<details>
<summary>Reading lines 720-770 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L720-L771">bindings.rs</a></summary>

```
}

/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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

```
</details>

I need to update the `return_type_to_rust_string` to exclude `Standard_Address` from the `OwnedPtr` wrapping:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L725-L726">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) if name != \"char\" && name != \"Standard_Address\" => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "725:zh"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":725,"endLine":725}
```
</details>

Now I need to update the `build_return_type_binding` to not treat `Standard_Address` as needing OwnedPtr:

<details>
<summary>Reading lines 1340-1360 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1340-L1361">bindings.rs</a></summary>

```
        };
    }

    let mapped = map_return_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = return_type_to_rust_string(ty, reexport_ctx);
    let cpp_type = type_to_cpp(ty);
    let needs_unique_ptr = ty.is_class() || ty.is_handle();

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
```
</details>

The `needs_unique_ptr` check needs to exclude `Standard_Address`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1347-L1348">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let needs_unique_ptr = (ty.is_class() && !ty.is_void_ptr()) || ty.is_handle();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1347:sk"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1347,"endLine":1347}
```
</details>

Now let me update where bindings are constructed to populate the `is_unsafe` field. Let me do the `ConstructorBinding` first:

<details>
<summary>Reading lines 2465-2488 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2465-L2489">bindings.rs</a></summary>

```
                let cpp_arg_exprs: Vec<String> = params
                    .iter()
                    .map(|p| p.cpp_arg_expr.clone())
                    .collect();
                (ffi_fn_name, cpp_arg_exprs)
            };

            ConstructorBinding {
                ffi_fn_name,
                impl_method_name,
                params,
                cpp_arg_exprs,
                doc_comment: trimmed.original.comment.clone(),
                source_line: trimmed.original.source_line,
                convenience_of,
            }
        })
        .collect()
}

// ── Upcast bindings ─────────────────────────────────────────────────────────

fn compute_upcast_bindings(
    class: &ParsedClass,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2472-L2484">bindings.rs -9+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            let is_unsafe = trimmed.original.has_unsafe_types();\n\n            ConstructorBinding {\n                ffi_fn_name,\n                impl_method_name,\n                params,\n                cpp_arg_exprs,\n                doc_comment: trimmed.original.comment.clone(),\n                source_line: trimmed.original.source_line,\n                convenience_of,\n                is_unsafe,\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2472:bm,2473:bu,2474:xc,2475:vm,2476:ws,2477:pe,2478:el,2479:ct,2480:uo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2472,"endLine":2483}
```
</details>

Also update the synthetic default constructor:

<details>
<summary>Reading lines 1634-1650 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1634-L1651">bindings.rs</a></summary>

```
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
            });
        }
        ctors
    } else {
        // Record skipped constructors for abstract/protected-destructor classes
        if effectively_abstract {
            for ctor in &class.constructors {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1636-L1646">bindings.rs -9+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            ctors.push(ConstructorBinding {\n                ffi_fn_name: format!(\"{}_ctor\", cpp_name),\n                impl_method_name: \"new\".to_string(),\n                params: Vec::new(),\n                cpp_arg_exprs: Vec::new(),\n                doc_comment: Some(\"Default constructor\".to_string()),\n                source_line: None,\n                convenience_of: None,\n                is_unsafe: false,\n            });",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1636:ov,1637:ut,1638:vw,1639:ov,1640:hc,1641:dk,1642:hy,1643:ua,1644:ys"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1636,"endLine":1645}
```
</details>

Now update `DirectMethodBinding`:

<details>
<summary>Reading lines 1762-1780 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1762-L1781">bindings.rs</a></summary>

```
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
            }
        })
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1766-L1776">bindings.rs -9+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            DirectMethodBinding {\n                rust_name: rust_name.clone(),\n                cxx_name: method.name.clone(),\n                is_const: method.is_const,\n                params,\n                return_type,\n                doc_comment: method.comment.clone(),\n                source_line: method.source_line,\n                is_unsafe: method.has_unsafe_types(),\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1766:hi,1767:zs,1768:ml,1769:eq,1770:vm,1771:gg,1772:js,1773:uq,1774:uo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1766,"endLine":1775}
```
</details>

Now update `WrapperMethodBinding`:

<details>
<summary>Reading lines 1846-1868 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1846-L1869">bindings.rs</a></summary>

```
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
            }
        })
        .collect();

    // ── Static methods ──────────────────────────────────────────────────────────
    let mut static_methods_raw: Vec<&StaticMethod> = Vec::new();
    for method in &class.static_methods {
        if exclude_methods.contains(&(class.name.clone(), method.name.clone())) {
            skipped_symbols.push(SkippedSymbol {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1850-L1862">bindings.rs -11+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            WrapperMethodBinding {\n                ffi_fn_name,\n                impl_method_name,\n                is_const: effective_is_const,\n                params,\n                return_type,\n                wrapper_kind,\n                cpp_method_name: method.name.clone(),\n                doc_comment: method.comment.clone(),\n                source_line: method.source_line,\n                is_unsafe: method.has_unsafe_types(),\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1850:du,1851:bu,1852:xc,1853:nh,1854:vm,1855:gg,1856:bl,1857:gw,1858:js,1859:uq,1860:uo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1850,"endLine":1861}
```
</details>

Now update `StaticMethodBinding`:

<details>
<summary>Reading lines 1923-1944 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1923-L1945">bindings.rs</a></summary>

```
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

    // ── to_owned ──────────────────────────────────────────────────────────────────────────────
    // Detect copyability using libclang copy/move constructor detection.
    // has_copy_constructor: Some(true) = explicit usable copy ctor,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1926-L1937">bindings.rs -10+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            StaticMethodBinding {\n                ffi_fn_name,\n                impl_method_name: impl_method_name.clone(),\n                params,\n                return_type,\n                cpp_method_name: method.name.clone(),\n                needs_static_lifetime,\n                doc_comment: method.comment.clone(),\n                source_line: method.source_line,\n                is_unsafe: method.has_unsafe_types(),\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1926:yd,1927:bu,1928:lb,1929:vm,1930:gg,1931:gw,1932:ba,1933:js,1934:uq,1935:uo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1926,"endLine":1936}
```
</details>

Now update `InheritedMethodBinding`. This is more complex — inherited methods get their types from the resolved ancestor. Let me check:

<details>
<summary>Reading lines 2975-2995 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2975-L2996">bindings.rs</a></summary>

```
                if resolved_method.is_const {
                    if let Some(ref mut rt) = return_type {
                        if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                            rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                        }
                    }
                }

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
                });
            }
        }
```
</details>

For inherited methods, I need to check the original method for unsafe types. Let me find how `resolved_method` is accessed:

<details>
<summary>Reading lines 2830-2870 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2830-L2871">bindings.rs</a></summary>

```
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
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*const {}", inner_ffi),
                                        format!("&{}", inner_rust),
                                        format!("const {}*", inner_cpp),
                                    )
                                }
                                Type::MutPtr(inner) => {
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*mut {}", inner_ffi),
                                        format!("&mut {}", inner_rust),
                                        format!("{}*", inner_cpp),
                                    )
                                }
                                _ => unreachable!("class_ptr_inner_name() returned Some for non-pointer type"),
                            };
```
</details>

I need to look at how `resolved_method` relates to the original `Method`:

<details>
<summary>Reading lines 2790-2820 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2790-L2821">bindings.rs</a></summary>

```
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
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*const {}", inner_ffi),
                                        format!("Option<&{}>", inner_rust),
                                        format!("const {}*", inner_cpp),
                                    )
                                }
                                Type::MutPtr(inner) => {
                                    let inner_ffi = type_to_ffi_full_name(inner);
```
</details>

The `resolved_method` is a `ResolvedMethod` from the symbol table, which has a `params` field with `ResolvedParam` types. I need to check if any of those params or the return type have unsafe types. Let me check `ResolvedParam`:

Searched for text `pub struct ResolvedParam` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:280](../crates/opencascade-binding-generator/src/resolver.rs#L280)

<details>
<summary>Reading lines 280-305 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L280-L306">resolver.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 305-320 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L305-L321">resolver.rs</a></summary>

```
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
```
</details>

Good, `ResolvedParam` has  which is a . For the inherited methods, I can check `needs_unsafe_fn()` on the original type. Let me update the `InheritedMethodBinding` creation:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2983-L3004">bindings.rs -11+21</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                // Check if inherited method has unsafe types (raw pointers / void pointers)\n                let is_unsafe = resolved_method.params.iter().any(|p| {\n                    p.ty.original.needs_unsafe_fn()\n                        && !p.is_nullable_ptr()\n                        && p.ty.original.class_ptr_inner_name().is_none()\n                }) || resolved_method.return_type.as_ref().map_or(false, |rt| {\n                    rt.original.needs_unsafe_fn() && rt.original.class_ptr_inner_name().is_none()\n                });\n\n                result.push(InheritedMethodBinding {\n                    ffi_fn_name,\n                    impl_method_name,\n                    is_const: resolved_method.is_const,\n                    params,\n                    return_type,\n                    cpp_method_name: resolved_method.cpp_name.clone(),\n                    source_class: ancestor_name.clone(),\n                    source_header: ancestor_class.source_header.clone(),\n                    source_line: resolved_method.source_line,\n                    is_unsafe,\n                });",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2983:lh,2984:vd,2985:pa,2986:pw,2987:lz,2988:km,2989:fs,2990:qk,2991:tf,2992:tm,2993:qx"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2983,"endLine":3003}
```
</details>

Now update `FunctionBinding` in `compute_all_function_bindings`:

<details>
<summary>Reading lines 3465-3488 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3465-L3489">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 3488-3500 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3488-L3501">bindings.rs</a></summary>

```
            namespace: func.namespace.clone(),
            short_name: func.short_name.clone(),
            module: func.rust_module.clone(),
            params,
            return_type,
            source_header: func.source_header.clone(),
            source_line: func.source_line,
            doc_comment: func.doc_comment.clone(),
            cpp_headers,
        });
    }

    (result, skipped)
```
</details>

**Todo list:**
- 🔄 Populate FunctionBinding.is_unsafe
- ⬜ Emit unsafe fn in emit_reexport_class
- ⬜ Emit unsafe fn in emit_free_function_wrapper
- ⬜ Build and test

I need to check what type `func` is in this context to determine how to compute `is_unsafe`.

<details>
<summary>Reading lines 3420-3445 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3420-L3446">bindings.rs</a></summary>

```
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
```
</details>

Searched for text `for func in &functions` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `compute_all_function_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:388](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L388)
- [bindings.rs:3208](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3208)

<details>
<summary>Reading lines 3208-3240 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3208-L3241">bindings.rs</a></summary>

```
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
    manual_type_names: &HashSet<String>,
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
```
</details>

The `func` is from `all_functions` which are `ResolvedFunction`s. I need to check what `has_unsafe_types` is available on the resolved function. Let me check the type:

Searched for regex `struct ResolvedFunction` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:210](../crates/opencascade-binding-generator/src/resolver.rs#L210)

<details>
<summary>Reading lines 210-240 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L210-L241">resolver.rs</a></summary>

```
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

```
</details>

`ResolvedFunction` doesn't have `has_unsafe_types()`. I need to compute `is_unsafe` from the params and return type, similar to what I did for `InheritedMethodBinding`. The params have , and return_type has `.original`.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3496-L3499">bindings.rs -1+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3496:fu",
      "content": "            cpp_headers,\n            is_unsafe: func.params.iter().any(|p| p.ty.original.needs_unsafe_fn())\n                || func.return_type.as_ref().map_or(false, |rt| rt.original.needs_unsafe_fn()),"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":3496,"endLine":3498}
```
</details>

**Todo list:**
- ✅ Populate FunctionBinding.is_unsafe
- 🔄 Emit unsafe fn in emit_reexport_class
- ⬜ Emit unsafe fn in emit_free_function_wrapper
- ⬜ Build and test

Now I need to modify `emit_reexport_class` to emit `pub unsafe fn` when `is_unsafe` is true. Let me find all the `pub fn` format strings in that function:

Searched for regex `"pub fn ` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:835](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L835)
- [bindings.rs:845](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L845)
- [bindings.rs:856](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L856)
- [bindings.rs:867](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L867)

Those look like FFI signature builders. But I need the re-export code generation. Let me search for the actual re-export emit code with different patterns:

Searched for regex `emit_reexport_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4758](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4758)

<details>
<summary>Reading lines 4780-4850 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4780-L4851">bindings.rs</a></summary>

```
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
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                prelude,
                ctor.ffi_fn_name,
                args.join(", ")
            ));
```
</details>

<details>
<summary>Reading lines 4850-4950 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4850-L4951">bindings.rs</a></summary>

```
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
        let doc = format_reexport_doc(&source_attr, &wm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
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
```
</details>

<details>
<summary>Reading lines 4950-5030 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4950-L5031">bindings.rs</a></summary>

```
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
        let doc = format_reexport_doc(&source_attr, &dm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
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
                if sm.needs_static_lifetime
                    && ty_str.starts_with('&')
                    && !ty_str.contains("'static")
                {
                    ty_str = ty_str.replacen('&', "&'static ", 1);
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

        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
            sm.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
            body,
        ));
    }

```
</details>

<details>
<summary>Reading lines 5030-5120 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5030-L5121">bindings.rs</a></summary>

```

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
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
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
        impl_methods.push(format!(
      
... (truncated, 4090 chars)
```
</details>

<details>
<summary>Reading lines 5120-5140 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5120-L5141">bindings.rs</a></summary>

```
            format_reexport_doc(
                &format!("Inherited: {}", format_source_attribution(
                    &im.source_header,
                    im.source_line,
                    &format!("{}::{}()", im.source_class, im.cpp_method_name),
                )),
                &no_doc,
            ),
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
```
</details>

> 💭 Now I need to modify all the `pub fn` format strings to conditionally use `pub unsafe fn`. Let me identify all the places:
> 
> 1. **Convenience constructors** (line 4832): `"pub fn {}({}) -> crate::OwnedPtr<Self> {{\n"`
> 2. **Regular constructors** (line 4843): `"pub fn {}({}) -> crate::OwnedPtr<Self> {{\n"`
> 3. **Wrapper methods** (line 4905): `"pub fn {}({}){} {{\n"`
> 4. **Direct methods** (line 4967): `"pub fn {}({}){} {{\n"`
> 
> 
> 5. **Static methods** (line 5021): `"pub fn {}({}){} {{\n"`
> 6. **Inherited methods** (line 5119): `"pub fn {}({}){} {{\n"`
> 
> For each one, I need to change `"pub fn"` to conditionally use `pub unsafe fn` based on whether the binding is unsafe. I also need to think about the safety of the method bodies—since unsafe functions already provide an unsafe context, the inner `unsafe {}` blocks around FFI calls can be removed.
> 
> Now I'm going to apply all 6 changes using hashline_edit to update the template strings.

Now I need to change the `pub fn` format strings to conditionally use `pub unsafe fn` in all 6 method emission sites. Let me batch these edits:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4831-L4834">bindings.rs -1+3</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4835-L4838">-1+3</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4831:ui|4832:fn",
      "content": "            let unsafe_kw = if ctor.is_unsafe { \"unsafe \" } else { \"\" };\n            impl_methods.push(format!(\n                \"{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\\n        Self::{}({})\\n    }}\\n\","
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4833:zc|4834:ng",
      "content": "                doc,\n                unsafe_kw,\n                ctor.impl_method_name,"
    }
  ]
}
```
**Output:**
```
{
  "applied": 0,
  "failed": 2,
  "errors": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4831:ui|4832:fn",
      "error": "hash mismatch at line 4831: expected 'ui|4832:fn', got 'ui'"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4833:zc|4834:ng",
      "error": "hash mismatch at line 4833: expected 'zc|4834:ng', got 'zc'"
    }
  ]
}
```
</details>

Let me use single-line hashes:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4831-L4836">bindings.rs -4+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            let unsafe_kw = if ctor.is_unsafe { \"unsafe \" } else { \"\" };\n            impl_methods.push(format!(\n                \"{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\\n        Self::{}({})\\n    }}\\n\",\n                doc,\n                unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4831:ui,4832:fn,4833:zc,4834:ng"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4831,"endLine":4835}
```
</details>

Now re-read for the regular constructor (the one after `else {`):

<details>
<summary>Reading lines 4839-4855 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4839-L4856">bindings.rs</a></summary>

```
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\n    }}\n",
                doc,
                ctor.impl_method_name,
                params.join(", "),
                prelude,
                ctor.ffi_fn_name,
                args.join(", ")
            ));
        }
    }

    // 2. Wrapper methods (impl delegates to ffi free functions)
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4843-L4848">bindings.rs -3+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            let unsafe_kw = if ctor.is_unsafe { \"unsafe \" } else { \"\" };\n            impl_methods.push(format!(\n                \"{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\\n{}        unsafe {{ crate::OwnedPtr::from_raw(crate::ffi::{}({})) }}\\n    }}\\n\",\n                doc,\n                unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4843:ui,4844:bm,4845:zc"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4843,"endLine":4847}
```
</details>

Now wrapper methods. Let me re-read the current state:

<details>
<summary>Reading lines 4903-4920 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4903-L4921">bindings.rs</a></summary>

```
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        let doc = format_reexport_doc(&source_attr, &wm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4907-L4912">bindings.rs -3+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let unsafe_kw = if wm.is_unsafe { \"unsafe \" } else { \"\" };\n        impl_methods.push(format!(\n            \"{}    pub {}fn {}({}){} {{\\n{}        {}\\n    }}\\n\",\n            doc,\n            unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4907:kf,4908:vs,4909:rw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4907,"endLine":4911}
```
</details>

Now direct methods. Re-read:

<details>
<summary>Reading lines 4968-4985 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4968-L4986">bindings.rs</a></summary>

```
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        let doc = format_reexport_doc(&source_attr, &dm.doc_comment);
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4971-L4976">bindings.rs -3+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let unsafe_kw = if dm.is_unsafe { \"unsafe \" } else { \"\" };\n        impl_methods.push(format!(\n            \"{}    pub {}fn {}({}){} {{\\n{}        {}\\n    }}\\n\",\n            doc,\n            unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4971:kf,4972:vs,4973:rw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4971,"endLine":4975}
```
</details>

Now static methods. Re-read:

<details>
<summary>Reading lines 5023-5040 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5023-L5041">bindings.rs</a></summary>

```
        let postamble = mut_ref_enum_postamble_params(&sm.params, "        ");
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            doc,
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5027-L5032">bindings.rs -3+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let unsafe_kw = if sm.is_unsafe { \"unsafe \" } else { \"\" };\n        impl_methods.push(format!(\n            \"{}    pub {}fn {}({}){} {{\\n{}        {}\\n    }}\\n\",\n            doc,\n            unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "5027:kf,5028:vs,5029:rw"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":5027,"endLine":5031}
```
</details>

Now inherited methods. Re-read:

<details>
<summary>Reading lines 5123-5140 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5123-L5141">bindings.rs</a></summary>

```
        let has_return = !return_type.is_empty();
        let body = wrap_body_with_postamble(&body, &postamble, has_return);

        let no_doc: Option<String> = None;
        impl_methods.push(format!(
            "{}    pub fn {}({}){} {{\n{}        {}\n    }}\n",
            format_reexport_doc(
                &format!("Inherited: {}", format_source_attribution(
                    &im.source_header,
                    im.source_line,
                    &format!("{}::{}()", im.source_class, im.cpp_method_name),
                )),
                &no_doc,
            ),
            im.impl_method_name,
            params.join(", "),
            return_type,
            prelude,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5127-L5139">bindings.rs -10+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let unsafe_kw = if im.is_unsafe { \"unsafe \" } else { \"\" };\n        impl_methods.push(format!(\n            \"{}    pub {}fn {}({}){} {{\\n{}        {}\\n    }}\\n\",\n            format_reexport_doc(\n                &format!(\"Inherited: {}\", format_source_attribution(\n                    &im.source_header,\n                    im.source_line,\n                    &format!(\"{}::{}()\", im.source_class, im.cpp_method_name),\n                )),\n                &no_doc,\n            ),\n            unsafe_kw,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "5127:kf,5128:vs,5129:vc,5130:uo,5131:mb,5132:hr,5133:gp,5134:pb,5135:br,5136:wu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":5127,"endLine":5138}
```
</details>

**Todo list:**
- ✅ Populate FunctionBinding.is_unsafe
- ✅ Emit unsafe fn in emit_reexport_class
- 🔄 Emit unsafe fn in emit_free_function_wrapper
- ⬜ Build and test

Now let me modify `emit_free_function_wrapper` in rust.rs:

<details>
<summary>Reading lines 710-760 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L710-L761">rust.rs</a></summary>

```
            })
        })
        .collect();

    let has_return = !return_type_str.is_empty();

    writeln!(output, "pub fn {}({}){} {{", func.rust_ffi_name, params.join(", "), return_type_str).unwrap();
    write!(output, "{}", prelude).unwrap();

    if postamble.is_empty() {
        writeln!(output, "    unsafe {{ {} }}", body).unwrap();
    } else if has_return {
        writeln!(output, "    let result_ = unsafe {{ {} }};", body).unwrap();
        write!(output, "{}", postamble).unwrap();
        writeln!(output, "    result_").unwrap();
    } else {
        writeln!(output, "    unsafe {{ {} }};", body).unwrap();
        // Trim trailing newline from postamble for clean formatting
        write!(output, "{}", postamble.trim_end_matches('\n')).unwrap();
        writeln!(output).unwrap();
    }

    writeln!(output, "}}").unwrap();
}

/// Generate a module re-export file
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    module_fn_bindings: &[&super::bindings::FunctionBinding],
    module_skipped_fns: &[&super::bindings::SkippedSymbol],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L716-L718">rust.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let unsafe_kw = if func.is_unsafe { \"unsafe \" } else { \"\" };\n    writeln!(output, \"pub {}fn {}({}){} {{\", unsafe_kw, func.rust_ffi_name, params.join(\", \"), return_type_str).unwrap();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "716:wh"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/rust.rs","startLine":716,"endLine":717}
```
</details>

**Todo list:**
- ✅ Populate FunctionBinding.is_unsafe
- ✅ Emit unsafe fn in emit_reexport_class
- ✅ Emit unsafe fn in emit_free_function_wrapper
- 🔄 Build and test

Now let me also check: the FFI signature builders (lines 835, 845, 856, 867) — these generate the `extern "C"` FFI signatures that go in the `unsafe extern "C++"` block. Those don't need `unsafe fn` since the whole block is already unsafe. 

Let me also check if there's a `build_reexport_body` that wraps FFI calls in `unsafe {}` — for `unsafe fn` methods the unsafe block is redundant but harmless. Let me build now:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (20 lines) — exit code 101</summary>

```
    Blocking waiting for file lock on build directory
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error: 6 positional arguments in format string, but there are 5 arguments
    --> crates/opencascade-binding-generator/src/codegen/bindings.rs:4833:18
     |
4833 | ...   "{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n...
     |        ^^        ^^   ^^ ^^                                             ^^ ^^
4834 | ...   doc,
     |       ---
4835 | ...   unsafe_kw,
     |       ---------
4836 | ...   params.join(", "),
     |       -----------------
4837 | ...   conv.full_method_name,
     |       ---------------------
4838 | ...   all_args.join(", ")
     |       -------------------

error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
real  17.9s
```
</details>

The convenience constructor format string has 6 `{}` placeholders but only 5 arguments — I need to add the  argument. Let me re-read:

<details>
<summary>Reading lines 4828-4842 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4828-L4843">bindings.rs</a></summary>

```
            let convenience_args: Vec<String> = ctor.params.iter().map(|p| p.rust_name.clone()).collect();
            let mut all_args = convenience_args;
            all_args.extend(conv.default_exprs.iter().cloned());
            let unsafe_kw = if ctor.is_unsafe { "unsafe " } else { "" };
            impl_methods.push(format!(
                "{}    pub {}fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
                unsafe_kw,
                params.join(", "),
                conv.full_method_name,
                all_args.join(", ")
            ));
        } else {
            // Regular constructor: delegates to ffi function
            let prelude = cstr_prelude_params(&ctor.params);
```
</details>

I see the issue — the format string has `{}` for: doc, pub, name, params, Self::name, args — that's 6 placeholders but  is missing from the arguments. The original code had it but when I edited I dropped it:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4834-L4838">bindings.rs -3+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                doc,\n                unsafe_kw,\n                ctor.impl_method_name,\n                params.join(\", \"),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4834:zc,4835:yp,4836:az"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":4834,"endLine":4837}
```
</details>

**Terminal:**
```sh
time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.03s
real  5.1s
```

Now regenerate bindings and do full build:

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (319 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 6.98s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 73 modules via exclude_modules
Loaded config: 1 module patterns, 75 exclude module patterns, 0 individual headers, 10 header exclusions -> 5761 headers
  Excluded 35 headers after dependency resolution
Parsing 5761 headers...
  Clang parse time: 2.42s
  Collected 3641 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan

Timing summary:
  Total clang parse time: 2.42s
  Total AST visit time: 3.07s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (4400 classes, 1237 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 11 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 35 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs (26 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/moni_tool.rs (14 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/interface.rs (49 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/resource.rs (4 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_data.rs (37 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/header_section.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_graph.rs (11 types, 0 extra)
... (truncated, 32494 chars total)
```
</details>

**Terminal:**
```sh
tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01RtHTCEjXnf96tuUeHZGNvz__vscode-1771312785857/content.txt"
```
<details>
<summary>Output (20 lines)</summary>

```
ic3d.rs (extra types only, 11 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xb_re
p_mesh.rs (function-only module, 1 functions)                                     Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 3 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hermi
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
to_geom.rs (function-only module, 48 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  49.8s
```
</details>

Bindings regenerated successfully. Now build the full project:

**Terminal:**
```sh
time cargo build 2>&1
```
<details>
<summary>Output (518 lines) — exit code 101</summary>

```
warning: opencascade-sys@0.2.0:   357 |                                   const TCollection_ExtendedString& theString2)
warning: opencascade-sys@0.2.0:       |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332189:19: error: no matching member function for call to 'IsDifferent'
warning: opencascade-sys@0.2.0:  332189 |     return self_->IsDifferent(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:195:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   195 |   Standard_EXPORT Standard_Boolean IsDifferent(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:202:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection_ExtendedString' for 1st argument
warning: opencascade-sys@0.2.0:   202 |   Standard_EXPORT Standard_Boolean IsDifferent(const TCollection_ExtendedString& other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332195:19: error: no matching member function for call to 'IsLess'
warning: opencascade-sys@0.2.0:  332195 |     return self_->IsLess(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:210:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   210 |   Standard_EXPORT Standard_Boolean IsLess(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:215:36: note: candidate function not viable: no kn
... (truncated, 62419 chars total)
```
</details>

**Terminal:**
```sh
tail -5 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01FJUtviz4ny2FTQhEcUuxJS__vscode-1771312785859/content.txt"
```
```

real  24.3s
```

**Terminal:**
```sh
tail -10 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01FJUtviz4ny2FTQhEcUuxJS__vscode-1771312785859/content.txt"
```
<details>
<summary>Output (7 lines)</summary>

```
  --- stderr


  error occurred in cc-rs: command did not execute successfully (status code exi
t status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                                           

real  24.3s
```
</details>

C++ compilation error. Let me check what the actual error is:

**Terminal:**
```sh
grep -E "error:|warning:" "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01FJUtviz4ny2FTQhEcUuxJS__vscode-1771312785859/content.txt" | grep "error:" | head -30
```
<details>
<summary>Output (30 lines)</summary>

```
ade-sys/generated/wrappers.cpp:332189:19: error: no matching member function for call to 'IsDifferent'                                                          warning: opencascade-sys@0.2.0:  332189 |     return self_->IsDifferent(other);
ade-sys/generated/wrappers.cpp:332195:19: error: no matching member function for call to 'IsLess'                                                               warning: opencascade-sys@0.2.0:  332195 |     return self_->IsLess(other);
ade-sys/generated/wrappers.cpp:332201:19: error: no matching member function for call to 'IsGreater'                                                            warning: opencascade-sys@0.2.0:  332201 |     return self_->IsGreater(other);
ade-sys/generated/wrappers.cpp:332240:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332240 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:332509:16: error: no matching constructor for initialization of 'TCollection_HExtendedString'                                    warning: opencascade-sys@0.2.0:  332509 |     return new TCollection_HExtendedSt
ade-sys/generated/wrappers.cpp:332533:78: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  332533 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:332584:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332584 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:381453:72: error: no matching member function for call to 'ToHString'                                                            warning: opencascade-sys@0.2.0:  381453 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:381456:60: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  381456 |     return new TCollection_ExtendedStr
ade-sys/generated/wrappers.cpp:381495:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  381495 |     return self_->AsciiToExtended(str)
ade-sys/generated/wrappers.cpp:381513:35: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short
... (truncated, 8748 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01VcdeAfhTQzK2fhDF6zWmv6__vscode-1771312785862/content.txt)

> 💭 The errors are about type mismatches involving `Standard_ExtString` (which is `const char16_t*` but the bindings are generating wrappers that use `const uint16_t*`), `Standard_GUID` constructors, and methods like `IsDifferent`, `IsLess`, `IsGreater`, `IsEqual` which now take raw pointer parameters that don't match.
> 
> These are C++ wrapper compilation errors caused by the fact that previously, functions involving raw pointers were excluded from binding generation. Now they're being included, but the C++ wrappers being generated for them have type mismatches.
> 
> The main issues:
> 
> 
> - `Standard_ExtString` is `const char16_t*` but the type mapping maps it to `const uint16_t*` — this is a `const T*` pointer where `T` is `Standard_ExtCharacter` (which is `char16_t`), but the C++ type mapping generates `uint16_t*` instead.
> - `Standard_GUID` constructor takes a `const char*` (raw pointer)
> - Methods like `IsEqual`, `IsDifferent`, `IsLess`, `IsGreater` probably take `Standard_ExtString` params
> 
> The root cause is that when raw pointers were excluded from binding generation, the type mappings weren't set up to handle them correctly, so now that they're being included, there's a mismatch between what the C++ types actually are and what the bindings are generating. These functions were previously excluded because they use raw pointer types, but now that they're being included, the C++ wrapper generation is struggling with the type mapping—specifically, `Standard_ExtString` (which is `const char16_t*`) is being incorrectly converted to `const uint16_t*` in the wrapper code. I need to trace through how the C++ wrapper generator handles these typedef'd pointer types and ensure it's using the correct underlying C++ types from the parsed declarations rather than making incorrect conversions. I'm seeing a pattern here—the constructor and comparison methods are failing because the parameter types don't match what the C++ bindings expect. The real issue is that these raw pointer functions were previously marked as unbindable and skipped entirely, so the type mapping was never tested. Now that I'm trying to include them, I need to figure out how the wrapper generator is converting C++ types for these pointer-based functions. Rather than trying to fix every type mapping issue individually, I should probably mark `Standard_ExtString`, `Standard_ExtCharacter`, and similar problematic types as unbindable or handle them specially. The core issue is that the C++ mapper is incorrectly treating `Standard_ExtCharacter` as `uint16_t` when it should map to `char16_t`, and this cascades into parameter and return type mismatches.

The errors are about type mismatches with `Standard_ExtString` (`const char16_t*` vs `const uint16_t*`), `Standard_GUID`, and related functions. These were previously excluded as raw-pointer functions. Now they're being included but the C++ wrapper types don't match. Let me investigate the C++ type mapping:

<details>
<summary>Searched for regex <code>Standard_ExtCharacter|Standard_ExtString|char16_t</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 8 results</summary>

- [parser.rs:40](../crates/opencascade-binding-generator/src/parser.rs#L40)
- [parser.rs:1381](../crates/opencascade-binding-generator/src/parser.rs#L1381)
- [parser.rs:1591](../crates/opencascade-binding-generator/src/parser.rs#L1591)
- [parser.rs:1592](../crates/opencascade-binding-generator/src/parser.rs#L1592)
- [parser.rs:1603](../crates/opencascade-binding-generator/src/parser.rs#L1603)
- [model.rs:452](../crates/opencascade-binding-generator/src/model.rs#L452)
- [type_mapping.rs:499](../crates/opencascade-binding-generator/src/type_mapping.rs#L499)
</details>

> 💭 I see the issue now. `Standard_ExtCharacter` is `char16_t` in C++ but it's being resolved to `uint16_t` in the parser (line 40: `.replace("Standard_ExtCharacter", "uint16_t")`). This produces `Type::U16` for `Standard_ExtCharacter`, and `ConstPtr(U16)` for `Standard_ExtString`. 
> 
> On the Rust side, `U16` maps to `u16` which is fine. But on the C++ side, the wrapper needs to use `Standard_ExtCharacter` (which is `char16_t`) not `uint16_t`, because they're different types in C++.
> 
> 
> Now I need to trace through how the C++ code generator converts the parsed `Type` objects back into actual C++ type names.

Searched for regex `fn.*type_to_cpp|fn.*cpp_type|U16|uint16` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

<details>
<summary>Searched for regex <code>type_to_cpp|cpp_type_string|U16|uint16</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [rust.rs:110](../crates/opencascade-binding-generator/src/codegen/rust.rs#L110)
- [bindings.rs:431](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L431)
- [bindings.rs:596](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L596)
- [bindings.rs:602](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L602)
- [bindings.rs:610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L610)
- [bindings.rs:611](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L611)
- [bindings.rs:612](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L612)
- [bindings.rs:613](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L613)
- [bindings.rs:614](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L614)
- [bindings.rs:622](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L622)
- [bindings.rs:624](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L624)
- [bindings.rs:625](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L625)
- [bindings.rs:629](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L629)
- [bindings.rs:686](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L686)
- [bindings.rs:1180](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1180)
- [bindings.rs:1191](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1191)
- [bindings.rs:1225](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1225)
</details>

The problem is in :  maps to `"uint16_t"` but `Standard_ExtCharacter` is actually `char16_t`, and `Standard_ExtString` is `const char16_t*`. When we have a , the C++ wrapper generates `const uint16_t*` which doesn't match the expected `const char16_t*`.

These are distinct C++ types that happen to be the same size but aren't implicitly convertible. The proper fix is to not lose the original `Standard_ExtString/Standard_ExtCharacter` type information. But that's a broader parser issue.

For now, the quickest fix is to add `Standard_ExtString` and related raw pointer types that have type conversion issues to the unbindable set, since they were unbindable before anyway. Actually, a better approach: I should add these specific problematic types to `is_unbindable()` or create a separate exclude. But actually, the real issue is that  is now being treated as a raw pointer that can be bound, but the C++ side can't handle it properly because `char16_t` ≠ `uint16_t`. 

Let me check all the error categories more carefully. Let me look at the `Standard_GUID` issue too:

Research C++ compilation errors

<details>
<summary>Reading lines 332180-332210 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L332180-L332211">wrappers.cpp</a></summary>

```
    return self_->IsEmpty();
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_equal_u16ptr(const TCollection_ExtendedString* self_, const uint16_t* other) {
    return self_->IsEqual(other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_equal_extendedstring(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* other) {
    return self_->IsEqual(*other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_different_u16ptr(const TCollection_ExtendedString* self_, const uint16_t* other) {
    return self_->IsDifferent(other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_different_extendedstring(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* other) {
    return self_->IsDifferent(*other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_less_u16ptr(const TCollection_ExtendedString* self_, const uint16_t* other) {
    return self_->IsLess(other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_less_extendedstring(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* other) {
    return self_->IsLess(*other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_greater_u16ptr(const TCollection_ExtendedString* self_, const uint16_t* other) {
    return self_->IsGreater(other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_is_greater_extendedstring(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* other) {
    return self_->IsGreater(*other);
}
extern "C" Standard_Boolean TCollection_ExtendedString_starts_with(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* theStartString) {
    return self_->StartsWith(*theStartString);
}
extern "C" Standard_Boolean TCollection_ExtendedString_ends_with(const TCollection_ExtendedString* self_, const TCollection_ExtendedString* theEndString) {
    return self_->EndsWith(*theEndString);
```
</details>

<details>
<summary>Reading lines 332235-332250 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L332235-L332251">wrappers.cpp</a></summary>

```
}
extern "C" void TCollection_ExtendedString_set_value_int_extendedstring(TCollection_ExtendedString* self_, Standard_Integer where, const TCollection_ExtendedString* what) {
    self_->SetValue(where, *what);
}
extern "C" const uint16_t* TCollection_ExtendedString_to_ext_string(const TCollection_ExtendedString* self_) {
    return self_->ToExtString();
}
extern "C" void TCollection_ExtendedString_trunc(TCollection_ExtendedString* self_, Standard_Integer ahowmany) {
    self_->Trunc(ahowmany);
}
extern "C" uint16_t TCollection_ExtendedString_value(const TCollection_ExtendedString* self_, Standard_Integer where) {
    return self_->Value(where);
}
extern "C" size_t TCollection_ExtendedString_hash_code(const TCollection_ExtendedString* self_) {
    return self_->HashCode();
}
```
</details>

<details>
<summary>Reading lines 332505-332540 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L332505-L332541">wrappers.cpp</a></summary>

```
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_charptr(const char* message) {
    return new TCollection_HExtendedString(message);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_u16ptr(const uint16_t* message) {
    return new TCollection_HExtendedString(message);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_u16(uint16_t aChar) {
    return new TCollection_HExtendedString(aChar);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_int_u16(Standard_Integer length, uint16_t filler) {
    return new TCollection_HExtendedString(length, filler);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_extendedstring(const TCollection_ExtendedString* aString) {
    return new TCollection_HExtendedString(*aString);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_handletcollectionhasciistring(const opencascade::handle<TCollection_HAsciiString>* aString) {
    return new TCollection_HExtendedString(*aString);
}
extern "C" TCollection_HExtendedString* TCollection_HExtendedString_ctor_handletcollectionhextendedstring(const opencascade::handle<TCollection_HExtendedString>* aString) {
    return new TCollection_HExtendedString(*aString);
}
extern "C" opencascade::handle<TCollection_HExtendedString>* TCollection_HExtendedString_cat(const TCollection_HExtendedString* self_, const opencascade::handle<TCollection_HExtendedString>* other) {
    return new opencascade::handle<TCollection_HExtendedString>(self_->Cat(*other));
}
extern "C" opencascade::handle<TCollection_HExtendedString>* TCollection_HExtendedString_split(TCollection_HExtendedString* self_, Standard_Integer where) {
    return new opencascade::handle<TCollection_HExtendedString>(self_->Split(where));
}
extern "C" opencascade::handle<TCollection_HExtendedString>* TCollection_HExtendedString_token(const TCollection_HExtendedString* self_, const uint16_t* separators, Standard_Integer whichone) {
    return new opencascade::handle<TCollection_HExtendedString>(self_->Token(separators, whichone));
}
extern "C" void TCollection_HExtendedString_assign_cat(TCollection_HExtendedString* self_, const opencascade::handle<TCollection_HExtendedString>* other) {
    self_->AssignCat(*other);
}
extern "C" void TCollection_HExtendedString_change_all(TCollection_HExtendedString* self_, uint16_t aChar, uint16_t NewChar) {
    self_->ChangeAll(aChar, NewChar);
}
```
</details>

<details>
<summary>Reading lines 332580-332590 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L332580-L332591">wrappers.cpp</a></summary>

```
extern "C" Standard_Integer TCollection_HExtendedString_search_from_end(const TCollection_HExtendedString* self_, const opencascade::handle<TCollection_HExtendedString>* what) {
    return self_->SearchFromEnd(*what);
}
extern "C" const uint16_t* TCollection_HExtendedString_to_ext_string(const TCollection_HExtendedString* self_) {
    return self_->ToExtString();
}
extern "C" void TCollection_HExtendedString_trunc(TCollection_HExtendedString* self_, Standard_Integer ahowmany) {
    self_->Trunc(ahowmany);
}
extern "C" uint16_t TCollection_HExtendedString_value(const TCollection_HExtendedString* self_, Standard_Integer where) {
    return self_->Value(where);
```
</details>

<details>
<summary>Reading lines 206865-206890 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L206865-L206891">wrappers.cpp</a></summary>

```
    return self_->Real(aResourceName);
}
extern "C" const char* Resource_Manager_value(const Resource_Manager* self_, const char* aResourceName) {
    return self_->Value(aResourceName);
}
extern "C" const uint16_t* Resource_Manager_ext_value(Resource_Manager* self_, const char* aResourceName) {
    return self_->ExtValue(aResourceName);
}
extern "C" void Resource_Manager_set_resource_charptr_int(Resource_Manager* self_, const char* aResourceName, Standard_Integer aValue) {
    self_->SetResource(aResourceName, aValue);
}
extern "C" void Resource_Manager_set_resource_charptr_real(Resource_Manager* self_, const char* aResourceName, Standard_Real aValue) {
    self_->SetResource(aResourceName, aValue);
}
extern "C" void Resource_Manager_set_resource_charptr2(Resource_Manager* self_, const char* aResourceName, const char* aValue) {
    self_->SetResource(aResourceName, aValue);
}
extern "C" void Resource_Manager_set_resource_charptr_u16ptr(Resource_Manager* self_, const char* aResourceName, const uint16_t* aValue) {
    self_->SetResource(aResourceName, aValue);
}
extern "C" const opencascade::handle<Standard_Type>& Resource_Manager_dynamic_type(const Resource_Manager* self_) {
    return self_->DynamicType();
}
extern "C" Standard_Boolean Resource_Manager_save(const Resource_Manager* self_) {
    return self_->Save();
}
```
</details>

<details>
<summary>Reading lines 220675-220690 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L220675-L220691">wrappers.cpp</a></summary>

```
    return new Standard_GUID();
}
extern "C" Standard_GUID* Standard_GUID_ctor_charptr(const char* aGuid) {
    return new Standard_GUID(aGuid);
}
extern "C" Standard_GUID* Standard_GUID_ctor_u16ptr(const uint16_t* aGuid) {
    return new Standard_GUID(aGuid);
}
extern "C" Standard_GUID* Standard_GUID_ctor_uuid(const Standard_UUID* aGuid) {
    return new Standard_GUID(*aGuid);
}
extern "C" Standard_UUID* Standard_GUID_to_uuid(const Standard_GUID* self_) {
    return new Standard_UUID(self_->ToUUID());
}
extern "C" Standard_Boolean Standard_GUID_is_same(const Standard_GUID* self_, const Standard_GUID* uid) {
    return self_->IsSame(*uid);
```
</details>

<details>
<summary>Reading lines 381445-381540 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L381445-L381541">wrappers.cpp</a></summary>

```
}
extern "C" opencascade::handle<TCollection_HAsciiString>* XSControl_Utils_to_h_string_charptr(const XSControl_Utils* self_, const char* strcon) {
    return new opencascade::handle<TCollection_HAsciiString>(self_->ToHString(strcon));
}
extern "C" TCollection_AsciiString* XSControl_Utils_to_a_string(const XSControl_Utils* self_, const char* strcon) {
    return new TCollection_AsciiString(self_->ToAString(strcon));
}
extern "C" opencascade::handle<TCollection_HExtendedString>* XSControl_Utils_to_h_string_u16ptr(const XSControl_Utils* self_, const uint16_t* strcon) {
    return new opencascade::handle<TCollection_HExtendedString>(self_->ToHString(strcon));
}
extern "C" TCollection_ExtendedString* XSControl_Utils_to_x_string(const XSControl_Utils* self_, const uint16_t* strcon) {
    return new TCollection_ExtendedString(self_->ToXString(strcon));
}
extern "C" opencascade::handle<TColStd_HSequenceOfHAsciiString>* XSControl_Utils_new_seq_c_str(const XSControl_Utils* self_) {
    return new opencascade::handle<TColStd_HSequenceOfHAsciiString>(self_->NewSeqCStr());
}
extern "C" opencascade::handle<TColStd_HSequenceOfHExtendedString>* XSControl_Utils_new_seq_e_str(const XSControl_Utils* self_) {
    return new opencascade::handle<TColStd_HSequenceOfHExtendedString>(self_->NewSeqEStr());
}
extern "C" TopoDS_Shape* XSControl_Utils_compound_from_seq(const XSControl_Utils* self_, const opencascade::handle<TopTools_HSequenceOfShape>* seqval) {
    return new TopoDS_Shape(self_->CompoundFromSeq(*seqval));
}
extern "C" TopoDS_Shape* XSControl_Utils_sorted_compound(const XSControl_Utils* self_, const TopoDS_Shape* shape, int32_t type, Standard_Boolean explore, Standard_Boolean compound) {
    return new TopoDS_Shape(self_->SortedCompound(*shape, static_cast<TopAbs_ShapeEnum>(type), explore, compound));
}
extern "C" TopoDS_Shape* XSControl_Utils_shape_value(const XSControl_Utils* self_, const opencascade::handle<TopTools_HSequenceOfShape>* seqv, Standard_Integer num) {
    return new TopoDS_Shape(self_->ShapeValue(*seqv, num));
}
extern "C" opencascade::handle<TopTools_HSequenceOfShape>* XSControl_Utils_new_seq_shape(const XSControl_Utils* self_) {
    return new opencascade::handle<TopTools_HSequenceOfShape>(self_->NewSeqShape());
}
extern "C" opencascade::handle<Standard_Transient>* XSControl_Utils_shape_binder(const XSControl_Utils* self_, const TopoDS_Shape* shape, Standard_Boolean hs) {
    return new opencascade::handle<Standard_Transient>(self_->ShapeBinder(*shape, hs));
}
extern "C" TopoDS_Shape* XSControl_Utils_binder_shape(const XSControl_Utils* self_, const opencascade::handle<Standard_Transient>* tr) {
    return new TopoDS_Shape(self_->BinderShape(*tr));
}
extern "C" opencascade::handle<Standard_Transient>* XSControl_Utils_seq_to_arr(const XSControl_Utils* self_, const opencascade::handle<Standard_Transient>* seq, Standard_Integer first) {
    return new opencascade::handle<Standard_Transient>(self_->SeqToArr(*seq, first));
}
extern "C" opencascade::handle<Standard_Transient>* XSControl_Utils_arr_to_seq(const XSControl_Utils* self_, const opencascade::handle<Standard_Transient>* arr) {
    return new opencascade::handle<Standard_Transient>(self_->ArrToSeq(*arr));
}
extern "C" void XSControl_Utils_trace_line(const XSControl_Utils* self_, const char* line) {
    self_->TraceLine(line);
}
extern "C" void XSControl_Utils_date_values(const XSControl_Utils* self_, const char* text, Standard_Integer* yy, Standard_Integer* mm, Standard_Integer* dd, Standard_Integer* hh, Standard_Integer* mn, Standard_Integer* ss) {
    self_->DateValues(text, *yy, *mm, *dd, *hh, *mn, *ss);
}
extern "C" const uint16_t* XSControl_Utils_ascii_to_extended(const XSControl_Utils* self_, const char* str) {
    return self_->AsciiToExtended(str);
}
extern "C" void XSControl_Utils_append_c_str(const XSControl_Utils* self_, const opencascade::handle<TColStd_HSequenceOfHAsciiString>* seqval, const char* strval) {
    self_->AppendCStr(*seqval, strval);
}
extern "
... (truncated, 6778 chars)
```
</details>

<details>
<summary>Reading lines 1-300 of <a href="../crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx#L1-L301">TCollection_ExtendedString.hxx</a></summary>

```
// Created on: 1993-02-22
// Created by: Mireille MERCIEN
// Copyright (c) 1993-1999 Matra Datavision
// Copyright (c) 1999-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _TCollection_ExtendedString_HeaderFile
#define _TCollection_ExtendedString_HeaderFile

#include <Standard.hxx>
#include <Standard_DefineAlloc.hxx>
#include <Standard_Handle.hxx>

#include <Standard_PExtCharacter.hxx>
#include <Standard_Integer.hxx>
#include <Standard_CString.hxx>
#include <Standard_Boolean.hxx>
#include <Standard_Character.hxx>
#include <Standard_ExtCharacter.hxx>
#include <Standard_Real.hxx>
#include <Standard_OStream.hxx>
#include <Standard_PCharacter.hxx>
#include <Standard_Macro.hxx>
class TCollection_AsciiString;

//! A variable-length sequence of "extended" (UNICODE) characters (16-bit character type).
//! It provides editing operations with built-in memory management
//! to make ExtendedString objects easier to use than ordinary extended character arrays.
//! ExtendedString objects follow "value semantics", that is, they are the actual strings,
//! not handles to strings, and are copied through assignment.
//! You may use HExtendedString objects to get handles to strings.
//!
//! Beware that class can transparently store UTF-16 string with surrogate pairs
//! (Unicode symbol represented by two 16-bit code units).
//! However, surrogate pairs are not considered by the following methods:
//! - Method ::Length() return the number of 16-bit code units, not the number of Unicode symbols.
//! - Methods taking/returning symbol index work with 16-bit code units, not true Unicode symbols,
//!   including ::Remove(), ::SetValue(), ::Value(), ::Search(), ::Trunc() and others.
//! If application needs to process surrogate pairs, NCollection_Utf16Iter class can be used
//! for iterating through Unicode string (UTF-32 code unit will be returned for each position).
class TCollection_ExtendedString
{
public:
  DEFINE_STANDARD_ALLOC

  //! Initializes a ExtendedString to an empty ExtendedString.
  Standard_EXPORT TCollection_ExtendedString();

  //! Creation by converting a CString to an extended
  //! string.  If <isMultiByte> is true then the string is
  //! treated as having UTF-8 coding.  If it is not a UTF-8
  //! then <isMultiByte> is ignored and each character is
  //! copied to ExtCharacter.
  Standard_EXPORT TCollection_ExtendedString(const Standard_CString astring,
                                             const Standard_Boolean isMultiByte = Standard_False);

  //! Creation by converting an ExtString to an extended string.
  Standard_EXPORT TCollection_ExtendedString(const Standard_ExtString astring);

#if !defined(_MSC_VER) || defined(_NATIVE_WCHAR_T_DEFINED)
  //! Initialize from wide-char string considering it as Unicode string
  //! (the size of wide char is a platform-dependent - e.g. on Windows wchar_t is UTF-16).
  //!
  //! This constructor is unavailable if application is built with deprecated msvc option
  //! "-Zc:wchar_t-", since OCCT itself is never built with this option.
  Standard_EXPORT TCollection_ExtendedString(const Standard_WideChar* theStringUtf);
#endif

  //! Initializes a AsciiString with a single character.
  Standard_EXPORT TCollection_ExtendedString(const Standard_Character aChar);

  //! Initializes a ExtendedString with a single character.
  Standard_EXPORT TCollection_ExtendedString(const Standard_ExtCharacter aChar);

  //! Initializes a
... (truncated, 13491 chars)
```
</details>

<details>
<summary>Reading lines 1-150 of <a href="../crates/occt-sys/OCCT/src/Standard/Standard_GUID.hxx#L1-L151">Standard_GUID.hxx</a></summary>

```
// Created on: 1997-06-19
// Created by: Christophe LEYNADIER
// Copyright (c) 1997-1999 Matra Datavision
// Copyright (c) 1999-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Standard_GUID_HeaderFile
#define _Standard_GUID_HeaderFile

#include <Standard_Handle.hxx>

#include <Standard_Integer.hxx>
#include <Standard_CString.hxx>
#include <Standard_UUID.hxx>
#include <Standard_PCharacter.hxx>
#include <Standard_PExtCharacter.hxx>
#include <Standard_OStream.hxx>

#define Standard_GUID_SIZE 36
#define Standard_GUID_SIZE_ALLOC Standard_GUID_SIZE + 1

class Standard_GUID
{
public:
  DEFINE_STANDARD_ALLOC

  Standard_EXPORT Standard_GUID();

  //! build a GUID from an ascii string with the
  //! following format:
  //! Length : 36 char
  //! "00000000-0000-0000-0000-000000000000"
  Standard_EXPORT Standard_GUID(const Standard_CString aGuid);

  //! build a GUID from an unicode string with the
  //! following format:
  //!
  //! "00000000-0000-0000-0000-000000000000"
  Standard_EXPORT Standard_GUID(const Standard_ExtString aGuid);

  Standard_EXPORT Standard_GUID(const Standard_Integer      a32b,
                                const Standard_ExtCharacter a16b1,
                                const Standard_ExtCharacter a16b2,
                                const Standard_ExtCharacter a16b3,
                                const Standard_Byte         a8b1,
                                const Standard_Byte         a8b2,
                                const Standard_Byte         a8b3,
                                const Standard_Byte         a8b4,
                                const Standard_Byte         a8b5,
                                const Standard_Byte         a8b6);

  Standard_EXPORT Standard_GUID(const Standard_UUID& aGuid);

  Standard_EXPORT Standard_GUID(const Standard_GUID& aGuid);

  Standard_EXPORT Standard_UUID ToUUID() const;

  //! translate the GUID into ascii string
  //! the aStrGuid is allocated by user.
  //! the guid have the following format:
  //!
  //! "00000000-0000-0000-0000-000000000000"
  Standard_EXPORT void ToCString(const Standard_PCharacter aStrGuid) const;

  //! translate the GUID into unicode string
  //! the aStrGuid is allocated by user.
  //! the guid have the following format:
  //!
  //! "00000000-0000-0000-0000-000000000000"
  Standard_EXPORT void ToExtString(const Standard_PExtCharacter aStrGuid) const;

  Standard_EXPORT Standard_Boolean IsSame(const Standard_GUID& uid) const;

  Standard_Boolean operator==(const Standard_GUID& uid) const { return IsSame(uid); }

  Standard_EXPORT Standard_Boolean IsNotSame(const Standard_GUID& uid) const;

  Standard_Boolean operator!=(const Standard_GUID& uid) const { return IsNotSame(uid); }

  Standard_EXPORT void Assign(const Standard_GUID& uid);

  void operator=(const Standard_GUID& uid) { Assign(uid); }

  Standard_EXPORT void Assign(const Standard_UUID& uid);

  void operator=(const Standard_UUID& uid) { Assign(uid); }

  //! Display the GUID with the following format:
  //!
  //! "00000000-0000-0000-0000-000000000000"
  Standard_EXPORT void ShallowDump(Standard_OStream& aStream) const;

  //! Check the format of a GUID string.
  //! It checks the size, the position of the '-' and the correct size of fields.
  Standard_EXPORT static Standard_Boolean CheckGUIDFormat(const Standard_CString aGuid);

  template <class T>
  friend struct std::hash;

private:
  Standard_I
... (truncated, 5026 chars)
```
</details>

Searched for text `Standard_ExtString` (`**/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx`), 1 result
- [Standard_TypeDef.hxx:93](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#L93)

Searched for text `Standard_ExtCharacter` (`**/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx`), 3 results
- [Standard_TypeDef.hxx:79](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#L79)
- [Standard_TypeDef.hxx:83](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#L83)
- [Standard_TypeDef.hxx:93](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#L93)

Read [Standard_TypeDef.hxx](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#85-85), lines 85 to 96

Read [Standard_TypeDef.hxx](../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#70-70), lines 70 to 90

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01NtxR8W8QiWeJW1vkD97dTF__vscode-1771312785875/content.txt)

<details>
<summary>Searched for regex <code>ExtString|ExtChar|uint16_t|char16_t|U16</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [model.rs:452](../crates/opencascade-binding-generator/src/model.rs#L452)
- [model.rs:453](../crates/opencascade-binding-generator/src/model.rs#L453)
- [model.rs:492](../crates/opencascade-binding-generator/src/model.rs#L492)
- [model.rs:515](../crates/opencascade-binding-generator/src/model.rs#L515)
- [model.rs:531](../crates/opencascade-binding-generator/src/model.rs#L531)
- [model.rs:707](../crates/opencascade-binding-generator/src/model.rs#L707)
- [model.rs:750](../crates/opencascade-binding-generator/src/model.rs#L750)
- [model.rs:818](../crates/opencascade-binding-generator/src/model.rs#L818)
- [resolver.rs:1382](../crates/opencascade-binding-generator/src/resolver.rs#L1382)
- [type_mapping.rs:50](../crates/opencascade-binding-generator/src/type_mapping.rs#L50)
- [type_mapping.rs:51](../crates/opencascade-binding-generator/src/type_mapping.rs#L51)
- [type_mapping.rs:499](../crates/opencascade-binding-generator/src/type_mapping.rs#L499)
</details>

<details>
<summary>Searched for regex <code>Standard_ExtString|Standard_PExtCharacter|ConstPtr.*U16|Ptr.*U16|is_raw_ptr|is_void_ptr</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 12 results</summary>

- [model.rs:577](../crates/opencascade-binding-generator/src/model.rs#L577)
- [model.rs:581](../crates/opencascade-binding-generator/src/model.rs#L581)
- [model.rs:601](../crates/opencascade-binding-generator/src/model.rs#L601)
- [model.rs:607](../crates/opencascade-binding-generator/src/model.rs#L607)
- [model.rs:697](../crates/opencascade-binding-generator/src/model.rs#L697)
- [parser.rs:1592](../crates/opencascade-binding-generator/src/parser.rs#L1592)
- [parser.rs:1606](../crates/opencascade-binding-generator/src/parser.rs#L1606)
- [bindings.rs:819](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L819)
- [bindings.rs:821](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L821)
- [bindings.rs:1347](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1347)
</details>

<details>
<summary>Reading lines 577-625 of <a href="../crates/opencascade-binding-generator/src/model.rs#L577-L626">model.rs</a></summary>

```
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
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
                    Type::Class(name) if name != "char" => Some(name.as_str()),
                    _ => None,
                }
            }
```
</details>

<details>
<summary>Reading lines 690-720 of <a href="../crates/opencascade-binding-generator/src/model.rs#L690-L721">model.rs</a></summary>

```
    }

    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr()
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I64 => "long long".to_string(),
            Type::U64 => "unsigned long long".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            Type::ConstPtr(inner) => format!("const {}*", inner.to_cpp_string()),
            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
            Type::Handle(name) => format!("Handle({})", name),
```
</details>

<details>
<summary>Reading lines 810-840 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810-L841">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 1585-1615 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1585-L1616">parser.rs</a></summary>

```
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I32),  // i16 isn't available, use i32
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

```
</details>

<details>
<summary>Searched for regex <code>needs_unsafe_fn|is_raw_ptr|is_void_ptr|skip.*method|unbindable</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:773](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L773)
- [bindings.rs:774](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L774)
- [bindings.rs:777](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L777)
- [bindings.rs:778](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L778)
- [bindings.rs:782](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L782)
- [bindings.rs:783](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L783)
- [bindings.rs:789](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L789)
- [bindings.rs:790](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L790)
- [bindings.rs:793](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L793)
- [bindings.rs:794](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L794)
- [bindings.rs:800](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800)
- [bindings.rs:801](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L801)
- [bindings.rs:804](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L804)
- [bindings.rs:805](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L805)
- [bindings.rs:809](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L809)
- [bindings.rs:810](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810)
- [bindings.rs:816](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L816)
- [bindings.rs:817](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L817)
- [bindings.rs:819](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L819)
- [bindings.rs:821](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L821)
</details>

<details>
<summary>Searched for text <code>is_unbindable</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 9 results</summary>

- [model.rs:50](../crates/opencascade-binding-generator/src/model.rs#L50)
- [model.rs:54](../crates/opencascade-binding-generator/src/model.rs#L54)
- [model.rs:231](../crates/opencascade-binding-generator/src/model.rs#L231)
- [model.rs:268](../crates/opencascade-binding-generator/src/model.rs#L268)
- [model.rs:273](../crates/opencascade-binding-generator/src/model.rs#L273)
- [model.rs:351](../crates/opencascade-binding-generator/src/model.rs#L351)
- [model.rs:356](../crates/opencascade-binding-generator/src/model.rs#L356)
- [model.rs:688](../crates/opencascade-binding-generator/src/model.rs#L688)
- [model.rs:802](../crates/opencascade-binding-generator/src/model.rs#L802)
</details>

<details>
<summary>Reading lines 685-700 of <a href="../crates/opencascade-binding-generator/src/model.rs#L685-L701">model.rs</a></summary>

```
    /// Nested types (Parent::Nested) are supported via name flattening
    /// (Parent::Nested → Parent_Nested in Rust FFI), BUT unresolved template types
    /// and unqualified names without underscore remain unbindable.
    pub fn is_unbindable(&self) -> bool {
        self.is_array() || self.is_rvalue_ref() || self.is_unresolved_template_type()
    }

    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr()
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
```
</details>

*2026-02-20 07:59 · 654s · Words: 13,580 in · 2,718 out · 46,137 ctx · 82 rounds*

---

<a id="user-3"></a>

## User (3)

> No, fix the broader parser issue. Don&#x27;t just skip the work. Don&#x27;t make them unbindable. The goal is to bind these pointer types.

*2026-02-20 07:59*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to continue the work of binding raw pointer types. Let me re-read my previous context to understand where I left off.
> 
> Looking at the conversation, I was in the process of implementing changes to make the binding generator handle raw pointers (void*, int*, double*, T*, const T*) by generating unsafe functions instead of skipping them. I had completed changes to:
> 
> 1. `model.rs` - Added `needs_unsafe_fn()` method and `has_unsafe_types()` methods, and removed `is_void_ptr()` and `is_raw_ptr()` from `is_unbindable()`
> 2. `type_mapping.rs` - Added `Standard_Address` → `*mut std::ffi::c_void` mapping
> 
> 
> 3. `bindings.rs` - Added `is_unsafe` fields to binding structs and updated code generation
> 
> The user is pushing back on just documenting why things are skipped—they want me to actually complete the implementation to bind these pointer types rather than leaving them unbindable. Let me check what's been applied in the codebase and what still needs to be done.

Let me check the current state of the codebase to see where things stand after the previous edits.

**Todo list:**
- ✅ Implement changes in model.rs
- ✅ Implement changes in type_mapping.rs
- 🔄 Add is_unsafe to binding structs
- ⬜ Handle Standard_Address in cpp type fns
- ⬜ Update build_param/return_type_binding
- ⬜ Compute is_unsafe in binding creation
- ⬜ Generate unsafe fn in codegen
- ⬜ Update free function codegen (rust.rs)
- ⬜ Build binding generator
- ⬜ Regenerate bindings and full build
- ⬜ Run tests

Let me first check what's already been done in each file:

<details>
<summary>Searched for text <code>needs_unsafe_fn</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 12 results</summary>

- [model.rs:64](../crates/opencascade-binding-generator/src/model.rs#L64)
- [model.rs:68](../crates/opencascade-binding-generator/src/model.rs#L68)
- [model.rs:236](../crates/opencascade-binding-generator/src/model.rs#L236)
- [model.rs:283](../crates/opencascade-binding-generator/src/model.rs#L283)
- [model.rs:288](../crates/opencascade-binding-generator/src/model.rs#L288)
- [model.rs:365](../crates/opencascade-binding-generator/src/model.rs#L365)
- [model.rs:369](../crates/opencascade-binding-generator/src/model.rs#L369)
- [model.rs:696](../crates/opencascade-binding-generator/src/model.rs#L696)
- [bindings.rs:2985](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2985)
- [bindings.rs:2989](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2989)
- [bindings.rs:3497](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3497)
- [bindings.rs:3498](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3498)
</details>

<details>
<summary>Searched for text <code>has_unsafe_types</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 8 results</summary>

- [model.rs:63](../crates/opencascade-binding-generator/src/model.rs#L63)
- [model.rs:235](../crates/opencascade-binding-generator/src/model.rs#L235)
- [model.rs:282](../crates/opencascade-binding-generator/src/model.rs#L282)
- [model.rs:364](../crates/opencascade-binding-generator/src/model.rs#L364)
- [bindings.rs:1774](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1774)
- [bindings.rs:1860](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1860)
- [bindings.rs:1935](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1935)
- [bindings.rs:2476](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2476)
</details>

Searched for text `Standard_Address` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 2 results
- [type_mapping.rs:158](../crates/opencascade-binding-generator/src/type_mapping.rs#L158)
- [type_mapping.rs:159](../crates/opencascade-binding-generator/src/type_mapping.rs#L159)

Good, significant progress has been made. Let me check what's already in bindings.rs:

<details>
<summary>Searched for text <code>is_unsafe</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:123](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L123)
- [bindings.rs:154](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L154)
- [bindings.rs:200](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L200)
- [bindings.rs:223](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L223)
- [bindings.rs:298](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L298)
- [bindings.rs:414](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L414)
- [bindings.rs:1644](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1644)
- [bindings.rs:1774](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1774)
- [bindings.rs:1860](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1860)
- [bindings.rs:1935](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1935)
- [bindings.rs:2476](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2476)
- [bindings.rs:2486](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2486)
- [bindings.rs:2984](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2984)
- [bindings.rs:3002](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3002)
- [bindings.rs:3497](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3497)
- [bindings.rs:4831](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4831)
- [bindings.rs:4844](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4844)
- [bindings.rs:4908](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4908)
- [bindings.rs:4972](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4972)
- [bindings.rs:5028](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5028)
</details>

Good - there's already significant scaffolding in bindings.rs. Let me also check rust.rs:

Searched for text `is_unsafe` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result
- [rust.rs:716](../crates/opencascade-binding-generator/src/codegen/rust.rs#L716)

Excellent - there's quite a bit of work already done. Let me try building to see what the current state is:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1; echo "EXIT: $?"
```
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
real  0.3s
EXIT: 0
```

The generator builds. Now let me check if regeneration and full build work:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/regenerate-bindings.sh 2>&1; echo "EXIT: $?"
```
<details>
<summary>Output (319 lines)</summary>

```
Building binding generator...
    Finished `release` profile [optimized] target(s) in 0.08s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 73 modules via exclude_modules
Loaded config: 1 module patterns, 75 exclude module patterns, 0 individual headers, 10 header exclusions -> 5761 headers
  Excluded 35 headers after dependency resolution
Parsing 5761 headers...
  Clang parse time: 2.01s
  Collected 3641 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan

Timing summary:
  Total clang parse time: 2.01s
  Total AST visit time: 3.08s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (4400 classes, 1237 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 11 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 35 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs (26 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/moni_tool.rs (14 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/interface.rs (49 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/resource.rs (4 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_data.rs (37 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/header_section.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_graph.rs (11 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_select.rs (77 types, 2 extra)
  Wrote: /Users/sethml/
... (truncated, 32369 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -20 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01WibSExKP8RvhN3MUWufdyr__vscode-1771312785898/content.txt
```
<details>
<summary>Output (20 lines)</summary>

```
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xb_re
p_mesh.rs (function-only module, 1 functions)                                     Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 3 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
to_geom.rs (function-only module, 48 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  40.1s
EXIT: 0
```
</details>

Regeneration worked. Now let me run the full build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1; echo "EXIT: $?"
```
<details>
<summary>Output (519 lines)</summary>

```
TCollection_ExtendedString& theString2)
warning: opencascade-sys@0.2.0:       |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332189:19: error: no matching member function for call to 'IsDifferent'
warning: opencascade-sys@0.2.0:  332189 |     return self_->IsDifferent(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:195:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   195 |   Standard_EXPORT Standard_Boolean IsDifferent(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:202:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection_ExtendedString' for 1st argument
warning: opencascade-sys@0.2.0:   202 |   Standard_EXPORT Standard_Boolean IsDifferent(const TCollection_ExtendedString& other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332195:19: error: no matching member function for call to 'IsLess'
warning: opencascade-sys@0.2.0:  332195 |     return self_->IsLess(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:210:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   210 |   Standard_EXPORT Standard_Boolean IsLess(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:215:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection
... (truncated, 62339 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -30 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01B6rx39nHJMEGBGzwieu7dZ__vscode-1771312785900/content.txt
```
<details>
<summary>Output (30 lines)</summary>

```
d/wrappers.cpp:381495:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                                   cargo:warning= 381495 |     return self_->AsciiToExtended(str);
  cargo:warning=        |            ^~~~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generate
d/wrappers.cpp:381513:35: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                                     cargo:warning= 381513 |     return self_->ExtendedToAscii(str);
  cargo:warning=        |                                   ^~~
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/XSCont
rol/XSControl_Utils.hxx:127:77: note: passing argument to parameter 'str' here    cargo:warning=  127 |   Standard_EXPORT Standard_CString ExtendedToAscii(const
 Standard_ExtString str) const;                                                   cargo:warning=      |                                                         
                    ^                                                             cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generate
d/wrappers.cpp:381531:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                                   cargo:warning= 381531 |     return self_->ToEString(*strval);
  cargo:warning=        |            ^~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generate
d/wrappers.cpp:381534:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                                   cargo:warning= 381534 |     return self_->ToEString(*strval);
  cargo:warning=        |            ^~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=fatal error: too many errors emitted, stopping now [-ferror-limi
t=]                                                                               cargo:warning=20 errors generated.
  exit status: 0
  exit status: 0
  exit status: 0
  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code
 exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unu
... (truncated, 4457 chars total)
```
</details>

C++ compile errors. Let me look at the specific errors more carefully:

**Terminal:**
```sh
grep 'error:' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01B6rx39nHJMEGBGzwieu7dZ__vscode-1771312785900/content.txt | head -30
```
<details>
<summary>Output (30 lines)</summary>

```
ade-sys/generated/wrappers.cpp:332189:19: error: no matching member function for call to 'IsDifferent'                                                          warning: opencascade-sys@0.2.0:  332189 |     return self_->IsDifferent(other);
ade-sys/generated/wrappers.cpp:332195:19: error: no matching member function for call to 'IsLess'                                                               warning: opencascade-sys@0.2.0:  332195 |     return self_->IsLess(other);
ade-sys/generated/wrappers.cpp:332201:19: error: no matching member function for call to 'IsGreater'                                                            warning: opencascade-sys@0.2.0:  332201 |     return self_->IsGreater(other);
ade-sys/generated/wrappers.cpp:332240:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332240 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:332509:16: error: no matching constructor for initialization of 'TCollection_HExtendedString'                                    warning: opencascade-sys@0.2.0:  332509 |     return new TCollection_HExtendedSt
ade-sys/generated/wrappers.cpp:332533:78: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  332533 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:332584:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332584 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:381453:72: error: no matching member function for call to 'ToHString'                                                            warning: opencascade-sys@0.2.0:  381453 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:381456:60: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  381456 |     return new TCollection_ExtendedStr
ade-sys/generated/wrappers.cpp:381495:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  381495 |     return self_->AsciiToExtended(str)
ade-sys/generated/wrappers.cpp:381513:35: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short
... (truncated, 8748 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_0192uSCAY7zqrQZRajwjZAJY__vscode-1771312785902/content.txt)

The errors are all about `Standard_ExtString` (`const char16_t*`) being mapped to `const uint16_t*`. This is a type mismatch issue where`Standard_ExtString` is currently mapped as `*const u16` in the Rust type mapping, but in C++, `Standard_ExtString` is `const char16_t*` which is incompatible with `const uint16_t*` (they're different types in C++ despite both being 16-bit).

The key issue is that these methods involving `Standard_ExtString` (which was previously `*const u16` in the Rust mapping) are **now being bound instead of skipped** because we relaxed the `is_unbindable()` check. Previously they were blocked by `is_raw_ptr()` in `is_unbindable()`. Now that raw pointers aren't unbindable, these methods pass through the filter but the C++ wrapper code uses `const uint16_t*` which doesn't match `const char16_t*`.

I need to:
1. Fix the C++ type generation for `Standard_ExtString` to use `const Standard_ExtString` (i.e., `const char16_t*`) 
2. Also check if there are similar issues with `Standard_GUID` and the `IsDifferent`/`IsEqual` comparison methods

Let me investigate more:

**Terminal:**
```sh
grep -n 'Standard_ExtString' /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

Not in type_mapping.rs. Let me check where it's handled:

Searched for text `Standard_ExtString` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [parser.rs:1592](../crates/opencascade-binding-generator/src/parser.rs#L1592)

So `Standard_ExtString` is parsed as `ConstPtr(U16)`, which maps to `*const u16` in Rust. Previously this was blocked by `is_raw_ptr()` returning true for `ConstPtr(U16)` since U16 is not a Class. Now it passes through.

The problem is in the C++ wrapper: when it generates a wrapper for a method with an `Standard_ExtString` param, it uses `const uint16_t*` (from the generic ConstPtr mapping), but C++ treats `const char16_t*` and `const uint16_t*` as incompatible types.

The fix should be in the C++ code generation — when we have a `ConstPtr(U16)` that came from `Standard_ExtString`, we need to use `Standard_ExtString` in the C++ wrapper. But we've lost the original type name information.

Let me think about the right approach. The cleanest fix is to handle this in the parser — parse `Standard_ExtString` as a type that preserves the original C++ type name for the wrapper layer, while still being `*const u16` on the Rust side. Or, we could keep the `ConstPtr(U16)` mapping but ensure the C++ type generation knows to use the original C++ type.

Actually, looking at the errors more carefully, these are all in specific classes:
- `TCollection_ExtendedString` constructors/methods 
- `TCollection_HExtendedString` constructors
- `Resource_Manager` methods
- `Standard_GUID` constructor
- `XSControl_Utils` methods

The real issue is: the C++ type for `U16` is `uint16_t`, but OCCT uses `Standard_ExtString` which is `const char16_t*` — these are distinct types in C++.

The cleanest fix: add `Standard_ExtString` to the C++ type mapping in `type_to_cpp()` and the param conversion functions. Since the parser converts it to `ConstPtr(U16)`, the issue is we lose the `Standard_ExtString` name. 

The better approach: parse `Standard_ExtString` as `ConstPtr(Class("char16_t"))` instead of `ConstPtr(U16)`, and map `char16_t` to `u16` in Rust but `char16_t` in C++. Or even simpler, we could keep it as a named type.

Wait, actually looking back at the errors, there are several distinct error categories:

1. `Standard_ExtString` type mismatch (char16_t* vs uint16_t*)
2. `Standard_GUID` constructor - constructor takes `Standard_ExtString` param
3. `IsDifferent`/`IsEqual`/`IsLess`/`IsGreater` - these take `Standard_ExtString` params

All are related to the `Standard_ExtString` issue. Let me look at how `Standard_GUID` is being called:

**Terminal:**
```sh
grep -B2 -A2 'Standard_GUID(aGuid)' /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp | head -10
```
<details>
<summary>Output (8 lines)</summary>

```
}
extern "C" Standard_GUID* Standard_GUID_ctor_charptr(const char* aGuid) {
    return new Standard_GUID(aGuid);
}
extern "C" Standard_GUID* Standard_GUID_ctor_u16ptr(const uint16_t* aGuid) {
    return new Standard_GUID(aGuid);
}
extern "C" Standard_GUID* Standard_GUID_ctor_uuid(const Standard_UUID* aGuid) {
```
</details>

Yes, `Standard_GUID` has a constructor that takes `const Standard_ExtString` (which is `const char16_t*`), but we're passing `const uint16_t*`. 

The root fix is in the parser — `Standard_ExtString` should be parsed as something that produces `char16_t` in C++ wrappers while mapping to `u16` in Rust. Let me look at how we could do this:

<details>
<summary>Reading lines 1575-1620 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1575-L1621">parser.rs</a></summary>

```
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim();

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I32),  // i16 isn't available, use i32
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Add system C++ standard library include paths to clang arguments
/// 
/// libclang doesn't automatically include these paths, so we need to detect
/// and add them manually. This is platform-specific.
fn add_system_include_paths(args: &mut Vec<String>) {
```
</details>

I see the issue. `Standard_ExtString` -> `ConstPtr(U16)` and `char16_t` -> `U16`. But in C++, `char16_t` and `uint16_t` are distinct types despite being the same width. The C++ wrapper generates `uint16_t` for `U16`, which doesn't match `char16_t`.

The cleanest fix: add a new `Char16` variant to , similar to how `char` is handled as . Let me check how `U16` flows through the system:

<details>
<summary>Searched for text <code>U16</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 8 results</summary>

- [bindings.rs:431](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L431)
- [bindings.rs:602](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L602)
- [bindings.rs:686](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L686)
- [bindings.rs:2079](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2079)
- [bindings.rs:2143](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2143)
</details>

The issue is on line 602:  in `type_to_cpp()`. When `Standard_ExtString` is , the C++ type becomes `const uint16_t*`, but OCCT expects `const char16_t*` (since `Standard_ExtString` is a typedef for `const char16_t*`).

The simplest approach: change how `Standard_ExtString` is parsed. Instead of , use `ConstPtr(Class("char16_t"))` where `char16_t` maps to `u16` in Rust but `char16_t` in C++. Similarly, `Standard_ExtCharacter` should be `Class("char16_t")`.

Let me also check what the `Standard_ExtCharacter` looks like in OCCT:

**Terminal:**
```sh
grep -n 'Standard_ExtCharacter\|Standard_ExtString\|char16_t' /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx 2>/dev/null | head -10
```
<details>
<summary>Output (5 lines)</summary>

```
72:// Unicode primitives, char16_t, char32_t
79:typedef uint16_t Standard_ExtCharacter;
83:typedef char16_t Standard_ExtCharacter;
84:typedef char16_t Standard_Utf16Char; //!< UTF-16 char (always unsigned)
93:typedef const Standard_ExtCharacter* Standard_ExtString;
```
</details>

> 💭 Interesting! So `Standard_ExtCharacter` is conditionally defined:
> - `typedef uint16_t Standard_ExtCharacter;` (line 79, presumably when some condition is NOT met)
> - `typedef char16_t Standard_ExtCharacter;` (line 83, presumably the default/common case)
> - `Standard_ExtString` is `const Standard_ExtCharacter*`
> 
> So depending on a preprocessor define, Standard_ExtCharacter could be either `uint16_t` or `char16_t`. Let me check the preprocessor conditions.

<details>
<summary>Reading lines 70-100 of <a href="../crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx#L70-L101">Standard_TypeDef.hxx</a></summary>

```
typedef std::time_t   Standard_Time;

// Unicode primitives, char16_t, char32_t
typedef char          Standard_Utf8Char;  //!< signed   UTF-8 char
typedef unsigned char Standard_Utf8UChar; //!< unsigned UTF-8 char
#if ((defined(__GNUC__) && !defined(__clang__)                                                     \
      && ((__GNUC__ == 4 && __GNUC_MINOR__ <= 3) || __GNUC__ < 4))                                 \
     || (defined(_MSC_VER) && (_MSC_VER < 1600)))
// compatibility with old GCC and MSVC compilers
typedef uint16_t Standard_ExtCharacter;
typedef uint16_t Standard_Utf16Char;
typedef uint32_t Standard_Utf32Char;
#else
typedef char16_t Standard_ExtCharacter;
typedef char16_t Standard_Utf16Char; //!< UTF-16 char (always unsigned)
typedef char32_t Standard_Utf32Char; //!< UTF-32 char (always unsigned)
#endif
// clang-format off
typedef wchar_t       Standard_WideChar;     //!< wide char (unsigned UTF-16 on Windows platform and signed UTF-32 on Linux)
// clang-format on

//
typedef const Standard_Character*    Standard_CString;
typedef const Standard_ExtCharacter* Standard_ExtString;

#endif // _Standard_TypeDef_HeaderFile

```
</details>

> 💭 So on modern compilers (which we'll be using), `Standard_ExtCharacter` is `char16_t` and `Standard_ExtString` is `const char16_t*`. The `uint16_t` fallback is only for old GCC <= 4.3 and old MSVC < 1600.
> 
> Now I need to figure out the best approach. Let me think about this:
> 
> Option A: Add a new `Type::Char16` variant
> - Pro: Clean separation in the type system
> - Con: Requires updating many match arms across the codebase
> 
> Option B: Keep using `Type::U16` but add special handling in `type_to_cpp()` for Standard_ExtString/Standard_ExtCharacter
> 
> 
> - Pro: Minimal changes
> - Con: Lossy - can't distinguish char16_t from uint16_t in type_to_cpp()
> 
> Option C: Use `ConstPtr(Class("Standard_ExtCharacter"))` and add Standard_ExtCharacter to type mappings
> - Pro: Preserves the OCCT typedef identity
> - Con: Need to make sure the class name maps correctly in both Rust and C++
> 
> The cleanest approach is to trace through what actually happens: in the parser, `Standard_ExtString` gets mapped to `ConstPtr(U16)`, and then in `type_to_cpp()` I need to handle the conversion back to the proper C++ type.
> 
> The real fix is to parse `Standard_ExtString` as `ConstPtr(Class("Standard_ExtCharacter"))` and `Standard_ExtCharacter` as `Class("Standard_ExtCharacter")` directly, then let the type mapping system handle converting that class name to `Standard_ExtCharacter` in C++ and `u16` in Rust. This way the OCCT type identity is preserved throughout the pipeline.

**Todo list:**
- 🔄 Fix char16_t/uint16_t mismatch
- ⬜ Rebuild and verify compilation
- ⬜ Run full test suite

Let me look at the relevant code sections to understand how types flow through the system:

Research type flow in generator

<details>
<summary>Reading lines 1570-1630 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1570-L1631">parser.rs</a></summary>

```

/// Map OCCT Standard_* typedefs to Rust primitive types
fn map_standard_type(type_name: &str) -> Option<Type> {
    // Strip any const/class/struct prefixes
    let clean = type_name
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim();

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I32),  // i16 isn't available, use i32
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Add system C++ standard library include paths to clang arguments
/// 
/// libclang doesn't automatically include these paths, so we need to detect
/// and add them manually. This is platform-specific.
fn add_system_include_paths(args: &mut Vec<String>) {
    #[cfg(target_os = "macos")]
    {
        // Try to get SDK path from xcrun
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
        {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
```
</details>

<details>
<summary>Reading lines 580-680 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L580-L681">bindings.rs</a></summary>

```
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

/// Convert a Type to C++ type string
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
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


/// Convert a Type to C++ parameter type for extern "C" wrapper functions (pointers instead of references)
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "const char*".to_string()
        }
        _ => type_to_cpp(ty),
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
          
... (truncated, 4222 chars)
```
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/model.rs#L1-L101">model.rs</a></summary>

```
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
```
</details>

<details>
<summary>Searched for text <code>map_type_to_rust</code> (<code>**/crates/opencascade-binding-generator/src/type_mapping.rs</code>), 20 results</summary>

- [type_mapping.rs:24](../crates/opencascade-binding-generator/src/type_mapping.rs#L24)
- [type_mapping.rs:99](../crates/opencascade-binding-generator/src/type_mapping.rs#L99)
- [type_mapping.rs:108](../crates/opencascade-binding-generator/src/type_mapping.rs#L108)
- [type_mapping.rs:131](../crates/opencascade-binding-generator/src/type_mapping.rs#L131)
- [type_mapping.rs:140](../crates/opencascade-binding-generator/src/type_mapping.rs#L140)
- [type_mapping.rs:191](../crates/opencascade-binding-generator/src/type_mapping.rs#L191)
- [type_mapping.rs:203](../crates/opencascade-binding-generator/src/type_mapping.rs#L203)
- [type_mapping.rs:463](../crates/opencascade-binding-generator/src/type_mapping.rs#L463)
- [type_mapping.rs:495](../crates/opencascade-binding-generator/src/type_mapping.rs#L495)
- [type_mapping.rs:496](../crates/opencascade-binding-generator/src/type_mapping.rs#L496)
- [type_mapping.rs:497](../crates/opencascade-binding-generator/src/type_mapping.rs#L497)
- [type_mapping.rs:498](../crates/opencascade-binding-generator/src/type_mapping.rs#L498)
- [type_mapping.rs:499](../crates/opencascade-binding-generator/src/type_mapping.rs#L499)
- [type_mapping.rs:500](../crates/opencascade-binding-generator/src/type_mapping.rs#L500)
- [type_mapping.rs:501](../crates/opencascade-binding-generator/src/type_mapping.rs#L501)
- [type_mapping.rs:502](../crates/opencascade-binding-generator/src/type_mapping.rs#L502)
- [type_mapping.rs:503](../crates/opencascade-binding-generator/src/type_mapping.rs#L503)
- [type_mapping.rs:534](../crates/opencascade-binding-generator/src/type_mapping.rs#L534)
- [type_mapping.rs:539](../crates/opencascade-binding-generator/src/type_mapping.rs#L539)
- [type_mapping.rs:543](../crates/opencascade-binding-generator/src/type_mapping.rs#L543)
</details>

<details>
<summary>Reading lines 1-180 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L181">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

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
  
... (truncated, 6784 chars)
```
</details>

<details>
<summary>Reading lines 100-350 of <a href="../crates/opencascade-binding-generator/src/model.rs#L100-L351">model.rs</a></summary>

```
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
        if self.params.is_empty() {
            return String::new();
 
... (truncated, 9527 chars)
```
</details>

<details>
<summary>Reading lines 680-800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L680-L801">bindings.rs</a></summary>

```
fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
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
        Type::Class(name) if name != "char" && name != "Standard_Address" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
        
... (truncated, 5053 chars)
```
</details>

<details>
<summary>Reading lines 100-600 of <a href="../crates/opencascade-binding-generator/src/model.rs#L100-L601">model.rs</a></summary>

```
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
        if self.params.is_empty() {
            return String::new();
 
... (truncated, 17961 chars)
```
</details>

<details>
<summary>Reading lines 177-220 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L177-L221">type_mapping.rs</a></summary>

```
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
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01YaHiyXywBbspEMFnQuwjTW__vscode-1771312785920/content.txt#1-1), lines 1 to 500

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01YaHiyXywBbspEMFnQuwjTW__vscode-1771312785920/content.txt#501-501), lines 501 to 501

<details>
<summary>Reading lines 598-780 of <a href="../crates/opencascade-binding-generator/src/model.rs#L598-L781">model.rs</a></summary>

```
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
                    Type::Class(name) if name != "char" => Some(name.as_str()),
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
    /// Ne
... (truncated, 8525 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01LkGoevnwggbaDJE865LVft__vscode-1771312785924/content.txt)

*2026-02-20 08:06 · 369s · Words: 8,034 in · 1,712 out · 35,052 ctx · 20 rounds*

---

<a id="user-4"></a>

## User (4)

> Why is signed 16-bit int being treated the same as unsigned 16-bit int? Can&#x27;t we tell the difference and have Type::I16 and Type::U16? I see we already have that for I32 vs U32. Why not just follow the same codepaths and add I16?

*2026-02-20 08:07*

### Assistant — Claude Opus 4.6

> 💭 The user is asking about why there's no `Type::I16` variant for signed 16-bit integers, when there's already `Type::I32` and `Type::U32` differentiation for 32-bit integers. Let me investigate the codebase to understand the current state.

**Todo list:**
- 🔄 Research current I16/U16 handling
- ⬜ Add Type::I16 variant to model.rs
- ⬜ Update parser to detect signed i16
- ⬜ Update all codepaths for I16
- ⬜ Build and test

Let me search for how U16 and I16 are currently handled across the codebase.

<details>
<summary>Searched for regex <code>U16|I16|u16|i16|int16|short</code> (<code>**/crates/opencascade-binding-generator/src/**/*.rs</code>), 20 results</summary>

- [bindings.rs:35](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L35)
- [bindings.rs:233](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L233)
- [bindings.rs:234](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L234)
- [bindings.rs:391](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L391)
- [bindings.rs:397](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L397)
- [bindings.rs:398](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L398)
- [bindings.rs:421](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L421)
- [bindings.rs:431](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L431)
- [bindings.rs:602](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L602)
- [bindings.rs:645](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L645)
- [bindings.rs:653](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L653)
- [bindings.rs:655](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L655)
- [bindings.rs:657](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L657)
- [bindings.rs:666](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L666)
- [bindings.rs:673](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L673)
- [bindings.rs:674](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L674)
</details>

<details>
<summary>Reading lines 440-500 of <a href="../crates/opencascade-binding-generator/src/model.rs#L440-L501">model.rs</a></summary>

```

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
    /// unsigned short / uint16_t / Standard_ExtCharacter
    U16,
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
            Type::U16 => "u16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.short_name(),
```
</details>

<details>
<summary>Searched for regex <code>U16|I16|short|Standard_ShortReal|int16</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 16 results</summary>

- [parser.rs:38](../crates/opencascade-binding-generator/src/parser.rs#L38)
- [parser.rs:40](../crates/opencascade-binding-generator/src/parser.rs#L40)
- [parser.rs:843](../crates/opencascade-binding-generator/src/parser.rs#L843)
- [parser.rs:1350](../crates/opencascade-binding-generator/src/parser.rs#L1350)
- [parser.rs:1381](../crates/opencascade-binding-generator/src/parser.rs#L1381)
- [parser.rs:1588](../crates/opencascade-binding-generator/src/parser.rs#L1588)
- [parser.rs:1591](../crates/opencascade-binding-generator/src/parser.rs#L1591)
- [parser.rs:1592](../crates/opencascade-binding-generator/src/parser.rs#L1592)
- [parser.rs:1602](../crates/opencascade-binding-generator/src/parser.rs#L1602)
- [parser.rs:1603](../crates/opencascade-binding-generator/src/parser.rs#L1603)
</details>

Found it. In `crates/opencascade-binding-generator/src/parser.rs#L1602`,  is mapped to  with the comment "i16 isn't available, use i32". And at `crates/opencascade-binding-generator/src/parser.rs#L1350` is handled together with . Let me look at the full parser context and then trace all  usage to know exactly what to add for `I16`.

<details>
<summary>Reading lines 1340-1395 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1340-L1396">parser.rs</a></summary>

```
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int),
            // not an NCollection template misresolution.
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                        TypeKind::SChar | TypeKind::UChar |
                        TypeKind::Short | TypeKind::UShort |
                        TypeKind::Int | TypeKind::UInt |
                        TypeKind::Long | TypeKind::ULong |
                        TypeKind::LongLong | TypeKind::ULongLong |
                        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble |
                        TypeKind::Typedef  // chain through another typedef (e.g., Standard_Integer)
                    ))
                    .unwrap_or(false);
            !is_primitive_typedef
        }
    };

    // Handle primitives via canonical type.
    // Skip this if:
    // 1. The spelling clearly identifies a class type (spelling_looks_like_class), OR
    // 2. The spelling contains '<' or '::' — template or namespace-scoped types
    //    whose canonical resolves to int/double/etc. are template misresolutions,
    //    not genuine primitives.
    let spelling_is_template_or_namespaced = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("volatile ")
            .trim();
        s.contains('<') || s.contains("::")
    };
    if !spelling_looks_like_class && !spelling_is_template_or_namespaced {

        match canonical_clean {
            "bool" => return Type::Bool,
            "int" => return Type::I32,
            "unsigned int" => return Type::U32,
            "unsigned short" | "uint16_t" | "char16_t" => return Type::U16,
            "long" => return Type::Long,
            "unsigned long" => return Type::ULong,
            "long long" => return Type::I64,
            "unsigned long long" => return Type::U64,
            "float" => return Type::F32,
            "double" => return Type::F64,
            _ => {}
        }
    }

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int" (template misresolution), construct the class type
    // directly instead of recursing into the pointee (whose display name might
    // already be "int", losing the typedef info).
```
</details>

<details>
<summary>Reading lines 1580-1615 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1580-L1616">parser.rs</a></summary>

```

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I32),  // i16 isn't available, use i32
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

```
</details>

**Todo list:**
- ✅ Research current I16/U16 handling
- 🔄 Add Type::I16 variant to model.rs
- ⬜ Update parser to detect signed i16
- ⬜ Update all codepaths for I16
- ⬜ Build and test

Now I have the full picture. Let me find every place  is referenced so I can add `I16` alongside it in all match arms.

<details>
<summary>Searched for regex <code>Type::U16</code> (<code>**/crates/opencascade-binding-generator/src/**/*.rs</code>), 18 results</summary>

- [bindings.rs:431](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L431)
- [bindings.rs:602](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L602)
- [bindings.rs:686](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L686)
- [bindings.rs:2079](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2079)
- [bindings.rs:2143](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2143)
- [type_mapping.rs:50](../crates/opencascade-binding-generator/src/type_mapping.rs#L50)
- [type_mapping.rs:499](../crates/opencascade-binding-generator/src/type_mapping.rs#L499)
- [resolver.rs:1382](../crates/opencascade-binding-generator/src/resolver.rs#L1382)
- [model.rs:492](../crates/opencascade-binding-generator/src/model.rs#L492)
- [model.rs:515](../crates/opencascade-binding-generator/src/model.rs#L515)
- [model.rs:531](../crates/opencascade-binding-generator/src/model.rs#L531)
- [model.rs:707](../crates/opencascade-binding-generator/src/model.rs#L707)
- [model.rs:750](../crates/opencascade-binding-generator/src/model.rs#L750)
- [model.rs:818](../crates/opencascade-binding-generator/src/model.rs#L818)
- [parser.rs:1381](../crates/opencascade-binding-generator/src/parser.rs#L1381)
- [parser.rs:1591](../crates/opencascade-binding-generator/src/parser.rs#L1591)
- [parser.rs:1592](../crates/opencascade-binding-generator/src/parser.rs#L1592)
- [parser.rs:1603](../crates/opencascade-binding-generator/src/parser.rs#L1603)
</details>

Now let me read each context to plan exact edits. Let me read the relevant lines in each file:Let me read the specific contexts needed for all edits:

<details>
<summary>Reading lines 448-455 of <a href="../crates/opencascade-binding-generator/src/model.rs#L448-L456">model.rs</a></summary>

```
    /// int / Standard_Integer
    I32,
    /// unsigned int
    U32,
    /// unsigned short / uint16_t / Standard_ExtCharacter
    U16,
    /// long long / int64_t
    I64,
```
</details>

<details>
<summary>Reading lines 488-495 of <a href="../crates/opencascade-binding-generator/src/model.rs#L488-L496">model.rs</a></summary>

```
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "uint".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
```
</details>

<details>
<summary>Reading lines 510-535 of <a href="../crates/opencascade-binding-generator/src/model.rs#L510-L536">model.rs</a></summary>

```
            self,
            Type::Void
                | Type::Bool
                | Type::I32
                | Type::U32
                | Type::U16
                | Type::I64
                | Type::U64
                | Type::Long
                | Type::ULong
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    /// Check if this type is suitable as a field in a POD struct.
    /// Only primitive numeric types (bool, integers, floats) are POD-safe.
    pub fn is_pod_field_type(&self) -> bool {
        matches!(
            self,
            Type::Bool | Type::I32 | Type::U32 | Type::U16 | Type::I64 | Type::U64
                | Type::Long | Type::ULong | Type::Usize | Type::F32 | Type::F64
        )
    }

```
</details>

<details>
<summary>Reading lines 703-712 of <a href="../crates/opencascade-binding-generator/src/model.rs#L703-L713">model.rs</a></summary>

```
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I64 => "long long".to_string(),
            Type::U64 => "unsigned long long".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
```
</details>

<details>
<summary>Reading lines 746-755 of <a href="../crates/opencascade-binding-generator/src/model.rs#L746-L756">model.rs</a></summary>

```
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Long => "std::ffi::c_long".to_string(),
            Type::ULong => "std::ffi::c_ulong".to_string(),
            Type::Usize => "usize".to_string(),
```
</details>

<details>
<summary>Reading lines 814-822 of <a href="../crates/opencascade-binding-generator/src/model.rs#L814-L823">model.rs</a></summary>

```
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Long => "std::ffi::c_long".to_string(),
            Type::ULong => "std::ffi::c_ulong".to_string(),
```
</details>

Now let me read the `crates/opencascade-binding-generator/src/codegen/bindings.rs` and `crates/opencascade-binding-generator/src/type_mapping.rs``crates/opencascade-binding-generator/src/resolver.rs` contexts:

<details>
<summary>Reading lines 427-435 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L427-L436">bindings.rs</a></summary>

```
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
```
</details>

<details>
<summary>Reading lines 598-606 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L598-L607">bindings.rs</a></summary>

```
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
```
</details>

<details>
<summary>Reading lines 682-690 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L682-L691">bindings.rs</a></summary>

```
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U16 => "u16".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U64 => "u64".to_string(),
        Type::Long => "std::ffi::c_long".to_string(),
        Type::ULong => "std::ffi::c_ulong".to_string(),
```
</details>

<details>
<summary>Reading lines 2074-2085 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2074-L2086">bindings.rs</a></summary>

```
fn pod_field_rust_type(ty: &Type) -> Option<&'static str> {
    match ty {
        Type::Bool => Some("bool"),
        Type::I32 => Some("i32"),
        Type::U32 => Some("u32"),
        Type::U16 => Some("u16"),
        Type::I64 => Some("i64"),
        Type::U64 => Some("u64"),
        Type::Long => Some("std::os::raw::c_long"),
        Type::ULong => Some("std::os::raw::c_ulong"),
        Type::Usize => Some("usize"),
        Type::F32 => Some("f32"),
```
</details>

<details>
<summary>Reading lines 2139-2148 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2139-L2149">bindings.rs</a></summary>

```
            } else {
                None
            }
        }
        Type::I32 | Type::U32 | Type::U16 | Type::I64 | Type::U64 | Type::Long | Type::ULong | Type::Usize => {
            // Integer literals should work directly
            if default_expr.parse::<i64>().is_ok() || default_expr.parse::<u64>().is_ok() {
                Some(default_expr.to_string())
            } else {
                None
```
</details>

<details>
<summary>Reading lines 46-55 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L46-L56">type_mapping.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 494-504 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L494-L505">type_mapping.rs</a></summary>

```
    match cpp_type {
        "void" => return map_type_to_rust(&Type::Void),
        "bool" | "Standard_Boolean" => return map_type_to_rust(&Type::Bool),
        "int" | "Standard_Integer" => return map_type_to_rust(&Type::I32),
        "unsigned int" => return map_type_to_rust(&Type::U32),
        "unsigned short" | "uint16_t" | "char16_t" | "Standard_ExtCharacter" => return map_type_to_rust(&Type::U16),
        "long" => return map_type_to_rust(&Type::Long),
        "unsigned long" => return map_type_to_rust(&Type::ULong),
        "float" => return map_type_to_rust(&Type::F32),
        "double" | "Standard_Real" => return map_type_to_rust(&Type::F64),
        _ => {}
```
</details>

<details>
<summary>Reading lines 1378-1386 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1378-L1387">resolver.rs</a></summary>

```
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
```
</details>

Now I have the full picture. Let me also check the  handling in the canonical match at `crates/opencascade-binding-generator/src/parser.rs` line 1377:Now I'll apply all the changes across all files. The changes needed are:

1. **`crates/opencascade-binding-generator/src/model.rs`**: Add `I16` variant, and add it to every match arm where  appears
2. **`crates/opencascade-binding-generator/src/parser.rs`**: Map  to  instead of , add canonical match for 
3. **`crates/opencascade-binding-generator/src/codegen/bindings.rs`**: Add `I16` to all match arms alongside 
4. **`crates/opencascade-binding-generator/src/type_mapping.rs`**: Add `I16` mapping
5. **`crates/opencascade-binding-generator/src/resolver.rs`**: Add `I16` to the C++ type string match

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L452-L456">model.rs -2+4</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L494-L496">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L518-L520">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L535-L536">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L711-L713">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L755-L757">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L824-L826">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "452:fg,453:za",
      "content": "    /// unsigned short / uint16_t / Standard_ExtCharacter\n    U16,\n    /// short / int16_t\n    I16,"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "492:ke",
      "content": "            Type::U16 => \"u16\".to_string(),\n            Type::I16 => \"i16\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "515:ld",
      "content": "                | Type::U16\n                | Type::I16"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "531:pm",
      "content": "            Type::Bool | Type::I32 | Type::U32 | Type::U16 | Type::I16 | Type::I64 | Type::U64"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "707:mp",
      "content": "            Type::U16 => \"uint16_t\".to_string(),\n            Type::I16 => \"int16_t\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "750:ke",
      "content": "            Type::U16 => \"u16\".to_string(),\n            Type::I16 => \"i16\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "818:ke",
      "content": "            Type::U16 => \"u16\".to_string(),\n            Type::I16 => \"i16\".to_string(),"
    }
  ]
}
```
**Output:**
```
{
  "applied": 7,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 452,
      "endLine": 825
    }
  ]
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1381-L1383">parser.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/parser.rs#L1603-L1606">-2+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            \"unsigned short\" | \"uint16_t\" | \"char16_t\" => return Type::U16,\n            \"short\" | \"int16_t\" => return Type::I16,",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1381:vl"
    },
    {
      "content": "        \"short\" => Some(Type::I16),\n        \"int16_t\" => Some(Type::I16),\n        \"unsigned short\" | \"uint16_t\" | \"char16_t\" => Some(Type::U16),",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1602:ux,1603:wa"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 1381,
      "endLine": 1605
    }
  ]
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L431-L433">bindings.rs -1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L603-L605">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L688-L690">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2082-L2084">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2147-L2148">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::U16 => \"u16\".to_string(),\n        Type::I16 => \"i16\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "431:or"
    },
    {
      "content": "        Type::U16 => \"uint16_t\".to_string(),\n        Type::I16 => \"int16_t\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "602:wa"
    },
    {
      "content": "        Type::U16 => \"u16\".to_string(),\n        Type::I16 => \"i16\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "686:or"
    },
    {
      "content": "        Type::U16 => Some(\"u16\"),\n        Type::I16 => Some(\"i16\"),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2079:bp"
    },
    {
      "content": "        Type::I32 | Type::U32 | Type::U16 | Type::I16 | Type::I64 | Type::U64 | Type::Long | Type::ULong | Type::Usize => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2143:mm"
    }
  ]
}
```
**Output:**
```
{
  "applied": 5,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 431,
      "endLine": 2147
    }
  ]
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L50-L62">type_mapping.rs -6+12</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L505-L507">-1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::U16 => RustTypeMapping {\n            rust_type: \"u16\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },\n        Type::I16 => RustTypeMapping {\n            rust_type: \"i16\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "50:ns,51:iu,52:sp,53:nu,54:ty,55:or"
    },
    {
      "content": "        \"unsigned short\" | \"uint16_t\" | \"char16_t\" | \"Standard_ExtCharacter\" => return map_type_to_rust(&Type::U16),\n        \"short\" | \"int16_t\" => return map_type_to_rust(&Type::I16),",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "499:ui"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 50,
      "endLine": 506
    }
  ]
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1382-L1384">resolver.rs -1+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::U16 => \"uint16_t\".to_string(),\n        Type::I16 => \"int16_t\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "1382:wa"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":1382,"endLine":1383}
```
</details>

**Todo list:**
- ✅ Research current I16/U16 handling
- ✅ Add Type::I16 variant to model.rs
- ✅ Update parser to detect signed i16
- ✅ Update all codepaths for I16
- 🔄 Build and test

Now let me build to check for any compilation errors (e.g. non-exhaustive match arms I missed):

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on package cache
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.70s
real  5.8s
```

Clean build. Now let me regenerate bindings and do a full build + test:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (518 lines) — exit code 101</summary>

```
warning: opencascade-sys@0.2.0:   357 |                                   const TCollection_ExtendedString& theString2)
warning: opencascade-sys@0.2.0:       |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'
warning: opencascade-sys@0.2.0:  332184 |     return self_->IsDifferent(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:195:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   195 |   Standard_EXPORT Standard_Boolean IsDifferent(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:202:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection_ExtendedString' for 1st argument
warning: opencascade-sys@0.2.0:   202 |   Standard_EXPORT Standard_Boolean IsDifferent(const TCollection_ExtendedString& other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'
warning: opencascade-sys@0.2.0:  332190 |     return self_->IsLess(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:210:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   210 |   Standard_EXPORT Standard_Boolean IsLess(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:215:36: note: candidate function not viable: no kn
... (truncated, 62419 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt#1-1), lines 1 to 5

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt
```
```

real  19.4s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -E 'error|Finished' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt
```
<details>
<summary>Output (38 lines)</summary>

```
ade-sys/generated/wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'                                                          warning: opencascade-sys@0.2.0:  332184 |     return self_->IsDifferent(other);
ade-sys/generated/wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'                                                               warning: opencascade-sys@0.2.0:  332190 |     return self_->IsLess(other);
ade-sys/generated/wrappers.cpp:332196:19: error: no matching member function for call to 'IsGreater'                                                            warning: opencascade-sys@0.2.0:  332196 |     return self_->IsGreater(other);
ade-sys/generated/wrappers.cpp:332235:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332235 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:332504:16: error: no matching constructor for initialization of 'TCollection_HExtendedString'                                    warning: opencascade-sys@0.2.0:  332504 |     return new TCollection_HExtendedSt
ade-sys/generated/wrappers.cpp:332528:78: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  332528 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:332579:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332579 |     return self_->ToExtString();
ade-sys/generated/wrappers.cpp:381448:72: error: no matching member function for call to 'ToHString'                                                            warning: opencascade-sys@0.2.0:  381448 |     return new opencascade::handle<TCo
ade-sys/generated/wrappers.cpp:381451:60: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  381451 |     return new TCollection_ExtendedStr
ade-sys/generated/wrappers.cpp:381490:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  381490 |     return self_->AsciiToExtended(str)
ade-sys/generated/wrappers.cpp:381508:35: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short
... (truncated, 10388 chars total)
```
</details>

**Terminal:**
```sh
grep 'Finished\|^error' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
grep -i 'finished\|error\[' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt
```
**Exit code:** 1

The output is from a build script and gets mangled. Let me just read the last ~20 lines:

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt#500-500), lines 500 to 523

> 💭 There are C++ compilation errors. The issue is that `Standard_ExtString` is `const char16_t*` but we're generating `const uint16_t*` in the C++ wrappers. The problem is that `char16_t` and `uint16_t` are not the same type in C++ (even though they're the same size).
> 
> Let me look at the specific errors. The wrapper generates functions returning `const uint16_t*` (from `Type::ConstPtr(Type::U16)` → `type_to_cpp` → `"const uint16_t*"`), but the OCCT method returns `Standard_ExtString` which is `const char16_t*`.
> 
> 
> The fix is to change the type mapping so that `Type::U16` generates `char16_t` instead of `uint16_t` in the C++ wrapper code, since that's what the OCCT library actually expects. the actual C++ method returns `const char16_t*`.
> 
> Wait - `char16_t` and `uint16_t` are different types in C++. The parser maps `char16_t` → `Type::U16`, but then `type_to_cpp` maps `Type::U16` → `"uint16_t"`, which loses the original distinction.
> 
> The fix is to update `type_to_cpp` to output `Standard_ExtCharacter` (OCCT's typedef for `char16_t`) instead of `uint16_t` for the `U16` type. This is actually a pre-existing issue that got exposed when the I16 changes unblocked more bindings. Let me trace through the full error messages to understand what's happening. _ExtCharacter"` in type_to_cpp, but that conflates `char16_t`, `uint16_t`, and `unsigned short` which all map to `Type::U16`. The real problem is figuring out which C++ type `type_to_cpp` should actually output. These `Standard_ExtString` errors seem like they should have existed before my changes since I only added `I16` for `short` — unless that somehow made new methods bindable that weren't before.
> 
> Let me trace through this more carefully. The errors are about `Standard_ExtString` (`const char16_t*`) conflicting with `const uint16_t*`, which suggests these are newly exposed bindings. My change from `"short" => Some(Type::I32)` to `"short" => Some(Type::I16)` shouldn't affect bindability since both are primitives that always pass the filter, so something else must be going on here.
> 
> I think the most direct fix is to update the C++ type mapping for `U16` to use `char16_t` instead of `uint16_t`, since that's what OCCT actually uses for 16-bit characters. The Rust FFI side doesn't care either way since it's still `u16`. But first I need to verify whether these errors existed before my changes by checking the git history.

Build has C++ compilation errors - `const uint16_t*` vs `const char16_t*` mismatch. Let me investigate:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat HEAD crates/opencascade-sys/generated/ 2>&1 | tail -5
```
<details>
<summary>Output (5 lines)</summary>

```
 .../opencascade-sys/generated/top_ope_b_rep_ds.rs  |    4 +-
 crates/opencascade-sys/generated/vrml_data.rs      |  327 ++++--
 crates/opencascade-sys/generated/wrappers.cpp      |  406 ++++++-
 crates/opencascade-sys/generated/xs_control.rs     |  134 ++-
 38 files changed, 3720 insertions(+), 1122 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c 'int16_t' crates/opencascade-sys/generated/wrappers.cpp
```
```
36
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'error:' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014qkQjaS5Vg4ywA7fVDezUV__vscode-1771312785956/content.txt
```
<details>
<summary>Output (35 lines)</summary>

```
6:ade-sys/generated/wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'                                                          warning: opencascade-sys@0.2.0:  332184 |     return self_->IsDifferent(other);
15:ade-sys/generated/wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'                                                               warning: opencascade-sys@0.2.0:  332190 |     return self_->IsLess(other);
24:ade-sys/generated/wrappers.cpp:332196:19: error: no matching member function for call to 'IsGreater'                                                            warning: opencascade-sys@0.2.0:  332196 |     return self_->IsGreater(other);
33:ade-sys/generated/wrappers.cpp:332235:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332235 |     return self_->ToExtString();
36:ade-sys/generated/wrappers.cpp:332504:16: error: no matching constructor for initialization of 'TCollection_HExtendedString'                                    warning: opencascade-sys@0.2.0:  332504 |     return new TCollection_HExtendedSt
74:ade-sys/generated/wrappers.cpp:332528:78: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  332528 |     return new opencascade::handle<TCo
80:ade-sys/generated/wrappers.cpp:332579:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  332579 |     return self_->ToExtString();
83:ade-sys/generated/wrappers.cpp:381448:72: error: no matching member function for call to 'ToHString'                                                            warning: opencascade-sys@0.2.0:  381448 |     return new opencascade::handle<TCo
94:ade-sys/generated/wrappers.cpp:381451:60: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                   warning: opencascade-sys@0.2.0:  381451 |     return new TCollection_ExtendedStr
100:ade-sys/generated/wrappers.cpp:381490:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                 warning: opencascade-sys@0.2.0:  381490 |     return self_->AsciiToExtended(str)
103:ade-sys/generated/wrappers.cpp:381508:35: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint1
... (truncated, 10184 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Car81J5P11e8dL1Dy2kMqd__vscode-1771312785965/content.txt)

> 💭 The core issue is clear: in C++, `char16_t` and `uint16_t` (aka `unsigned short`) are distinct types. When the parser sees `Standard_ExtCharacter` (which is `char16_t`) and `Standard_ExtString` (which is `const char16_t*`), it maps them to `Type::U16`. But then the C++ code generator maps `Type::U16` to `uint16_t`, which is incompatible with `char16_t`.
> 
> There are two categories of errors:
> 1. **char16_t/uint16_t mismatch**: `Standard_ExtString` (`const char16_t*`) vs `const uint16_t*` 
> 
> 
> 2. **Method overload issues**: Methods like `IsDifferent`, `IsLess`, `IsGreater`, and `IsEqual` expect `Standard_ExtString` parameters but receive `const uint16_t*` instead, plus constructor incompatibilities in `TCollection_ExtendedString` and `Standard_GUID`.
> 
> These errors seem pre-existing since the `U16` mapping was already in place, but I need to check whether these methods were previously being excluded from generation.
> 
> Actually, these are coming through now because we made raw pointers bindable in the previous session by removing `is_raw_ptr()` from `is_unbindable()`. The real problem is that the C++ type mapping outputs `uint16_t` when OCCT's API actually expects `char16_t`. I should update the type mapping functions to use `char16_t` for `Type::U16` instead.
> 
> But I'm realizing `char16_t` and `unsigned short` aren't implicitly convertible in C++11, so I need to be careful about which OCCT types actually use `Standard_ExtCharacter` versus raw `unsigned short`.
> 
> The pragmatic approach is to map `Type::U16` directly to `Standard_ExtCharacter` since that's what OCCT primarily uses, and both it and `uint16_t` are fundamentally 16-bit unsigned integers anyway. I'm realizing I need a separate `Type::Char16` variant to properly handle `char16_t` and `Standard_ExtCharacter`, since they're distinct types in C++ even though they're ABI-compatible with `uint16_t`. The current approach of mapping everything to U16 creates incompatibilities when OCCT methods expect `const char16_t*` but receive `const uint16_t*`. is `char16_t`. Are there any OCCT methods that take `unsigned short` parameters? Let me check...
> 
> Looking at the parser mappings:
> - `Standard_ExtCharacter` → `U16` (this is `char16_t`)
> - `unsigned short`/`uint16_t`/`char16_t` from canonical type resolution → `U16`
> 
> The fact that all three canonical forms map to U16 means the parser can't distinguish them. But in type_to_cpp, we always emit `uint16_t`.
> 
> The simplest clean fix: since `Standard_ExtCharacter` = `char16_t`, and most OCCT APIs use `Standard_ExtCharacter`, change type_to_cpp to emit `char16_t` for U16. If there happen to be actual `unsigned short` params that need `unsigned short` in the wrapper, they'll still be compatible with `char16_t` at the binary level, though C++ won't allow implicit conversion.
> 
> Let me think about this more pragmatically. The errors are all about `const char16_t*` (`Standard_ExtString`) vs `const uint16_t*`. If I switch U16's C++ representation to `char16_t`, then `Standard_ExtString` and `Standard_ExtCharacter` will work correctly, and any `unsigned short` uses will either work directly or need a cast—which is acceptable since OCCT's primary 16-bit types are `Standard_ExtCharacter` and `Standard_ExtString`. I'm reconsidering whether adding a full `Type::Char16` variant is worth the complexity—let me check if OCCT actually uses both `char16_t` and `unsigned short`/`uint16_t` as distinct parameter types, or if `Standard_ExtCharacter` is the only place this matters. The simpler fix might be to just map `Standard_ExtCharacter` directly to the C++ type `char16_t` without creating a new type variant.

The errors are from `char16_t` vs `uint16_t` mismatch in C++ wrappers. These are pre-existing issues from the raw pointer work in the prior session. Let me check what OCCT defines `Standard_ExtCharacter` as:

**Terminal:**
```sh
grep -n 'Standard_ExtCharacter\|Standard_ExtString' /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx
```
```
79:typedef uint16_t Standard_ExtCharacter;
83:typedef char16_t Standard_ExtCharacter;
93:typedef const Standard_ExtCharacter* Standard_ExtString;
```

**Terminal:**
```sh
grep -B2 -A2 'Standard_ExtCharacter' /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx | head -20
```
<details>
<summary>Output (15 lines)</summary>

```
     || (defined(_MSC_VER) && (_MSC_VER < 1600)))
// compatibility with old GCC and MSVC compilers
typedef uint16_t Standard_ExtCharacter;
typedef uint16_t Standard_Utf16Char;
typedef uint32_t Standard_Utf32Char;
#else
typedef char16_t Standard_ExtCharacter;
typedef char16_t Standard_Utf16Char; //!< UTF-16 char (always unsigned)
typedef char32_t Standard_Utf32Char; //!< UTF-32 char (always unsigned)
--
//
typedef const Standard_Character*    Standard_CString;
typedef const Standard_ExtCharacter* Standard_ExtString;

#endif // _Standard_TypeDef_HeaderFile
```
</details>

> 💭 So `Standard_ExtCharacter` is either `uint16_t` (old compilers) or `char16_t` (modern compilers). On the macOS with modern clang, it's `char16_t`. So both `uint16_t` and `char16_t` are 16-bit unsigned, but they're distinct C++ types.
> 
> The cleanest fix: Use `Standard_ExtCharacter` in the C++ wrapper code for Type::U16. This is correct regardless of the platform's typedef. Wait, but not all U16 types come from Standard_ExtCharacter. If some OCCT method takes an actual `unsigned short` parameter (not through the typedef), then `Standard_ExtCharacter` (which is `char16_t`) would be wrong.
>  
> For the extern "C" interface, what matters is ABI compatibility—the wrapper function signature and the actual OCCT call just need to be compatible at the binary level. The real problem is that the C++ compiler sees these as different types even though they're the same size.
> 
> The solution is to emit `Standard_ExtCharacter` for `Type::U16` in the C++ wrapper instead of `uint16_t`. This works because `Standard_ExtCharacter` is defined as `char16_t` on modern compilers and `uint16_t` on older ones, so it'll always match what OCCT's API expects—and this makes sense since `Type::U16` was specifically designed to represent `Standard_ExtCharacter` in the first place.
> 
> But I'm realizing there's a deeper conflict here: if we use `Standard_ExtCharacter` for methods that actually expect plain `unsigned short`, we'd be creating a type mismatch since `char16_t` and `unsigned short` aren't the same thing. The real problem is that we need to distinguish between OCCT methods that use `Standard_ExtCharacter` versus non-OCCT methods that use `unsigned short`.
> 
> Actually, since we're wrapping OCCT specifically, all the U16 types we encounter come from `Standard_ExtCharacter` anyway, so mapping U16 to `Standard_ExtCharacter` in the type mappings is the right call.

`Standard_ExtCharacter` is `char16_t` on modern compilers (not `uint16_t`). The fix is to use `Standard_ExtCharacter` in C++ wrapper code for , since that's what OCCT actually uses. Let me update the C++ type mappings:

<details>
<summary>Reading lines 598-606 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L598-L607">bindings.rs</a></summary>

```
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
```
</details>

<details>
<summary>Reading lines 703-712 of <a href="../crates/opencascade-binding-generator/src/model.rs#L703-L713">model.rs</a></summary>

```

    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I16 => "int16_t".to_string(),
```
</details>

<details>
<summary>Reading lines 1378-1388 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1378-L1389">resolver.rs</a></summary>

```
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L603-L604">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1382-L1383">resolver.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::U16 => \"Standard_ExtCharacter\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "603:wa"
    },
    {
      "content": "        Type::U16 => \"Standard_ExtCharacter\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "1382:wa"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 603,
      "endLine": 603
    },
    {
      "file": "crates/opencascade-binding-generator/src/resolver.rs",
      "startLine": 1382,
      "endLine": 1382
    }
  ]
}
```
</details>

Now rebuild the binding generator, regenerate, and try again:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.15s
real  4.2s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (312 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 7.99s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 73 modules via exclude_modules
Loaded config: 1 module patterns, 75 exclude module patterns, 0 individual headers, 10 header exclusions -> 5761 headers
  Excluded 35 headers after dependency resolution
Parsing 5761 headers...
  Clang parse time: 2.94s
  Collected 3641 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan

Timing summary:
  Total clang parse time: 2.94s
  Total AST visit time: 3.55s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (4400 classes, 1237 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 11 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 35 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs (26 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/moni_tool.rs (14 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/interface.rs (49 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/resource.rs (4 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_data.rs (37 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/header_section.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_graph.rs (11 types, 0 extra)
... (truncated, 32314 chars total)
```
</details>

*2026-02-20 08:15 · 487s · Words: 6,082 in · 2,280 out · 69,273 ctx · 33 rounds*

---

<a id="user-5"></a>

## User (5)

> Figure out where the parser is converting Standard_ExtCharacter to U16, and figure out how to have it convert to I16. Revert any changes that are emitting U16 as Standard_ExtCharacter, and instead ensure that I16 gets emitted as int16_t or equivalent 16-bit signed C type. Then continue.

> **⚠️ This request was canceled.**

*2026-02-20 08:17*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to:
> 
> 1. Figure out where `Standard_ExtCharacter` is being parsed as `U16` - and determine if it should be `I16` instead
> 2. Revert the changes that emit `U16` as `Standard_ExtCharacter` in C++ code (I changed `type_to_cpp` in bindings.rs and resolver.rs from `"uint16_t"` to `"Standard_ExtCharacter"`)
> 3. Instead, ensure `I16` gets emitted as `int16_t` (which it already does from my earlier changes)
> 4. Continue with the build
> 
> Let me think about this. The issue is:
> 
> 
> - `Standard_ExtCharacter` is defined as `char16_t` on modern compilers, not `uint16_t`
> - `U16` was being emitted as `uint16_t` in C++ wrappers, but OCCT functions return `Standard_ExtString` which is `const char16_t*`
> - The mismatch is `uint16_t` vs `char16_t` - they're different types in C++
> 
> The real fix should be: `Standard_ExtCharacter` should map to a different type entirely, or `U16` should emit `char16_t` in C++, or we need a `Char16` type.
> 
> But the user is saying to revert those changes and just have `I16` emit as `int16_t` instead. The `Standard_ExtCharacter`/`char16_t` issue is separate from the `short`/`int16_t` issue. Looking at this more clearly: `Standard_ExtCharacter` is `char16_t` (unsigned 16-bit character), `short` is signed 16-bit integer, and `unsigned short` is `uint16_t`. The user's original question was about `short` being treated as `I32` instead of having its own `I16` type, which is what needs fixing.
> 
> The real problem is that I changed the C++ emission from `uint16_t` to `Standard_ExtCharacter`, which created the char16_t mismatch. So I need to revert `Type::U16` back to `"uint16_t"` in both bindings.rs and resolver.rs, while keeping the `I16 => "int16_t"` mappings as they are. The build errors with `char16_t` vs `uint16_t` were already present before my changes—they're a separate issue that appeared after regenerating with the new I16 changes.
> 
> Looking at the actual errors more carefully, they're about `Standard_ExtString` (which is `const char16_t*`), but the wrapper is generating `const uint16_t*` for the return type. These aren't compatible in C++, so I need to either emit `char16_t` instead of `uint16_t` for `U16` in the C++ output, or treat `Standard_ExtCharacter` as its own distinct type.
> 
> The user's suggestion is to revert those `Standard_ExtCharacter` changes and just emit `I16` as `int16_t` instead. Let me check what the current state of those lines looks like.

**Todo list:**
- 🔄 Research Standard_ExtCharacter parsing
- ⬜ Revert U16→Standard_ExtCharacter emission
- ⬜ Ensure I16 emits int16_t
- ⬜ Regenerate and build

Let me first understand the current state of the relevant code:

<details>
<summary>Reading lines 1588-1612 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1588-L1613">parser.rs</a></summary>

```
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I16),
        "int16_t" => Some(Type::I16),
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
```
</details>

<details>
<summary>Reading lines 35-45 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L35-L46">parser.rs</a></summary>

```
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_ExtCharacter", "uint16_t")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


```
</details>

<details>
<summary>Reading lines 598-610 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L598-L611">bindings.rs</a></summary>

```
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "Standard_ExtCharacter".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
```
</details>

<details>
<summary>Reading lines 1378-1390 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1378-L1391">resolver.rs</a></summary>

```
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "Standard_ExtCharacter".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "Standard_Real".to_string(),
```
</details>

**Todo list:**
- ✅ Research Standard_ExtCharacter parsing
- 🔄 Revert U16→Standard_ExtCharacter emission
- ⬜ Ensure I16 emits int16_t
- ⬜ Regenerate and build

The issue: `Standard_ExtCharacter` is `char16_t` on modern compilers, but `uint16_t` on old ones. These are distinct C++ types. The parser maps `Standard_ExtCharacter` → , and I changed the C++ emission to use `Standard_ExtCharacter` — which causes mismatches when the generated wrapper declares `uint16_t` params/returns but OCCT functions use `char16_t`.

The fix: revert the C++ emission back to `uint16_t` for . The `char16_t` ↔ `uint16_t` mismatch is a separate problem (Standard_ExtCharacter should probably be its own type eventually).

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L603-L604">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1382-L1383">resolver.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::U16 => \"uint16_t\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "603:gk"
    },
    {
      "content": "        Type::U16 => \"uint16_t\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "1382:gk"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 603,
      "endLine": 603
    },
    {
      "file": "crates/opencascade-binding-generator/src/resolver.rs",
      "startLine": 1382,
      "endLine": 1382
    }
  ]
}
```
</details>

**Todo list:**
- ✅ Research Standard_ExtCharacter parsing
- ✅ Revert U16→Standard_ExtCharacter emission
- 🔄 Verify I16 emits int16_t everywhere
- ⬜ Regenerate and build

Now let me verify I16 emits correctly across all files:

<details>
<summary>Searched for regex <code>Type::I16</code> (<code>**/crates/opencascade-binding-generator/src/**/*.rs</code>), 17 results</summary>

- [bindings.rs:432](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L432)
- [bindings.rs:604](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L604)
- [bindings.rs:689](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L689)
- [bindings.rs:2083](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2083)
- [bindings.rs:2147](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2147)
- [model.rs:495](../crates/opencascade-binding-generator/src/model.rs#L495)
- [model.rs:519](../crates/opencascade-binding-generator/src/model.rs#L519)
- [model.rs:535](../crates/opencascade-binding-generator/src/model.rs#L535)
- [model.rs:712](../crates/opencascade-binding-generator/src/model.rs#L712)
- [model.rs:756](../crates/opencascade-binding-generator/src/model.rs#L756)
- [model.rs:825](../crates/opencascade-binding-generator/src/model.rs#L825)
- [resolver.rs:1383](../crates/opencascade-binding-generator/src/resolver.rs#L1383)
- [parser.rs:1382](../crates/opencascade-binding-generator/src/parser.rs#L1382)
- [parser.rs:1603](../crates/opencascade-binding-generator/src/parser.rs#L1603)
- [parser.rs:1604](../crates/opencascade-binding-generator/src/parser.rs#L1604)
- [type_mapping.rs:56](../crates/opencascade-binding-generator/src/type_mapping.rs#L56)
- [type_mapping.rs:506](../crates/opencascade-binding-generator/src/type_mapping.rs#L506)
</details>

All  mappings are consistent: `i16` in Rust, `int16_t` in C++. Now let me build and regenerate:

**Todo list:**
- ✅ Research Standard_ExtCharacter parsing
- ✅ Revert U16→Standard_ExtCharacter emission
- ✅ Verify I16 emits int16_t everywhere
- 🔄 Regenerate and build

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (518 lines) — exit code 101</summary>

```
warning: opencascade-sys@0.2.0:   357 |                                   const TCollection_ExtendedString& theString2)
warning: opencascade-sys@0.2.0:       |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'
warning: opencascade-sys@0.2.0:  332184 |     return self_->IsDifferent(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:195:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   195 |   Standard_EXPORT Standard_Boolean IsDifferent(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:202:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection_ExtendedString' for 1st argument
warning: opencascade-sys@0.2.0:   202 |   Standard_EXPORT Standard_Boolean IsDifferent(const TCollection_ExtendedString& other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'
warning: opencascade-sys@0.2.0:  332190 |     return self_->IsLess(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:210:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   210 |   Standard_EXPORT Standard_Boolean IsLess(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:215:36: note: candidate function not viable: no kn
... (truncated, 62419 chars total)
```
</details>

**Terminal:**
```sh
grep -c 'error' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Epdbi8FmLejvXxaaaowfGn__vscode-1771312785987/content.txt && tail -10 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Epdbi8FmLejvXxaaaowfGn__vscode-1771312785987/content.txt
```
<details>
<summary>Output (8 lines)</summary>

```
38
  --- stderr


  error occurred in cc-rs: command did not execute successfully (status code exi
t status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                                           

real  17.5s
```
</details>

Still errors. Let me check what the errors are - probably the same `char16_t` vs `uint16_t` issue that existed before my changes:

**Terminal:**
```sh
grep 'error:' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Epdbi8FmLejvXxaaaowfGn__vscode-1771312785987/content.txt | grep -v 'too many\|ToolExec\|occurred\|failed to run' | sed 's/.*wrappers.cpp:/wrappers.cpp:/' | sort -u
```
<details>
<summary>Output (32 lines)</summary>

```
wrappers.cpp:206866:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of type 'Standard_ExtString' (aka 'const char16_t *')                                                   cargo:warning= 206866 |     return self_->ExtValue(aResourceName);
wrappers.cpp:206878:12: error: no matching member function for call to 'SetResource'                                                                            cargo:warning= 206878 |     self_->SetResource(aResourceName, aValue);
wrappers.cpp:220676:16: error: no matching constructor for initialization of 'Standard_GUID'                                                                    cargo:warning= 220676 |     return new Standard_GUID(aGuid);
wrappers.cpp:332124:16: error: no matching constructor for initialization of 'TCollection_ExtendedString'                                                       cargo:warning= 332124 |     return new TCollection_ExtendedString(astring);
wrappers.cpp:332151:56: error: cannot initialize a parameter of type 'Standard_ExtString' (aka 'const char16_t *') with an lvalue of type 'const uint16_t *' (aka 'const unsigned short *')                                                     cargo:warning= 332151 |     return new TCollection_ExtendedString(self_->Token
wrappers.cpp:332178:19: error: no matching member function for call to 'IsEqual'                                                                                cargo:warning= 332178 |     return self_->IsEqual(other);
wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'                                                                            cargo:warning= 332184 |     return self_->IsDifferent(other);
wrappers.cpp:332184:19: error: no matching member function for call to 'IsDifferent'                                                          warning: opencascade-sys@0.2.0:  332184 |     return self_->IsDifferent(other);
wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'                                                                                 cargo:warning= 332190 |     return self_->IsLess(other);
wrappers.cpp:332190:19: error: no matching member function for call to 'IsLess'                                                               warning: opencascade-sys@0.2.0:  332190 |     return self_->IsLess(other);
wrappers.cpp:332196:19: error: no matching member function for call to 'IsGreater'                                                                              cargo:warning= 332196 |     return self_->IsGreater(other);
wrappers.cpp:332196:19: error: no matching member function for call to 'IsGreater'                                                            warning: opencascade-sys@0.2.0:  332196 |     return self_->IsGreater(other);
wrappers.cpp:332235:12: error: cannot initialize return object of type 'const uint16_t *' (aka 'const unsigned short *') with an rvalue of
... (truncated, 8656 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01AqwhECvouzC4ss4eKyK68E__vscode-1771312785989/content.txt#1-1), lines 1 to 30

> 💭 These are all errors related to `uint16_t` vs `char16_t` (Standard_ExtCharacter) mismatch. The problem is that `Standard_ExtCharacter` is `char16_t` on this system, but `Type::U16` is being emitted as `uint16_t` in C++. These are different types in C++.
> 
> These errors were likely pre-existing from the raw pointer work that made previously-excluded functions bindable. The `short` → `I16` change I made shouldn't cause new instances of this - these are from `Standard_ExtCharacter` / `Standard

*2026-02-20 08:20 · 182s · Words: 4,731 in · 803 out · 14 rounds*

---

<a id="user-6"></a>

## User (6)

> OK, stop. What version of Standard_ExtCharacter is the parser seeing - the uint16_t one or the char16_t one? If it&#x27;s seeing the uint16_t one, I think the issue is that the built-in #defines in the binding generator&#x27;s run of libclang differ from the ones used in the actual build of the bindings. Investigate which version is being see, what pre-defined #defines are used in the binding generator, and how they match what&#x27;s used during a build.

*2026-02-20 08:30*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to investigate which version of `Standard_ExtCharacter` the parser (binding generator using libclang) sees vs what the actual C++ build sees. The issue is that on modern compilers, `Standard_ExtCharacter` is `char16_t`, but the parser might be seeing it as `uint16_t` if the preprocessor defines differ.
> 
> Let me trace through:
> 1. The parser uses libclang to parse headers
> 2. The actual build uses the system C++ compiler (clang++)
> 3. The conditional in Standard_TypeDef.hxx checks for old GCC (<= 4.3) or old MSVC (< 1600)
> 
> 
> 4. On modern macOS with clang, the `#else` branch should be taken → `char16_t`
> 
> But I found the real issue: the parser has a string replacement that explicitly converts `Standard_ExtCharacter` to `uint16_t` before libclang even processes the type. This happens in the preprocessing step, so regardless of what the actual type definition resolves to, the parser is forcing it to be `uint16_t` in its internal representation.
> 
> The problem is that the C++ compiler sees `char16_t` (the modern standard type), but the generated bindings use `uint16_t` because of this forced replacement. So there's a mismatch between what the wrapper functions expect and what OCCT actually provides.
> 
> I need to either remove this replacement entirely and let libclang resolve the typedef correctly, or change it to map to `char16_t` instead. But first I should check what compiler flags and defines are being passed to clang during binding generation to understand why this replacement was added in the first place.

**Todo list:**
- 🔄 Find parser's string replacement
- ⬜ Check binding generator clang args
- ⬜ Compare with build.rs clang args

First, the smoking gun — the parser has this explicit string replacement at line 40:

<details>
<summary>Reading lines 25-50 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L25-L51">parser.rs</a></summary>

```

/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_ExtCharacter", "uint16_t")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


/// Walk the AST to collect all typedef/using declarations that resolve to
/// template specializations (NCollection, math_VectorBase, etc.).
/// Populates the thread-local TYPEDEF_MAP.
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();
```
</details>

There it is — line 40 hard-codes `Standard_ExtCharacter` → `uint16_t`. But on modern compilers (which is what the actual build uses), it's actually `char16_t`. Now let me check what defines the binding generator uses when invoking libclang, and compare with the actual build:

Searched for regex `clang_args|parse_args|translation_unit|Index::new|clang::Clang|compile_commands|compiler_args` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 1 result
- [parser.rs:174](../crates/opencascade-binding-generator/src/parser.rs#L174)

<details>
<summary>Reading lines 155-230 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L155-L231">parser.rs</a></summary>

```
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() -> HashSet<String> {
    TYPEDEF_MAP.with(|m| m.borrow().values().cloned().collect())
}



/// Parse a collection of OCCT header files
/// 
/// Uses batch parsing: creates a synthetic source file that includes all headers,
/// parses once, then extracts entities from each target header. This is much faster
/// than parsing each header separately since OCCT headers have deep include chains.
pub fn parse_headers(
    headers: &[impl AsRef<Path>],
    include_dirs: &[impl AsRef<Path>],
    verbose: bool,
) -> Result<Vec<ParsedHeader>> {
    let clang =
        Clang::new().map_err(|e| anyhow::anyhow!("Failed to initialize libclang: {}", e))?;
    let index = Index::new(&clang, false, true);

    // Build canonical path set for target headers
    let header_paths: Vec<std::path::PathBuf> = headers
        .iter()
        .map(|h| {
            let path = h.as_ref();
            // Try to resolve relative paths using include directories
            if path.is_relative() {
                for inc_dir in include_dirs {
                    let full_path = inc_dir.as_ref().join(path);
                    if let Ok(canonical) = full_path.canonicalize() {
                        return canonical;
                    }
                }
            }
            // Fall back to canonicalizing the path as-is
            path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
        })
        .collect();
    let header_set: std::collections::HashSet<&std::path::Path> = 
        header_paths.iter().map(|p| p.as_path()).collect();

    // Create synthetic source that includes all headers
    let mut synthetic_source = String::new();
    for header in headers {
        synthetic_source.push_str(&format!("#include \"{}\"\n", header.as_ref().display()));
    }

    // Build clang arguments
    let mut args: Vec<String> = vec![
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++17".to_string(),
        "-Wno-pragma-once-outside-header".to_string(),
    ];
    add_system_include_paths(&mut args);
    for include_dir in include_dirs {
        args.push(format!("-I{}", include_dir.as_ref().display()));
    }

    if verbose {
        eprintln!("Clang args: {:?}", args);
    }

    // Parse the synthetic source with all includes
    let parse_start = Instant::now();
    let tu = index
        .parser("synthetic.cpp")
        .arguments(&args)
        .unsaved(&[clang::Unsaved::new("synthetic.cpp", &synthetic_source)])
        .detailed_preprocessing_record(true)
        .skip_function_bodies(true)
        .parse()
        .context("Failed to parse translation unit")?;
    let parse_time = parse_start.elapsed();
    eprintln!("  Clang parse time: {:.2}s", parse_time.as_secs_f64());
```
</details>

Now let me check what the actual build uses:

<details>
<summary>Searched for regex <code>flag|std=|compiler|define|__GNUC__|_MSC_VER</code> (<code>**/crates/opencascade-sys/build.rs</code>), 11 results</summary>

- [build.rs:47](../crates/opencascade-sys/build.rs#L47)
- [build.rs:97](../crates/opencascade-sys/build.rs#L97)
- [build.rs:102](../crates/opencascade-sys/build.rs#L102)
- [build.rs:106](../crates/opencascade-sys/build.rs#L106)
- [build.rs:107](../crates/opencascade-sys/build.rs#L107)
- [build.rs:108](../crates/opencascade-sys/build.rs#L108)
- [build.rs:115](../crates/opencascade-sys/build.rs#L115)
- [build.rs:116](../crates/opencascade-sys/build.rs#L116)
- [build.rs:117](../crates/opencascade-sys/build.rs#L117)
</details>

Now let me check what system include paths the binding generator adds:

Searched for regex `add_system_include_paths` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results
- [parser.rs:210](../crates/opencascade-binding-generator/src/parser.rs#L210)
- [parser.rs:1622](../crates/opencascade-binding-generator/src/parser.rs#L1622)

<details>
<summary>Reading lines 1622-1680 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1622-L1681">parser.rs</a></summary>

```
fn add_system_include_paths(args: &mut Vec<String>) {
    #[cfg(target_os = "macos")]
    {
        // Try to get SDK path from xcrun
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
        {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                // Add C++ standard library headers
                let cxx_include = format!("{}/usr/include/c++/v1", sdk_path);
                if std::path::Path::new(&cxx_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(cxx_include);
                }
                
                // Add general system headers
                let sys_include = format!("{}/usr/include", sdk_path);
                if std::path::Path::new(&sys_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(sys_include);
                }
            }
        }
        
        // Try to find clang's resource directory for built-in headers
        if let Ok(output) = std::process::Command::new("clang")
            .args(["--print-resource-dir"])
            .output()
        {
            if output.status.success() {
                let resource_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let builtin_include = format!("{}/include", resource_dir);
                if std::path::Path::new(&builtin_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(builtin_include);
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Common Linux C++ standard library paths
        let paths = [
            "/usr/include/c++/13",
            "/usr/include/c++/12", 
            "/usr/include/c++/11",
            "/usr/include/c++/10",
            "/usr/include/x86_64-linux-gnu/c++/13",
            "/usr/include/x86_64-linux-gnu/c++/12",
            "/usr/include/x86_64-linux-gnu/c++/11",
            "/usr/include/x86_64-linux-gnu/c++/10",
            "/usr/include",
        ];
        
        for path in paths {
```
</details>

**Todo list:**
- ✅ Find parser's string replacement
- ✅ Check binding generator clang args
- 🔄 Compare with build.rs clang args

Now let me verify what the parser actually sees for `Standard_ExtCharacter` by checking what clang Now let me verify what the parser actually sees for `Standard_ExtCharacter` by checking what clang resolves. The key conditional is:

```cpp
#if ((defined(__GNUC__) && !defined(__clang__) && ...) || (defined(_MSC_VER) && ...))
typedef uint16_t Standard_ExtCharacter;  // old compilers
#else
typedef char16_t Standard_ExtCharacter;  // modern compilers
#endif
```

The binding generator uses `-std=c++17` while the build uses `-std=c++14`. But both use modern clang on macOS, so both should take the `#else` branch (char16_t). Let me verify by checking what libclang actually resolves:

**Terminal:**
```sh
cat > /Users/sethml/src/opencascade-rs/tmp/check_extchar.cpp << 'EOF'
#include "Standard_TypeDef.hxx"

// Force the preprocessor to show what Standard_ExtCharacter resolves to
Standard_ExtCharacter test_var;
Standard_ExtString test_str;
EOF

OCCT_INC=$(find /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/build/include -maxdepth 0 2>/dev/null || echo "/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Standard")

# Check with both c++17 (binding generator) and c++14 (build)
echo "=== Using -std=c++17 (binding generator) ==="
clang++ -std=c++17 -I"$OCCT_INC" -E -dM /Users/sethml/src/opencascade-rs/tmp/check_extchar.cpp 2>/dev/null | grep -E '__GNUC__|__clang__|_MSC_VER'

echo ""
echo "=== Checking Standard_ExtCharacter resolution ==="
clang++ -std=c++17 -I"$OCCT_INC" -fsyntax-only -Xclang -ast-dump=json /Users/sethml/src/opencascade-rs/tmp/check_extchar.cpp 2>/dev/null | python3 -c "
import json, sys
data = json.load(sys.stdin)
def find_vars(node, depth=0):
    if node.get('kind') == 'VarDecl' and node.get('name', '').startswith('test_'):
        print(f\"{node['name']}: {node.get('type', {}).get('qualType', 'unknown')}\")
    for child in node.get('inner', []):
        find_vars(child, depth+1)
find_vars(data)
"
```

The heredoc got corrupted. Let me use a script file instead:

Created [check_extchar.py](../tmp/check_extchar.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/check_extchar.py 2>&1
```
<details>
<summary>Output (10 lines) — exit code 1</summary>

```
=== Predefined macros (clang++) ===
#define __GNUC_MINOR__ 2
#define __GNUC__ 4
#define __clang__ 1

=== Standard_ExtCharacter resolution ===
clang error: <stdin>:1:10: fatal error: 'Standard_TypeDef.hxx' file not found
    1 | #include "Standard_TypeDef.hxx"
      |          ^~~~~~~~~~~~~~~~~~~~~~
1 error generated.
```
</details>

**Terminal:**
```sh
ls /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/build/include/Standard_TypeDef.hxx 2>/dev/null || find /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT -name "Standard_TypeDef.hxx" -type f 2>/dev/null
```
```
/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Standard/Standard_TypeDef.hxx
```

**Terminal:**
```sh
ls /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/build/include/ 2>/dev/null | head -3; echo "---"; ls /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-*/out/../../../../OCCT/build/include/ 2>/dev/null | head -3
```
```
---
/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/:
Adaptor2d_Curve2d.hxx
Adaptor2d_Line2d.hxx
```

Replacing 1 lines with 1 lines in [check_extchar.py](../tmp/check_extchar.py)

```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/check_extchar.py 2>&1
```
<details>
<summary>Output (12 lines)</summary>

```
=== Predefined macros (clang++) ===
#define __GNUC_MINOR__ 2
#define __GNUC__ 4
#define __clang__ 1

=== Standard_ExtCharacter resolution ===
  test_var: qualType=Standard_ExtCharacter, desugared=char16_t
  test_str: qualType=Standard_ExtString, desugared=const Standard_ExtCharacter *

=== With -std=c++14 (build.rs uses this) ===
  test_var: qualType=Standard_ExtCharacter, desugared=char16_t
  test_str: qualType=Standard_ExtString, desugared=const Standard_ExtCharacter *
```
</details>

**Todo list:**
- ✅ Find parser's string replacement
- ✅ Check binding generator clang args
- ✅ Compare with build.rs clang args

Here's what's happening:

**Both clang invocations (binding generator with `-std=c++17` and build with `-std=c++14`) resolve `Standard_ExtCharacter` to `char16_t`**, not `uint16_t`. The predefined macros show `__GNUC__=4`, `__GNUC_MINOR__=2`, `__clang__=1`. The OCCT `#ifdef` condition checks for `__GNUC__ && !__clang__`, so since `__clang__` is defined, the Here's what's happening:

**Both clang invocations (binding generator with `-std=c++17` and build with `-std=c++14`) resolve `Standard_ExtCharacter` to `char16_t`**, not `uint16_t`. The predefined macros show `__GNUC__=4`, `__GNUC_MINOR__=2`, `__clang__=1`. The OCCT `#ifdef` condition checks for `__GNUC__ && !__clang__`, so since `__clang__` is defined, the `#else` branch is taken → `char16_t`.

**The root problem** is `crates/opencascade-binding-generator/src/parser.rs#L40`:
```rust
.replace("Standard_ExtCharacter", "uint16_t")
```

This hardcoded string replacement incorrectly maps `Standard_ExtCharacter` → `uint16_t`, but the actual type is `char16_t`. When we then emit `uint16_t` in C++ wrapper code, it mismatches with OCCT functions that accept/return `char16_t` (via `Standard_ExtCharacter`).

**The C++ type mismatch**: `uint16_t` (unsigned short) and `char16_t` are distinct types in C++ — you can't implicitly convert between them.

**The fix**: The `normalize_template_spelling` function should map `Standard_ExtCharacter` → `char16_t` (not `uint16_t`), and  should represent `char16_t`/`Standard_ExtCharacter` — or more cleanly, we should use `Standard_ExtCharacter` in C++ emission so it matches regardless of platform. However, the prior attempt to use `Standard_ExtCharacter` in emission failed because these are being mapped to `uint16_t` in the parser first.

The cleanest fix: change line 40 to `char16_t`, and use `char16_t` consistently as the C++ type for  in the code generator.

*2026-02-20 08:32 · 114s · Words: 781 in · 1,116 out · 107,980 ctx · 17 rounds*

---

<a id="user-7"></a>

## User (7)

> First, let&#x27;s have the binding generator use the same -std=c++14 as the build. Make that change.
> Then, I don&#x27;t think we should have these replacements at all - I think we should use libclang&#x27;s typedef handling to resolve these types. Try deleting all &quot;Standard_*&quot; mappings from normalize_template_spelling() and verify that Standard_ExtCharacter gets mapped to the same type as uint16_t by clang. Note that this may be something like &quot;signed short&quot;, and you may need to modify other parts of the binding generator to map &quot;signed short&quot; to rust I16.

*2026-02-20 08:36*

### Assistant — Claude Opus 4.6

> 💭 Let me break this into clear steps:
> 
> 1. Change binding generator from `-std=c++17` to `-std=c++14`
> 2. Remove Standard_* replacements from `normalize_template_spelling()`
> 3. Verify that clang resolves Standard_ExtCharacter correctly without the manual replacements
> 4. Fix any type mapping issues that arise
> 
> Let me start by understanding what types clang actually resolves Standard_* types to, then make all the changes.

**Todo list:**
- 🔄 Change to -std=c++14
- ⬜ Remove Standard_* from normalize
- ⬜ Check clang canonical types
- ⬜ Fix type mapping gaps
- ⬜ Build and test

<details>
<summary>Reading lines 203-213 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L203-L214">parser.rs</a></summary>

```
    // Build clang arguments
    let mut args: Vec<String> = vec![
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++17".to_string(),
        "-Wno-pragma-once-outside-header".to_string(),
    ];
    add_system_include_paths(&mut args);
    for include_dir in include_dirs {
        args.push(format!("-I{}", include_dir.as_ref().display()));
    }
```
</details>

<details>
<summary>Reading lines 26-43 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L26-L44">parser.rs</a></summary>

```
/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_ExtCharacter", "uint16_t")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}
```
</details>

Now let me check what clang's canonical type resolution gives for all these Standard_* types:

Using "Create File"

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/check_all_standard.py 2>&1
```
<details>
<summary>Output (13 lines)</summary>

```
=== -std=c++14 ===
  var_integer               qualType=Standard_Integer               desugared=int                  canonical=
  var_real                  qualType=Standard_Real                  desugared=double               canonical=
  var_boolean               qualType=Standard_Boolean               desugared=bool                 canonical=
  var_shortreal             qualType=Standard_ShortReal             desugared=float                canonical=
  var_character             qualType=Standard_Character             desugared=char                 canonical=
  var_extcharacter          qualType=Standard_ExtCharacter          desugared=char16_t             canonical=
  var_byte                  qualType=Standard_Byte                  desugared=unsigned char        canonical=
  var_utf8char              qualType=Standard_Utf8Char              desugared=char                 canonical=
  var_size                  qualType=Standard_Size                  desugared=unsigned long        canonical=
  var_address               qualType=Standard_Address               desugared=void *               canonical=
  var_cstring               qualType=Standard_CString               desugared=const Standard_Character * canonical=
  var_extstring             qualType=Standard_ExtString             desugared=const Standard_ExtCharacter * canonical=
```
</details>

Now I can see what clang resolves each to:

| Standard_* | Desugared by clang |
|---|---|
| Standard_Integer | int |
| Standard_Real | double |
| Standard_Boolean | bool |
| Standard_ShortReal | float |
| Standard_Character | char |
| Standard_ExtCharacter | **char16_t** |
| Standard_Byte | unsigned char |
| Standard_Utf8Char | char |
| Standard_Size | unsigned long |
| Standard_Address | void * |

So `Standard_ExtCharacter` → `char16_t`, not `uint16_t`. The `normalize_template_spelling` function is used for typedef map entries (NCollection etc), not for type resolution. Clang already resolves these correctly. Let me check where `normalize_template_spelling` is called:

Searched for regex `normalize_template_spelling` (`**/crates/opencascade-binding-generator/src/parser.rs`), 4 results
- [parser.rs:30](../crates/opencascade-binding-generator/src/parser.rs#L30)
- [parser.rs:69](../crates/opencascade-binding-generator/src/parser.rs#L69)
- [parser.rs:133](../crates/opencascade-binding-generator/src/parser.rs#L133)
- [parser.rs:150](../crates/opencascade-binding-generator/src/parser.rs#L150)

<details>
<summary>Reading lines 55-80 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L55-L81">parser.rs</a></summary>

```
        {
            if let Some(name) = entity.get_name() {
                // Only record if the typedef name looks like an OCCT type
                // (contains underscore — e.g., math_Vector, TopTools_ListOfShape)
                if !name.contains('_') {
                    return EntityVisitResult::Recurse;
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        map.insert(key, name.clone());
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    eprintln!("  Collected {} NCollection/template typedefs", map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
```
</details>

<details>
<summary>Reading lines 125-155 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L125-L156">parser.rs</a></summary>

```
                        // private nested typedefs inside class bodies (e.g.,
                        // Express_Entity::DataMapOfStringInteger). Same filter
                        // as collect_ncollection_typedefs.
                        if !typedef_name.contains('_') {
                            continue;
                        }
                        // Only add if not already collected by clang scan
                        if !existing_values.contains(typedef_name) {
                            let key = normalize_template_spelling(template_type);
                            map.insert(key, typedef_name.to_string());
                            count += 1;
                        }
                    }
                }
            }
        }
    });
    if count > 0 {
        eprintln!("  Supplemented {} additional NCollection typedefs from header text scan", count);
    }
}

/// Look up a type's display name in the typedef map.
/// Returns the typedef name if found.
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
```
</details>

**Todo list:**
- ✅ Change to -std=c++14
- 🔄 Remove Standard_* from normalize
- ⬜ Check clang canonical types
- ⬜ Fix type mapping gaps
- ⬜ Build and test

The  function is only used for NCollection typedef map lookups — matching template display names like `NCollection_List<int>` to their typedef names. The Standard_* replacements exist so that e.g. `NCollection_List<Standard_Integer>` normalizes to `NCollection_List<int>` for matching. Since clang already desugars these in display names, these replacements may be redundant, but let me check.

Let me also check what the parser's `parse_type` returns for `Standard_ExtCharacter` and `char16_t` — both need to map to the right :

<details>
<summary>Reading lines 1362-1395 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1362-L1396">parser.rs</a></summary>

```
    // Handle primitives via canonical type.
    // Skip this if:
    // 1. The spelling clearly identifies a class type (spelling_looks_like_class), OR
    // 2. The spelling contains '<' or '::' — template or namespace-scoped types
    //    whose canonical resolves to int/double/etc. are template misresolutions,
    //    not genuine primitives.
    let spelling_is_template_or_namespaced = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("volatile ")
            .trim();
        s.contains('<') || s.contains("::")
    };
    if !spelling_looks_like_class && !spelling_is_template_or_namespaced {

        match canonical_clean {
            "bool" => return Type::Bool,
            "int" => return Type::I32,
            "unsigned int" => return Type::U32,
            "unsigned short" | "uint16_t" | "char16_t" => return Type::U16,
            "short" | "int16_t" => return Type::I16,
            "long" => return Type::Long,
            "unsigned long" => return Type::ULong,
            "long long" => return Type::I64,
            "unsigned long long" => return Type::U64,
            "float" => return Type::F32,
            "double" => return Type::F64,
            _ => {}
        }
    }

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int" (template misresolution), construct the class type
    // directly instead of recursing into the pointee (whose display name might
```
</details>

Good. `canonical_clean` at line 1381 already handles `char16_t` → . But wait — `Standard_ExtCharacter` desugars to `char16_t`, but what does clang's canonical type string look like? Let me check what `canonical_clean` would be for `Standard_ExtCharacter`:

The clang data showed `desugared=char16_t` with no canonical. The parser likely uses `get_canonical_type().get_display_name()` which should give `char16_t`. Let me verify by checking how `canonical_clean` is computed:

<details>
<summary>Searched for regex <code>canonical_clean</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 9 results</summary>

- [parser.rs:1306](../crates/opencascade-binding-generator/src/parser.rs#L1306)
- [parser.rs:1377](../crates/opencascade-binding-generator/src/parser.rs#L1377)
- [parser.rs:1400](../crates/opencascade-binding-generator/src/parser.rs#L1400)
- [parser.rs:1510](../crates/opencascade-binding-generator/src/parser.rs#L1510)
- [parser.rs:1525](../crates/opencascade-binding-generator/src/parser.rs#L1525)
- [parser.rs:1527](../crates/opencascade-binding-generator/src/parser.rs#L1527)
- [parser.rs:1528](../crates/opencascade-binding-generator/src/parser.rs#L1528)
</details>

<details>
<summary>Reading lines 1298-1315 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1298-L1316">parser.rs</a></summary>

```
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = canonical_spelling
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim();
    // Guard against clang misresolving NCollection template specializations.
    // When clang can't fully instantiate templates like NCollection_DataMap<A,B>,
    // it falls back to canonical type "int". Detect this by checking if the
    // display name is clearly a class/typedef (not a known primitive typedef)
    // while the canonical says it's a primitive.
```
</details>

Good —  comes from `get_canonical_type().get_display_name()`. For `Standard_ExtCharacter`, canonical type is `char16_t`, so  = `"char16_t"`, which matches line 1381.

Now let me also check whether `Standard_ExtCharacter` passes through `spelling_looks_like_class`, since it starts with uppercase:

<details>
<summary>Reading lines 1315-1380 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1315-L1381">parser.rs</a></summary>

```
    // while the canonical says it's a primitive.
    //
    // However, legitimate typedefs to primitives (e.g., `typedef unsigned int Poly_MeshPurpose`)
    // must still resolve to their canonical primitive type. We distinguish these by
    // checking the typedef's underlying type: if it's a builtin primitive or another
    // typedef (i.e., a chain like Graphic3d_ZLayerId -> Standard_Integer -> int),
    // it's a genuine primitive typedef. NCollection typedefs have underlying types
    // that are template specializations (Record/Elaborated/Unexposed), not primitives.
    let spelling_looks_like_class = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("struct ")
            .trim_start_matches("class ")
            .trim_start_matches("typename ")
            .trim();
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && map_standard_type(s).is_none()
            && s != "Standard_Boolean"
            && !s.contains('<')
            && !s.contains("::");

        if !looks_like_class {
            false
        } else {
            // Check if this is a typedef whose underlying type is a primitive.
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int),
            // not an NCollection template misresolution.
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                        TypeKind::SChar | TypeKind::UChar |
                        TypeKind::Short | TypeKind::UShort |
                        TypeKind::Int | TypeKind::UInt |
                        TypeKind::Long | TypeKind::ULong |
                        TypeKind::LongLong | TypeKind::ULongLong |
                        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble |
                        TypeKind::Typedef  // chain through another typedef (e.g., Standard_Integer)
                    ))
                    .unwrap_or(false);
            !is_primitive_typedef
        }
    };

    // Handle primitives via canonical type.
    // Skip this if:
    // 1. The spelling clearly identifies a class type (spelling_looks_like_class), OR
    // 2. The spelling contains '<' or '::' — template or namespace-scoped types
    //    whose canonical resolves to int/double/etc. are template misresolutions,
    //    not genuine primitives.
    let spelling_is_template_or_namespaced = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("volatile ")
            .trim();
        s.contains('<') || s.contains("::")
    };
    if !spelling_looks_like_class && !spelling_is_template_or_namespaced {

        match canonical_clean {
            "bool" => return Type::Bool,
            "int" => return Type::I32,
            "unsigned int" => return Type::U32,
```
</details>

Key insight at lines 1330-1331: `spelling_looks_like_class` checks `map_standard_type(s).is_none()`. If `Standard_ExtCharacter` is handled by `map_standard_type()`, it returns , making `looks_like_class` false, which lets it fall through to the canonical type matching at line 1377.

Let me check `map_standard_type`:

<details>
<summary>Reading lines 1575-1615 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1575-L1616">parser.rs</a></summary>

```
    let clean = type_name
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim();

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::U16),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::U16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I16),
        "int16_t" => Some(Type::I16),
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
```
</details>

I see the full picture now. `map_standard_type` at line 1592 already maps `Standard_ExtCharacter` →  directly. And at line 1605, `char16_t` also maps to . So even without the  replacements, the type resolution should work fine because:

1. When the parser sees a type spelled `Standard_ExtCharacter`, `map_standard_type` returns  
2. When clang canonicalizes to `char16_t`, the canonical match at line 1381 also returns 

The  Standard_* replacements are only for NCollection typedef map keys — ensuring `NCollection_List<Standard_Integer>` matches  Standard_* replacements are only for NCollection typedef map keys — ensuring `NCollection_List<Standard_Integer>` matches `NCollection_List<int>`. But does clang use `Standard_Integer` or `int` in template display names? Let me check that, but first let me apply the straightforward changes and see if they work.

Now, the actual question is: if I remove Standard_* from , will NCollection typedefs with Standard types in their template args still be found? Clang's `get_display_name()` for a typedef's underlying type may use either the typedef name or the canonical name.

Let me just make the changes and test — the build will tell us if anything broke:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L26-L32">parser.rs -18+6</a>, <a href="../crates/opencascade-binding-generator/src/parser.rs#L195-L196">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        \"-std=c++14\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "207:dq"
    },
    {
      "content": "/// Normalize a C++ type spelling for typedef map lookup.\n/// Removes whitespace so that typedef keys match display names even when\n/// clang uses different whitespace conventions.\nfn normalize_template_spelling(s: &str) -> String {\n    s.chars().filter(|c| !c.is_whitespace()).collect()\n}",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "26:vf,27:qn,28:ug,29:di,30:qc,31:cc,32:vw,33:mo,34:dw,35:zo,36:bl,37:sw,38:eb,39:hz,40:cl,41:ta,42:tj,43:io"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 26,
      "endLine": 195
    }
  ]
}
```
</details>

**Todo list:**
- ✅ Change to -std=c++14
- ✅ Remove Standard_* from normalize
- 🔄 Build and check impact
- ⬜ Fix type mapping gaps
- ⬜ Regenerate and full build

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.53s
real  3.6s
```

Good. Now let me regenerate bindings and check how many typedefs are affected. I'll run the binding generator with verbose output to see the typedef collection count:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep 'Collected.*typedef\|Supplemented.*typedef\|Total.*bound\|Total.*skipped\|Total.*functions' /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01Kt5ny5TwRiWc7fVAF7cuJF__vscode-1771312786024/content.txt
```
```
  Collected 3567 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan
```

Typedef counts look similar. Let me check the diff in generated files to see if anything material changed:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/ 2>&1 | tail -5
```
<details>
<summary>Output (5 lines)</summary>

```
 .../opencascade-sys/generated/top_ope_b_rep_ds.rs  |    4 +-
 crates/opencascade-sys/generated/vrml_data.rs      |  327 ++++--
 crates/opencascade-sys/generated/wrappers.cpp      |  460 ++++++--
 crates/opencascade-sys/generated/xs_control.rs     |  134 ++-
 40 files changed, 3816 insertions(+), 1393 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/ 2>&1 | grep '^[-+]' | grep -i 'uint16\|char16\|int16\|Standard_Ext' | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
-//   Reason: has unbindable types: return: raw pointer (const uint16_t*)
-//   // pub fn ext_value(&mut self, aResourceName: *const char) -> /* const uint16_t* */;
-//   Reason: has unbindable types: param 'aValue': raw pointer (const uint16_t*)
-//   // pub fn set_resource(&mut self, aResourceName: *const char, aValue: /* const uint16_t* */);
-//   Reason: has unbindable types: param 'aGuid': raw pointer (const uint16_t*)
-//   // pub fn new_u16ptr(aGuid: /* const uint16_t* */) -> OwnedPtr<Self>;
-//   Reason: has unbindable types: param 'astring': raw pointer (const uint16_t*)
-//   // pub fn new_u16ptr(astring: /* const uint16_t* */) -> OwnedPtr<Self>;
-//   Reason: has unbindable types: param 'other': raw pointer (const uint16_t*)
-//   // pub fn is_equal(&self, other: /* const uint16_t* */) -> bool;
-//   Reason: has unbindable types: param 'other': raw pointer (const uint16_t*)
-//   // pub fn is_different(&self, other: /* const uint16_t* */) -> bool;
-//   Reason: has unbindable types: param 'other': raw pointer (const uint16_t*)
-//   // pub fn is_less(&self, other: /* const uint16_t* */) -> bool;
-//   Reason: has unbindable types: param 'other': raw pointer (const uint16_t*)
-//   // pub fn is_greater(&self, other: /* const uint16_t* */) -> bool;
-//   Reason: has unbindable types: param 'separators': raw pointer (const uint16_t*)
-//   // pub fn token(&self, separators: /* const uint16_t* */, whichone: i32) -> OwnedPtr<TCollection_ExtendedString>;
-//   Reason: has unbindable types: return: raw pointer (const uint16_t*)
-//   // pub fn to_ext_string(&self) -> /* const uint16_t* */;
```
</details>

These are all the previously-skipped raw pointer methods that now reference `uint16_t`. Let me now try the full build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (518 lines) — exit code 101</summary>

```
warning: opencascade-sys@0.2.0:   357 |                                   const TCollection_ExtendedString& theString2)
warning: opencascade-sys@0.2.0:       |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332135:19: error: no matching member function for call to 'IsDifferent'
warning: opencascade-sys@0.2.0:  332135 |     return self_->IsDifferent(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:195:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   195 |   Standard_EXPORT Standard_Boolean IsDifferent(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:202:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const TCollection_ExtendedString' for 1st argument
warning: opencascade-sys@0.2.0:   202 |   Standard_EXPORT Standard_Boolean IsDifferent(const TCollection_ExtendedString& other) const;
warning: opencascade-sys@0.2.0:       |                                    ^           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:332141:19: error: no matching member function for call to 'IsLess'
warning: opencascade-sys@0.2.0:  332141 |     return self_->IsLess(other);
warning: opencascade-sys@0.2.0:         |            ~~~~~~~^~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:210:36: note: candidate function not viable: no known conversion from 'const uint16_t *' (aka 'const unsigned short *') to 'const Standard_ExtString' (aka 'const char16_t *const') for 1st argument
warning: opencascade-sys@0.2.0:   210 |   Standard_EXPORT Standard_Boolean IsLess(const Standard_ExtString other) const;
warning: opencascade-sys@0.2.0:       |                                    ^      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx:215:36: note: candidate function not viable: no kn
... (truncated, 62419 chars total)
```
</details>

**Terminal:**
```sh
tail -15 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_012n9phaxiHfmUxaqAFTuMFw__vscode-1771312786028/content.txt
```
<details>
<summary>Output (12 lines)</summary>

```
  exit status: 0
  exit status: 0
  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code
 exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                                       
  --- stderr


  error occurred in cc-rs: command did not execute successfully (status code exi
t status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                                           

real  18.7s
```
</details>

Still C++ compile errors. Same `uint16_t` vs `char16_t` mismatch. The problem is: the binding generator's parser uses `map_standard_type("Standard_ExtCharacter")` → , then  emits as `uint16_t` in C++. But the actual C++ type is `char16_t`.

The root cause:  is being used for both `uint16_t` (unsigned short) AND `char16_t`. These are different C++ types. The fix: `Standard_ExtCharacter` should resolve through clang to `char16_t`, and `char16_t` should emit as `char16_t` in C++ (not `uint16_t`).

Let me check: should  emit as `char16_t` or `uint16_t`? Since `Standard_ExtCharacter` IS `char16_t`, we need 's C++ emission to be `char16_t`. But `uint16_t` is `unsigned short`, which is a different type.

The cleanest fix: use `char16_t` as the C++ type for  since that's what OCCT uses, and it's compatible both ways on modern compilers. Let me check all the C++ emission points for :

<details>
<summary>Searched for regex <code>U16.*uint16_t|uint16_t.*U16</code> (<code>**/crates/opencascade-binding-generator/src/**/*.rs</code>), 6 results</summary>

- [bindings.rs:603](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L603)
- [parser.rs:1369](../crates/opencascade-binding-generator/src/parser.rs#L1369)
- [parser.rs:1593](../crates/opencascade-binding-generator/src/parser.rs#L1593)
- [resolver.rs:1382](../crates/opencascade-binding-generator/src/resolver.rs#L1382)
- [type_mapping.rs:505](../crates/opencascade-binding-generator/src/type_mapping.rs#L505)
- [model.rs:711](../crates/opencascade-binding-generator/src/model.rs#L711)
</details>

The problem:  emits `uint16_t` in C++ (bindings.rs:603, resolver.rs:1382, model.rs:711), but OCCT uses `char16_t` for `Standard_ExtCharacter`. These are distinct C++ types.

The correct fix: change C++ emission of  to `char16_t`, since that's the actual type on all modern compilers. `char16_t` and `uint16_t` have the same size/alignment, and Rust's `u16` is compatible with both via FFI.

<details>
<summary>Reading lines 708-715 of <a href="../crates/opencascade-binding-generator/src/model.rs#L708-L716">model.rs</a></summary>

```
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I16 => "int16_t".to_string(),
            Type::I64 => "long long".to_string(),
            Type::U64 => "unsigned long long".to_string(),
            Type::Long => "long".to_string(),
```
</details>

<details>
<summary>Reading lines 600-607 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L600-L608">bindings.rs</a></summary>

```
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
```
</details>

<details>
<summary>Reading lines 1380-1386 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1380-L1387">resolver.rs</a></summary>

```
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L711-L712">model.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L603-L604">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1382-L1383">resolver.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "711:mp",
      "content": "            Type::U16 => \"char16_t\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "603:wa",
      "content": "            Type::U16 => \"char16_t\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "1382:wa",
      "content": "            Type::U16 => \"char16_t\".to_string(),"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 711,
      "endLine": 711
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 603,
      "endLine": 603
    },
    {
      "file": "crates/opencascade-binding-generator/src/resolver.rs",
      "startLine": 1382,
      "endLine": 1382
    }
  ]
}
```
</details>

All 3 C++ emission points changed from `"uint16_t"` to `"char16_t"`. Now let's rebuild and test.

**Todo list:**
- ✅ Fix U16 → char16_t emission
- 🔄 Rebuild binding generator
- ⬜ Regenerate bindings
- ⬜ Full cargo build
- ⬜ Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.73s
real  3.9s
```

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- 🔄 Regenerate bindings
- ⬜ Full cargo build
- ⬜ Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01DaiWQ6h2QQFQLxGZdEGxqd__vscode-1771312786038/content.txt
```
<details>
<summary>Output (5 lines)</summary>

```
Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  49.6s
```
</details>

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- ✅ Regenerate bindings
- 🔄 Full cargo build
- ⬜ Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

**Terminal:**
```sh
grep -c "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011NSm1ybCWTiqFJY1egFoV5__vscode-1771312786041/content.txt; tail -10 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011NSm1ybCWTiqFJY1egFoV5__vscode-1771312786041/content.txt
```
<details>
<summary>Output (8 lines)</summary>

```
7
NullString_destructor(Message_ProgressScope::NullString* self_) { delete self_; }                                                                               warning: opencascade-sys@0.2.0:         |                                       
                                                                  ^      ~~~~~  warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys
/OCCT/src/Message/Message_ProgressScope.hxx:195:9: note: forward declaration of 'Message_ProgressScope::NullString'                                             warning: opencascade-sys@0.2.0:   195 |   class NullString; //!< auxiliary type 
for passing NULL name to Message_ProgressScope constructor                      warning: opencascade-sys@0.2.0:       |         ^
warning: opencascade-sys@0.2.0: 1 warning generated.
error: could not compile `opencascade-sys` (lib) due to 6 previous errors
real  108.0s
```
</details>

**Terminal:**
```sh
grep "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011NSm1ybCWTiqFJY1egFoV5__vscode-1771312786041/content.txt
```
<details>
<summary>Output (7 lines)</summary>

```
error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
error[E0412]: cannot find type `char` in module `crate::ffi`
error[E0412]: cannot find type `char` in module `crate::ffi`
error: could not compile `opencascade-sys` (lib) due to 6 previous errors
```
</details>

> 💭 Two issues:
> 1. Missing lifetime specifiers (4 errors)
> 2. `char` not found in `crate::ffi` (2 errors) - this is likely because `char16_t` got mapped to some Rust type incorrectly
> 
> Let me check the details.

Two distinct issues: missing lifetimes and `char` type not found. Let me investigate.

**Terminal:**
```sh
grep -B5 "cannot find type.*char\|missing lifetime" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011NSm1ybCWTiqFJY1egFoV5__vscode-1771312786041/content.txt | grep -E "^\s*-->" 2>&1
```
**Exit code:** 1

**Terminal:**
```sh
grep -B5 "cannot find type\|missing lifetime" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_011NSm1ybCWTiqFJY1egFoV5__vscode-1771312786041/content.txt | head -60
```
<details>
<summary>Output (41 lines)</summary>

```
NullString_destructor(Message_ProgressScope::NullString* self_) { delete self_; }                                                                               warning: opencascade-sys@0.2.0:         |                                       
                                                                  ^      ~~~~~  warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys
/OCCT/src/Message/Message_ProgressScope.hxx:195:9: note: forward declaration of 'Message_ProgressScope::NullString'                                             warning: opencascade-sys@0.2.0:   195 |   class NullString; //!< auxiliary type 
for passing NULL name to Message_ProgressScope constructor                      warning: opencascade-sys@0.2.0:       |         ^
warning: opencascade-sys@0.2.0: 1 warning generated.
error[E0106]: missing lifetime specifier
--
647 ~         theLen: &'a mut i32,
648 |         isAttribute: bool,
649 ~     ) -> Option<&'a mut crate::ffi::char> {
    |

error[E0106]: missing lifetime specifier
--
     |
6469 -     pub unsafe fn get_callback() -> Option<&mut MAllocHook_Callback> {
6469 +     pub unsafe fn get_callback() -> Option<MAllocHook_Callback> {
     |

error[E0106]: missing lifetime specifier
--
     |
6482 -     pub unsafe fn get_log_file_handler() -> Option<&mut MAllocHook_LogFil
eHandler> {                                                                     6482 +     pub unsafe fn get_log_file_handler() -> Option<MAllocHook_LogFileHand
ler> {                                                                               |

error[E0106]: missing lifetime specifier
--
     |
6495 -     pub unsafe fn get_collect_by_size() -> Option<&mut MAllocHook_Collect
BySize> {                                                                       6495 +     pub unsafe fn get_collect_by_size() -> Option<MAllocHook_CollectBySiz
e> {                                                                                 |

error[E0412]: cannot find type `char` in module `crate::ffi`
--
    |
640 -     ) -> Option<&mut crate::ffi::char> {
640 +     ) -> Option<&mut char> {
    |

error[E0412]: cannot find type `char` in module `crate::ffi`
```
</details>

> 💭 Two issues:
> 
> 1. **`crate::ffi::char`** - The type `char` (C++ `char`) is being emitted as `crate::ffi::char` but `char` is not an FFI type - it should be `std::os::raw::c_char` or `i8`/`u8`. This is a C++ `char` type that isn't being handled properly by the binding generator.
> 
> 2. **Missing lifetime specifiers** - References in return types without lifetimes. These are from functions returning `Option<&mut SomeType>` without lifetime parameters.
> 
> Let me check if these errors are new (from our changes) or pre-existing. The `char` issue could be from removing the Standard_* normalizations, or it could be from the char16_t change. Let me look at the generated code.
> 
> 
> Now I need to examine which file these errors are coming from and what the generated code actually looks like.

Two separate issues: `crate::ffi::char` type and missing lifetimes. Let me investigate both.

**Terminal:**
```sh
grep -rn "crate::ffi::char[^1]" crates/opencascade-sys/generated/src/ 2>&1 | grep -v char16
```
```
grep: crates/opencascade-sys/generated/src/: No such file or directory
```

**Terminal:**
```sh
find crates/opencascade-sys/generated -name "*.rs" | xargs grep -l "crate::ffi::char[^1]" 2>/dev/null
```
```
crates/opencascade-sys/generated/ldom.rs
```

**Terminal:**
```sh
grep -n "crate::ffi::char[^1]" crates/opencascade-sys/generated/ldom.rs
```
```
640:    ) -> Option<&mut crate::ffi::char> {
649:    ) -> Option<&mut crate::ffi::char> {
```

**Terminal:**
```sh
grep -n "MAllocHook\|missing lifetime" crates/opencascade-sys/generated/*.rs 2>/dev/null | grep -i "get_callback\|get_log_file\|get_collect" | head -20
```
<details>
<summary>Output (9 lines)</summary>

```
crates/opencascade-sys/generated/ffi.rs:297033:    pub fn OSD_MAllocHook_get_callback() -> *mut OSD_MAllocHook_Callback;
crates/opencascade-sys/generated/ffi.rs:297035:    pub fn OSD_MAllocHook_get_log_file_handler() -> *mut OSD_MAllocHook_LogFileHandler;
crates/opencascade-sys/generated/ffi.rs:297037:    pub fn OSD_MAllocHook_get_collect_by_size() -> *mut OSD_MAllocHook_CollectBySize;
crates/opencascade-sys/generated/osd.rs:6469:    pub unsafe fn get_callback() -> Option<&mut MAllocHook_Callback> {
crates/opencascade-sys/generated/osd.rs:6471:            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_callback() };
crates/opencascade-sys/generated/osd.rs:6482:    pub unsafe fn get_log_file_handler() -> Option<&mut MAllocHook_LogFileHandler> {
crates/opencascade-sys/generated/osd.rs:6484:            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_log_file_handler() };
crates/opencascade-sys/generated/osd.rs:6495:    pub unsafe fn get_collect_by_size() -> Option<&mut MAllocHook_CollectBySize> {
crates/opencascade-sys/generated/osd.rs:6497:            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_collect_by_size() };
```
</details>

> 💭 These are pre-existing issues - unrelated to our char16_t change. Static methods returning raw pointers that get wrapped into `Option<&mut T>` without a lifetime.
> 
> But wait - the question is whether these errors existed before our changes? Let me check git to see if these files changed.
> 
> Actually, the user said in the original conversation that the build was failing due to char16_t → uint16_t mismatch. These other errors might be new (from the normalize_template_spelling cleanup or the I16 changes) or might have been pre-existing. Let me check:
> 
> 1. `crate::ffi::char` - This is definitely a new issue. C++ `char` type is being mapped to a Rust type `char` which doesn't exist in the FFI module. This is probably because previously, the `Standard_Character` type was being normalized to something else, or `char` wasn't appearing at all.
> 
> 
> 2. The missing lifetime on `Option<&mut T>` from static methods could be pre-existing or newly exposed by our changes - I need to check the binding generator to understand how `char` types are being handled and whether the normalize_template_spelling cleanup is surfacing previously hidden issues.

Let me check if these errors are new (from our changes) or pre-existing.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --name-only HEAD crates/opencascade-sys/generated/ | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
crates/opencascade-sys/generated/adaptor3d.rs
crates/opencascade-sys/generated/adv_app2_var.rs
crates/opencascade-sys/generated/adv_approx.rs
crates/opencascade-sys/generated/b_rep_approx.rs
crates/opencascade-sys/generated/b_rep_g_prop.rs
crates/opencascade-sys/generated/b_rep_mesh.rs
crates/opencascade-sys/generated/b_rep_prim_api.rs
crates/opencascade-sys/generated/b_rep_top_adaptor.rs
crates/opencascade-sys/generated/b_spl_c_lib.rs
crates/opencascade-sys/generated/c_pnts.rs
crates/opencascade-sys/generated/extrema.rs
crates/opencascade-sys/generated/ffi.rs
crates/opencascade-sys/generated/geom_int.rs
crates/opencascade-sys/generated/gp.rs
crates/opencascade-sys/generated/graphic3d.rs
crates/opencascade-sys/generated/hlr_algo.rs
crates/opencascade-sys/generated/hlrb_rep.rs
crates/opencascade-sys/generated/int_patch.rs
crates/opencascade-sys/generated/ldom.rs
crates/opencascade-sys/generated/lib.rs
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git stash && cargo build 2>&1 | grep "^error" | head -10
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1 | grep "^error"
```
```
real  0.8s
```
**Exit code:** 1

> 💭 Hmm, it seems like the stash brought back the original generated code which doesn't have the issues. Let me unstash and proceed.
> 
> Wait - the exit code was 1 but no errors? Maybe grep didn't match because perhaps there are no "^error" lines. Let me check properly.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1 | grep -i "error\|Finished"
```
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
real  0.7s
```

Good - clean state builds fine. Now restore our changes:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git stash pop
```
<details>
<summary>Output (57 lines)</summary>

```
On branch main
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
modified:   crates/opencascade-binding-generator/src/codegen/bindings.rs
modified:   crates/opencascade-binding-generator/src/codegen/rust.rs
modified:   crates/opencascade-binding-generator/src/model.rs
modified:   crates/opencascade-binding-generator/src/parser.rs
modified:   crates/opencascade-binding-generator/src/resolver.rs
modified:   crates/opencascade-binding-generator/src/type_mapping.rs
modified:   crates/opencascade-sys/generated/adaptor3d.rs
modified:   crates/opencascade-sys/generated/adv_app2_var.rs
modified:   crates/opencascade-sys/generated/adv_approx.rs
modified:   crates/opencascade-sys/generated/b_rep_approx.rs
modified:   crates/opencascade-sys/generated/b_rep_g_prop.rs
modified:   crates/opencascade-sys/generated/b_rep_mesh.rs
modified:   crates/opencascade-sys/generated/b_rep_prim_api.rs
modified:   crates/opencascade-sys/generated/b_rep_top_adaptor.rs
modified:   crates/opencascade-sys/generated/b_spl_c_lib.rs
modified:   crates/opencascade-sys/generated/c_pnts.rs
modified:   crates/opencascade-sys/generated/extrema.rs
modified:   crates/opencascade-sys/generated/ffi.rs
modified:   crates/opencascade-sys/generated/geom_int.rs
modified:   crates/opencascade-sys/generated/gp.rs
modified:   crates/opencascade-sys/generated/graphic3d.rs
modified:   crates/opencascade-sys/generated/hlr_algo.rs
modified:   crates/opencascade-sys/generated/hlrb_rep.rs
modified:   crates/opencascade-sys/generated/int_patch.rs
modified:   crates/opencascade-sys/generated/ldom.rs
modified:   crates/opencascade-sys/generated/lib.rs
modified:   crates/opencascade-sys/generated/math.rs
modified:   crates/opencascade-sys/generated/message.rs
modified:   crates/opencascade-sys/generated/n_collection.rs
modified:   crates/opencascade-sys/generated/osd.rs
modified:   crates/opencascade-sys/generated/poly.rs
modified:   crates/opencascade-sys/generated/quantity.rs
modified:   crates/opencascade-sys/generated/resource.rs
modified:   crates/opencascade-sys/generated/rw_gltf.rs
modified:   crates/opencascade-sys/generated/rw_obj.rs
modified:   crates/opencascade-sys/generated/rw_obj_tools.rs
modified:   crates/opencascade-sys/generated/rw_ply.rs
modified:   crates/opencascade-sys/generated/standard.rs
modified:   crates/opencascade-sys/generated/step_file.rs
modified:   crates/opencascade-sys/generated/t_collection.rs
modified:   crates/opencascade-sys/generated/t_doc_std.rs
modified:   crates/opencascade-sys/generated/top_ope_b_rep_build.rs
modified:   crates/opencascade-sys/generated/top_ope_b_rep_ds.rs
modified:   crates/opencascade-sys/generated/vrml_data.
... (truncated, 3417 chars total)
```
</details>

So these errors are all new from our changes. Let me investigate both:

1. **`crate::ffi::char`** - C++ `char` type being incorrectly mapped
2. **Missing lifetimes** - static functions returning `Option<&mut T>` without lifetime

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- ✅ Regenerate bindings
- 🔄 Fix crate::ffi::char error
- ⬜ Fix missing lifetime errors
- ⬜ Full cargo build
- ⬜ Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD crates/opencascade-sys/generated/ldom.rs | head -80
```
<details>
<summary>Output (49 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/ldom.rs b/crates/opencascade-sys/generated/ldom.rs
index a57629e..c045ca1 100644
--- a/crates/opencascade-sys/generated/ldom.rs
+++ b/crates/opencascade-sys/generated/ldom.rs
@@ -632,17 +632,25 @@ impl CharReference {
     pub fn new() -> crate::OwnedPtr<Self> {
         unsafe { crate::OwnedPtr::from_raw(crate::ffi::LDOM_CharReference_ctor()) }
     }
-}
 
-// ── Skipped symbols for CharReference (2 total) ──
-// SKIPPED: **Source:** `LDOM_CharReference.hxx`:47 - `LDOM_CharReference::Decode`
-//   Reason: has unbindable types: param 'theSrc': raw pointer (char*); return: raw pointer (char*)
-//   // pub fn decode(theSrc: /* char* */, theLen: &mut i32) -> /* char* */;
-//
-// SKIPPED: **Source:** `LDOM_CharReference.hxx`:49 - `LDOM_CharReference::Encode`
-//   Reason: has unbindable types: return: raw pointer (char*)
-//   // pub fn encode(theSrc: *const char, theLen: &mut i32, isAttribute: bool) -> /* char* */;
-//
+    /// **Source:** `LDOM_CharReference.hxx`:47 - `LDOM_CharReference::Decode()`
+    pub unsafe fn decode(
+        theSrc: *mut std::ffi::c_char,
+        theLen: &mut i32,
+    ) -> Option<&mut crate::ffi::char> {
+        unsafe { crate::ffi::LDOM_CharReference_decode(theSrc, theLen) }
+    }
+
+    /// **Source:** `LDOM_CharReference.hxx`:49 - `LDOM_CharReference::Encode()`
+    pub unsafe fn encode(
+        theSrc: &str,
+        theLen: &mut i32,
+        isAttribute: bool,
+    ) -> Option<&mut crate::ffi::char> {
+        let c_theSrc = std::ffi::CString::new(theSrc).unwrap();
+        unsafe { crate::ffi::LDOM_CharReference_encode(c_theSrc.as_ptr(), theLen, isAttribute) }
+    }
+}
 
 // ========================
 // From LDOM_CharacterData.hxx
@@ -1525,7 +1533,7 @@ impl HandleLDOMMemManager {
 // ── Skipped symbols for MemManager (1 total) ──
 // SKIPPED: **Source:** `LDOM_MemManager.hxx`:43 - `LDOM_MemManager::Allocate`
 //   Reason: return type 'void*' is unknown
-//   // pub fn allocate(&mut self, aSize: i32) -> /* void* */;
+//   // pub fn allocate(&mut self, aSize: i32) -> *mut void;
 //
 
 // ========================
```
</details>

> 💭 I see - these functions were previously skipped because they had raw pointers (the `is_unbindable` check rejected them). But now with the prior session's changes to `is_unbindable()` to allow raw pointers through (as `unsafe` functions), these functions are being generated. The issue is:
> 
> 1. `char*` is being converted to `*mut std::ffi::c_char` for parameters (good), but the return type is being mapped to `Option<&mut crate::ffi::char>` instead of `Option<&mut std::ffi::c_char>` or `*mut std::ffi::c_char`
> 
> 
> 2. The binding generator is incorrectly mapping the `char` type - it's emitting `crate::ffi::char` instead of `std::ffi::c_char`. I need to check how the char type is being handled in the type mapping logic.
> 3. There's also a lifetime issue with these new unsafe functions - they're returning `Option<&mut T>` from static functions without any lifetime to bind to, which is a separate problem from the char type mapping.

The `crate::ffi::char` issue comes from the prior session's unsafe fn work - previously these functions were skipped as unbindable. Now they're generated but `char*` return types aren't properly mapped. Let me check how `char` is handled:

<details>
<summary>Searched for text <code>char</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 18 results</summary>

- [model.rs:433](../crates/opencascade-binding-generator/src/model.rs#L433)
- [model.rs:434](../crates/opencascade-binding-generator/src/model.rs#L434)
- [model.rs:452](../crates/opencascade-binding-generator/src/model.rs#L452)
- [model.rs:556](../crates/opencascade-binding-generator/src/model.rs#L556)
- [model.rs:559](../crates/opencascade-binding-generator/src/model.rs#L559)
- [model.rs:603](../crates/opencascade-binding-generator/src/model.rs#L603)
- [model.rs:604](../crates/opencascade-binding-generator/src/model.rs#L604)
- [model.rs:607](../crates/opencascade-binding-generator/src/model.rs#L607)
- [model.rs:608](../crates/opencascade-binding-generator/src/model.rs#L608)
- [model.rs:618](../crates/opencascade-binding-generator/src/model.rs#L618)
- [model.rs:626](../crates/opencascade-binding-generator/src/model.rs#L626)
- [model.rs:636](../crates/opencascade-binding-generator/src/model.rs#L636)
- [model.rs:639](../crates/opencascade-binding-generator/src/model.rs#L639)
- [model.rs:660](../crates/opencascade-binding-generator/src/model.rs#L660)
- [model.rs:687](../crates/opencascade-binding-generator/src/model.rs#L687)
- [model.rs:698](../crates/opencascade-binding-generator/src/model.rs#L698)
- [model.rs:711](../crates/opencascade-binding-generator/src/model.rs#L711)
</details>

Searched for text `"char"` (`**/crates/opencascade-binding-generator/src/parser.rs`), 3 results
- [parser.rs:1575](../crates/opencascade-binding-generator/src/parser.rs#L1575)
- [parser.rs:1578](../crates/opencascade-binding-generator/src/parser.rs#L1578)
- [parser.rs:1579](../crates/opencascade-binding-generator/src/parser.rs#L1579)

> 💭 I see. `char` is stored as `Type::Class("char")`. When it appears as `const char*`, it's handled specially as a string. But when it appears as just `char` (like the return type of `Decode` which returns `char*`), it gets emitted as `crate::ffi::char` in the reexport code, which doesn't exist.
> 
> The issue is that `char` is a C primitive type, not an OCCT class. It should probably be mapped to something like `std::os::raw::c_char` or `i8`. But the existing code treats it as `Type::Class("char")` so that `const char*` can be handled as string conversion.
> 
> 
> Looking at the generated code, the parameter is correctly mapped to `*mut std::ffi::c_char`, but the return type is incorrectly wrapped as `Option<&mut crate::ffi::char>` when it should just be `*mut std::ffi::c_char` since `char*` is a raw pointer, not a class pointer that needs the optional reference pattern.
> 
> The core issue is that the reexport layer is applying the wrong conversion pattern — it's treating the raw pointer return as if it were a class reference, when raw pointers should stay as `*mut T` and the function should be marked `unsafe fn`.
> 
> These problems stem from the prior session's incomplete work on allowing raw pointers through `is_unbindable()`. The functions are now being generated instead of skipped, but the code generation for them is broken. I need to figure out whether these errors existed before our current changes or if something we did with char16_t and normalize_template_spelling triggered them.
> 
> Looking at the git stash, both the binding generator changes and the generated files were stashed, so the committed code still has the old `is_unbindable` that blocks raw pointers. These new errors are actually a combination of the prior session's `is_unbindable()` change plus regeneration—they would've appeared even without our char16_t work. The prior session never finished testing that change.
> 
> I can either fix the char and lifetime issues properly in the binding generator, or temporarily revert the `is_unbindable` changes to isolate just the char16_t and normalization fixes that the user actually asked for.
> 
> Let me focus on the immediate issue: `crate::ffi::char` is being emitted because `Type::Class("char")` falls through to generic class handling when it's not the special-cased `const char*` pattern. The real fix is to add `char` as a proper type mapping to `c_char` instead of treating it as a generic class.
> 
> The other issues—missing lifetimes in `Option<&mut T>` and `char*` return types—are all symptoms of unsafe function handling not properly converting references to raw pointers. Rather than tackle the binding generator's complex unsafe handling right now, I should focus on what the user actually asked for: fixing the char16_t and I16 issues. The rest of these problems came from incomplete work in the prior session, so I'll address them directly.
> 
> Now I need to examine how the reexport layer constructs return types for these functions to understand where the conversion should happen.

These errors come from the prior session's incomplete unsafe fn work (modifying `is_unbindable()` to allow raw pointers through). Previously these functions were skipped. Now they're generated but the reexport code doesn't handle raw pointer returns correctly. Let me investigate and fix both issues.

Research reexport codegen for pointers

<details>
<summary>Searched for regex <code>reexport|Option.*&amp;mut|raw_pointer|type_to_ffi|type_to_rust|char.*ffi|is_unbindable|needs_unsafe|has_unsafe|MutPtr|ConstPtr</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 20 results</summary>

- [rust.rs:79](../crates/opencascade-binding-generator/src/codegen/rust.rs#L79)
- [rust.rs:97](../crates/opencascade-binding-generator/src/codegen/rust.rs#L97)
- [rust.rs:98](../crates/opencascade-binding-generator/src/codegen/rust.rs#L98)
- [rust.rs:640](../crates/opencascade-binding-generator/src/codegen/rust.rs#L640)
- [rust.rs:649](../crates/opencascade-binding-generator/src/codegen/rust.rs#L649)
- [rust.rs:663](../crates/opencascade-binding-generator/src/codegen/rust.rs#L663)
- [rust.rs:675](../crates/opencascade-binding-generator/src/codegen/rust.rs#L675)
- [rust.rs:682](../crates/opencascade-binding-generator/src/codegen/rust.rs#L682)
- [rust.rs:688](../crates/opencascade-binding-generator/src/codegen/rust.rs#L688)
- [rust.rs:741](../crates/opencascade-binding-generator/src/codegen/rust.rs#L741)
- [rust.rs:828](../crates/opencascade-binding-generator/src/codegen/rust.rs#L828)
- [rust.rs:832](../crates/opencascade-binding-generator/src/codegen/rust.rs#L832)
- [rust.rs:837](../crates/opencascade-binding-generator/src/codegen/rust.rs#L837)
- [rust.rs:843](../crates/opencascade-binding-generator/src/codegen/rust.rs#L843)
- [rust.rs:845](../crates/opencascade-binding-generator/src/codegen/rust.rs#L845)
- [rust.rs:858](../crates/opencascade-binding-generator/src/codegen/rust.rs#L858)
- [rust.rs:864](../crates/opencascade-binding-generator/src/codegen/rust.rs#L864)
- [rust.rs:868](../crates/opencascade-binding-generator/src/codegen/rust.rs#L868)
- [rust.rs:871](../crates/opencascade-binding-generator/src/codegen/rust.rs#L871)
</details>

<details>
<summary>Searched for regex <code>reexport|type_to_ffi|type_to_rust|raw_pointer|emit_reexport|is_unsafe|Option.*&amp;mut|char.*ffi|MutPtr|ConstPtr</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:123](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L123)
- [bindings.rs:154](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L154)
- [bindings.rs:200](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L200)
- [bindings.rs:223](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L223)
- [bindings.rs:298](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L298)
- [bindings.rs:311](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L311)
- [bindings.rs:334](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L334)
- [bindings.rs:355](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L355)
- [bindings.rs:375](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L375)
- [bindings.rs:414](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L414)
- [bindings.rs:425](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L425)
- [bindings.rs:448](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L448)
- [bindings.rs:449](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L449)
- [bindings.rs:451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L451)
- [bindings.rs:455](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L455)
- [bindings.rs:458](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L458)
- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)
- [bindings.rs:471](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L471)
- [bindings.rs:615](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L615)
</details>

<details>
<summary>Searched for regex <code>is_unbindable|needs_unsafe|has_unsafe|is_raw_pointer|const char|raw pointer</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 20 results</summary>

- [model.rs:50](../crates/opencascade-binding-generator/src/model.rs#L50)
- [model.rs:54](../crates/opencascade-binding-generator/src/model.rs#L54)
- [model.rs:61](../crates/opencascade-binding-generator/src/model.rs#L61)
- [model.rs:63](../crates/opencascade-binding-generator/src/model.rs#L63)
- [model.rs:64](../crates/opencascade-binding-generator/src/model.rs#L64)
- [model.rs:68](../crates/opencascade-binding-generator/src/model.rs#L68)
- [model.rs:231](../crates/opencascade-binding-generator/src/model.rs#L231)
- [model.rs:234](../crates/opencascade-binding-generator/src/model.rs#L234)
- [model.rs:235](../crates/opencascade-binding-generator/src/model.rs#L235)
- [model.rs:236](../crates/opencascade-binding-generator/src/model.rs#L236)
- [model.rs:267](../crates/opencascade-binding-generator/src/model.rs#L267)
- [model.rs:268](../crates/opencascade-binding-generator/src/model.rs#L268)
- [model.rs:273](../crates/opencascade-binding-generator/src/model.rs#L273)
- [model.rs:280](../crates/opencascade-binding-generator/src/model.rs#L280)
- [model.rs:282](../crates/opencascade-binding-generator/src/model.rs#L282)
- [model.rs:283](../crates/opencascade-binding-generator/src/model.rs#L283)
- [model.rs:288](../crates/opencascade-binding-generator/src/model.rs#L288)
- [model.rs:350](../crates/opencascade-binding-generator/src/model.rs#L350)
- [model.rs:351](../crates/opencascade-binding-generator/src/model.rs#L351)
- [model.rs:356](../crates/opencascade-binding-generator/src/model.rs#L356)
</details>

<details>
<summary>Reading lines 60-130 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L60-L131">rust.rs</a></summary>

```

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, void ptrs, etc.) — but NOT streams,
    // which are manually-defined opaque types that we DO want to collect.
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "u16" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// =============================================================================
// FFI MODULE GENERATION
// =============================================================================
//
// These functions generate the FFI module containing ALL types,
// plus per-module re-export files. This avoids cross-module type filtering
// issues and simplifies the architecture.

/// Generate the ffi.rs file containing ALL types from all modules
///
/// This generates extern "C" declarations with all types using full C++ names
/// (e.g., gp_Pnt, TopoDS_Shape) to avoid collisions and make the mapping obvious.
```
</details>

<details>
<summary>Reading lines 620-740 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L620-L741">rust.rs</a></summary>

```
    // Source attribution + doc comment
    let source_attr = format_source_attribution(
        &func.source_header,
        func.source_line,
        &format!("{}::{}", func.namespace, func.short_name),
    );
    writeln!(output, "/// {}", source_attr).unwrap();
    if let Some(ref comment) = func.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }

    // Build parameter list using pre-computed re-export types
    let params: Vec<String> = func.params.iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
        .collect();

    // Build args with .into() for enum params, CString conversion for &str params,
    // and &mut i32 local for &mut enum params
    let args: Vec<String> = func.params.iter()
        .map(|p| {
            if p.mut_ref_enum_rust_type.is_some() {
                format!("&mut {}_i32_", p.rust_name)
            } else if p.rust_reexport_type == "&str" {
                format!("c_{}.as_ptr()", p.rust_name)
            } else if p.enum_rust_type.is_some() {
                format!("{}.into()", p.rust_name)
            } else {
                p.rust_name.clone()
            }
        })
        .collect();

    // Generate prelude for CString (&str) params and &mut enum params
    let prelude: String = func.params.iter()
        .map(|p| {
            let mut s = String::new();
            if p.rust_reexport_type == "&str" {
                s.push_str(&format!("    let c_{} = std::ffi::CString::new({}).unwrap();\n", p.rust_name, p.rust_name));
            }
            if p.mut_ref_enum_rust_type.is_some() {
                s.push_str(&format!("    let mut {}_i32_: i32 = (*{}).into();\n", p.rust_name, p.rust_name));
            }
            s
        })
        .collect();

    // Build return type string
    let return_type_str = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", rt.rust_reexport_type))
        .unwrap_or_default();

    // Build call expression
    let call_expr = format!("crate::ffi::{}({})", func.cpp_wrapper_name, args.join(", "));

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
            call_expr
        }
    } else {
        call_expr
    };

    // Generate postamble for &mut enum writeback
    let postamble: String = func.params.iter()
        .filter_map(|p| {
            p.mut_ref_enum_rust_type.as_ref().map(|enum_type| {
                format!("    *{} = {}::try_from({}_i32_).unwrap();\n", p.rust_name, enum_type, p.rust_name)
            })
        })
        .collect();

    let has_return = !return_type_str.is_empty();

    let unsafe_kw = if func.is_unsafe { "unsafe " } else { "" };
    writeln!(output, "pub {}fn {}({}){} {{", unsafe_kw, func.rust_ffi_name, params.join(", "), return_type_str).unwrap();
    write!(output, "{}", prelude).unwrap();

    if postamble.is_empty() {
        writeln!(output, "    unsafe {{ {} }}", body).u
... (truncated, 4737 chars)
```
</details>

<details>
<summary>Reading lines 741-900 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L741-L901">rust.rs</a></summary>

```
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    module_fn_bindings: &[&super::bindings::FunctionBinding],
    module_skipped_fns: &[&super::bindings::SkippedSymbol],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");

    // Generate re-exports for free functions from pre-computed FunctionBindings.
    for func in module_fn_bindings {
        // All free functions become real wrapper functions (not pub use re-exports)
        // so IDE "go to definition" lands in the public module.
        emit_free_function_wrapper(&mut output, func);
    }

    if !module_fn_bindings.is_empty() {
        output.push('\n');
    }

    // Generate Rust enum definitions for enums in this module
    let rust_module = crate::module_graph::module_to_rust_name(module_name);
    if let Some(enum_ids) = symbol_table.enums_by_module.get(&rust_module) {
        for enum_id in enum_ids {
            if let Some(resolved_enum) = symbol_table.enums.get(enum_id) {
                if !matches!(resolved_enum.status, crate::resolver::BindingStatus::Included) {
                    continue;
                }
                emit_rust_enum(&mut output, resolved_enum);
            }
        }
    }

    // Re-export collection types belonging to this module
    for coll in collections {
        output.push_str(&format!(
            "pub use crate::ffi::{} as {};\n",
            coll.typedef_name, coll.short_name
        ));
    }
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
... (truncated, 6619 chars)
```
</details>

<details>
<summary>Reading lines 100-250 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L100-L251">bindings.rs</a></summary>

```

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
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/return
... (truncated, 5867 chars)
```
</details>

<details>
<summary>Reading lines 280-500 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L280-L501">bindings.rs</a></summary>

```
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
    pub is_class_ptr
... (truncated, 9382 chars)
```
</details>

<details>
<summary>Reading lines 580-660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L580-L661">bindings.rs</a></summary>

```
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

/// Convert a Type to C++ type string
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
            Type::U16 => "char16_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
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


/// Convert a Type to C++ parameter type for extern "C" wrapper functions (pointers instead of references)
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "const char*".to_string()
        }
        _ => type_to_cpp(ty),
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
```
</details>

<details>
<summary>Reading lines 425-510 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L425-L511">bindings.rs</a></summary>

```
fn type_to_ffi_full_name(ty: &Type) -> String {
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
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else {
                Type::ffi_safe_class_name(name) // Parent::Nested -> Parent_Nested
            }
        }
        Type::Handle(name) => format!("Handle{}", name.replace("_", "")),
        Type::ConstRef(inner) => format!("*const {}", type_to_ffi_full_name(inner)),
        Type::MutRef(inner) => format!("*mut {}", type_to_ffi_full_name(inner)),
        Type::RValueRef(_) => "()".to_string(),
        Type::ConstPtr(inner) => {
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "*const std::ffi::c_char".to_string()
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
            format!("*mut {}", Type::ffi_safe_class_name(name))
        }
        Type::Handle(name) => {
            format!("*mut Handle{}", name.replace("_", ""))
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
```
</details>

<details>
<summary>Reading lines 280-425 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L280-L426">bindings.rs</a></summary>

```
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
    pub is_class_ptr
... (truncated, 6601 chars)
```
</details>

Searched for regex `fn emit_reexport_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:4762](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4762)

<details>
<summary>Reading lines 4762-5000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4762-L5001">bindings.rs</a></summary>

```
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
        let self_param = if wm.is
... (truncated, 9254 chars)
```
</details>

<details>
<summary>Searched for regex <code>fn type_to_reexport|fn reexport_return_type|rust_reexport_type|class_ptr_return|is_class_ptr_return</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 40 results</summary>

- [bindings.rs:311](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L311)
- [bindings.rs:334](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L334)
- [bindings.rs:344](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L344)
- [bindings.rs:355](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L355)
- [bindings.rs:375](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L375)
- [bindings.rs:383](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L383)
- [bindings.rs:1137](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1137)
- [bindings.rs:1163](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1163)
- [bindings.rs:1179](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1179)
- [bindings.rs:1208](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1208)
- [bindings.rs:1224](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1224)
- [bindings.rs:1253](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1253)
- [bindings.rs:1280](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1280)
- [bindings.rs:1288](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1288)
- [bindings.rs:1308](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1308)
- [bindings.rs:1313](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1313)
- [bindings.rs:1330](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1330)
- [bindings.rs:1337](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1337)
- [bindings.rs:1342](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1342)
- [bindings.rs:1348](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1348)
- [bindings.rs:1354](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1354)
- [bindings.rs:1359](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1359)
- [bindings.rs:1763](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1763)
- [bindings.rs:1764](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1764)
- [bindings.rs:1847](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1847)
- [bindings.rs:1848](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1848)
- [bindings.rs:2812](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2812)
- [bindings.rs:2839](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2839)
- [bindings.rs:2852](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2852)
- [bindings.rs:2879](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2879)
- [bindings.rs:2902](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2902)
- [bindings.rs:2941](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2941)
- [bindings.rs:2964](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2964)
- [bindings.rs:2973](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2973)
- [bindings.rs:2981](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2981)
</details>

<details>
<summary>Searched for regex <code>fn is_unbindable|fn needs_unsafe_fn|fn has_unsafe_types|fn is_raw_pointer|const char|class_ptr_inner_name</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 27 results</summary>

- [model.rs:50](../crates/opencascade-binding-generator/src/model.rs#L50)
- [model.rs:63](../crates/opencascade-binding-generator/src/model.rs#L63)
- [model.rs:64](../crates/opencascade-binding-generator/src/model.rs#L64)
- [model.rs:231](../crates/opencascade-binding-generator/src/model.rs#L231)
- [model.rs:235](../crates/opencascade-binding-generator/src/model.rs#L235)
- [model.rs:236](../crates/opencascade-binding-generator/src/model.rs#L236)
- [model.rs:268](../crates/opencascade-binding-generator/src/model.rs#L268)
- [model.rs:273](../crates/opencascade-binding-generator/src/model.rs#L273)
- [model.rs:282](../crates/opencascade-binding-generator/src/model.rs#L282)
- [model.rs:283](../crates/opencascade-binding-generator/src/model.rs#L283)
- [model.rs:288](../crates/opencascade-binding-generator/src/model.rs#L288)
- [model.rs:351](../crates/opencascade-binding-generator/src/model.rs#L351)
- [model.rs:364](../crates/opencascade-binding-generator/src/model.rs#L364)
- [model.rs:365](../crates/opencascade-binding-generator/src/model.rs#L365)
- [model.rs:433](../crates/opencascade-binding-generator/src/model.rs#L433)
- [model.rs:556](../crates/opencascade-binding-generator/src/model.rs#L556)
- [model.rs:603](../crates/opencascade-binding-generator/src/model.rs#L603)
- [model.rs:604](../crates/opencascade-binding-generator/src/model.rs#L604)
- [model.rs:607](../crates/opencascade-binding-generator/src/model.rs#L607)
- [model.rs:618](../crates/opencascade-binding-generator/src/model.rs#L618)
- [model.rs:622](../crates/opencascade-binding-generator/src/model.rs#L622)
- [model.rs:636](../crates/opencascade-binding-generator/src/model.rs#L636)
- [model.rs:687](../crates/opencascade-binding-generator/src/model.rs#L687)
- [model.rs:692](../crates/opencascade-binding-generator/src/model.rs#L692)
- [model.rs:698](../crates/opencascade-binding-generator/src/model.rs#L698)
- [model.rs:700](../crates/opencascade-binding-generator/src/model.rs#L700)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01B1AHy97YUGR7ga5LxpRJb9__vscode-1771312786072/content.txt)

<details>
<summary>Reading lines 550-720 of <a href="../crates/opencascade-binding-generator/src/model.rs#L550-L721">model.rs</a></summary>

```

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

    /// Check if this is a Standard_Address (void*) type
    /// These can't be bound through the FFI
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
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
                    Type::Class(name) if name != "char" => Some(name.as_str()),
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
             
... (truncated, 7442 chars)
```
</details>

<details>
<summary>Reading lines 1260-1380 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1260-L1381">bindings.rs</a></summary>

```
            is_class_ptr: true,
        };
    }

    // By-value class/handle params — opaque types
    // must be passed by reference. We convert them to const T& at the FFI
    // boundary; the C++ wrapper passes the reference to the original method
    // which accepts by value (C++ handles the implicit copy).
    let effective_ty = match ty {
        Type::Class(name) if name != "char" && !ffi_ctx.all_enums.contains(name) => {
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
    let cpp_type = type_to_cpp_extern_c_param(&effective_ty);
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
        let cpp_type = type_to_cpp(ty);
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
    let cpp_type = type_to_cpp(ty);
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
fn compute_dir
... (truncated, 4536 chars)
```
</details>

<details>
<summary>Reading lines 590-710 of <a href="../crates/opencascade-binding-generator/src/model.rs#L590-L711">model.rs</a></summary>

```

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
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
                    Type::Class(name) if name != "char" => Some(name.as_str()),
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
    /// Rvalue re
... (truncated, 5453 chars)
```
</details>

Searched for regex `fn build_reexport_body|fn type_to_rust_string|fn return_type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:682](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L682)
- [bindings.rs:726](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L726)
- [bindings.rs:4732](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4732)

<details>
<summary>Reading lines 660-780 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L660-L781">bindings.rs</a></summary>

```
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
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
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
        Type::Class(name) if name != "char" && name != "Standard_Address" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
       
... (truncated, 4947 chars)
```
</details>

<details>
<summary>Reading lines 4732-4762 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4732-L4763">bindings.rs</a></summary>

```
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
        if rt == "String" {
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
```
</details>

<details>
<summary>Reading lines 4990-5100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4990-L5101">bindings.rs</a></summary>

```
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
                if sm.needs_static_lifetime
                    && ty_str.starts_with('&')
                    && !ty_str.contains("'static")
                {
                    ty_str = ty_str.replacen('&', "&'static ", 1);
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
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
        impl_methods.push(format!(
            "    /// Wrap in a Handle (reference-counted smart pointer)\n    pub fn to_handle(obj: crate::OwnedPtr<Self>) -> crate::OwnedPtr<crate::ffi::{}> {{\n        unsafe {{ crate::OwnedPtr::from_raw(crate::f
... (truncated, 4625 chars)
```
</details>

<details>
<summary>Reading lines 5100-5200 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5100-L5201">bindings.rs</a></summary>

```
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
        let unsafe_kw = if im.is_unsafe { "unsafe " } else { "" };
        impl_methods.push(format!(
            "{}    pub {}fn {}({}){} {{\n{}        {}\n    }}\n",
            format_reexport_doc(
                &format!("Inherited: {}", format_source_attribution(
                    &im.source_header,
                    im.source_line,
                    &format!("{}::{}()", im.source_class, im.cpp_method_name),
                )),
                &no_doc,
            ),
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
        let handle_type_name = format!("Handle{}", cn.replace("_", ""));
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
           
... (truncated, 4879 chars)
```
</details>

<details>
<summary>Reading lines 340-395 of <a href="../crates/opencascade-binding-generator/src/model.rs#L340-L396">model.rs</a></summary>

```
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
```
</details>

Searched for regex `needs_static_lifetime` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:217](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L217)
- [bindings.rs:1923](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1923)
- [bindings.rs:1935](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1935)
- [bindings.rs:5003](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5003)

<details>
<summary>Reading lines 1910-1960 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1910-L1961">bindings.rs</a></summary>

```
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
```
</details>

Searched for regex `crate::ffi::char` (`**/crates/opencascade-sys/generated/**`), 2 results
- [ldom.rs:640](../crates/opencascade-sys/generated/ldom.rs#L640)
- [ldom.rs:649](../crates/opencascade-sys/generated/ldom.rs#L649)

<details>
<summary>Searched for regex <code>Option&lt;&amp;mut.*&gt;.*\{</code> (<code>**/crates/opencascade-sys/generated/**</code>), 10 results</summary>

- [hlrb_rep.rs:3199](../crates/opencascade-sys/generated/hlrb_rep.rs#L3199)
- [math.rs:5226](../crates/opencascade-sys/generated/math.rs#L5226)
- [math.rs:5240](../crates/opencascade-sys/generated/math.rs#L5240)
- [math.rs:5253](../crates/opencascade-sys/generated/math.rs#L5253)
- [ldom.rs:640](../crates/opencascade-sys/generated/ldom.rs#L640)
- [ldom.rs:649](../crates/opencascade-sys/generated/ldom.rs#L649)
- [osd.rs:6469](../crates/opencascade-sys/generated/osd.rs#L6469)
- [osd.rs:6482](../crates/opencascade-sys/generated/osd.rs#L6482)
- [osd.rs:6495](../crates/opencascade-sys/generated/osd.rs#L6495)
- [b_rep_builder_api.rs:730](../crates/opencascade-sys/generated/b_rep_builder_api.rs#L730)
</details>

<details>
<summary>Reading lines 630-660 of <a href="../crates/opencascade-sys/generated/ldom.rs#L630-L661">ldom.rs</a></summary>

```
    /// **Source:** `LDOM_CharReference.hxx` - `LDOM_CharReference::LDOM_CharReference()`
    /// Default constructor
    pub fn new() -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::LDOM_CharReference_ctor()) }
    }

    /// **Source:** `LDOM_CharReference.hxx`:47 - `LDOM_CharReference::Decode()`
    pub unsafe fn decode(
        theSrc: *mut std::ffi::c_char,
        theLen: &mut i32,
    ) -> Option<&mut crate::ffi::char> {
        unsafe { crate::ffi::LDOM_CharReference_decode(theSrc, theLen) }
    }

    /// **Source:** `LDOM_CharReference.hxx`:49 - `LDOM_CharReference::Encode()`
    pub unsafe fn encode(
        theSrc: &str,
        theLen: &mut i32,
        isAttribute: bool,
    ) -> Option<&mut crate::ffi::char> {
        let c_theSrc = std::ffi::CString::new(theSrc).unwrap();
        unsafe { crate::ffi::LDOM_CharReference_encode(c_theSrc.as_ptr(), theLen, isAttribute) }
    }
}

// ========================
// From LDOM_CharacterData.hxx
// ========================

/// **Source:** `LDOM_CharacterData.hxx`:26 - `LDOM_CharacterData`
pub use crate::ffi::LDOM_CharacterData as CharacterData;
```
</details>

<details>
<summary>Reading lines 6460-6510 of <a href="../crates/opencascade-sys/generated/osd.rs#L6460-L6511">osd.rs</a></summary>

```
    /// with the predefined handler LogFileHandler, which static instance
    /// is returned by GetLogFileHandler().
    /// To clear the handler, pass NULL here.
    pub fn set_callback(theCB: &mut MAllocHook_Callback) {
        unsafe { crate::ffi::OSD_MAllocHook_set_callback(theCB as *mut _) }
    }

    /// **Source:** `OSD_MAllocHook.hxx`:161 - `OSD_MAllocHook::GetCallback()`
    /// Get current handler of allocation/deallocation events
    pub unsafe fn get_callback() -> Option<&mut MAllocHook_Callback> {
        {
            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_callback() };
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { &mut *ptr })
            }
        }
    }

    /// **Source:** `OSD_MAllocHook.hxx`:164 - `OSD_MAllocHook::GetLogFileHandler()`
    /// Get static instance of LogFileHandler handler
    pub unsafe fn get_log_file_handler() -> Option<&mut MAllocHook_LogFileHandler> {
        {
            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_log_file_handler() };
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { &mut *ptr })
            }
        }
    }

    /// **Source:** `OSD_MAllocHook.hxx`:167 - `OSD_MAllocHook::GetCollectBySize()`
    /// Get static instance of CollectBySize handler
    pub unsafe fn get_collect_by_size() -> Option<&mut MAllocHook_CollectBySize> {
        {
            let ptr = unsafe { crate::ffi::OSD_MAllocHook_get_collect_by_size() };
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { &mut *ptr })
            }
        }
    }
}

/// **Source:** `OSD_MAllocHook.hxx`:34 - `OSD_MAllocHook_Callback`
///
/// Interface of a class that should handle allocation/deallocation events
pub use crate::ffi::OSD_MAllocHook_Callback as MAllocHook_Callback;
```
</details>

Excellent research. Let me now fix both issues.Let me read the relevant sections:

<details>
<summary>Reading lines 726-775 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L726-L776">bindings.rs</a></summary>

```
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::Handle{}>",
                name.replace("_", "")
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
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

```
</details>

<details>
<summary>Reading lines 4990-5020 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4990-L5021">bindings.rs</a></summary>

```
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
                if sm.needs_static_lifetime
                    && ty_str.starts_with('&')
                    && !ty_str.contains("'static")
                {
                    ty_str = ty_str.replacen('&', "&'static ", 1);
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
```
</details>

<details>
<summary>Reading lines 1910-1935 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1910-L1936">bindings.rs</a></summary>

```
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
                .map(|ty| ty.is_reference())
                .unwrap_or(false);

            StaticMethodBinding {
                ffi_fn_name,
                impl_method_name: impl_method_name.clone(),
                params,
                return_type,
                cpp_method_name: method.name.clone(),
                needs_static_lifetime,
```
</details>

Now I have the full picture. Let me fix both issues:

**Fix 1:** `MutPtr(Class("char"))` should not match the class pointer arm - add a  guard (same as ConstPtr has on line 742). 

**Fix 2:** `needs_static_lifetime` check and the `'static` insertion for `Option<&...>` patterns.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L758-L759">bindings.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1923-L1928">-5+5</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5002-L5010">-7+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "758:ng",
      "content": "        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\") => {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1923:gh,1924:af,1925:od,1926:dx,1927:dk",
      "content": "            let needs_static_lifetime = method\n                .return_type\n                .as_ref()\n                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\")))\n                .unwrap_or(false);"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "5002:cs,5003:gk,5004:zj,5005:id,5006:mn,5007:qk,5008:gl",
      "content": "                let mut ty_str = rt.rust_reexport_type.clone();\n                if sm.needs_static_lifetime {\n                    if ty_str.starts_with('&') && !ty_str.contains(\"'static\") {\n                        ty_str = ty_str.replacen('&', \"&'static \", 1);\n                    } else if ty_str.starts_with(\"Option<&\") && !ty_str.contains(\"'static\") {\n                        ty_str = ty_str.replacen(\"Option<&\", \"Option<&'static \", 1);\n                    }\n                }"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 758,
      "endLine": 5009
    }
  ]
}
```
</details>

Now I also need to check if the same lifetime issue exists in `rust.rs` for free functions:

Searched for regex `needs_static_lifetime|Option.*static` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

Searched for regex `static.*lifetime|Option.*&` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result
- [rust.rs:11](../crates/opencascade-binding-generator/src/codegen/rust.rs#L11)

No lifetime handling in `crates/opencascade-binding-generator/src/codegen/rust.rs` for free functions. Let me check if free functions can return class pointers:

<details>
<summary>Searched for regex <code>return_type|fn emit</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 12 results</summary>

- [rust.rs:56](../crates/opencascade-binding-generator/src/codegen/rust.rs#L56)
- [rust.rs:66](../crates/opencascade-binding-generator/src/codegen/rust.rs#L66)
- [rust.rs:384](../crates/opencascade-binding-generator/src/codegen/rust.rs#L384)
- [rust.rs:538](../crates/opencascade-binding-generator/src/codegen/rust.rs#L538)
- [rust.rs:614](../crates/opencascade-binding-generator/src/codegen/rust.rs#L614)
- [rust.rs:674](../crates/opencascade-binding-generator/src/codegen/rust.rs#L674)
- [rust.rs:682](../crates/opencascade-binding-generator/src/codegen/rust.rs#L682)
- [rust.rs:683](../crates/opencascade-binding-generator/src/codegen/rust.rs#L683)
- [rust.rs:714](../crates/opencascade-binding-generator/src/codegen/rust.rs#L714)
- [rust.rs:717](../crates/opencascade-binding-generator/src/codegen/rust.rs#L717)
- [rust.rs:905](../crates/opencascade-binding-generator/src/codegen/rust.rs#L905)
</details>

<details>
<summary>Reading lines 670-690 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L670-L691">rust.rs</a></summary>

```
        })
        .collect();

    // Build return type string
    let return_type_str = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", rt.rust_reexport_type))
        .unwrap_or_default();

    // Build call expression
    let call_expr = format!("crate::ffi::{}({})", func.cpp_wrapper_name, args.join(", "));

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
```
</details>

Free functions don't have the same `needs_static_lifetime` logic. Free functions returning `Option<&T>` would also have the lifetime issue. Let me check if any free functions actually return class pointers:

**Terminal:**
```sh
grep -n "Option<&" crates/opencascade-sys/generated/*.rs | grep -v "impl \|fn.*(&\|'static\|'_" | grep "pub fn\|pub unsafe fn" | head -20
```
```
crates/opencascade-sys/generated/osd.rs:6469:    pub unsafe fn get_callback() -> Option<&mut MAllocHook_Callback> {
crates/opencascade-sys/generated/osd.rs:6482:    pub unsafe fn get_log_file_handler() -> Option<&mut MAllocHook_LogFileHandler> {
crates/opencascade-sys/generated/osd.rs:6495:    pub unsafe fn get_collect_by_size() -> Option<&mut MAllocHook_CollectBySize> {
```

Only the 3 static methods in osd.rs. My fix to the static method lifetime logic should cover these. Now let me also check the `MutPtr(Class("char"))` pattern - I need to make sure there's a fallback for it in `return_type_to_rust_string`. After my fix, `MutPtr(Class("char"))` won't match the class pointer arm, so it'll fall through to `type_to_rust_string`:

Searched for regex `fn type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:682](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L682)

<details>
<summary>Reading lines 682-727 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L682-L728">bindings.rs</a></summary>

```
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
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::Handle{}", name.replace("_", "")),
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
```
</details>

When `MutPtr(Class("char"))` falls through to  (line 770 `_ => type_to_rust_string(ty, reexport_ctx)`), it will hit line 721: , and then  on line 698 returns `"std::ffi::c_char"`. So the result will be `"*mut std::ffi::c_char"` - exactly what we want.

Now let me also check that `char*` parameters are handled properly too. Let me check `build_param_binding`:

Searched for text `fn build_param_binding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1119](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1119)

<details>
<summary>Reading lines 1119-1180 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1119-L1181">bindings.rs</a></summary>

```
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
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
```
</details>

The parameter handling looks like it should be fine for `char*` since it goes through  for non-nullable raw pointer params.

Now let me rebuild and test:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.13s
real  2.2s
```

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- ✅ Fix crate::ffi::char error
- ✅ Fix missing lifetime errors
- 🔄 Regenerate & full build
- ⬜ Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "error|complete|generated"
```

**Terminal:**
```sh
tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_01EZcinXVdVLxcKXJXL8fA1p__vscode-1771312786106/content.txt
```
<details>
<summary>Output (5 lines)</summary>

```
  Formatting complete.
Code generation complete!
  263 modules generated
Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  44.6s
```
</details>

Good, regeneration successful. Now full build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds with only a pre-existing warning about `Message_ProgressScope::NullString`. Now let me run the tests.

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- ✅ Fix crate::ffi::char error
- ✅ Fix missing lifetime errors
- ✅ Regenerate & full build
- 🔄 Run tests

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -E "test result:|error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5193ea57-bcb9-4f12-a62c-4e21315e9d23/toolu_014Q4Tw8SEsqgYrMGeQVojo9__vscode-1771312786111/content.txt
```
<details>
<summary>Output (18 lines)</summary>

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 22 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; fin
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
```
</details>

All tests pass. Let me mark everything complete.

**Todo list:**
- ✅ Fix U16 → char16_t emission
- ✅ Rebuild binding generator
- ✅ Fix crate::ffi::char error
- ✅ Fix missing lifetime errors
- ✅ Regenerate & full build
- ✅ Run tests

Everything builds and all tests pass. Here's a summary of the three changes from this session (continuing the prior session's work):

**1.  C++ emission: `uint16_t` → `char16_t`** (3 files)
- `crates/opencascade-binding-generator/src/model.rs#L711`, `crates/opencascade-binding-generator/src/codegen/bindings.rs#L603``crates/opencascade-binding-generator/src/resolver.rs#L1382`: Changed `to_cpp_string()` / `type_to_cpp()` for  from `"uint16_t"` to `"char16_t"`, matching what OCCT's `Standard_ExtCharacter` actually is on clang.

**2. `MutPtr(Class("char"))` guard in `return_type_to_rust_string`** (`crates/opencascade-binding-generator/src/codegen/bindings.rs#L758`)
- Added  guard to the `MutPtr(Class(_))` arm, so `char*` returns fall through to  → `*mut std::ffi::c_char` instead of `Option<&mut crate::ffi::char>`.

**3. `'static` lifetime for static method `Option<&T>` returns** (`crates/opencascade-binding-generator/src/codegen/bindings.rs#L1923-L1927` and `crates/opencascade-binding-generator/src/codegen/bindings.rs#L5002-L5009`)
- Extended `needs_static_lifetime` to also be true for class pointer returns (not just C++ references)
- Extended the `'static` insertion to handle `Option<&...>` patterns (not just bare `&...`)

*2026-02-20 08:56 · 1201s · Words: 18,407 in · 4,028 out · 47,824 ctx · 70 rounds*

---

<a id="user-8"></a>

## User (8)

> commit

*2026-02-20 09:09*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants to commit the current changes. Let me check what's changed and write a commit message.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat HEAD
```
<details>
<summary>Output (47 lines)</summary>

```
 .../src/codegen/bindings.rs                        |   84 +-
 .../src/codegen/rust.rs                            |    3 +-
 crates/opencascade-binding-generator/src/model.rs  |   69 +-
 crates/opencascade-binding-generator/src/parser.rs |   24 +-
 .../opencascade-binding-generator/src/resolver.rs  |    3 +-
 .../src/type_mapping.rs                            |   16 +
 crates/opencascade-sys/generated/adaptor3d.rs      |    2 +-
 crates/opencascade-sys/generated/adv_app2_var.rs   | 1236 +++++++++++++++-----
 crates/opencascade-sys/generated/adv_approx.rs     |    4 +-
 crates/opencascade-sys/generated/b_rep_approx.rs   |    4 +-
 crates/opencascade-sys/generated/b_rep_g_prop.rs   |   86 +-
 crates/opencascade-sys/generated/b_rep_mesh.rs     |   39 +-
 crates/opencascade-sys/generated/b_rep_prim_api.rs |   12 +-
 .../opencascade-sys/generated/b_rep_top_adaptor.rs |    2 +-
 crates/opencascade-sys/generated/b_spl_c_lib.rs    |   31 +-
 crates/opencascade-sys/generated/c_pnts.rs         |    8 +-
 crates/opencascade-sys/generated/extrema.rs        |   12 +-
 crates/opencascade-sys/generated/ffi.rs            | 1092 +++++++++++++++--
 crates/opencascade-sys/generated/geom_int.rs       |    4 +-
 crates/opencascade-sys/generated/gp.rs             |   32 +-
 crates/opencascade-sys/generated/graphic3d.rs      |    4 +-
 crates/opencascade-sys/generated/hlr_algo.rs       |  105 +-
 crates/opencascade-sys/generated/hlrb_rep.rs       |  672 ++++++-----
 crates/opencascade-sys/generated/int_patch.rs      |    6 +-
 crates/opencascade-sys/generated/ldom.rs           |   27 +-
 crates/opencascade-sys/generated/lib.rs            |    1 +
 crates/opencascade-sys/generated/math.rs           |    8 +-
 crates/opencascade-sys/generated/message.rs        |    2 +-
 crates/opencascade-sys/generated/n_collection.rs   |   54 +-
 crates/opencascade-sys/generated/osd.rs            |   90 +-
 crates/opencascade-sys/generated/poly.rs           |   25 +-
 crates/opencascade-sys/generated/quantity.rs       |  235 ++--
 crates/opencascade-sys/generated/resource.rs       |   38 +-
 crates/opencascade-sys/generated/rw_gltf.rs        |   16 +-
 crates/opencascade-sys/generated/rw_obj.rs         |    2 +-
 crates/opencascade-sys/generated/rw_obj_tools.rs   |   10 +
 crates/opencascade-sys/generated/rw_ply.rs         |    2 +-
 crates/opencascade-sys/generated/standard.rs       |   56 +-
 crates/opencascade-sys/generated/step_file.rs
... (truncated, 3637 chars total)
```
</details>

> 💭 Let me write a commit message for all the changes.

Created [commit-msg.txt](../tmp/commit-msg.txt)

> 💭 Let me use replace_string_in_file to update the commit message, or just write to a new file with a different name.

**Terminal:**
```sh
cat > /Users/sethml/src/opencascade-rs/tmp/commit-msg2.txt << 'EOF'
Add Type::I16, fix char16_t vs uint16_t, clean up normalize_template_spelling

Binding generator changes:
- Add Type::I16 for signed 16-bit integers (short / int16_t), separate from
  Type::U16 (unsigned short / char16_t / Standard_ExtCharacter)
- Change Type::U16 C++ emission from uint16_t to char16_t, matching what
  OCCT's Standard_ExtCharacter actually is on modern clang
- Remove all Standard_* hardcoded replacements from normalize_template_spelling();
  rely on libclang's typedef resolution instead
- Change parser clang args from -std=c++17 to -std=c++14 to match the build
- Fix MutPtr(Class("char")) return type: emit *mut std::ffi::c_char instead of
  Option<&mut crate::ffi::char>
- Fix missing 'static lifetime for static methods returning Option<&T> /
  Option<&mut T> from class pointer returns

Regenerated bindings reflect all the above changes.
EOF
```

> 💭 Good, now let me commit using this file.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg2.txt
```
```
[main dde667e] Add Type::I16, fix char16_t vs uint16_t, clean up normalize_template_spelling
 47 files changed, 4004 insertions(+), 1446 deletions(-)
 create mode 100644 crates/opencascade-sys/generated/xb_rep_mesh.rs
```

> 💭 Done.

Committed as `dde667e`.

*2026-02-20 09:10 · 26s · Words: 1 in · 336 out · 55,244 ctx · 5 rounds*

---
