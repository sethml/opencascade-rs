#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_message.hxx");
        #[doc = " ======================== Message_ProgressRange ========================"]
        #[doc = "Auxiliary class representing a part of the global progress scale allocated by a step of the progress scope, see Message_ProgressScope::Next(). A range object takes responsibility of advancing the progress by the size of allocated step, which is then performed depending on how it is used: - If Message_ProgressScope object is created using this range as argument, then this respondibility is taken over by that scope. - Otherwise, a range advances progress directly upon destruction. A range object can be copied, the responsibility for progress advancement is then taken by the copy. The same range object may be used (either copied or used to create scope) only once. Any consequent attempts to use range will give no result on the progress; in debug mode, an assert message will be generated. @sa Message_ProgressScope for more details"]
        #[cxx_name = "Message_ProgressRange"]
        type ProgressRange;
        #[doc = "Constructor of the empty range"]
        #[cxx_name = "Message_ProgressRange_ctor"]
        fn ProgressRange_ctor() -> UniquePtr<ProgressRange>;
        #[doc = "Copy constructor disarms the source"]
        #[cxx_name = "Message_ProgressRange_ctor_progressrange"]
        fn ProgressRange_ctor_progressrange(theOther: &ProgressRange) -> UniquePtr<ProgressRange>;
        #[doc = "Returns true if ProgressIndicator signals UserBreak"]
        #[cxx_name = "UserBreak"]
        fn user_break(self: &ProgressRange) -> bool;
        #[doc = "Returns false if ProgressIndicator signals UserBreak"]
        #[cxx_name = "More"]
        fn more(self: &ProgressRange) -> bool;
        #[doc = "Returns true if this progress range is attached to some indicator."]
        #[cxx_name = "IsActive"]
        fn is_active(self: &ProgressRange) -> bool;
        #[doc = "Closes the current range and advances indicator"]
        #[cxx_name = "Close"]
        fn close(self: Pin<&mut ProgressRange>);
    }
    impl UniquePtr<ProgressRange> {}
}
pub use ffi::ProgressRange;
impl ProgressRange {
    #[doc = "Constructor of the empty range"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::ProgressRange_ctor()
    }

    #[doc = "Copy constructor disarms the source"]
    pub fn new_progressrange(theOther: &ffi::ProgressRange) -> cxx::UniquePtr<Self> {
        ffi::ProgressRange_ctor_progressrange(theOther)
    }
}
