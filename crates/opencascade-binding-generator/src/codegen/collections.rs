//! Code generation for NCollection-based collection types
//!
//! Generates iterator wrappers and helper functions for OCCT collection typedefs like
//! TopTools_ListOfShape, TopTools_SequenceOfShape, etc.
//!
//! This module recognizes collection typedefs by pattern matching on their names
//! and generates the appropriate C++ and Rust wrapper code.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;

thread_local! {
    /// Maps a collection's mangled FFI name (e.g. `NCollection_IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher`)
    /// to a short, readable overload-suffix token (e.g. `imap_shape`). Populated by
    /// [`discover_collections`]; read by `model::Type::suffix_token` so overloaded
    /// methods taking collections get concise names rather than the full mangled type.
    static COLLECTION_SUFFIX_TOKENS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// The short overload-suffix token for a collection's mangled FFI name, if one is
/// registered (e.g. `NCollection_List_TopoDS_Shape` → `list_shape`).
pub fn collection_suffix_token(mangled: &str) -> Option<String> {
    COLLECTION_SUFFIX_TOKENS.with(|m| m.borrow().get(mangled).cloned())
}

/// Information about a collection typedef
#[derive(Debug, Clone)]
pub struct CollectionInfo {
    /// The mangled canonical Rust/C++ identifier used as the FFI type name
    /// (e.g., "NCollection_List_TopoDS_Shape"). This is what method signatures
    /// resolve to and what the generated opaque struct / FFI functions are named.
    pub typedef_name: String,
    /// The C++ template spelling this collection instantiates
    /// (e.g., "NCollection_List<TopoDS_Shape>"). Emitted as a C++ `typedef` to
    /// `typedef_name` so the existing generation can keep using the mangled name.
    pub cpp_spelling: String,
    /// The module this collection belongs to. Always "n_collection" for the
    /// canonical NCollection types.
    pub module: String,
    /// The short Rust name without module prefix (e.g., "List_TopoDS_Shape").
    pub short_name: String,
    /// The element type (for simple collections) or key type (for data maps)
    pub element_type: String,
    /// The element/key type's module (e.g., "topo_ds")
    pub element_module: String,
    /// The value type for data maps (None for simple collections)
    pub value_type: Option<String>,
    /// The value type's module for data maps
    pub value_module: Option<String>,
    /// The C++ header that defines the element/key type (from the parsed class's
    /// source header). `None` for primitive element types.
    pub element_header: Option<String>,
    /// The C++ header that defines the value type (for data maps). `None` for
    /// primitive value types or non-map collections.
    pub value_header: Option<String>,
    /// The kind of collection
    pub kind: CollectionKind,
}

/// The kind of NCollection template being wrapped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKind {
    /// NCollection_List - doubly-linked list, uses const_iterator
    List,
    /// NCollection_Sequence - dynamic array, 1-indexed access
    Sequence,
    /// NCollection_IndexedMap - set with integer index access
    IndexedMap,
    /// NCollection_Map - unordered set, uses const_iterator
    Map,
    /// NCollection_DataMap - key-value map, uses OCCT Iterator class
    DataMap,
    /// NCollection_IndexedDataMap - key-value map with index access
    IndexedDataMap,
    /// NCollection_Array1 - fixed-size 1D array (typedef of template instantiation)
    Array1,
    /// NCollection_Array2 - fixed-size 2D array (typedef of template instantiation)
    Array2,
    /// NCollection_HArray1 - handle-managed fixed-size 1D array (inherits Array1 + Transient)
    HArray1,
    /// NCollection_HSequence - handle-managed dynamic sequence (inherits Sequence + Transient)
    HSequence,
}

impl CollectionKind {
    /// Get a human-readable description of this collection kind
    pub fn description(&self) -> &'static str {
        match self {
            CollectionKind::List => "Doubly-linked list",
            CollectionKind::Sequence => "Dynamic array (1-indexed)",
            CollectionKind::IndexedMap => "Set with integer index access (1-indexed)",
            CollectionKind::Map => "Unordered set",
            CollectionKind::DataMap => "Key-value map",
            CollectionKind::IndexedDataMap => "Key-value map with index access (1-indexed)",
            CollectionKind::Array1 => "Fixed-size 1D array (1-indexed)",
            CollectionKind::Array2 => "Fixed-size 2D array (row/col indexed)",
            CollectionKind::HArray1 => "Handle-managed fixed-size 1D array (1-indexed)",
            CollectionKind::HSequence => "Handle-managed dynamic sequence (1-indexed)",
        }
    }

    /// Returns true if this is a data map (key-value) collection
    pub fn is_data_map(&self) -> bool {
        matches!(self, CollectionKind::DataMap | CollectionKind::IndexedDataMap)
    }

    /// The `NCollection_*` template base name for this kind.
    pub fn template_base(&self) -> &'static str {
        match self {
            CollectionKind::List => "NCollection_List",
            CollectionKind::Sequence => "NCollection_Sequence",
            CollectionKind::IndexedMap => "NCollection_IndexedMap",
            CollectionKind::Map => "NCollection_Map",
            CollectionKind::DataMap => "NCollection_DataMap",
            CollectionKind::IndexedDataMap => "NCollection_IndexedDataMap",
            CollectionKind::Array1 => "NCollection_Array1",
            CollectionKind::Array2 => "NCollection_Array2",
            CollectionKind::HArray1 => "NCollection_HArray1",
            CollectionKind::HSequence => "NCollection_HSequence",
        }
    }

    /// The OCCT header that defines this collection template.
    pub fn header(&self) -> String {
        format!("{}.hxx", self.template_base())
    }

    /// Parse a `NCollection_*` template base name into a kind. Longest-prefix aware
    /// so `NCollection_IndexedMap` is not mistaken for `NCollection_Map`.
    pub fn from_template_base(base: &str) -> Option<CollectionKind> {
        Some(match base {
            "NCollection_List" => CollectionKind::List,
            "NCollection_Sequence" => CollectionKind::Sequence,
            "NCollection_IndexedMap" => CollectionKind::IndexedMap,
            "NCollection_Map" => CollectionKind::Map,
            "NCollection_DataMap" => CollectionKind::DataMap,
            "NCollection_IndexedDataMap" => CollectionKind::IndexedDataMap,
            "NCollection_Array1" => CollectionKind::Array1,
            "NCollection_Array2" => CollectionKind::Array2,
            "NCollection_HArray1" => CollectionKind::HArray1,
            "NCollection_HSequence" => CollectionKind::HSequence,
            _ => return None,
        })
    }

    /// Whether this kind is a handle-managed (Transient) collection.
    pub fn is_handle(&self) -> bool {
        matches!(self, CollectionKind::HArray1 | CollectionKind::HSequence)
    }

    /// Short code used in overload-suffix tokens (e.g. `imap` for IndexedMap).
    pub fn short_code(&self) -> &'static str {
        match self {
            CollectionKind::List => "list",
            CollectionKind::Sequence => "seq",
            CollectionKind::IndexedMap => "imap",
            CollectionKind::Map => "map",
            CollectionKind::DataMap => "dmap",
            CollectionKind::IndexedDataMap => "idmap",
            CollectionKind::Array1 => "arr1",
            CollectionKind::Array2 => "arr2",
            CollectionKind::HArray1 => "harr1",
            CollectionKind::HSequence => "hseq",
        }
    }
}

/// The short overload-suffix leaf for an element/value type name: the type's leaf
/// (after the module prefix and any `::`), lowercased. e.g. `TopoDS_Shape` → `shape`,
/// `gp_Pnt` → `pnt`, `double` → `double`. Nested collection args recurse into a full
/// short token (`NCollection_List<double>` → `list_double`).
fn short_leaf(type_name: &str) -> String {
    if type_name.contains('<') {
        if let Some(inner) = parse_canonical_collection(type_name) {
            return suffix_token_for(&inner);
        }
        // Non-collection template arg — mangle and take the leaf.
        return short_leaf(&crate::config::template_alias_name(type_name));
    }
    let leaf = type_name.rsplit("::").next().unwrap_or(type_name);
    let leaf = leaf.strip_prefix("NCollection_").unwrap_or(leaf);
    match leaf.find('_') {
        Some(pos) => leaf[pos + 1..].to_lowercase(),
        None => leaf.to_lowercase(),
    }
}

/// Build the short overload-suffix token for a collection, e.g. `imap_shape`,
/// `arr1_pnt`, `dmap_shape_int`. Drops the hasher argument (never element/value).
fn suffix_token_for(info: &CollectionInfo) -> String {
    let mut token = format!("{}_{}", info.kind.short_code(), short_leaf(&info.element_type));
    if let Some(value) = &info.value_type {
        token.push('_');
        token.push_str(&short_leaf(value));
    }
    token
}


/// Maps a C++ element type to its Rust FFI representation.
/// Primitive types like Standard_Real map to f64, while compound types
/// like gp_Pnt2d stay as-is (they exist as opaque structs in the FFI).
pub fn rust_element_type(cpp_type: &str) -> &str {

    match cpp_type {
        "Standard_Real" | "double" => "f64",
        "Standard_Integer" | "int" => "i32",
        "Standard_Boolean" | "bool" => "bool",
        "Standard_ShortReal" | "float" => "f32",
        _ => cpp_type,
    }
}

/// Returns true if the element type is a C++ primitive that maps to a Rust native type.
pub fn is_primitive_element(cpp_type: &str) -> bool {

    matches!(
        cpp_type,
        "Standard_Real" | "double"
            | "Standard_Integer" | "int"
            | "Standard_Boolean" | "bool"
            | "Standard_ShortReal" | "float"
    )
}

/// Helper to extract module name from a type (e.g., "topo_ds" from "TopoDS_Shape")
fn type_to_module(type_name: &str) -> String {
    if let Some(underscore_pos) = type_name.find('_') {
        crate::module_graph::module_to_rust_name(&type_name[..underscore_pos])
    } else {
        type_name.to_lowercase()
    }
}

/// Split a template spelling `Base<A, B, ...>` into its base name and the list of
/// top-level template arguments (respecting nested `<>`). Returns `None` if the
/// spelling is not a `Base<...>` template.
fn split_template_args(spelling: &str) -> Option<(String, Vec<String>)> {
    let open = spelling.find('<')?;
    if !spelling.ends_with('>') {
        return None;
    }
    let base = spelling[..open].trim().to_string();
    let inner = &spelling[open + 1..spelling.len() - 1];
    let mut args = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in inner.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                args.push(inner[start..i].trim().to_string());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = inner[start..].trim();
    if !last.is_empty() {
        args.push(last.to_string());
    }
    Some((base, args))
}

/// Parse a canonical `NCollection_*<...>` template spelling into a `CollectionInfo`.
/// The mangled `typedef_name` is derived via `config::template_alias_name`, matching
/// the opaque type name that method signatures resolve to. Returns `None` for
/// spellings that are not recognized NCollection collection templates.
pub fn parse_canonical_collection(spelling: &str) -> Option<CollectionInfo> {
    let (base, args) = split_template_args(spelling)?;
    let kind = CollectionKind::from_template_base(&base)?;
    if args.is_empty() {
        return None;
    }

    let typedef_name = crate::config::template_alias_name(spelling);
    // Short name drops the "NCollection_" prefix, e.g. "List_TopoDS_Shape".
    let short_name = typedef_name
        .strip_prefix("NCollection_")
        .unwrap_or(&typedef_name)
        .to_string();

    let (element_type, value_type) = if kind.is_data_map() {
        // DataMap<Key, Value[, Hasher]> / IndexedDataMap<Key, Value[, Hasher]>
        let value = args.get(1)?.clone();
        (args[0].clone(), Some(value))
    } else {
        // List/Sequence/Array1/Array2/HArray1/HSequence<Elem>, Map/IndexedMap<Key[, Hasher]>
        (args[0].clone(), None)
    };

    Some(CollectionInfo {
        typedef_name,
        cpp_spelling: spelling.to_string(),
        module: "n_collection".to_string(),
        short_name,
        element_module: type_to_module(&element_type),
        value_module: value_type.as_deref().map(type_to_module),
        element_type,
        value_type,
        element_header: None,
        value_header: None,
        kind,
    })
}

/// A curated seed of canonical collection spellings to always materialize, even if
/// they are not (yet) auto-discovered from method signatures. Kept small — most
/// collections are discovered automatically via [`discover_collections`].
fn seed_collection_spellings() -> &'static [&'static str] {
    &[
        "NCollection_Array1<gp_Pnt>",
        "NCollection_Array1<double>",
        "NCollection_List<TopoDS_Shape>",
        "NCollection_Sequence<TopoDS_Shape>",
        // Handle-managed collections. These are also declared as handle-able template
        // instantiations in bindings.toml (which registers them at parse time so
        // `Handle(NCollection_HArray1<gp_Pnt>)` parameters resolve); the collection
        // owns the base type, element methods, and `to_handle`.
        "NCollection_HArray1<gp_Pnt>",
        "NCollection_HArray1<double>",
        "NCollection_HSequence<TopoDS_Shape>",
    ]
}

/// Discover collections to generate from the set of referenced NCollection template
/// spellings, keeping only those whose element/key/value types are bindable. A type is
/// bindable if it is a primitive, or `header_of` returns the C++ header that defines it
/// (i.e. it is a known parsed class). The resolved header is stored so the C++ wrapper
/// includes the correct file (element headers are not always `{TypeName}.hxx`). The
/// curated seed spellings are always included.
pub fn discover_collections(
    referenced_spellings: &std::collections::HashSet<String>,
    header_of: impl Fn(&str) -> Option<String>,
) -> Vec<CollectionInfo> {
    let mut by_name: std::collections::BTreeMap<String, CollectionInfo> =
        std::collections::BTreeMap::new();

    let candidates = referenced_spellings
        .iter()
        .map(|s| s.as_str())
        .chain(seed_collection_spellings().iter().copied());

    for spelling in candidates {
        let Some(mut info) = parse_canonical_collection(spelling) else {
            continue;
        };
        // Gate on bindability: the C++ wrapper and FFI can only be generated when the
        // element (and value, for maps) types are primitives or known classes with a
        // resolvable defining header.
        let element_header = if is_primitive_element(&info.element_type) {
            None
        } else {
            match header_of(&info.element_type) {
                Some(h) => Some(h),
                None => continue,
            }
        };
        let value_header = match &info.value_type {
            Some(v) if is_primitive_element(v) => None,
            Some(v) => match header_of(v) {
                Some(h) => Some(h),
                None => continue,
            },
            None => None,
        };
        info.element_header = element_header;
        info.value_header = value_header;
        // Register a short overload-suffix token (`imap_shape`, `arr1_pnt`, …) so
        // overloaded methods taking this collection get concise names instead of the
        // full mangled type. Also register the Handle form for handle kinds.
        let token = suffix_token_for(&info);
        COLLECTION_SUFFIX_TOKENS.with(|m| {
            m.borrow_mut().insert(info.typedef_name.clone(), token);
        });
        by_name.entry(info.typedef_name.clone()).or_insert(info);
    }

    by_name.into_values().collect()
}

/// Generate C++ wrapper code for a single collection type
pub fn generate_cpp_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let typedef_name = &info.typedef_name;
    let short_name = &info.short_name;
    let element_type = &info.element_type;
    
    output.push_str("// ========================\n");
    output.push_str(&format!("// {} - {}\n", typedef_name, collection_kind_description(info.kind)));
    output.push_str("// ========================\n\n");

    // Bridge the mangled canonical identifier to the C++ template spelling so the
    // rest of the generation (which uses `typedef_name`) refers to the real type.
    output.push_str(&format!("typedef {} {typedef_name};\n\n", info.cpp_spelling));

    // Constructor
    output.push_str(&format!(
        "extern \"C\" {typedef_name}* {typedef_name}_new() {{\n"
    ));
    output.push_str(&format!(
        "    return new {typedef_name}();\n"
    ));
    output.push_str("}\n\n");

    // Destructor
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_destructor({typedef_name}* self_) {{\n"
    ));
    output.push_str("    delete self_;\n");
    output.push_str("}\n\n");

    // Iterator struct and functions based on collection kind.
    // HArray1/HSequence inherit from Array1/Sequence, so they reuse the same
    // element-access generation; handle (to_handle) support is added below.
    match info.kind {
        CollectionKind::List | CollectionKind::Map => {
            generate_cpp_const_iterator_collection(&mut output, typedef_name, short_name, element_type, info.kind);
        }
        CollectionKind::Sequence | CollectionKind::IndexedMap | CollectionKind::HSequence => {
            generate_cpp_indexed_collection(&mut output, typedef_name, short_name, element_type, info.kind);
        }
        CollectionKind::DataMap => {
            let value_type = info.value_type.as_ref().expect("DataMap must have value_type");
            generate_cpp_data_map_collection(&mut output, typedef_name, short_name, element_type, value_type);
        }
        CollectionKind::IndexedDataMap => {
            let value_type = info.value_type.as_ref().expect("IndexedDataMap must have value_type");
            generate_cpp_indexed_data_map_collection(&mut output, typedef_name, short_name, element_type, value_type);
        }
        CollectionKind::Array1 | CollectionKind::HArray1 => {
            generate_cpp_array1_collection(&mut output, typedef_name, element_type);
        }
        CollectionKind::Array2 => {
            generate_cpp_array2_collection(&mut output, typedef_name, element_type);
        }
    }

    // Handle-managed collections (HArray1/HSequence) also expose to_handle (wrap an
    // owned collection pointer in an OCCT Handle) and get/get_mut (borrow the pointee).
    if info.kind.is_handle() {
        let handle_ctype = format!("opencascade::handle<{}>", info.cpp_spelling);
        let handle_name = crate::type_mapping::handle_type_name(typedef_name);
        output.push_str(&format!(
            "extern \"C\" {handle_ctype}* {typedef_name}_to_handle({typedef_name}* self_) {{\n"
        ));
        output.push_str(&format!("    return new {handle_ctype}(self_);\n"));
        output.push_str("}\n\n");
        output.push_str(&format!(
            "extern \"C\" const {typedef_name}* {handle_name}_get(const {handle_ctype}* h) {{ return h->get(); }}\n"
        ));
        output.push_str(&format!(
            "extern \"C\" {typedef_name}* {handle_name}_get_mut({handle_ctype}* h) {{ return h->get(); }}\n"
        ));
        // Handle lifetime machinery. The collection owns its handle type, so these are
        // emitted here (the per-toolkit handle loop only covers class-owned handles,
        // and a collection's type is owned by TKernel but used across toolkits).
        output.push_str(&format!(
            "extern \"C\" void {handle_name}_destructor({handle_ctype}* self_) {{ delete self_; }}\n"
        ));
        output.push_str(&format!(
            "extern \"C\" OcctResult<{handle_ctype}*> {handle_name}_clone(const {handle_ctype}* self_) {{ try {{ return {{new {handle_ctype}(*self_), nullptr}}; }} OCCT_CATCH_RETURN }}\n"
        ));
        output.push_str(&format!(
            "extern \"C\" bool {handle_name}_is_null(const {handle_ctype}* self_) {{ return self_->IsNull(); }}\n\n"
        ));
    }

    output
}

/// Generate C++ code for collections using const_iterator (List, Map)
fn generate_cpp_const_iterator_collection(
    output: &mut String,
    typedef_name: &str,
    short_name: &str,
    element_type: &str,
    kind: CollectionKind,
) {
    let iterator_name = format!("{}Iterator", short_name);
    
    // Iterator struct
    output.push_str(&format!(
        "struct {iterator_name} {{\n"
    ));
    output.push_str(&format!(
        "    {typedef_name}::const_iterator current;\n"
    ));
    output.push_str(&format!(
        "    {typedef_name}::const_iterator end;\n"
    ));
    output.push_str("};\n\n");
    
    // Iterator destructor
    output.push_str(&format!(
        "extern \"C\" void {iterator_name}_destructor({iterator_name}* self_) {{\n"
    ));
    output.push_str("    delete self_;\n");
    output.push_str("}\n\n");
    
    // Create iterator function
    output.push_str(&format!(
        "extern \"C\" {iterator_name}* {typedef_name}_iter(const {typedef_name}* coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = new {iterator_name}();\n"
    ));
    output.push_str("    iter->current = coll->cbegin();\n");
    output.push_str("    iter->end = coll->cend();\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function
    output.push_str(&format!(
        "extern \"C\" {element_type}* {iterator_name}_next({iterator_name}* iter) {{\n"
    ));
    output.push_str("    if (iter->current == iter->end) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    if is_primitive_element(element_type) {
        // Primitive element: borrow the element in place (Rust copies it out).
        output.push_str(&format!(
            "    auto* result = const_cast<{element_type}*>(&(*iter->current));\n"
        ));
    } else {
        output.push_str(&format!(
            "    auto result = new {element_type}(*iter->current);\n"
        ));
    }
    output.push_str("    ++iter->current;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_size(const {typedef_name}* coll) {{\n"
    ));
    output.push_str("    return coll->Size();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_clear({typedef_name}* coll) {{\n"
    ));
    output.push_str("    coll->Clear();\n");
    output.push_str("}\n\n");

    // Add element function
    match kind {
        CollectionKind::List => {
            output.push_str(&format!(
                "extern \"C\" void {typedef_name}_append({typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    coll->Append(*item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "extern \"C\" void {typedef_name}_prepend({typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    coll->Prepend(*item);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::Map => {
            output.push_str(&format!(
                "extern \"C\" int {typedef_name}_add({typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    return coll->Add(*item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "extern \"C\" bool {typedef_name}_contains(const {typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    return coll->Contains(*item);\n");
            output.push_str("}\n\n");
        }
        _ => {}
    }
}

/// Generate C++ code for collections using indexed access (Sequence, IndexedMap)
fn generate_cpp_indexed_collection(
    output: &mut String,
    typedef_name: &str,
    short_name: &str,
    element_type: &str,
    kind: CollectionKind,
) {
    let iterator_name = format!("{}Iterator", short_name);
    
    // Iterator struct with index
    output.push_str(&format!(
        "struct {iterator_name} {{\n"
    ));
    output.push_str(&format!(
        "    const {typedef_name}* coll;\n"
    ));
    output.push_str("    int index;  // 1-indexed (OCCT convention)\n");
    output.push_str("    int extent;\n");
    output.push_str("};\n\n");
    
    // Iterator destructor
    output.push_str(&format!(
        "extern \"C\" void {iterator_name}_destructor({iterator_name}* self_) {{\n"
    ));
    output.push_str("    delete self_;\n");
    output.push_str("}\n\n");
    
    // Create iterator function
    let extent_method = match kind {
        CollectionKind::Sequence => "Length",
        CollectionKind::IndexedMap => "Extent",
        _ => "Size",
    };
    
    output.push_str(&format!(
        "extern \"C\" {iterator_name}* {typedef_name}_iter(const {typedef_name}* coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = new {iterator_name}();\n"
    ));
    output.push_str("    iter->coll = coll;\n");
    output.push_str("    iter->index = 1;\n");
    output.push_str(&format!(
        "    iter->extent = coll->{extent_method}();\n"
    ));
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function
    let access_method = match kind {
        CollectionKind::Sequence => "Value",
        CollectionKind::IndexedMap => "FindKey",
        _ => "Value",
    };
    
    output.push_str(&format!(
        "extern \"C\" {element_type}* {iterator_name}_next({iterator_name}* iter) {{\n"
    ));
    output.push_str("    if (iter->index > iter->extent) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    if is_primitive_element(element_type) {
        // Primitive element: borrow the element in place (Rust copies it out).
        output.push_str(&format!(
            "    auto* result = const_cast<{element_type}*>(&iter->coll->{access_method}(iter->index));\n"
        ));
    } else {
        output.push_str(&format!(
            "    auto result = new {element_type}(iter->coll->{access_method}(iter->index));\n"
        ));
    }
    output.push_str("    ++iter->index;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");

    // Size function
    let size_method = match kind {
        CollectionKind::Sequence => "Length",
        CollectionKind::IndexedMap => "Extent",
        _ => "Size",
    };

    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_size(const {typedef_name}* coll) {{\n"
    ));
    output.push_str(&format!(
        "    return coll->{size_method}();\n"
    ));
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_clear({typedef_name}* coll) {{\n"
    ));
    output.push_str("    coll->Clear();\n");
    output.push_str("}\n\n");

    // Add element function
    match kind {
        CollectionKind::Sequence => {
            output.push_str(&format!(
                "extern \"C\" void {typedef_name}_append({typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    coll->Append(*item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "extern \"C\" const {element_type}* {typedef_name}_value(const {typedef_name}* coll, int index) {{\n"
            ));
            output.push_str("    return &coll->Value(index);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::IndexedMap => {
            output.push_str(&format!(
                "extern \"C\" int {typedef_name}_add({typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    return coll->Add(*item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "extern \"C\" const {element_type}* {typedef_name}_find_key(const {typedef_name}* coll, int index) {{\n"
            ));
            output.push_str("    return &coll->FindKey(index);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "extern \"C\" int {typedef_name}_find_index(const {typedef_name}* coll, const {element_type}* item) {{\n"
            ));
            output.push_str("    return coll->FindIndex(*item);\n");
            output.push_str("}\n\n");
        }
        _ => {}
    }
}

/// Generate C++ code for NCollection_DataMap (key-value map using OCCT Iterator)
fn generate_cpp_data_map_collection(
    output: &mut String,
    typedef_name: &str,
    short_name: &str,
    key_type: &str,
    value_type: &str,
) {
    let iterator_name = format!("{}Iterator", short_name);
    
    // Iterator struct using OCCT's built-in Iterator class
    output.push_str(&format!(
        "struct {iterator_name} {{\n"
    ));
    output.push_str(&format!(
        "    {typedef_name}::Iterator inner;\n"
    ));
    output.push_str("};\n\n");
    
    // Iterator destructor
    output.push_str(&format!(
        "extern \"C\" void {iterator_name}_destructor({iterator_name}* self_) {{\n"
    ));
    output.push_str("    delete self_;\n");
    output.push_str("}\n\n");
    
    // Create iterator function
    output.push_str(&format!(
        "extern \"C\" {iterator_name}* {typedef_name}_iter(const {typedef_name}* coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = new {iterator_name}();\n"
    ));
    output.push_str("    iter->inner.Initialize(*coll);\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function - returns key
    output.push_str(&format!(
        "extern \"C\" {key_type}* {iterator_name}_next_key({iterator_name}* iter) {{\n"
    ));
    output.push_str("    if (!iter->inner.More()) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    if is_primitive_element(key_type) {
        output.push_str(&format!("    auto* result = const_cast<{key_type}*>(&iter->inner.Key());\n"));
    } else {
        output.push_str(&format!("    auto result = new {key_type}(iter->inner.Key());\n"));
    }
    output.push_str("    iter->inner.Next();\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");
    
    // Find function - lookup value by key
    output.push_str(&format!(
        "extern \"C\" {value_type}* {typedef_name}_find(const {typedef_name}* coll, const {key_type}* key) {{\n"
    ));
    output.push_str("    const auto* found = coll->Seek(*key);\n");
    output.push_str("    if (found == nullptr) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    if is_primitive_element(value_type) {
        // Primitive value: return a borrowed pointer into the map (Rust copies it out).
        output.push_str(&format!("    return const_cast<{value_type}*>(found);\n"));
    } else {
        output.push_str(&format!("    return new {value_type}(*found);\n"));
    }
    output.push_str("}\n\n");
    
    // Contains function
    output.push_str(&format!(
        "extern \"C\" bool {typedef_name}_contains(const {typedef_name}* coll, const {key_type}* key) {{\n"
    ));
    output.push_str("    return coll->IsBound(*key);\n");
    output.push_str("}\n\n");
    
    // Bind function - add key-value pair
    output.push_str(&format!(
        "extern \"C\" bool {typedef_name}_bind({typedef_name}* coll, const {key_type}* key, const {value_type}* value) {{\n"
    ));
    output.push_str("    return coll->Bind(*key, *value);\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_size(const {typedef_name}* coll) {{\n"
    ));
    output.push_str("    return coll->Extent();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_clear({typedef_name}* coll) {{\n"
    ));
    output.push_str("    coll->Clear();\n");
    output.push_str("}\n\n");
}

/// Generate C++ code for NCollection_IndexedDataMap (key-value map with index access)
fn generate_cpp_indexed_data_map_collection(
    output: &mut String,
    typedef_name: &str,
    short_name: &str,
    key_type: &str,
    value_type: &str,
) {
    let iterator_name = format!("{}Iterator", short_name);
    
    // Iterator struct with index
    output.push_str(&format!(
        "struct {iterator_name} {{\n"
    ));
    output.push_str(&format!(
        "    const {typedef_name}* coll;\n"
    ));
    output.push_str("    int index;  // 1-indexed (OCCT convention)\n");
    output.push_str("    int extent;\n");
    output.push_str("};\n\n");
    
    // Iterator destructor
    output.push_str(&format!(
        "extern \"C\" void {iterator_name}_destructor({iterator_name}* self_) {{\n"
    ));
    output.push_str("    delete self_;\n");
    output.push_str("}\n\n");
    
    // Create iterator function (iterates over keys)
    output.push_str(&format!(
        "extern \"C\" {iterator_name}* {typedef_name}_iter(const {typedef_name}* coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = new {iterator_name}();\n"
    ));
    output.push_str("    iter->coll = coll;\n");
    output.push_str("    iter->index = 1;\n");
    output.push_str("    iter->extent = coll->Extent();\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function - returns key
    output.push_str(&format!(
        "extern \"C\" {key_type}* {iterator_name}_next_key({iterator_name}* iter) {{\n"
    ));
    output.push_str("    if (iter->index > iter->extent) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    if is_primitive_element(key_type) {
        output.push_str(&format!("    auto* result = const_cast<{key_type}*>(&iter->coll->FindKey(iter->index));\n"));
    } else {
        output.push_str(&format!("    auto result = new {key_type}(iter->coll->FindKey(iter->index));\n"));
    }
    output.push_str("    ++iter->index;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");
    
    // FindFromKey - lookup value by key
    output.push_str(&format!(
        "extern \"C\" const {value_type}* {typedef_name}_find_from_key(const {typedef_name}* coll, const {key_type}* key) {{\n"
    ));
    output.push_str("    return &coll->FindFromKey(*key);\n");
    output.push_str("}\n\n");
    
    // FindFromIndex - lookup value by index (1-indexed)
    output.push_str(&format!(
        "extern \"C\" const {value_type}* {typedef_name}_find_from_index(const {typedef_name}* coll, int index) {{\n"
    ));
    output.push_str("    return &coll->FindFromIndex(index);\n");
    output.push_str("}\n\n");
    
    // FindKey - get key by index (1-indexed)
    output.push_str(&format!(
        "extern \"C\" {key_type}* {typedef_name}_find_key(const {typedef_name}* coll, int index) {{\n"
    ));
    if is_primitive_element(key_type) {
        // Primitive key: return a borrowed pointer to the key in the map (Rust copies it out).
        output.push_str(&format!(
            "    return const_cast<{key_type}*>(&coll->FindKey(index));\n"
        ));
    } else {
        output.push_str(&format!(
            "    return new {key_type}(coll->FindKey(index));\n"
        ));
    }
    output.push_str("}\n\n");
    
    // FindIndex - get index for a key (returns 0 if not found)
    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_find_index(const {typedef_name}* coll, const {key_type}* key) {{\n"
    ));
    output.push_str("    return coll->FindIndex(*key);\n");
    output.push_str("}\n\n");
    
    // Contains - check if key exists
    output.push_str(&format!(
        "extern \"C\" bool {typedef_name}_contains(const {typedef_name}* coll, const {key_type}* key) {{\n"
    ));
    output.push_str("    return coll->Contains(*key);\n");
    output.push_str("}\n\n");
    
    // Add - add key-value pair, returns index
    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_add({typedef_name}* coll, const {key_type}* key, const {value_type}* value) {{\n"
    ));
    output.push_str("    return coll->Add(*key, *value);\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "extern \"C\" int {typedef_name}_size(const {typedef_name}* coll) {{\n"
    ));
    output.push_str("    return coll->Extent();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_clear({typedef_name}* coll) {{\n"
    ));
    output.push_str("    coll->Clear();\n");
    output.push_str("}\n\n");
}

/// Generate C++ code for NCollection_Array1 (1D fixed-size array)
fn generate_cpp_array1_collection(
    output: &mut String,
    typedef_name: &str,
    element_type: &str,
) {
    // Constructor with bounds
    output.push_str(&format!(
        "extern \"C\" {typedef_name}* {typedef_name}_ctor_int2(Standard_Integer theLower, Standard_Integer theUpper) {{\n"
    ));
    output.push_str(&format!(
        "    return new {typedef_name}(theLower, theUpper);\n"
    ));
    output.push_str("}\n\n");

    // Constructor with bounds and init value
    output.push_str(&format!(
        "extern \"C\" {typedef_name}* {typedef_name}_ctor_int2_value(Standard_Integer theLower, Standard_Integer theUpper, const {element_type}* theValue) {{\n"
    ));
    output.push_str(&format!(
        "    auto arr = new {typedef_name}(theLower, theUpper);\n"
    ));
    output.push_str("    arr->Init(*theValue);\n");
    output.push_str("    return arr;\n");
    output.push_str("}\n\n");

    // Length
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_length(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->Length();\n");
    output.push_str("}\n\n");

    // Lower bound
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_lower(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->Lower();\n");
    output.push_str("}\n\n");

    // Upper bound
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_upper(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->Upper();\n");
    output.push_str("}\n\n");

    // Value (const pointer return)
    output.push_str(&format!(
        "extern \"C\" const {element_type}* {typedef_name}_value(const {typedef_name}* arr, Standard_Integer theIndex) {{\n"
    ));
    output.push_str("    return &arr->Value(theIndex);\n");
    output.push_str("}\n\n");

    // SetValue
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_set_value({typedef_name}* arr, Standard_Integer theIndex, const {element_type}* theItem) {{\n"
    ));
    output.push_str("    arr->SetValue(theIndex, *theItem);\n");
    output.push_str("}\n\n");

    // Init (fill all elements with a value)
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_init({typedef_name}* arr, const {element_type}* theValue) {{\n"
    ));
    output.push_str("    arr->Init(*theValue);\n");
    output.push_str("}\n\n");
}

/// Generate C++ code for NCollection_Array2 (2D fixed-size array)
fn generate_cpp_array2_collection(
    output: &mut String,
    typedef_name: &str,
    element_type: &str,
) {
    // Constructor with row/col bounds
    output.push_str(&format!(
        "extern \"C\" {typedef_name}* {typedef_name}_ctor_int4(Standard_Integer theRowLower, Standard_Integer theRowUpper, Standard_Integer theColLower, Standard_Integer theColUpper) {{\n"
    ));
    output.push_str(&format!(
        "    return new {typedef_name}(theRowLower, theRowUpper, theColLower, theColUpper);\n"
    ));
    output.push_str("}\n\n");

    // Constructor with row/col bounds and init value
    output.push_str(&format!(
        "extern \"C\" {typedef_name}* {typedef_name}_ctor_int4_value(Standard_Integer theRowLower, Standard_Integer theRowUpper, Standard_Integer theColLower, Standard_Integer theColUpper, const {element_type}* theValue) {{\n"
    ));
    output.push_str(&format!(
        "    auto arr = new {typedef_name}(theRowLower, theRowUpper, theColLower, theColUpper);\n"
    ));
    output.push_str("    arr->Init(*theValue);\n");
    output.push_str("    return arr;\n");
    output.push_str("}\n\n");

    // NbRows
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_nb_rows(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->NbRows();\n");
    output.push_str("}\n\n");

    // NbColumns
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_nb_columns(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->NbColumns();\n");
    output.push_str("}\n\n");

    // LowerRow
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_lower_row(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->LowerRow();\n");
    output.push_str("}\n\n");

    // UpperRow
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_upper_row(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->UpperRow();\n");
    output.push_str("}\n\n");

    // LowerCol
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_lower_col(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->LowerCol();\n");
    output.push_str("}\n\n");

    // UpperCol
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_upper_col(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->UpperCol();\n");
    output.push_str("}\n\n");

    // Length (total elements)
    output.push_str(&format!(
        "extern \"C\" Standard_Integer {typedef_name}_length(const {typedef_name}* arr) {{\n"
    ));
    output.push_str("    return arr->Length();\n");
    output.push_str("}\n\n");

    // Value (const pointer return, row/col indexed)
    output.push_str(&format!(
        "extern \"C\" const {element_type}* {typedef_name}_value(const {typedef_name}* arr, Standard_Integer theRow, Standard_Integer theCol) {{\n"
    ));
    output.push_str("    return &arr->Value(theRow, theCol);\n");
    output.push_str("}\n\n");

    // SetValue
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_set_value({typedef_name}* arr, Standard_Integer theRow, Standard_Integer theCol, const {element_type}* theItem) {{\n"
    ));
    output.push_str("    arr->SetValue(theRow, theCol, *theItem);\n");
    output.push_str("}\n\n");

    // Init (fill all elements with a value)
    output.push_str(&format!(
        "extern \"C\" void {typedef_name}_init({typedef_name}* arr, const {element_type}* theValue) {{\n"
    ));
    output.push_str("    arr->Init(*theValue);\n");
    output.push_str("}\n\n");
}

fn collection_kind_description(kind: CollectionKind) -> &'static str {
    kind.description()
}

// =============================================================================
// Rust Code Generation
// =============================================================================

/// Generate Rust FFI code for a collection type
pub fn generate_rust_ffi_collections<T: std::borrow::Borrow<CollectionInfo>>(collections: &[T]) -> (String, String) {
    if collections.is_empty() {
        return (String::new(), String::new());
    }
    
    // Generate opaque struct declarations (outside extern "C" block)
    let mut type_decls = String::new();
    type_decls.push_str("// ========================\n");
    type_decls.push_str("// Collection types (opaque)\n");
    type_decls.push_str("// ========================\n\n");
    for item in collections {
        let info = item.borrow();
        writeln!(type_decls, "#[repr(C)]").unwrap();
        writeln!(type_decls, "pub struct {} {{ _opaque: [u8; 0] }}", info.typedef_name).unwrap();
        // Iterator type
        let iter_name = format!("{}Iterator", info.short_name);
        writeln!(type_decls, "#[repr(C)]").unwrap();
        writeln!(type_decls, "pub struct {} {{ _opaque: [u8; 0] }}", iter_name).unwrap();
    }
    type_decls.push('\n');
    
    let mut ffi_decls = String::new();
    
    ffi_decls.push_str("    // ========================\n");
    ffi_decls.push_str("    // Collection type wrappers\n");
    ffi_decls.push_str("    // ========================\n\n");
    
    for item in collections {
        let info = item.borrow();
        ffi_decls.push_str(&generate_rust_ffi_collection(info));
        if info.kind.is_handle() {
            let handle_type = crate::type_mapping::handle_type_name(&info.typedef_name);
            ffi_decls.push_str(&format!(
                "    /// Wrap an owned {} in a Handle (reference-counted)\n    pub fn {}_to_handle(self_: *mut {}) -> *mut {};\n\n",
                info.typedef_name, info.typedef_name, info.typedef_name, handle_type
            ));
            ffi_decls.push_str(&format!(
                "    pub fn {}_get(h: *const {}) -> *const {};\n\n",
                handle_type, handle_type, info.typedef_name
            ));
            ffi_decls.push_str(&format!(
                "    pub fn {}_get_mut(h: *mut {}) -> *mut {};\n\n",
                handle_type, handle_type, info.typedef_name
            ));
        }
    }

    (type_decls, ffi_decls)
}

/// Generate Rust FFI declarations for a single collection
/// Uses full C++ type names (e.g., TopoDS_Shape, TopTools_ListOfShape)
fn generate_rust_ffi_collection(info: &CollectionInfo) -> String {
    match info.kind {
        CollectionKind::Array1 | CollectionKind::HArray1 => return generate_rust_ffi_array1(info),
        CollectionKind::Array2 => return generate_rust_ffi_array2(info),
        _ => {}
    }

    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let iter_name = format!("{}Iterator", info.short_name);
    // Map primitive element/value C++ types to their Rust FFI names (int -> i32, etc.);
    // class types pass through unchanged (referenced as opaque struct pointers).
    let elem = rust_element_type(&info.element_type);
    let val = info.value_type.as_deref().map(rust_element_type);

    // Constructor
    output.push_str(&format!("    /// Create a new empty {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_new() -> *mut {};\n\n", coll_name, coll_name));

    // Destructor
    output.push_str(&format!("    /// Destroy a {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_destructor(self_: *mut {});\n\n", coll_name, coll_name));

    // Size method
    output.push_str(&format!("    /// Get number of elements in {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_size(coll: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Clear method
    output.push_str(&format!("    /// Remove all elements from {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_clear(coll: *mut {});\n\n", coll_name, coll_name));

    // Add/append method based on collection kind
    match info.kind {
        CollectionKind::List => {
            output.push_str("    /// Append an element to the list\n");
            output.push_str(&format!("    pub fn {}_append(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, elem));
            output.push_str("    /// Prepend an element to the list\n");
            output.push_str(&format!("    pub fn {}_prepend(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, elem));
        }
        CollectionKind::Sequence | CollectionKind::HSequence => {
            output.push_str("    /// Append an element to the sequence\n");
            output.push_str(&format!("    pub fn {}_append(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, elem));
            output.push_str("    /// Get element at 1-based index\n");
            output.push_str(&format!("    pub fn {}_value(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, elem));
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            output.push_str("    /// Add an element to the map/set\n");
            output.push_str(&format!("    pub fn {}_add(coll: *mut {}, item: *const {}) -> i32;\n\n", coll_name, coll_name, elem));
            if info.kind == CollectionKind::IndexedMap {
                output.push_str("    /// Get element at 1-based index\n");
                output.push_str(&format!("    pub fn {}_find_key(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, elem));
            }
        }
        CollectionKind::DataMap => {
            if let Some(value_type) = val {
                output.push_str("    /// Bind a key to a value\n");
                output.push_str(&format!("    pub fn {}_bind(coll: *mut {}, key: *const {}, value: *const {}) -> bool;\n\n", coll_name, coll_name, elem, value_type));
                output.push_str("    /// Find a value by key (returns nullptr if not found)\n");
                output.push_str(&format!("    pub fn {}_find(coll: *const {}, key: *const {}) -> *mut {};\n\n", coll_name, coll_name, elem, value_type));
                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn {}_contains(coll: *const {}, key: *const {}) -> bool;\n\n", coll_name, coll_name, elem));
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(value_type) = val {
                output.push_str("    /// Add a key-value pair, returns index (existing or new)\n");
                output.push_str(&format!("    pub fn {}_add(coll: *mut {}, key: *const {}, value: *const {}) -> i32;\n\n", coll_name, coll_name, elem, value_type));
                output.push_str("    /// Find value by key (returns reference)\n");
                output.push_str(&format!("    pub fn {}_find_from_key(coll: *const {}, key: *const {}) -> *const {};\n\n", coll_name, coll_name, elem, value_type));
                output.push_str("    /// Find value by 1-based index (returns reference)\n");
                output.push_str(&format!("    pub fn {}_find_from_index(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, value_type));
                output.push_str("    /// Find key by 1-based index\n");
                output.push_str(&format!("    pub fn {}_find_key(coll: *const {}, index: i32) -> *mut {};\n\n", coll_name, coll_name, elem));
                output.push_str("    /// Find index by key (returns 0 if not found)\n");
                output.push_str(&format!("    pub fn {}_find_index(coll: *const {}, key: *const {}) -> i32;\n\n", coll_name, coll_name, elem));
                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn {}_contains(coll: *const {}, key: *const {}) -> bool;\n\n", coll_name, coll_name, elem));
            }
        }
        CollectionKind::Array1 | CollectionKind::Array2 | CollectionKind::HArray1 => {
            unreachable!("Array types handled by dedicated functions")
        }
    }

    // Iterator creation
    output.push_str("    /// Create an iterator over the collection\n");
    output.push_str(&format!("    pub fn {}_iter(coll: *const {}) -> *mut {};\n\n", coll_name, coll_name, iter_name));

    // Iterator next - DataMaps iterate over keys, others iterate over elements
    let next_suffix = match info.kind {
        CollectionKind::DataMap | CollectionKind::IndexedDataMap => "_next_key",
        _ => "_next",
    };
    let next_fn_name = format!("{}{}", iter_name, next_suffix);
    output.push_str("    /// Advance iterator and get next element (nullptr when done)\n");
    output.push_str(&format!("    pub fn {}(iter: *mut {}) -> *mut {};\n\n", next_fn_name, iter_name, elem));
    
    // Iterator destructor
    output.push_str(&format!("    /// Destroy a {}\n", iter_name));
    output.push_str(&format!("    pub fn {}_destructor(self_: *mut {});\n\n", iter_name, iter_name));
    
    output
}

/// Generate Rust FFI declarations for an Array1 collection
fn generate_rust_ffi_array1(info: &CollectionInfo) -> String {
    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let elem = rust_element_type(&info.element_type);


    // Default constructor
    output.push_str(&format!("    /// Create a new empty {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_new() -> *mut {};\n\n", coll_name, coll_name));

    // Destructor
    output.push_str(&format!("    /// Destroy a {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_destructor(self_: *mut {});\n\n", coll_name, coll_name));

    // Constructor with bounds
    output.push_str(&format!("    /// Create {} with lower and upper bounds\n", coll_name));
    output.push_str(&format!("    pub fn {}_ctor_int2(theLower: i32, theUpper: i32) -> *mut {};\n\n", coll_name, coll_name));

    // Constructor with bounds and init value
    output.push_str(&format!("    /// Create {} with bounds, all elements initialized to theValue\n", coll_name));
    output.push_str(&format!("    pub fn {}_ctor_int2_value(theLower: i32, theUpper: i32, theValue: *const {}) -> *mut {};\n\n", coll_name, elem, coll_name));

    // Length
    output.push_str("    /// Get number of elements\n");
    output.push_str(&format!("    pub fn {}_length(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Lower
    output.push_str("    /// Get lower bound index\n");
    output.push_str(&format!("    pub fn {}_lower(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Upper
    output.push_str("    /// Get upper bound index\n");
    output.push_str(&format!("    pub fn {}_upper(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Value
    output.push_str("    /// Get element at index (bounds-checked)\n");
    output.push_str(&format!("    pub fn {}_value(arr: *const {}, theIndex: i32) -> *const {};\n\n", coll_name, coll_name, elem));

    // SetValue
    output.push_str("    /// Set element at index (bounds-checked)\n");
    output.push_str(&format!("    pub fn {}_set_value(arr: *mut {}, theIndex: i32, theItem: *const {});\n\n", coll_name, coll_name, elem));

    // Init
    output.push_str("    /// Set all elements to the same value\n");
    output.push_str(&format!("    pub fn {}_init(arr: *mut {}, theValue: *const {});\n\n", coll_name, coll_name, elem));

    output
}

/// Generate Rust FFI declarations for an Array2 collection
fn generate_rust_ffi_array2(info: &CollectionInfo) -> String {
    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let elem = rust_element_type(&info.element_type);


    // Default constructor
    output.push_str(&format!("    /// Create a new empty {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_new() -> *mut {};\n\n", coll_name, coll_name));

    // Destructor
    output.push_str(&format!("    /// Destroy a {}\n", coll_name));
    output.push_str(&format!("    pub fn {}_destructor(self_: *mut {});\n\n", coll_name, coll_name));

    // Constructor with row/col bounds
    output.push_str(&format!("    /// Create {} with row and column bounds\n", coll_name));
    output.push_str(&format!("    pub fn {}_ctor_int4(theRowLower: i32, theRowUpper: i32, theColLower: i32, theColUpper: i32) -> *mut {};\n\n", coll_name, coll_name));

    // Constructor with row/col bounds and init value
    output.push_str(&format!("    /// Create {} with bounds, all elements initialized to theValue\n", coll_name));
    output.push_str(&format!("    pub fn {}_ctor_int4_value(theRowLower: i32, theRowUpper: i32, theColLower: i32, theColUpper: i32, theValue: *const {}) -> *mut {};\n\n", coll_name, elem, coll_name));

    // NbRows
    output.push_str("    /// Get number of rows\n");
    output.push_str(&format!("    pub fn {}_nb_rows(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // NbColumns
    output.push_str("    /// Get number of columns\n");
    output.push_str(&format!("    pub fn {}_nb_columns(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // LowerRow
    output.push_str("    /// Get lower row bound\n");
    output.push_str(&format!("    pub fn {}_lower_row(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // UpperRow
    output.push_str("    /// Get upper row bound\n");
    output.push_str(&format!("    pub fn {}_upper_row(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // LowerCol
    output.push_str("    /// Get lower column bound\n");
    output.push_str(&format!("    pub fn {}_lower_col(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // UpperCol
    output.push_str("    /// Get upper column bound\n");
    output.push_str(&format!("    pub fn {}_upper_col(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Length
    output.push_str("    /// Get total number of elements\n");
    output.push_str(&format!("    pub fn {}_length(arr: *const {}) -> i32;\n\n", coll_name, coll_name));

    // Value
    output.push_str("    /// Get element at row/col (bounds-checked)\n");
    output.push_str(&format!("    pub fn {}_value(arr: *const {}, theRow: i32, theCol: i32) -> *const {};\n\n", coll_name, coll_name, elem));

    // SetValue
    output.push_str("    /// Set element at row/col (bounds-checked)\n");
    output.push_str(&format!("    pub fn {}_set_value(arr: *mut {}, theRow: i32, theCol: i32, theItem: *const {});\n\n", coll_name, coll_name, elem));

    // Init
    output.push_str("    /// Set all elements to the same value\n");
    output.push_str(&format!("    pub fn {}_init(arr: *mut {}, theValue: *const {});\n\n", coll_name, coll_name, elem));

    output
}



/// Generate C++ wrappers header for all collections
pub fn collect_collection_headers(collections: &[CollectionInfo]) -> Vec<String> {
    let mut headers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for info in collections {
        // The collection template header (e.g. NCollection_Array1.hxx), not the
        // mangled instantiation name.
        headers.insert(info.kind.header());
        // Element/value headers come from the parsed class's actual source header
        // (resolved in `discover_collections`), since e.g. Standard_DumpValue is
        // defined in Standard_Dump.hxx, not Standard_DumpValue.hxx.
        if let Some(ref h) = info.element_header {
            headers.insert(h.clone());
        }
        if let Some(ref h) = info.value_header {
            headers.insert(h.clone());
        }
    }

    let mut result: Vec<_> = headers.into_iter().collect();
    result.sort();
    result
}


pub fn generate_cpp_collections(collections: &[CollectionInfo]) -> String {
    if collections.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    
    output.push_str("\n// ========================\n");
    output.push_str("// Collection type wrappers\n");
    output.push_str("// ========================\n\n");
    
    // Generate wrappers for each collection
    for info in collections {
        output.push_str(&generate_cpp_collection(info));
    }
    
    output
}
