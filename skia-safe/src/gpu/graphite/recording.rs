use std::fmt;

use skia_bindings as sb;

use crate::prelude::*;

pub type Recording = RefHandle<sb::skgpu_graphite_Recording>;

// `Send` but deliberately NOT `Sync`: a `Recording` is produced on one thread
// (via `Recorder::snap`) and may be moved to and inserted on another thread (via
// `Context::insert_recording`) — a designed Graphite hand-off — so `Send` is
// required. Sharing `&Recording` across threads, however, would let two
// `Context`s read the same recording concurrently, racing on its internals;
// this mirrors `Context`/`Recorder`, which are also `!Sync`.
unsafe impl Send for Recording {}

impl NativeDelete for sb::skgpu_graphite_Recording {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_Recording_delete(ptr) }
    }
}

impl fmt::Debug for Recording {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recording").finish()
    }
}

impl Recording {
    // Recording is typically created by Recorder::snap() and consumed by Context::insert_recording()
    // No public constructor is needed as it's managed internally by Skia
}
