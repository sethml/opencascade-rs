// Module TopoDS
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    TopoDS_Compound_as_shape as Compound_as_shape,
    TopoDS_Compound_ctor as Compound_ctor,
    TopoDS_Compound_to_owned as Compound_to_owned,
    TopoDS_Edge_to_owned as Edge_to_owned,
    TopoDS_Face_ctor as Face_ctor,
    TopoDS_Face_to_owned as Face_to_owned,
    TopoDS_Shape_to_owned as Shape_to_owned,
    TopoDS_Shell_as_shape as Shell_as_shape,
    TopoDS_Shell_ctor as Shell_ctor,
    TopoDS_Shell_to_owned as Shell_to_owned,
    TopoDS_Solid_to_owned as Solid_to_owned,
    TopoDS_Vertex_to_owned as Vertex_to_owned,
    TopoDS_Wire_to_owned as Wire_to_owned,
    TopoDS_cast_to_compound as cast_to_compound,
    TopoDS_cast_to_edge as cast_to_edge,
    TopoDS_cast_to_face as cast_to_face,
    TopoDS_cast_to_shell as cast_to_shell,
    TopoDS_cast_to_solid as cast_to_solid,
    TopoDS_cast_to_vertex as cast_to_vertex,
    TopoDS_cast_to_wire as cast_to_wire,
    cast_compound_to_shape,
    cast_edge_to_shape,
    cast_face_to_shape,
    cast_shell_to_shape,
    cast_solid_to_shape,
    cast_vertex_to_shape,
    cast_wire_to_shape,
};

// Type aliases
pub type Builder = crate::ffi::TopoDS_Builder;
pub type Compound = crate::ffi::TopoDS_Compound;
pub type Edge = crate::ffi::TopoDS_Edge;
pub type Face = crate::ffi::TopoDS_Face;
pub type Shape = crate::ffi::TopoDS_Shape;
pub type Shell = crate::ffi::TopoDS_Shell;
pub type Solid = crate::ffi::TopoDS_Solid;
pub type Vertex = crate::ffi::TopoDS_Vertex;
pub type Wire = crate::ffi::TopoDS_Wire;
