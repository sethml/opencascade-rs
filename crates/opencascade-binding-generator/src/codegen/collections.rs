//! Code generation for NCollection-based collection types
//!
//! Generates iterator wrappers and helper functions for OCCT collection typedefs like
//! TopTools_ListOfShape, TopTools_SequenceOfShape, etc.
//!
//! This module recognizes collection typedefs by pattern matching on their names
//! and generates the appropriate C++ and Rust wrapper code.

use std::collections::HashMap;
use std::fmt::Write;

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
    /// NCollection_Array1 - fixed-size 1D array (typedef of template instantiation)
    Array1,
    /// NCollection_Array2 - fixed-size 2D array (typedef of template instantiation)
    Array2,
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
    
    // TColgp Array1 types (typedef NCollection_Array1<T>)
    map.insert("TColgp_Array1OfCirc2d", CollectionMetadata::Simple {
        element_type: "gp_Circ2d",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfDir", CollectionMetadata::Simple {
        element_type: "gp_Dir",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfDir2d", CollectionMetadata::Simple {
        element_type: "gp_Dir2d",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfLin2d", CollectionMetadata::Simple {
        element_type: "gp_Lin2d",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfPnt", CollectionMetadata::Simple {
        element_type: "gp_Pnt",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfPnt2d", CollectionMetadata::Simple {
        element_type: "gp_Pnt2d",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfVec", CollectionMetadata::Simple {
        element_type: "gp_Vec",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfVec2d", CollectionMetadata::Simple {
        element_type: "gp_Vec2d",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfXY", CollectionMetadata::Simple {
        element_type: "gp_XY",
        kind: CollectionKind::Array1,
    });
    map.insert("TColgp_Array1OfXYZ", CollectionMetadata::Simple {
        element_type: "gp_XYZ",
        kind: CollectionKind::Array1,
    });

    // TColgp Array2 types (typedef NCollection_Array2<T>)
    map.insert("TColgp_Array2OfCirc2d", CollectionMetadata::Simple {
        element_type: "gp_Circ2d",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfDir", CollectionMetadata::Simple {
        element_type: "gp_Dir",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfDir2d", CollectionMetadata::Simple {
        element_type: "gp_Dir2d",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfLin2d", CollectionMetadata::Simple {
        element_type: "gp_Lin2d",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfPnt", CollectionMetadata::Simple {
        element_type: "gp_Pnt",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfPnt2d", CollectionMetadata::Simple {
        element_type: "gp_Pnt2d",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfVec", CollectionMetadata::Simple {
        element_type: "gp_Vec",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfVec2d", CollectionMetadata::Simple {
        element_type: "gp_Vec2d",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfXY", CollectionMetadata::Simple {
        element_type: "gp_XY",
        kind: CollectionKind::Array2,
    });
    map.insert("TColgp_Array2OfXYZ", CollectionMetadata::Simple {
        element_type: "gp_XYZ",
        kind: CollectionKind::Array2,
    });

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
    let (module_cpp, module) = if let Some(underscore_pos) = typedef_name.find('_') {
        let module_cpp = &typedef_name[..underscore_pos];
        (module_cpp, crate::module_graph::module_to_rust_name(module_cpp))
    } else {
        return None;
    };
    
    // Extract short name (e.g., "ListOfShape" from "TopTools_ListOfShape")
    let short_name = crate::type_mapping::short_name_for_module(typedef_name, module_cpp);
    
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
        CollectionKind::Array1 => {
            generate_cpp_array1_collection(&mut output, typedef_name, element_type);
        }
        CollectionKind::Array2 => {
            generate_cpp_array2_collection(&mut output, typedef_name, element_type);
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
    output.push_str(&format!(
        "    auto result = new {element_type}(*iter->current);\n"
    ));
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
    output.push_str(&format!(
        "    auto result = new {element_type}(iter->coll->{access_method}(iter->index));\n"
    ));
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
    output.push_str(&format!(
        "    auto result = new {key_type}(iter->inner.Key());\n"
    ));
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
    output.push_str(&format!(
        "    return new {value_type}(*found);\n"
    ));
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
    output.push_str(&format!(
        "    auto result = new {key_type}(iter->coll->FindKey(iter->index));\n"
    ));
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
    output.push_str(&format!(
        "    return new {key_type}(coll->FindKey(index));\n"
    ));
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
pub fn generate_rust_ffi_collections(collections: &[CollectionInfo]) -> (String, String) {
    if collections.is_empty() {
        return (String::new(), String::new());
    }
    
    // Generate opaque struct declarations (outside extern "C" block)
    let mut type_decls = String::new();
    type_decls.push_str("// ========================\n");
    type_decls.push_str("// Collection types (opaque)\n");
    type_decls.push_str("// ========================\n\n");
    for info in collections {
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
    
    for info in collections {
        ffi_decls.push_str(&generate_rust_ffi_collection(info));
    }
    
    (type_decls, ffi_decls)
}

/// Generate Rust FFI declarations for a single collection
/// Uses full C++ type names (e.g., TopoDS_Shape, TopTools_ListOfShape)
fn generate_rust_ffi_collection(info: &CollectionInfo) -> String {
    match info.kind {
        CollectionKind::Array1 => return generate_rust_ffi_array1(info),
        CollectionKind::Array2 => return generate_rust_ffi_array2(info),
        _ => {}
    }

    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let iter_name = format!("{}Iterator", info.short_name);
    
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
            output.push_str(&format!("    pub fn {}_append(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, info.element_type));
            output.push_str("    /// Prepend an element to the list\n");
            output.push_str(&format!("    pub fn {}_prepend(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, info.element_type));
        }
        CollectionKind::Sequence => {
            output.push_str("    /// Append an element to the sequence\n");
            output.push_str(&format!("    pub fn {}_append(coll: *mut {}, item: *const {});\n\n", coll_name, coll_name, info.element_type));
            output.push_str("    /// Get element at 1-based index\n");
            output.push_str(&format!("    pub fn {}_value(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, info.element_type));
        }
        CollectionKind::IndexedMap | CollectionKind::Map => {
            output.push_str("    /// Add an element to the map/set\n");
            output.push_str(&format!("    pub fn {}_add(coll: *mut {}, item: *const {}) -> i32;\n\n", coll_name, coll_name, info.element_type));
            if info.kind == CollectionKind::IndexedMap {
                output.push_str("    /// Get element at 1-based index\n");
                output.push_str(&format!("    pub fn {}_find_key(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, info.element_type));
            }
        }
        CollectionKind::DataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("    /// Bind a key to a value\n");
                output.push_str(&format!("    pub fn {}_bind(coll: *mut {}, key: *const {}, value: *const {}) -> bool;\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("    /// Find a value by key (returns nullptr if not found)\n");
                output.push_str(&format!("    pub fn {}_find(coll: *const {}, key: *const {}) -> *mut {};\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn {}_contains(coll: *const {}, key: *const {}) -> bool;\n\n", coll_name, coll_name, info.element_type));
            }
        }
        CollectionKind::IndexedDataMap => {
            if let Some(ref value_type) = info.value_type {
                output.push_str("    /// Add a key-value pair, returns index (existing or new)\n");
                output.push_str(&format!("    pub fn {}_add(coll: *mut {}, key: *const {}, value: *const {}) -> i32;\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("    /// Find value by key (returns reference)\n");
                output.push_str(&format!("    pub fn {}_find_from_key(coll: *const {}, key: *const {}) -> *const {};\n\n", coll_name, coll_name, info.element_type, value_type));
                output.push_str("    /// Find value by 1-based index (returns reference)\n");
                output.push_str(&format!("    pub fn {}_find_from_index(coll: *const {}, index: i32) -> *const {};\n\n", coll_name, coll_name, value_type));
                output.push_str("    /// Find key by 1-based index\n");
                output.push_str(&format!("    pub fn {}_find_key(coll: *const {}, index: i32) -> *mut {};\n\n", coll_name, coll_name, info.element_type));
                output.push_str("    /// Find index by key (returns 0 if not found)\n");
                output.push_str(&format!("    pub fn {}_find_index(coll: *const {}, key: *const {}) -> i32;\n\n", coll_name, coll_name, info.element_type));
                output.push_str("    /// Check if key exists\n");
                output.push_str(&format!("    pub fn {}_contains(coll: *const {}, key: *const {}) -> bool;\n\n", coll_name, coll_name, info.element_type));
            }
        }
        CollectionKind::Array1 | CollectionKind::Array2 => {
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
    output.push_str(&format!("    pub fn {}(iter: *mut {}) -> *mut {};\n\n", next_fn_name, iter_name, info.element_type));
    
    // Iterator destructor
    output.push_str(&format!("    /// Destroy a {}\n", iter_name));
    output.push_str(&format!("    pub fn {}_destructor(self_: *mut {});\n\n", iter_name, iter_name));
    
    output
}

/// Generate Rust FFI declarations for an Array1 collection
fn generate_rust_ffi_array1(info: &CollectionInfo) -> String {
    let mut output = String::new();
    let coll_name = &info.typedef_name;
    let elem = &info.element_type;

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
    let elem = &info.element_type;

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
        headers.insert(format!("{}.hxx", info.element_type));
        headers.insert(format!("{}.hxx", info.typedef_name));
        if let Some(ref value_type) = info.value_type {
            headers.insert(format!("{}.hxx", value_type));
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
