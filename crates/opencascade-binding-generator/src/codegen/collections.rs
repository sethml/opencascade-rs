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

/// Get the appropriate Rust FFI type name for a given OCCT type.
/// 
/// This checks if the type is a known collection - if so, it returns the short name
/// (e.g., "ListOfShape" instead of "TopTools_ListOfShape").
/// For non-collection types, it returns the full OCCT type name unchanged.
fn ffi_type_name(occt_type: &str) -> String {
    if let Some(info) = parse_collection_typedef(occt_type) {
        info.short_name
    } else {
        occt_type.to_string()
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
    known_collections()
        .keys()
        .filter_map(|name| parse_collection_typedef(name))
        .collect()
}

/// Get collection types that belong to a specific module
pub fn collections_for_module(module_name: &str) -> Vec<CollectionInfo> {
    all_known_collections()
        .into_iter()
        .filter(|c| c.module == module_name)
        .collect()
}

// =============================================================================
// C++ Code Generation
// =============================================================================

/// Generate C++ header code for a collection type
pub fn generate_cpp_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let typedef_name = &info.typedef_name;
    let short_name = &info.short_name;
    let element_type = &info.element_type;
    
    output.push_str(&format!("// ========================\n"));
    output.push_str(&format!("// {} - {}\n", typedef_name, collection_kind_description(info.kind)));
    output.push_str(&format!("// ========================\n\n"));
    
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
    
    // Add element function
    match kind {
        CollectionKind::List => {
            output.push_str(&format!(
                "inline void {typedef_name}_append({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    coll.Append(item);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::Map => {
            output.push_str(&format!(
                "inline bool {typedef_name}_add({typedef_name}& coll, const {element_type}& item) {{\n"
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
    
    // Add element function
    match kind {
        CollectionKind::Sequence => {
            output.push_str(&format!(
                "inline void {typedef_name}_append({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    coll.Append(item);\n");
            output.push_str("}\n\n");
        }
        CollectionKind::IndexedMap => {
            output.push_str(&format!(
                "inline int {typedef_name}_add({typedef_name}& coll, const {element_type}& item) {{\n"
            ));
            output.push_str("    return coll.Add(item);\n");
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
}

fn collection_kind_description(kind: CollectionKind) -> &'static str {
    kind.description()
}

// =============================================================================
// Rust Code Generation
// =============================================================================

/// Generate Rust FFI code for a collection type
pub fn generate_rust_ffi_collection(info: &CollectionInfo) -> String {
    match info.kind {
        CollectionKind::List | CollectionKind::Sequence | CollectionKind::IndexedMap | CollectionKind::Map => {
            generate_rust_ffi_simple_collection(info)
        }
        CollectionKind::DataMap => {
            generate_rust_ffi_data_map_collection(info)
        }
        CollectionKind::IndexedDataMap => {
            generate_rust_ffi_indexed_data_map_collection(info)
        }
    }
}

/// Generate Rust FFI code for simple (single element type) collections
fn generate_rust_ffi_simple_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let typedef_name = &info.typedef_name;
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let element_cxx_name = &info.element_type;
    
    output.push_str(&format!("        // ========================\n"));
    output.push_str(&format!("        // {} - {}\n", typedef_name, collection_kind_description(info.kind)));
    output.push_str(&format!("        // ========================\n\n"));
    
    // Collection type
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}\"]\n"
    ));
    output.push_str(&format!(
        "        type {short_name};\n\n"
    ));
    
    // Constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_new\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_new() -> UniquePtr<{short_name}>;\n\n"
    ));
    
    // Iterator type
    output.push_str(&format!(
        "        type {iterator_name};\n\n"
    ));
    
    // Iterator constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_iter\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_iterator(coll: &{short_name}) -> UniquePtr<{iterator_name}>;\n\n"
    ));
    
    // Iterator next
    output.push_str(&format!(
        "        #[cxx_name = \"{iterator_name}_next\"]\n"
    ));
    output.push_str(&format!(
        "        fn {iterator_name}_next(\n"
    ));
    output.push_str(&format!(
        "            iter: Pin<&mut {iterator_name}>,\n"
    ));
    output.push_str(&format!(
        "        ) -> UniquePtr<{element_cxx_name}>;\n\n"
    ));
    
    // Add element function(s)
    match info.kind {
        CollectionKind::List | CollectionKind::Sequence => {
            output.push_str(&format!(
                "        #[cxx_name = \"{typedef_name}_append\"]\n"
            ));
            output.push_str(&format!(
                "        fn {short_name}_append(coll: Pin<&mut {short_name}>, item: &{element_cxx_name});\n\n"
            ));
        }
        CollectionKind::IndexedMap => {
            output.push_str(&format!(
                "        /// Adds item to the map, returns its index (existing or new)\n"
            ));
            output.push_str(&format!(
                "        #[cxx_name = \"{typedef_name}_add\"]\n"
            ));
            output.push_str(&format!(
                "        fn {short_name}_add(coll: Pin<&mut {short_name}>, item: &{element_cxx_name}) -> i32;\n\n"
            ));
            
            output.push_str(&format!(
                "        /// Returns 0 if item is not in the map\n"
            ));
            output.push_str(&format!(
                "        #[cxx_name = \"{typedef_name}_find_index\"]\n"
            ));
            output.push_str(&format!(
                "        fn {short_name}_find_index(coll: &{short_name}, item: &{element_cxx_name}) -> i32;\n\n"
            ));
        }
        CollectionKind::Map => {
            output.push_str(&format!(
                "        /// Returns true if item was newly added\n"
            ));
            output.push_str(&format!(
                "        #[cxx_name = \"{typedef_name}_add\"]\n"
            ));
            output.push_str(&format!(
                "        fn {short_name}_add(coll: Pin<&mut {short_name}>, item: &{element_cxx_name}) -> bool;\n\n"
            ));
            
            output.push_str(&format!(
                "        #[cxx_name = \"{typedef_name}_contains\"]\n"
            ));
            output.push_str(&format!(
                "        fn {short_name}_contains(coll: &{short_name}, item: &{element_cxx_name}) -> bool;\n\n"
            ));
        }
        _ => {}
    }
    
    output
}

/// Generate Rust FFI code for DataMap (key-value map)
fn generate_rust_ffi_data_map_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let typedef_name = &info.typedef_name;
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let key_ffi_name = &info.element_type;  // key type stored in element_type
    let value_type = info.value_type.as_ref().expect("DataMap must have value_type");
    let value_ffi_name = ffi_type_name(value_type);  // May be short name if it's a collection
    
    output.push_str(&format!("        // ========================\n"));
    output.push_str(&format!("        // {} - {}\n", typedef_name, collection_kind_description(info.kind)));
    output.push_str(&format!("        // ========================\n\n"));
    
    // Collection type
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}\"]\n"
    ));
    output.push_str(&format!(
        "        type {short_name};\n\n"
    ));
    
    // Constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_new\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_new() -> UniquePtr<{short_name}>;\n\n"
    ));
    
    // Iterator type
    output.push_str(&format!(
        "        type {iterator_name};\n\n"
    ));
    
    // Iterator constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_iter\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_iterator(coll: &{short_name}) -> UniquePtr<{iterator_name}>;\n\n"
    ));
    
    // Iterator next - returns key
    output.push_str(&format!(
        "        #[cxx_name = \"{iterator_name}_next_key\"]\n"
    ));
    output.push_str(&format!(
        "        fn {iterator_name}_next_key(\n"
    ));
    output.push_str(&format!(
        "            iter: Pin<&mut {iterator_name}>,\n"
    ));
    output.push_str(&format!(
        "        ) -> UniquePtr<{key_ffi_name}>;\n\n"
    ));
    
    // Find - lookup value by key
    output.push_str(&format!(
        "        /// Returns None if key is not found\n"
    ));
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_find\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_find(coll: &{short_name}, key: &{key_ffi_name}) -> UniquePtr<{value_ffi_name}>;\n\n"
    ));
    
    // Contains - check if key exists
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_contains\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_contains(coll: &{short_name}, key: &{key_ffi_name}) -> bool;\n\n"
    ));
    
    // Bind - add key-value pair
    output.push_str(&format!(
        "        /// Binds key to value, returns true if newly added\n"
    ));
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_bind\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_bind(coll: Pin<&mut {short_name}>, key: &{key_ffi_name}, value: &{value_ffi_name}) -> bool;\n\n"
    ));
    
    // Size
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_size\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_size(coll: &{short_name}) -> i32;\n\n"
    ));
    
    output
}

/// Generate Rust FFI code for IndexedDataMap (key-value map with index access)
fn generate_rust_ffi_indexed_data_map_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let typedef_name = &info.typedef_name;
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let key_ffi_name = &info.element_type;  // key type stored in element_type
    let value_type = info.value_type.as_ref().expect("IndexedDataMap must have value_type");
    let value_ffi_name = ffi_type_name(value_type);  // May be short name if it's a collection
    
    output.push_str(&format!("        // ========================\n"));
    output.push_str(&format!("        // {} - {}\n", typedef_name, collection_kind_description(info.kind)));
    output.push_str(&format!("        // ========================\n\n"));
    
    // Collection type
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}\"]\n"
    ));
    output.push_str(&format!(
        "        type {short_name};\n\n"
    ));
    
    // Constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_new\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_new() -> UniquePtr<{short_name}>;\n\n"
    ));
    
    // Iterator type
    output.push_str(&format!(
        "        type {iterator_name};\n\n"
    ));
    
    // Iterator constructor
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_iter\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_iterator(coll: &{short_name}) -> UniquePtr<{iterator_name}>;\n\n"
    ));
    
    // Iterator next - returns key
    output.push_str(&format!(
        "        #[cxx_name = \"{iterator_name}_next_key\"]\n"
    ));
    output.push_str(&format!(
        "        fn {iterator_name}_next_key(\n"
    ));
    output.push_str(&format!(
        "            iter: Pin<&mut {iterator_name}>,\n"
    ));
    output.push_str(&format!(
        "        ) -> UniquePtr<{key_ffi_name}>;\n\n"
    ));
    
    // FindFromKey - lookup value by key (returns reference)
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_find_from_key\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_find_from_key<'a>(coll: &'a {short_name}, key: &{key_ffi_name}) -> &'a {value_ffi_name};\n\n"
    ));
    
    // FindFromIndex - lookup value by index (1-indexed, returns reference)
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_find_from_index\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_find_from_index<'a>(coll: &'a {short_name}, index: i32) -> &'a {value_ffi_name};\n\n"
    ));
    
    // FindKey - get key by index (1-indexed)
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_find_key\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_find_key(coll: &{short_name}, index: i32) -> UniquePtr<{key_ffi_name}>;\n\n"
    ));
    
    // FindIndex - get index for a key (returns 0 if not found)
    output.push_str(&format!(
        "        /// Returns 0 if key is not found\n"
    ));
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_find_index\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_find_index(coll: &{short_name}, key: &{key_ffi_name}) -> i32;\n\n"
    ));
    
    // Contains - check if key exists
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_contains\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_contains(coll: &{short_name}, key: &{key_ffi_name}) -> bool;\n\n"
    ));
    
    // Add - add key-value pair, returns index
    output.push_str(&format!(
        "        /// Adds key-value pair, returns index\n"
    ));
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_add\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_add(coll: Pin<&mut {short_name}>, key: &{key_ffi_name}, value: &{value_ffi_name}) -> i32;\n\n"
    ));
    
    // Size
    output.push_str(&format!(
        "        #[cxx_name = \"{typedef_name}_size\"]\n"
    ));
    output.push_str(&format!(
        "        fn {short_name}_size(coll: &{short_name}) -> i32;\n\n"
    ));
    
    output
}

/// Generate Rust impl code for a collection type (iterator wrapper and methods)
pub fn generate_rust_impl_collection(info: &CollectionInfo) -> String {
    match info.kind {
        CollectionKind::List | CollectionKind::Sequence | CollectionKind::IndexedMap | CollectionKind::Map => {
            generate_rust_impl_simple_collection(info)
        }
        CollectionKind::DataMap => {
            generate_rust_impl_data_map_collection(info)
        }
        CollectionKind::IndexedDataMap => {
            generate_rust_impl_indexed_data_map_collection(info)
        }
    }
}

/// Generate Rust impl code for simple (single element type) collections
fn generate_rust_impl_simple_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let rust_iter_name = format!("{}Iter", short_name);
    let element_short_name = element_short_name(&info.element_type);
    
    output.push_str(&format!("// ========================\n"));
    output.push_str(&format!("// {} iterator\n", short_name));
    output.push_str(&format!("// ========================\n\n"));
    
    // Iterator wrapper struct
    output.push_str(&format!(
        "pub struct {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    inner: cxx::UniquePtr<ffi::{iterator_name}>,\n"
    ));
    output.push_str("}\n\n");
    
    // Iterator impl
    output.push_str(&format!(
        "impl Iterator for {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    type Item = cxx::UniquePtr<{element_short_name}>;\n\n"
    ));
    output.push_str("    fn next(&mut self) -> Option<Self::Item> {\n");
    output.push_str(&format!(
        "        let item = ffi::{iterator_name}_next(self.inner.pin_mut());\n"
    ));
    output.push_str("        if item.is_null() {\n");
    output.push_str("            None\n");
    output.push_str("        } else {\n");
    output.push_str("            Some(item)\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Impl block on collection type
    output.push_str(&format!(
        "impl {short_name} {{\n"
    ));
    
    // iter() method
    output.push_str(&format!(
        "    pub fn iter(&self) -> {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "        {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "            inner: ffi::{short_name}_iterator(self),\n"
    ));
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    // from_iter() method
    output.push_str(&format!(
        "    pub fn from_iter<'a>(items: impl IntoIterator<Item = &'a {element_short_name}>) -> cxx::UniquePtr<Self> {{\n"
    ));
    output.push_str(&format!(
        "        let mut coll = ffi::{short_name}_new();\n"
    ));
    output.push_str("        for item in items {\n");
    
    match info.kind {
        CollectionKind::List | CollectionKind::Sequence => {
            output.push_str(&format!(
                "            ffi::{short_name}_append(coll.pin_mut(), item);\n"
            ));
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            output.push_str(&format!(
                "            ffi::{short_name}_add(coll.pin_mut(), item);\n"
            ));
        }
        _ => {}
    }
    
    output.push_str("        }\n");
    output.push_str("        coll\n");
    output.push_str("    }\n");
    
    // Additional methods based on collection kind
    match info.kind {
        CollectionKind::IndexedMap => {
            output.push_str("\n");
            output.push_str(&format!(
                "    /// Returns Some(index) if found, None if not in map\n"
            ));
            output.push_str(&format!(
                "    pub fn find_index(&self, item: &{element_short_name}) -> Option<i32> {{\n"
            ));
            output.push_str(&format!(
                "        let idx = ffi::{short_name}_find_index(self, item);\n"
            ));
            output.push_str("        if idx == 0 {\n");
            output.push_str("            None\n");
            output.push_str("        } else {\n");
            output.push_str("            Some(idx)\n");
            output.push_str("        }\n");
            output.push_str("    }\n");
        }
        CollectionKind::Map => {
            output.push_str("\n");
            output.push_str(&format!(
                "    pub fn contains(&self, item: &{element_short_name}) -> bool {{\n"
            ));
            output.push_str(&format!(
                "        ffi::{short_name}_contains(self, item)\n"
            ));
            output.push_str("    }\n");
        }
        _ => {}
    }
    
    output.push_str("}\n");
    
    output
}

/// Generate Rust impl code for DataMap (key-value map)
fn generate_rust_impl_data_map_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let rust_iter_name = format!("{}KeyIter", short_name);  // "KeyIter" to indicate it iterates keys
    let key_short_name = element_short_name(&info.element_type);
    let value_type = info.value_type.as_ref().expect("DataMap must have value_type");
    let value_short_name = element_short_name(value_type);
    
    output.push_str(&format!("// ========================\n"));
    output.push_str(&format!("// {} key iterator\n", short_name));
    output.push_str(&format!("// ========================\n\n"));
    
    // Key iterator wrapper struct
    output.push_str(&format!(
        "pub struct {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    inner: cxx::UniquePtr<ffi::{iterator_name}>,\n"
    ));
    output.push_str("}\n\n");
    
    // Iterator impl - iterates over keys
    output.push_str(&format!(
        "impl Iterator for {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    type Item = cxx::UniquePtr<{key_short_name}>;\n\n"
    ));
    output.push_str("    fn next(&mut self) -> Option<Self::Item> {\n");
    output.push_str(&format!(
        "        let item = ffi::{iterator_name}_next_key(self.inner.pin_mut());\n"
    ));
    output.push_str("        if item.is_null() {\n");
    output.push_str("            None\n");
    output.push_str("        } else {\n");
    output.push_str("            Some(item)\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Impl block on collection type
    output.push_str(&format!(
        "impl {short_name} {{\n"
    ));
    
    // keys() method - iterate over keys
    output.push_str(&format!(
        "    /// Iterate over keys\n"
    ));
    output.push_str(&format!(
        "    pub fn keys(&self) -> {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "        {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "            inner: ffi::{short_name}_iterator(self),\n"
    ));
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    // find() method - lookup value by key
    output.push_str(&format!(
        "    /// Find value by key, returns None if not found\n"
    ));
    output.push_str(&format!(
        "    pub fn find(&self, key: &{key_short_name}) -> Option<cxx::UniquePtr<{value_short_name}>> {{\n"
    ));
    output.push_str(&format!(
        "        let value = ffi::{short_name}_find(self, key);\n"
    ));
    output.push_str("        if value.is_null() {\n");
    output.push_str("            None\n");
    output.push_str("        } else {\n");
    output.push_str("            Some(value)\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    // contains() method
    output.push_str(&format!(
        "    pub fn contains(&self, key: &{key_short_name}) -> bool {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_contains(self, key)\n"
    ));
    output.push_str("    }\n\n");
    
    // len() method
    output.push_str(&format!(
        "    pub fn len(&self) -> i32 {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_size(self)\n"
    ));
    output.push_str("    }\n\n");
    
    // is_empty() method
    output.push_str(&format!(
        "    pub fn is_empty(&self) -> bool {{\n"
    ));
    output.push_str("        self.len() == 0\n");
    output.push_str("    }\n");
    
    output.push_str("}\n");
    
    output
}

/// Generate Rust impl code for IndexedDataMap (key-value map with index access)
fn generate_rust_impl_indexed_data_map_collection(info: &CollectionInfo) -> String {
    let mut output = String::new();
    
    let short_name = &info.short_name;
    let iterator_name = format!("{}Iterator", short_name);
    let rust_iter_name = format!("{}KeyIter", short_name);  // "KeyIter" to indicate it iterates keys
    let key_short_name = element_short_name(&info.element_type);
    let value_type = info.value_type.as_ref().expect("IndexedDataMap must have value_type");
    let value_short_name = element_short_name(value_type);
    
    output.push_str(&format!("// ========================\n"));
    output.push_str(&format!("// {} key iterator\n", short_name));
    output.push_str(&format!("// ========================\n\n"));
    
    // Key iterator wrapper struct
    output.push_str(&format!(
        "pub struct {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    inner: cxx::UniquePtr<ffi::{iterator_name}>,\n"
    ));
    output.push_str("}\n\n");
    
    // Iterator impl - iterates over keys
    output.push_str(&format!(
        "impl Iterator for {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "    type Item = cxx::UniquePtr<{key_short_name}>;\n\n"
    ));
    output.push_str("    fn next(&mut self) -> Option<Self::Item> {\n");
    output.push_str(&format!(
        "        let item = ffi::{iterator_name}_next_key(self.inner.pin_mut());\n"
    ));
    output.push_str("        if item.is_null() {\n");
    output.push_str("            None\n");
    output.push_str("        } else {\n");
    output.push_str("            Some(item)\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Impl block on collection type
    output.push_str(&format!(
        "impl {short_name} {{\n"
    ));
    
    // keys() method - iterate over keys
    output.push_str(&format!(
        "    /// Iterate over keys\n"
    ));
    output.push_str(&format!(
        "    pub fn keys(&self) -> {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "        {rust_iter_name} {{\n"
    ));
    output.push_str(&format!(
        "            inner: ffi::{short_name}_iterator(self),\n"
    ));
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    // find_from_key() method - lookup value by key (returns reference)
    output.push_str(&format!(
        "    /// Find value by key (panics if key not found)\n"
    ));
    output.push_str(&format!(
        "    pub fn find_from_key(&self, key: &{key_short_name}) -> &{value_short_name} {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_find_from_key(self, key)\n"
    ));
    output.push_str("    }\n\n");
    
    // find_from_index() method - lookup value by index (returns reference)
    output.push_str(&format!(
        "    /// Find value by index (1-indexed, panics if out of range)\n"
    ));
    output.push_str(&format!(
        "    pub fn find_from_index(&self, index: i32) -> &{value_short_name} {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_find_from_index(self, index)\n"
    ));
    output.push_str("    }\n\n");
    
    // find_key() method - get key by index
    output.push_str(&format!(
        "    /// Get key by index (1-indexed)\n"
    ));
    output.push_str(&format!(
        "    pub fn find_key(&self, index: i32) -> cxx::UniquePtr<{key_short_name}> {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_find_key(self, index)\n"
    ));
    output.push_str("    }\n\n");
    
    // find_index() method - get index for key
    output.push_str(&format!(
        "    /// Get index for key, returns None if not found\n"
    ));
    output.push_str(&format!(
        "    pub fn find_index(&self, key: &{key_short_name}) -> Option<i32> {{\n"
    ));
    output.push_str(&format!(
        "        let idx = ffi::{short_name}_find_index(self, key);\n"
    ));
    output.push_str("        if idx == 0 {\n");
    output.push_str("            None\n");
    output.push_str("        } else {\n");
    output.push_str("            Some(idx)\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    // contains() method
    output.push_str(&format!(
        "    pub fn contains(&self, key: &{key_short_name}) -> bool {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_contains(self, key)\n"
    ));
    output.push_str("    }\n\n");
    
    // len() method
    output.push_str(&format!(
        "    pub fn len(&self) -> i32 {{\n"
    ));
    output.push_str(&format!(
        "        ffi::{short_name}_size(self)\n"
    ));
    output.push_str("    }\n\n");
    
    // is_empty() method
    output.push_str(&format!(
        "    pub fn is_empty(&self) -> bool {{\n"
    ));
    output.push_str("        self.len() == 0\n");
    output.push_str("    }\n");
    
    output.push_str("}\n");
    
    output
}

/// Get the short name for an element type (e.g., "Shape" from "TopoDS_Shape")
fn element_short_name(element_type: &str) -> String {
    if let Some(underscore_pos) = element_type.find('_') {
        element_type[underscore_pos + 1..].to_string()
    } else {
        element_type.to_string()
    }
}

// =============================================================================
// In-Module Generation (for embedding in per-module files)
// =============================================================================

/// Generate C++ code for collections to embed in a module's wrapper header
/// 
/// This includes the C++ headers and all collection wrapper code.
pub fn generate_module_cpp_collections(collections: &[CollectionInfo]) -> String {
    if collections.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    
    output.push_str("\n// ========================\n");
    output.push_str("// Collection type wrappers\n");
    output.push_str("// ========================\n\n");
    
    // Generate code for each collection
    for info in collections {
        output.push_str(&generate_cpp_collection(info));
    }
    
    output
}

/// Get additional C++ headers needed for collections in a module
pub fn get_collection_headers(collections: &[CollectionInfo]) -> Vec<String> {
    let mut headers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for info in collections {
        // Add element type header
        headers.insert(format!("{}.hxx", info.element_type));
        // Add collection type header
        headers.insert(format!("{}.hxx", info.typedef_name));
    }
    
    let mut headers_sorted: Vec<_> = headers.into_iter().collect();
    headers_sorted.sort();
    headers_sorted
}

/// Generate Rust FFI declarations for collections to embed in a module's ffi block
/// 
/// Returns a tuple of:
/// - Type alias declarations (for cross-module element types)
/// - FFI function declarations
pub fn generate_module_rust_ffi_collections(
    collections: &[CollectionInfo],
    module_name: &str,
) -> (String, String) {
    if collections.is_empty() {
        return (String::new(), String::new());
    }
    
    let mut type_aliases = String::new();
    let mut ffi_decls = String::new();
    
    // Collect element types that are from other modules
    let mut cross_module_elements: std::collections::HashSet<(&str, &str)> = std::collections::HashSet::new();
    for info in collections {
        if info.element_module != module_name {
            cross_module_elements.insert((&info.element_type, &info.element_module));
        }
    }
    
    // Generate type aliases for cross-module element types
    if !cross_module_elements.is_empty() {
        type_aliases.push_str("                // Collection element types from other modules\n");
        for (element_type, element_module) in &cross_module_elements {
            let element_short = element_short_name(element_type);
            type_aliases.push_str(&format!(
                "                type {element_type} = crate::{element_module}::ffi::{element_short};\n"
            ));
        }
        type_aliases.push_str("\n");
    }
    
    // Generate FFI declarations
    ffi_decls.push_str("                // ========================\n");
    ffi_decls.push_str("                // Collection type wrappers\n");
    ffi_decls.push_str("                // ========================\n\n");
    
    for info in collections {
        ffi_decls.push_str(&generate_rust_ffi_collection(info));
    }
    
    (type_aliases, ffi_decls)
}

/// Generate Rust re-exports and impl blocks for collections to append after a module's ffi block
pub fn generate_module_rust_impl_collections(
    collections: &[CollectionInfo],
    module_name: &str,
) -> String {
    if collections.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    
    // Re-exports for collection types
    output.push_str("// Collection type re-exports\n");
    output.push_str("pub use ffi::{\n");
    for (i, info) in collections.iter().enumerate() {
        if i > 0 {
            output.push_str(",\n");
        }
        output.push_str(&format!("    {}", info.short_name));
    }
    output.push_str(",\n};\n\n");
    
    // Import element types for impl blocks (from other modules)
    let mut cross_module_elements: std::collections::HashSet<(&str, &str)> = std::collections::HashSet::new();
    for info in collections {
        if info.element_module != module_name {
            cross_module_elements.insert((&info.element_type, &info.element_module));
        }
    }
    
    for (element_type, element_module) in &cross_module_elements {
        let element_short = element_short_name(element_type);
        output.push_str(&format!(
            "use crate::{element_module}::{element_short};\n"
        ));
    }
    if !cross_module_elements.is_empty() {
        output.push_str("\n");
    }
    
    // Generate impl blocks for each collection
    for info in collections {
        output.push_str(&generate_rust_impl_collection(info));
        output.push_str("\n");
    }
    
    output
}

/// Generate the complete collections module C++ header (DEPRECATED - use generate_module_cpp_collections)
pub fn generate_collections_cpp_header(collections: &[CollectionInfo]) -> String {
    let mut output = String::new();
    
    output.push_str("// Generated C++ helpers for OCCT collection types\n");
    output.push_str("// Pattern: Each collection type gets:\n");
    output.push_str("//   - TypeName_new() -> UniquePtr<Type>           // Construction\n");
    output.push_str("//   - TypeName_iter(coll) -> UniquePtr<Iterator>  // Create iterator\n");
    output.push_str("//   - TypeNameIterator_next(iter) -> UniquePtr<T> // Advance (nullptr when done)\n");
    output.push_str("//   - TypeName_append/add(coll, item)             // Add elements\n\n");
    
    output.push_str("#pragma once\n");
    output.push_str("#include \"rust/cxx.h\"\n");
    output.push_str("#include <memory>\n\n");
    
    // Collect unique headers needed
    let mut headers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for info in collections {
        // Add element type header
        headers.insert(format!("{}.hxx", info.element_type));
        // Add collection type header
        headers.insert(format!("{}.hxx", info.typedef_name));
    }
    
    let mut headers_sorted: Vec<_> = headers.into_iter().collect();
    headers_sorted.sort();
    
    for header in headers_sorted {
        output.push_str(&format!("#include <{}>\n", header));
    }
    output.push_str("\n");
    
    // Generate code for each collection
    for info in collections {
        output.push_str(&generate_cpp_collection(info));
    }
    
    output
}

/// Generate the complete collections module Rust code
pub fn generate_collections_rust_module(collections: &[CollectionInfo]) -> String {
    let mut output = String::new();
    
    output.push_str("//! Generated collection helpers for OCCT NCollection types\n");
    output.push_str("//!\n");
    output.push_str("//! These wrappers provide idiomatic Rust iteration over OCCT collection types.\n\n");
    
    output.push_str("#[cxx::bridge]\n");
    output.push_str("pub mod ffi {\n");
    output.push_str("    unsafe extern \"C++\" {\n");
    output.push_str("        include!(\"opencascade-sys/generated/collections.hxx\");\n\n");
    
    // Add type aliases for element types from other modules
    let mut element_types: std::collections::HashSet<(&str, &str)> = std::collections::HashSet::new();
    for info in collections {
        element_types.insert((&info.element_type, &info.element_module));
    }
    
    for (element_type, element_module) in &element_types {
        let element_short = element_short_name(element_type);
        output.push_str(&format!(
            "        type {element_type} = crate::{element_module}::ffi::{element_short};\n"
        ));
    }
    output.push_str("\n");
    
    // Generate FFI for each collection
    for info in collections {
        output.push_str(&generate_rust_ffi_collection(info));
    }
    
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Re-export collection types
    output.push_str("pub use ffi::{\n");
    for (i, info) in collections.iter().enumerate() {
        if i > 0 {
            output.push_str(",\n");
        }
        output.push_str(&format!("    {}", info.short_name));
    }
    output.push_str(",\n};\n\n");
    
    // Import element types for impl blocks
    for (element_type, element_module) in &element_types {
        let element_short = element_short_name(element_type);
        output.push_str(&format!(
            "use crate::{element_module}::{element_short};\n"
        ));
    }
    output.push_str("\n");
    
    // Generate impl blocks for each collection
    for info in collections {
        output.push_str(&generate_rust_impl_collection(info));
        output.push_str("\n");
    }
    
    output
}
