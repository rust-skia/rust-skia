use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;

use skia_bindings as sb;

use crate::graphite::{Recording, TextureInfo, types::BackendApi};
use crate::prelude::*;
use crate::{Canvas, ImageInfo};

// `skgpu::graphite::Recorder` is handed out as `std::unique_ptr<Recorder>`
// (Context::makeRecorder) and derives from `SkRecorder`, not `SkRefCnt`. It is
// NOT ref-counted, so it must be a `RefHandle` whose drop `delete`s it.
pub type Recorder = RefHandle<sb::skgpu_graphite_Recorder>;

/// A non-owning, lifetime-bound view of a [`Recorder`] that is owned elsewhere
/// (for example by a Graphite-backed `Canvas`/surface, via `Canvas::recorder`).
///
/// Dropping a `BorrowedRecorder` does **not** `delete` the underlying recorder —
/// the owner keeps that responsibility — and the lifetime ties the borrow to the
/// object it was obtained from, so it cannot dangle.
///
/// Only shared access to the underlying [`Recorder`] is exposed (via `Deref`);
/// the mutating operations are forwarded as inherent methods. Handing out
/// `&mut Recorder` would be unsound: safe code could `mem::replace` a recorder
/// created by `Context::make_recorder` into it and end up owning (and later
/// dropping, i.e. `delete`ing) a recorder the surface still owns.
#[derive(Debug)]
pub struct BorrowedRecorder<'a> {
    recorder: ManuallyDrop<Recorder>,
    _owner: PhantomData<&'a Canvas>,
}

impl Deref for BorrowedRecorder<'_> {
    type Target = Recorder;

    fn deref(&self) -> &Recorder {
        &self.recorder
    }
}

impl<'a> BorrowedRecorder<'a> {
    pub(crate) fn from_canvas(recorder: Recorder, _canvas: &'a Canvas) -> Self {
        Self {
            recorder: ManuallyDrop::new(recorder),
            _owner: PhantomData,
        }
    }

    /// See [`Recorder::snap`].
    pub fn snap(&mut self) -> Option<Recording> {
        self.recorder.snap()
    }

    /// See [`Recorder::make_deferred_canvas`].
    pub fn make_deferred_canvas(
        &mut self,
        image_info: &ImageInfo,
        texture_info: &TextureInfo,
    ) -> Option<&Canvas> {
        self.recorder.make_deferred_canvas(image_info, texture_info)
    }
}

// Deliberately NOT `Send`/`Sync`: Skia documents a Graphite `Recorder` as
// single-threaded (it and its child objects must be used on one thread), and its
// `&self`/`&mut self` methods drive C++ with no internal lock. Matches the
// Ganesh `gpu::RecordingContext`, which is neither `Send` nor `Sync`.

impl NativeDrop for sb::skgpu_graphite_Recorder {
    fn drop(&mut self) {
        unsafe { sb::C_Recorder_delete(self) }
    }
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
        Recording::from_ptr(unsafe { sb::C_Recorder_snap(self.native_mut()) })
    }

    // Note: Canvas creation in Graphite is typically done through Surface creation
    // Surface::canvas() is the recommended way to get a canvas for drawing
    // See graphite::surfaces module for surface creation functions

    /// Returns a canvas that records into a proxy surface (instantiated on
    /// replay), targeting a texture with the given `image_info` / `texture_info`.
    ///
    /// The returned canvas is owned by the recorder and borrows it: it is only
    /// valid until the next [`snap`](Self::snap) — which the borrow checker
    /// enforces, since `snap` needs `&mut self`. Returns `None` if a deferred
    /// canvas is already outstanding for the current recording (only one may
    /// exist per recording, until the next `snap`).
    pub fn make_deferred_canvas(
        &mut self,
        image_info: &ImageInfo,
        texture_info: &TextureInfo,
    ) -> Option<&Canvas> {
        let canvas_ptr = unsafe {
            sb::C_Recorder_makeDeferredCanvas(
                self.native_mut(),
                image_info.native(),
                texture_info.native(),
            )
        };
        if canvas_ptr.is_null() {
            None
        } else {
            Some(Canvas::borrow_from_native(unsafe { &*canvas_ptr }))
        }
    }

    /// Get the backend API used by this recorder
    ///
    /// # Returns
    /// The backend API (Vulkan, Metal, etc.)
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_Recorder_backend(self.native()) }
    }
}
