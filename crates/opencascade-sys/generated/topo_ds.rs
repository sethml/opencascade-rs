#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_topo_ds.hxx");
        #[doc = "Describes a shape which - references an underlying shape with the potential to be given a location and an orientation - has a location for the underlying shape, giving its placement in the local coordinate system - has an orientation for the underlying shape, in terms of its geometry (as opposed to orientation in relation to other shapes). Note: A Shape is empty if it references an underlying shape which has an empty list of shapes."]
        #[cxx_name = "TopoDS_Shape"]
        type Shape;
        #[doc = "Creates a NULL Shape referring to nothing."]
        #[cxx_name = "TopoDS_Shape_ctor"]
        fn Shape_ctor() -> UniquePtr<Shape>;
    }
    impl UniquePtr<Shape> {}
}
pub use ffi::Shape;
pub use ffi::Shape_ctor;
