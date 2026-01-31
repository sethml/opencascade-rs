// Module Poly
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    HandlePoly_Triangulation_Get,
    HandlePoly_Triangulation_ctor,
    Poly_Connect_ctor as Connect_ctor,
    Poly_Triangle_ctor as Triangle_ctor,
    Poly_Triangulation_Node as Triangulation_Node,
    Poly_Triangulation_Normal as Triangulation_Normal,
    Poly_Triangulation_UV as Triangulation_UV,
    Poly_Triangulation_ctor as Triangulation_ctor,
    compute_normals,
};

// Type aliases
pub use crate::ffi::HandlePoly_Triangulation;
pub type Connect = crate::ffi::Poly_Connect;
pub type Triangle = crate::ffi::Poly_Triangle;
pub type Triangulation = crate::ffi::Poly_Triangulation;
