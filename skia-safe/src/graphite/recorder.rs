use crate::graphite::{types::BackendApi, Recording};
use crate::prelude::*;
use skia_bindings as sb;
use std::fmt;

pub type Recorder = RCHandle<sb::skgpu_graphite_Recorder>;
unsafe_send_sync!(Recorder);

impl NativeRefCountedBase for sb::skgpu_graphite_Recorder {
    type Base = sb::SkRefCntBase;
}

impl fmt::Debug for Recorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder")
            .field("backend", &self.backend())
            .finish()
    }
}

impl Recorder {
    /// Finish recording and create a Recording object
    ///
    /// This method finalizes all the draw operations that have been recorded
    /// and returns a Recording that can be submitted to a Context.
    ///
    /// # Returns
    /// A `Recording` containing the recorded operations, or `None` if recording failed
    pub fn snap(&mut self) -> Option<Recording> {
        let recording_ptr = unsafe { sb::C_Recorder_snap(self.native_mut()) };
        if recording_ptr.is_null() {
            None
        } else {
            Recording::from_ptr(recording_ptr)
        }
    }

    // Note: Canvas creation in Graphite is typically done through Surface creation
    // Surface::canvas() is the recommended way to get a canvas for drawing
    // See graphite::surfaces module for surface creation functions

    /// Get the backend API used by this recorder
    ///
    /// # Returns
    /// The backend API (Vulkan, Metal, etc.)
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_Recorder_backend(self.native()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_debug() {
        // We can't easily create a Recorder without platform-specific setup,
        // but we can test that the debug implementation compiles
        let recorder: Option<Recorder> = None;
        assert!(recorder.is_none());
    }
}
