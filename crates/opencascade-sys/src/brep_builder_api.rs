// Module BRepBuilderAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepBuilderAPI_GTransform_ctor as GTransform_ctor,
    BRepBuilderAPI_MakeEdge_CurveSurface2d as MakeEdge_CurveSurface2d,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve as MakeEdge_HandleGeomCurve,
    BRepBuilderAPI_MakeEdge_circle as MakeEdge_circle,
    BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt as MakeEdge_gp_Pnt_gp_Pnt,
    BRepBuilderAPI_MakeFace_surface as MakeFace_surface,
    BRepBuilderAPI_MakeFace_wire as MakeFace_wire,
    BRepBuilderAPI_MakeShapeOnMesh_ctor as MakeShapeOnMesh_ctor,
    BRepBuilderAPI_MakeSolid_ctor as MakeSolid_ctor,
    BRepBuilderAPI_MakeVertex_gp_Pnt as MakeVertex_gp_Pnt,
    BRepBuilderAPI_MakeWire_ctor as MakeWire_ctor,
    BRepBuilderAPI_MakeWire_edge_edge as MakeWire_edge_edge,
    BRepBuilderAPI_MakeWire_edge_edge_edge as MakeWire_edge_edge_edge,
    BRepBuilderAPI_Transform_ctor as Transform_ctor,
};

// Type aliases
pub type GTransform = crate::ffi::BRepBuilderAPI_GTransform;
pub type MakeEdge = crate::ffi::BRepBuilderAPI_MakeEdge;
pub type MakeFace = crate::ffi::BRepBuilderAPI_MakeFace;
pub type MakeShapeOnMesh = crate::ffi::BRepBuilderAPI_MakeShapeOnMesh;
pub type MakeSolid = crate::ffi::BRepBuilderAPI_MakeSolid;
pub type MakeVertex = crate::ffi::BRepBuilderAPI_MakeVertex;
pub type MakeWire = crate::ffi::BRepBuilderAPI_MakeWire;
pub type Transform = crate::ffi::BRepBuilderAPI_Transform;
