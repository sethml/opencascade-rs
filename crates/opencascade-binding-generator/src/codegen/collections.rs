//! Code generation for NCollection-based collection types
//!
//! Generates iterator wrappers and helper functions for OCCT collection typedefs like
//! TopTools_ListOfShape, TopTools_SequenceOfShape, etc.
//!
//! This module recognizes collection typedefs by pattern matching on their names
//! and generates the appropriate C++ and Rust wrapper code.

use std::collections::HashMap;

/// Information about a collection typedef
#[derive(Debug, Clone)]
pub struct CollectionInfo {
    /// The full typedef name (e.g., "TopTools_ListOfShape")
    pub typedef_name: String,
    /// The module this collection belongs to (e.g., "top_tools")
    pub module: String,
    /// The short Rust name without module prefix (e.g., "ListOfShape")
    pub short_name: String,
    /// The element type (for simple collections) or key type (for data maps)
    pub element_type: String,
    /// The element/key type's module (e.g., "topo_ds")
    pub element_module: String,
    /// The value type for data maps (None for simple collections)
    pub value_type: Option<String>,
    /// The value type's module for data maps
    pub value_module: Option<String>,
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
        }
    }
    
    /// Returns true if this is a data map (key-value) collection
    pub fn is_data_map(&self) -> bool {
        matches!(self, CollectionKind::DataMap | CollectionKind::IndexedDataMap)
    }
}

/// Metadata for a collection type
#[derive(Debug, Clone)]
enum CollectionMetadata {
    /// Simple collection with single element type
    Simple {
        element_type: &'static str,
        kind: CollectionKind,
    },
    /// Data map with key and value types
    DataMap {
        key_type: &'static str,
        value_type: &'static str,
        kind: CollectionKind,
    },
}

/// Known collection typedefs and their metadata
fn known_collections() -> HashMap<&'static str, CollectionMetadata> {
    let mut map = HashMap::new();
    
    // TopTools shape collections (simple element type)
    map.insert("TopTools_ListOfShape", CollectionMetadata::Simple {
        element_type: "TopoDS_Shape",
        kind: CollectionKind::List,
    });
    map.insert("TopTools_SequenceOfShape", CollectionMetadata::Simple {
        element_type: "TopoDS_Shape",
        kind: CollectionKind::Sequence,
    });
    map.insert("TopTools_IndexedMapOfShape", CollectionMetadata::Simple {
        element_type: "TopoDS_Shape",
        kind: CollectionKind::IndexedMap,
    });
    map.insert("TopTools_MapOfShape", CollectionMetadata::Simple {
        element_type: "TopoDS_Shape",
        kind: CollectionKind::Map,
    });
    
    // TopTools data map collections (key-value types)
    map.insert("TopTools_DataMapOfShapeShape", CollectionMetadata::DataMap {
        key_type: "TopoDS_Shape",
        value_type: "TopoDS_Shape",
        kind: CollectionKind::DataMap,
    });
    map.insert("TopTools_IndexedDataMapOfShapeListOfShape", CollectionMetadata::DataMap {
        key_type: "TopoDS_Shape",
        value_type: "TopTools_ListOfShape",
        kind: CollectionKind::IndexedDataMap,
    });
    
    // TColgp geometry collections (for future expansion)
    // map.insert("TColgp_Array1OfPnt", CollectionMetadata::Simple {
    //     element_type: "gp_Pnt",
    //     kind: CollectionKind::Array1,
    // });
    
    map
}

/// Helper to extract module name from a type (e.g., "topo_ds" from "TopoDS_Shape")
fn type_to_module(type_name: &str) -> String {
    if let Some(underscore_pos) = type_name.find('_') {
        crate::module_graph::module_to_rust_name(&type_name[..underscore_pos])
    } else {
        type_name.to_lowercase()
    }
}

/// Parse a typedef name to extract collection info if it's a known collection type
pub fn parse_collection_typedef(typedef_name: &str) -> Option<CollectionInfo> {
    let known = known_collections();
    let metadata = known.get(typedef_name)?;
    
    // Extract module from typedef name (e.g., "TopTools" from "TopTools_ListOfShape")
    let module = if let Some(underscore_pos) = typedef_name.find('_') {
        crate::module_graph::module_to_rust_name(&typedef_name[..underscore_pos])
    } else {
        return None;
    };
    
    // Extract short name (e.g., "ListOfShape" from "TopTools_ListOfShape")
    let short_name = if let Some(underscore_pos) = typedef_name.find('_') {
        typedef_name[underscore_pos + 1..].to_string()
    } else {
        return None;
    };
    
    match metadata {
        CollectionMetadata::Simple { element_type, kind } => {
            Some(CollectionInfo {
                typedef_name: typedef_name.to_string(),
                module,
                short_name,
                element_type: element_type.to_string(),
                element_module: type_to_module(element_type),
                value_type: None,
                value_module: None,
                kind: *kind,
            })
        }
        CollectionMetadata::DataMap { key_type, value_type, kind } => {
            Some(CollectionInfo {
                typedef_name: typedef_name.to_string(),
                module,
                short_name,
                element_type: key_type.to_string(),  // key_type stored in element_type
                element_module: type_to_module(key_type),
                value_type: Some(value_type.to_string()),
                value_module: Some(type_to_module(value_type)),
                kind: *kind,
            })
        }
    }
}

/// Get all known collection types
pub fn all_known_collections() -> Vec<CollectionInfo> {
    let mut result: Vec<_> = known_collections()
        .keys()
        .filter_map(|name| parse_collection_typedef(name))
        .collect();
    // Sort for deterministic ordering
    result.sort_by(|a, b| a.typedef_name.cmp(&b.typedef_name));
    result
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
    
    // Constructor
    output.push_str(&format!(
        "inline std::unique_ptr<{typedef_name}> {typedef_name}_new() {{\n"
    ));
    output.push_str(&format!(
        "    return std::make_unique<{typedef_name}>();\n"
    ));
    output.push_str("}\n\n");
    
    // Iterator struct and functions based on collection kind
    match info.kind {
        CollectionKind::List | CollectionKind::Map => {
            generate_cpp_const_iterator_collection(&mut output, typedef_name, short_name, element_type, info.kind);
        }
        CollectionKind::Sequence | CollectionKind::IndexedMap => {
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
    
    // Create iterator function
    output.push_str(&format!(
        "inline std::unique_ptr<{iterator_name}> {typedef_name}_iter(const {typedef_name}& coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = std::make_unique<{iterator_name}>();\n"
    ));
    output.push_str("    iter->current = coll.cbegin();\n");
    output.push_str("    iter->end = coll.cend();\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function
    output.push_str(&format!(
        "inline std::unique_ptr<{element_type}> {iterator_name}_next({iterator_name}& iter) {{\n"
    ));
    output.push_str("    if (iter.current == iter.end) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    output.push_str(&format!(
        "    auto result = std::make_unique<{element_type}>(*iter.current);\n"
    ));
    output.push_str("    ++iter.current;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "inline int {typedef_name}_size(const {typedef_name}& coll) {{\n"
    ));
    output.push_str("    return coll.Size();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "inline void {typedef_name}_clear({typedef_name}& coll) {{\n"
    ));
    output.push_str("    coll.Clear();\n");
    output.push_str("}\n\n");

    // Add element function
    match kind {
        CollectionKind::List => {
            output.push_str(&format!(
                "inline void {typedef_name}_append({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    coll.Append(item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "inline void {typedef_name}_prepend({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    coll.Prepend(item);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::Map => {
            output.push_str(&format!(
                "inline int {typedef_name}_add({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    return coll.Add(item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "inline bool {typedef_name}_contains(const {typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    return coll.Contains(item);\n");
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
    
    // Create iterator function
    let extent_method = match kind {
        CollectionKind::Sequence => "Length",
        CollectionKind::IndexedMap => "Extent",
        _ => "Size",
    };
    
    output.push_str(&format!(
        "inline std::unique_ptr<{iterator_name}> {typedef_name}_iter(const {typedef_name}& coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = std::make_unique<{iterator_name}>();\n"
    ));
    output.push_str("    iter->coll = &coll;\n");
    output.push_str("    iter->index = 1;\n");
    output.push_str(&format!(
        "    iter->extent = coll.{extent_method}();\n"
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
        "inline std::unique_ptr<{element_type}> {iterator_name}_next({iterator_name}& iter) {{\n"
    ));
    output.push_str("    if (iter.index > iter.extent) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    output.push_str(&format!(
        "    auto result = std::make_unique<{element_type}>(iter.coll->{access_method}(iter.index));\n"
    ));
    output.push_str("    ++iter.index;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");

    // Size function
    let size_method = match kind {
        CollectionKind::Sequence => "Length",
        CollectionKind::IndexedMap => "Extent",
        _ => "Size",
    };

    output.push_str(&format!(
        "inline int {typedef_name}_size(const {typedef_name}& coll) {{\n"
    ));
    output.push_str(&format!(
        "    return coll.{size_method}();\n"
    ));
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "inline void {typedef_name}_clear({typedef_name}& coll) {{\n"
    ));
    output.push_str("    coll.Clear();\n");
    output.push_str("}\n\n");

    // Add element function
    match kind {
        CollectionKind::Sequence => {
            output.push_str(&format!(
                "inline void {typedef_name}_append({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    coll.Append(item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "inline const {element_type}& {typedef_name}_value(const {typedef_name}& coll, int index) {{\n"
            ));
            output.push_str("    return coll.Value(index);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::IndexedMap => {
            output.push_str(&format!(
                "inline int {typedef_name}_add({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    return coll.Add(item);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "inline const {element_type}& {typedef_name}_find_key(const {typedef_name}& coll, int index) {{\n"
            ));
            output.push_str("    return coll.FindKey(index);\n");
            output.push_str("}\n\n");

            output.push_str(&format!(
                "inline int {typedef_name}_find_index(const {typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    return coll.FindIndex(item);\n");
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
    
    // Create iterator function
    output.push_str(&format!(
        "inline std::unique_ptr<{iterator_name}> {typedef_name}_iter(const {typedef_name}& coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = std::make_unique<{iterator_name}>();\n"
    ));
    output.push_str("    iter->inner.Initialize(coll);\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function - returns key
    output.push_str(&format!(
        "inline std::unique_ptr<{key_type}> {iterator_name}_next_key({iterator_name}& iter) {{\n"
    ));
    output.push_str("    if (!iter.inner.More()) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    output.push_str(&format!(
        "    auto result = std::make_unique<{key_type}>(iter.inner.Key());\n"
    ));
    output.push_str("    iter.inner.Next();\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");
    
    // Find function - lookup value by key
    output.push_str(&format!(
        "inline std::unique_ptr<{value_type}> {typedef_name}_find(const {typedef_name}& coll, const {key_type}& key) {{\n"
    ));
    output.push_str("    const auto* found = coll.Seek(key);\n");
    output.push_str("    if (found == nullptr) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    output.push_str(&format!(
        "    return std::make_unique<{value_type}>(*found);\n"
    ));
    output.push_str("}\n\n");
    
    // Contains function
    output.push_str(&format!(
        "inline bool {typedef_name}_contains(const {typedef_name}& coll, const {key_type}& key) {{\n"
    ));
    output.push_str("    return coll.IsBound(key);\n");
    output.push_str("}\n\n");
    
    // Bind function - add key-value pair
    output.push_str(&format!(
        "inline bool {typedef_name}_bind({typedef_name}& coll, const {key_type}& key, const {value_type}& value) {{\n"
    ));
    output.push_str("    return coll.Bind(key, value);\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "inline int {typedef_name}_size(const {typedef_name}& coll) {{\n"
    ));
    output.push_str("    return coll.Extent();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "inline void {typedef_name}_clear({typedef_name}& coll) {{\n"
    ));
    output.push_str("    coll.Clear();\n");
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
    
    // Create iterator function (iterates over keys)
    output.push_str(&format!(
        "inline std::unique_ptr<{iterator_name}> {typedef_name}_iter(const {typedef_name}& coll) {{\n"
    ));
    output.push_str(&format!(
        "    auto iter = std::make_unique<{iterator_name}>();\n"
    ));
    output.push_str("    iter->coll = &coll;\n");
    output.push_str("    iter->index = 1;\n");
    output.push_str("    iter->extent = coll.Extent();\n");
    output.push_str("    return iter;\n");
    output.push_str("}\n\n");
    
    // Iterator next function - returns key
    output.push_str(&format!(
        "inline std::unique_ptr<{key_type}> {iterator_name}_next_key({iterator_name}& iter) {{\n"
    ));
    output.push_str("    if (iter.index > iter.extent) {\n");
    output.push_str("        return nullptr;\n");
    output.push_str("    }\n");
    output.push_str(&format!(
        "    auto result = std::make_unique<{key_type}>(iter.coll->FindKey(iter.index));\n"
    ));
    output.push_str("    ++iter.index;\n");
    output.push_str("    return result;\n");
    output.push_str("}\n\n");
    
    // FindFromKey - lookup value by key
    output.push_str(&format!(
        "inline const {value_type}& {typedef_name}_find_from_key(const {typedef_name}& coll, const {key_type}& key) {{\n"
    ));
    output.push_str("    return coll.FindFromKey(key);\n");
    output.push_str("}\n\n");
    
    // FindFromIndex - lookup value by index (1-indexed)
    output.push_str(&format!(
        "inline const {value_type}& {typedef_name}_find_from_index(const {typedef_name}& coll, int index) {{\n"
    ));
    output.push_str("    return coll.FindFromIndex(index);\n");
    output.push_str("}\n\n");
    
    // FindKey - get key by index (1-indexed)
    output.push_str(&format!(
        "inline std::unique_ptr<{key_type}> {typedef_name}_find_key(const {typedef_name}& coll, int index) {{\n"
    ));
    output.push_str(&format!(
        "    return std::make_unique<{key_type}>(coll.FindKey(index));\n"
    ));
    output.push_str("}\n\n");
    
    // FindIndex - get index for a key (returns 0 if not found)
    output.push_str(&format!(
        "inline int {typedef_name}_find_index(const {typedef_name}& coll, const {key_type}& key) {{\n"
    ));
    output.push_str("    return coll.FindIndex(key);\n");
    output.push_str("}\n\n");
    
    // Contains - check if key exists
    output.push_str(&format!(
        "inline bool {typedef_name}_contains(const {typedef_name}& coll, const {key_type}& key) {{\n"
    ));
    output.push_str("    return coll.Contains(key);\n");
    output.push_str("}\n\n");
    
    // Add - add key-value pair, returns index
    output.push_str(&format!(
        "inline int {typedef_name}_add({typedef_name}& coll, const {key_type}& key, const {value_type}& value) {{\n"
    ));
    output.push_str("    return coll.Add(key, value);\n");
    output.push_str("}\n\n");

    // Size function
    output.push_str(&format!(
        "inline int {typedef_name}_size(const {typedef_name}& coll) {{\n"
    ));
    output.push_str("    return coll.Extent();\n");
    output.push_str("}\n\n");

    // Clear function
    output.push_str(&format!(
        "inline void {typedef_name}_clear({typedef_name}& coll) {{\n"
    ));
    output.push_str("    coll.Clear();\n");
    output.push_str("}\n\n");
}

fn collection_kind_description(kind: CollectionKind) -> &'static str {
    kind.description()
}

// =============================================================================
// Rust Code Generation
// =============================================================================

/// Generate Rust FFI code for a collection type
pub fn generate_unified_rust_ffi_collections(collections: &[CollectionInfo]) -> (String, String) {
    if collections.is_empty() {
        return (String::new(), String::new());
    }
    
    let mut ffi_decls = String::new();
    
    ffi_decls.push_str("                // ========================\n");
    ffi_decls.push_str("                // Collection type wrappers\n");
    ffi_decls.push_str("                // ========================\n\n");
    
    for info in collections {
        ffi_decls.push_str(&generate_unified_rust_ffi_collection(info));
    }
    
    (String::new(), ffi_decls)
}

/// Generate Rust FFI declarations for a single collection in unified mode
/// Uses full C++ type names (e.g., TopoDS_Shape, TopTools_ListOfShape)
fn generate_unified_rust_ffi_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let iter_name = format!("{}Iterator", info.short_name);
    
    // Type declaration for collection
    output.push_str(&format!("                /// {}\n", info.kind.description()));
    output.push_str(&format!("                type {};\n\n", coll_name));
    
    // Iterator type
    output.push_str(&format!("                /// Iterator for {}\n", coll_name));
    output.push_str(&format!("                type {};\n\n", iter_name));
    
    // Constructor
    output.push_str(&format!("                /// Create a new empty {}\n", coll_name));
    output.push_str(&format!("                fn {}_new() -> UniquePtr<{}>;\n\n", coll_name, coll_name));
    
    // Size method
    output.push_str(&format!("                /// Get number of elements in {}\n", coll_name));
    output.push_str(&format!("                fn {}_size(coll: &{}) -> i32;\n\n", coll_name, coll_name));
    
    // Clear method (for mutable collections)
    output.push_str(&format!("                /// Remove all elements from {}\n", coll_name));
    output.push_str(&format!("                fn {}_clear(coll: Pin<&mut {}>);\n\n", coll_name, coll_name));
    
    // Add/append method based on collection kind
    match info.kind {
        CollectionKind::List => {
            output.push_str("                /// Append an element to the list\n");
            output.push_str(&format!("                fn {}_append(coll: Pin<&mut {}>, item: &{});\n\n", coll_name, coll_name, info.element_type));
            output.push_str("                /// Prepend an element to the list\n");
            output.push_str(&format!("                fn {}_prepend(coll: Pin<&mut {}>, item: &{});\n\n", coll_name, coll_name, info.element_type));
        }
        CollectionKind::Sequence => {
            output.push_str("                /// Append an element to the sequence\n");
            output.push_str(&format!("                fn {}_append(coll: Pin<&mut {}>, item: &{});\n\n", coll_name, coll_name, info.element_type));
            output.push_str("                /// Get element at 1-based index\n");
            output.push_str(&format!("                fn {}_value(coll: &{}, index: i32) -> &{};\n\n", coll_name, coll_name, info.element_type));
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            output.push_str("                /// Add an element to the map/set\n");
            output.push_str(&format!("                fn {}_add(coll: Pin<&mut {}>, item: &{}) -> i32;\n\n", coll_name, coll_name, info.element_type));
            if info.kind == CollectionKind::IndexedMap {
                output.push_str("                /// Get element at 1-based index\n");
                output.push_str(&format!("                fn {}_find_key(coll: &{}, index: i32) -> &{};\n\n", coll_name, coll_name, info.element_type));
            }
        }
        CollectionKind::DataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("                /// Bind a key to a value\n");
                output.push_str(&format!("                #[cxx_name = \"{}_bind\"]\n", coll_name));
                output.push_str(&format!("                fn {}_bind(coll: Pin<&mut {}>, key: &{}, value: &{}) -> bool;\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("                /// Find a value by key (returns nullptr if not found)\n");
                output.push_str(&format!("                #[cxx_name = \"{}_find\"]\n", coll_name));
                output.push_str(&format!("                fn {}_find(coll: &{}, key: &{}) -> UniquePtr<{}>;\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("                /// Check if key exists\n");
                output.push_str(&format!("                #[cxx_name = \"{}_contains\"]\n", coll_name));
                output.push_str(&format!("                fn {}_contains(coll: &{}, key: &{}) -> bool;\n\n", coll_name, coll_name, info.element_type));
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("                /// Add a key-value pair, returns index (existing or new)\n");
                output.push_str(&format!("                #[cxx_name = \"{}_add\"]\n", coll_name));
                output.push_str(&format!("                fn {}_add(coll: Pin<&mut {}>, key: &{}, value: &{}) -> i32;\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("                /// Find value by key (returns reference)\n");
                output.push_str(&format!("                #[cxx_name = \"{}_find_from_key\"]\n", coll_name));
                output.push_str(&format!("                fn {}_find_from_key<'a>(coll: &'a {}, key: &{}) -> &'a {};\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("                /// Find value by 1-based index (returns reference)\n");
                output.push_str(&format!("                #[cxx_name = \"{}_find_from_index\"]\n", coll_name));
                output.push_str(&format!("                fn {}_find_from_index<'a>(coll: &'a {}, index: i32) -> &'a {};\n\n", coll_name, coll_name, value_type));
                output.push_str("                /// Find key by 1-based index\n");
                output.push_str(&format!("                #[cxx_name = \"{}_find_key\"]\n", coll_name));
                output.push_str(&format!("                fn {}_find_key(coll: &{}, index: i32) -> UniquePtr<{}>;\n\n", coll_name, coll_name, info.element_type));
                output.push_str("                /// Find index by key (returns 0 if not found)\n");
                output.push_str(&format!("                #[cxx_name = \"{}_find_index\"]\n", coll_name));
                output.push_str(&format!("                fn {}_find_index(coll: &{}, key: &{}) -> i32;\n\n", coll_name, coll_name, info.element_type));
                output.push_str("                /// Check if key exists\n");
                output.push_str(&format!("                #[cxx_name = \"{}_contains\"]\n", coll_name));
                output.push_str(&format!("                fn {}_contains(coll: &{}, key: &{}) -> bool;\n\n", coll_name, coll_name, info.element_type));
            }
        }
    }
    
    // Iterator creation
    output.push_str("                /// Create an iterator over the collection\n");
    output.push_str(&format!("                fn {}_iter(coll: &{}) -> UniquePtr<{}>;\n\n", coll_name, coll_name, iter_name));

    // Iterator next - DataMaps iterate over keys, others iterate over elements
    let next_suffix = match info.kind {
        CollectionKind::DataMap | CollectionKind::IndexedDataMap => "_next_key",
        _ => "_next",
    };
    let next_fn_name = format!("{}{}", iter_name, next_suffix);
    output.push_str("                /// Advance iterator and get next element (nullptr when done)\n");
    output.push_str(&format!("                #[cxx_name = \"{}\"]\n", next_fn_name));
    output.push_str(&format!("                fn {}(iter: Pin<&mut {}>) -> UniquePtr<{}>;\n\n", next_fn_name, iter_name, info.element_type));
    
    output
}

/// Generate Rust impl blocks for collections in unified mode
pub fn generate_unified_rust_impl_collections(collections: &[CollectionInfo]) -> String {
    if collections.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    
    for info in collections {
        output.push_str(&generate_unified_rust_impl_collection(info));
        output.push('\n');
    }
    
    output
}

/// Generate Rust impl block for a single collection in unified mode
fn generate_unified_rust_impl_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let iter_name = format!("{}Iterator", info.short_name);
    
    // Collection impl block
    output.push_str(&format!("impl ffi::{} {{\n", coll_name));
    
    output.push_str("    /// Create a new empty collection\n");
    output.push_str("    pub fn new() -> cxx::UniquePtr<Self> {\n");
    output.push_str(&format!("        ffi::{}_new()\n", coll_name));
    output.push_str("    }\n\n");
    
    output.push_str("    /// Get number of elements\n");
    output.push_str("    pub fn len(&self) -> i32 {\n");
    output.push_str(&format!("        ffi::{}_size(self)\n", coll_name));
    output.push_str("    }\n\n");
    
    output.push_str("    /// Check if empty\n");
    output.push_str("    pub fn is_empty(&self) -> bool {\n");
    output.push_str("        self.len() == 0\n");
    output.push_str("    }\n\n");
    
    output.push_str("    /// Remove all elements\n");
    output.push_str("    pub fn clear(self: std::pin::Pin<&mut Self>) {\n");
    output.push_str(&format!("        ffi::{}_clear(self)\n", coll_name));
    output.push_str("    }\n\n");
    
    // Kind-specific methods
    match info.kind {
        CollectionKind::List => {
            output.push_str("    /// Append an element to the list\n");
            output.push_str(&format!("    pub fn append(self: std::pin::Pin<&mut Self>, item: &ffi::{}) {{\n", info.element_type));
            output.push_str(&format!("        ffi::{}_append(self, item)\n", coll_name));
            output.push_str("    }\n\n");
            
            output.push_str("    /// Prepend an element to the list\n");
            output.push_str(&format!("    pub fn prepend(self: std::pin::Pin<&mut Self>, item: &ffi::{}) {{\n", info.element_type));
            output.push_str(&format!("        ffi::{}_prepend(self, item)\n", coll_name));
            output.push_str("    }\n\n");
        }
        CollectionKind::Sequence => {
            output.push_str("    /// Append an element to the sequence\n");
            output.push_str(&format!("    pub fn append(self: std::pin::Pin<&mut Self>, item: &ffi::{}) {{\n", info.element_type));
            output.push_str(&format!("        ffi::{}_append(self, item)\n", coll_name));
            output.push_str("    }\n\n");
            
            output.push_str("    /// Get element at 1-based index\n");
            output.push_str(&format!("    pub fn value(&self, index: i32) -> &ffi::{} {{\n", info.element_type));
            output.push_str(&format!("        ffi::{}_value(self, index)\n", coll_name));
            output.push_str("    }\n\n");
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            output.push_str("    /// Add an element to the map/set\n");
            output.push_str(&format!("    pub fn add(self: std::pin::Pin<&mut Self>, item: &ffi::{}) -> i32 {{\n", info.element_type));
            output.push_str(&format!("        ffi::{}_add(self, item)\n", coll_name));
            output.push_str("    }\n\n");
            
            if info.kind == CollectionKind::IndexedMap {
                output.push_str("    /// Get element at 1-based index\n");
                output.push_str(&format!("    pub fn find_key(&self, index: i32) -> &ffi::{} {{\n", info.element_type));
                output.push_str(&format!("        ffi::{}_find_key(self, index)\n", coll_name));
                output.push_str("    }\n\n");
            }
        }
        CollectionKind::DataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("    /// Bind a key to a value\n");
                output.push_str(&format!("    pub fn bind(self: std::pin::Pin<&mut Self>, key: &ffi::{}, value: &ffi::{}) -> bool {{\n", info.element_type, value_type));
                output.push_str(&format!("        ffi::{}_bind(self, key, value)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Find a value by key (returns nullptr if not found)\n");
                output.push_str(&format!("    pub fn find(&self, key: &ffi::{}) -> cxx::UniquePtr<ffi::{}> {{\n", info.element_type, value_type));
                output.push_str(&format!("        ffi::{}_find(self, key)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn contains(&self, key: &ffi::{}) -> bool {{\n", info.element_type));
                output.push_str(&format!("        ffi::{}_contains(self, key)\n", coll_name));
                output.push_str("    }\n\n");
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("    /// Add a key-value pair, returns index (existing or new)\n");
                output.push_str(&format!("    pub fn add(self: std::pin::Pin<&mut Self>, key: &ffi::{}, value: &ffi::{}) -> i32 {{\n", info.element_type, value_type));
                output.push_str(&format!("        ffi::{}_add(self, key, value)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Find value by key\n");
                output.push_str(&format!("    pub fn find_from_key(&self, key: &ffi::{}) -> &ffi::{} {{\n", info.element_type, value_type));
                output.push_str(&format!("        ffi::{}_find_from_key(self, key)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Find value by 1-based index\n");
                output.push_str(&format!("    pub fn find_from_index(&self, index: i32) -> &ffi::{} {{\n", value_type));
                output.push_str(&format!("        ffi::{}_find_from_index(self, index)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Find key by 1-based index\n");
                output.push_str(&format!("    pub fn find_key(&self, index: i32) -> cxx::UniquePtr<ffi::{}> {{\n", info.element_type));
                output.push_str(&format!("        ffi::{}_find_key(self, index)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Find index by key (returns 0 if not found)\n");
                output.push_str(&format!("    pub fn find_index(&self, key: &ffi::{}) -> i32 {{\n", info.element_type));
                output.push_str(&format!("        ffi::{}_find_index(self, key)\n", coll_name));
                output.push_str("    }\n\n");

                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn contains(&self, key: &ffi::{}) -> bool {{\n", info.element_type));
                output.push_str(&format!("        ffi::{}_contains(self, key)\n", coll_name));
                output.push_str("    }\n\n");
            }
        }
    }
    
    // Iterator
    output.push_str("    /// Create an iterator over the collection\n");
    output.push_str(&format!("    pub fn iter(&self) -> cxx::UniquePtr<ffi::{}> {{\n", iter_name));
    output.push_str(&format!("        ffi::{}_iter(self)\n", coll_name));
    output.push_str("    }\n");
    
    output.push_str("}\n\n");
    
    // Iterator impl block
    let next_suffix = match info.kind {
        CollectionKind::DataMap | CollectionKind::IndexedDataMap => "_next_key",
        _ => "_next",
    };
    let next_fn_name = format!("{}{}", iter_name, next_suffix);
    output.push_str(&format!("impl ffi::{} {{\n", iter_name));
    output.push_str("    /// Get next element (nullptr when done)\n");
    output.push_str(&format!("    pub fn next(self: std::pin::Pin<&mut Self>) -> cxx::UniquePtr<ffi::{}> {{\n", info.element_type));
    output.push_str(&format!("        ffi::{}(self)\n", next_fn_name));
    output.push_str("    }\n");
    output.push_str("}\n");
    
    output
}

/// Generate unified C++ wrappers header for all collections
pub fn generate_unified_cpp_collections(collections: &[CollectionInfo]) -> String {
    if collections.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    
    output.push_str("\n// ========================\n");
    output.push_str("// Collection type wrappers\n");
    output.push_str("// ========================\n\n");
    
    // Collect unique headers needed
    let mut headers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for info in collections {
        headers.insert(format!("{}.hxx", info.element_type));
        headers.insert(format!("{}.hxx", info.typedef_name));
        if let Some(ref value_type) = info.value_type {
            headers.insert(format!("{}.hxx", value_type));
        }
    }
    
    // Include headers (sorted for determinism)
    let mut sorted_headers: Vec<_> = headers.into_iter().collect();
    sorted_headers.sort();
    for header in sorted_headers {
        output.push_str(&format!("#include <{}>\n", header));
    }
    output.push('\n');
    
    // Generate wrappers for each collection
    for info in collections {
        output.push_str(&generate_cpp_collection(info));
    }
    
    output
}

