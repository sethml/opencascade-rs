// Thread-safety wrappers for STEPControl_Reader and STEPControl_Writer.
//
// OCCT's STEP I/O subsystem uses global mutable state internally:
// - STEPControl_Controller::Init() writes to static globals
// - The STEP parser uses non-reentrant lex/yacc state machines
// - Message_Messenger instances are shared without synchronization
//
// This means that even two *independent* Reader or Writer instances on
// separate threads can corrupt each other's state. The wrappers below
// hold a MutexGuard for the Reader/Writer's entire lifetime, serializing
// all STEP I/O. Because MutexGuard is !Send, the wrappers are also !Send,
// preventing them from being moved to another thread.

use std::sync::{Mutex, MutexGuard};

/// Global mutex protecting OCCT's STEP I/O global state.
/// Shared between `StepReader` and `StepWriter` since they use the
/// same underlying globals (controller, parser, messenger).
static STEP_MUTEX: Mutex<()> = Mutex::new(());

/// Thread-safe wrapper around [`Reader`] (STEPControl_Reader).
///
/// Acquires [`STEP_MUTEX`] at construction and holds it until dropped,
/// serializing all STEP read operations process-wide.
///
/// All [`Reader`] methods are available via `Deref`/`DerefMut`.
pub struct StepReader {
    inner: crate::OwnedPtr<Reader>,
    _lock: MutexGuard<'static, ()>,
}

impl StepReader {
    /// Creates a new STEP reader, acquiring the global STEP I/O lock.
    ///
    /// # Panics
    /// Panics if the STEP mutex is poisoned (a previous STEP operation panicked).
    pub fn new() -> Self {
        let lock = STEP_MUTEX.lock().unwrap();
        Self {
            inner: Reader::new(),
            _lock: lock,
        }
    }

    /// Consumes this wrapper and returns the inner `OwnedPtr<Reader>`,
    /// releasing the STEP I/O lock.
    ///
    /// # Safety
    /// The caller must ensure thread-safe usage of the returned reader.
    pub fn into_inner(self) -> crate::OwnedPtr<Reader> {
        self.inner
    }
}

impl std::ops::Deref for StepReader {
    type Target = Reader;
    fn deref(&self) -> &Reader {
        &self.inner
    }
}

impl std::ops::DerefMut for StepReader {
    fn deref_mut(&mut self) -> &mut Reader {
        &mut self.inner
    }
}

/// Thread-safe wrapper around [`Writer`] (STEPControl_Writer).
///
/// Acquires [`STEP_MUTEX`] at construction and holds it until dropped,
/// serializing all STEP write operations process-wide.
///
/// All [`Writer`] methods are available via `Deref`/`DerefMut`.
pub struct StepWriter {
    inner: crate::OwnedPtr<Writer>,
    _lock: MutexGuard<'static, ()>,
}

impl StepWriter {
    /// Creates a new STEP writer, acquiring the global STEP I/O lock.
    ///
    /// # Panics
    /// Panics if the STEP mutex is poisoned (a previous STEP operation panicked).
    pub fn new() -> Self {
        let lock = STEP_MUTEX.lock().unwrap();
        Self {
            inner: Writer::new(),
            _lock: lock,
        }
    }

    /// Consumes this wrapper and returns the inner `OwnedPtr<Writer>`,
    /// releasing the STEP I/O lock.
    ///
    /// # Safety
    /// The caller must ensure thread-safe usage of the returned writer.
    pub fn into_inner(self) -> crate::OwnedPtr<Writer> {
        self.inner
    }
}

impl std::ops::Deref for StepWriter {
    type Target = Writer;
    fn deref(&self) -> &Writer {
        &self.inner
    }
}

impl std::ops::DerefMut for StepWriter {
    fn deref_mut(&mut self) -> &mut Writer {
        &mut self.inner
    }
}
