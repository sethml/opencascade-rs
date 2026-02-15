//! Owned pointer for C++ objects allocated with `new` / freed with `delete`.
//!
//! This replaces CXX's `UniquePtr<T>` for the extern "C" FFI approach.
//! Objects are heap-allocated by C++ wrapper functions and must be
//! freed by calling the corresponding `_destroy` function.

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// Trait for C++ types that can be deleted via an extern "C" destructor function.
///
/// # Safety
///
/// Implementations must call the correct C++ destructor for the type.
/// The pointer must have been allocated by the corresponding C++ constructor wrapper.
pub unsafe trait CppDeletable {
    unsafe fn cpp_delete(ptr: *mut Self);
}

/// An owning pointer to a C++ object allocated with `new`.
///
/// When dropped, calls the C++ destructor via `CppDeletable::cpp_delete`.
/// This is the extern "C" equivalent of CXX's `UniquePtr<T>`.
///
/// `OwnedPtr<T>` implements `Deref<Target = T>` and `DerefMut`, so methods
/// defined on `T` (the opaque FFI type) can be called directly.
#[repr(transparent)]
pub struct OwnedPtr<T: CppDeletable> {
    ptr: NonNull<T>,
}

impl<T: CppDeletable> OwnedPtr<T> {
    /// Create an `OwnedPtr` from a raw pointer returned by C++.
    ///
    /// # Safety
    ///
    /// - `ptr` must be non-null and point to a valid C++ object
    /// - The object must have been allocated with `new` (or equivalent)
    /// - Caller transfers ownership to the `OwnedPtr`
    #[inline]
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("OwnedPtr::from_raw received null pointer from C++"),
        }
    }

    /// Returns the raw const pointer to the C++ object.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    /// Returns the raw mutable pointer to the C++ object.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Consumes the `OwnedPtr` and returns the raw pointer without calling the destructor.
    ///
    /// The caller is responsible for freeing the object.
    #[inline]
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }

    /// Returns true (for API compatibility; OwnedPtr is never null).
    #[inline]
    pub fn is_null(&self) -> bool {
        false
    }
}

impl<T: CppDeletable> Drop for OwnedPtr<T> {
    fn drop(&mut self) {
        unsafe {
            T::cpp_delete(self.ptr.as_ptr());
        }
    }
}

impl<T: CppDeletable> Deref for OwnedPtr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: CppDeletable> DerefMut for OwnedPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: CppDeletable + fmt::Debug> fmt::Debug for OwnedPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OwnedPtr").field(&self.ptr).finish()
    }
}

// OwnedPtr is Send/Sync if T is — most OCCT types are not thread-safe,
// so T typically won't implement these.
unsafe impl<T: CppDeletable + Send> Send for OwnedPtr<T> {}
unsafe impl<T: CppDeletable + Sync> Sync for OwnedPtr<T> {}
