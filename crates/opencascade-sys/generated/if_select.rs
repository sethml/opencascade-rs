#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    #[doc = "Qualifies an execution status : RetVoid  : normal execution which created nothing, or no data to process RetDone  : normal execution with a result RetError : error in command or input data, no execution RetFail  : execution was run and has failed RetStop  : indicates end or stop (such as Raise)"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IFSelect_ReturnStatus {
        IFSelect_RetVoid,
        IFSelect_RetDone,
        IFSelect_RetError,
        IFSelect_RetFail,
        IFSelect_RetStop,
    }
    unsafe extern "C++" {
        include!("wrapper_if_select.hxx");
    }
}
