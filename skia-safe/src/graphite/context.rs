use crate::graphite::{InsertRecordingInfo, InsertStatus, Recorder, RecorderOptions, SubmitInfo};
use crate::prelude::*;
use crate::{IPoint, ImageInfo, Surface};
use skia_bindings as sb;
use std::fmt;

// `skgpu::graphite::Context` is `final` with no base class and is handed out as
// `std::unique_ptr<Context>` (Context::MakeMetal etc.). It is NOT ref-counted,
// so it must be modeled as a `RefHandle` whose drop `delete`s it — modeling it
// as an `RCHandle` would call `SkRefCntBase::unref()` on a non-ref-counted
// object (UB; the real `~Context()` never runs -> leak).
pub type Context = RefHandle<sb::skgpu_graphite_Context>;

// Deliberately NOT `Send`/`Sync`: a Graphite `Context` is thread-affine (it must
// be used on the thread it was created on) and has no internal lock. This matches
// the Ganesh `gpu::DirectContext`, which is likewise neither `Send` nor `Sync`.
//
// The mutating C++ methods (`makeRecorder`, `insertRecording`, `submit`, …) are
// non-`const` and so take `&mut self`, mirroring `DirectContext::submit`; only
// `isDeviceLost() const` keeps `&self`.

impl NativeDrop for sb::skgpu_graphite_Context {
    fn drop(&mut self) {
        unsafe { sb::C_Context_delete(self) }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("is_device_lost", &self.is_device_lost())
            .finish()
    }
}

impl Context {
    /// Create a new recorder for recording draw operations
    ///
    /// # Arguments
    /// - `options` - Configuration for the recorder, or `None` for default options
    ///
    /// # Returns
    /// A new `Recorder` instance, or `None` if creation failed
    pub fn make_recorder(&mut self, options: Option<&RecorderOptions>) -> Option<Recorder> {
        let default_options;
        let options_ptr = match options {
            Some(opts) => opts.native() as *const _,
            None => {
                default_options = RecorderOptions::default();
                default_options.native() as *const _
            }
        };

        let recorder_ptr = unsafe { sb::C_Context_makeRecorder(self.native_mut(), options_ptr) };
        Recorder::from_ptr(recorder_ptr)
    }

    /// Insert a recording into the context for later submission
    ///
    /// # Arguments
    /// - `info` - Information about the recording to insert
    ///
    /// # Returns
    /// [`InsertStatus::Success`], or the specific failure reason (see the
    /// [`InsertStatus`] variants).
    pub fn insert_recording(&mut self, info: &InsertRecordingInfo<'_>) -> InsertStatus {
        unsafe { sb::C_Context_insertRecording(self.native_mut(), info.native()) }
    }

    /// Submit pending work to the GPU
    ///
    /// # Arguments
    /// - `submit_info` - Information about the submission, or `None` for defaults
    ///
    /// # Returns
    /// `true` if submission was successful, `false` otherwise
    pub fn submit(&mut self, submit_info: Option<&SubmitInfo>) -> bool {
        let default_info;
        let info_ptr = match submit_info {
            Some(info) => info.native() as *const _,
            None => {
                default_info = SubmitInfo::default();
                default_info.native() as *const _
            }
        };

        unsafe { sb::C_Context_submit(self.native_mut(), info_ptr) }
    }

    /// Submit pending work and block until it has completed on the GPU.
    ///
    /// Submits with `SyncToCpu::kYes`, so — unlike [`submit`](Self::submit) —
    /// this returns only after the GPU has finished the submitted work.
    ///
    /// # Returns
    /// `true` if submission succeeded.
    pub fn submit_and_wait(&mut self) -> bool {
        self.submit(Some(&SubmitInfo::with_sync_to_cpu(true)))
    }

    /// Pump any already-finished asynchronous work (invoking its finished procs).
    ///
    /// This does **not** block or report completion status: the underlying
    /// `Context::checkAsyncWorkCompletion()` returns `void`. To wait for GPU
    /// completion, use [`submit_and_wait`](Self::submit_and_wait).
    pub fn check_async_work_completion(&mut self) {
        unsafe { sb::C_Context_checkAsyncWorkCompletion(self.native_mut()) }
    }

    /// Delete a backend texture that was created through this context
    ///
    /// # Arguments
    /// - `texture` - The backend texture to delete
    pub fn delete_backend_texture(&mut self, texture: &crate::graphite::BackendTexture) {
        unsafe {
            sb::C_Context_deleteBackendTexture(self.native_mut(), texture.native());
        }
    }

    /// Check if the GPU device has been lost
    ///
    /// # Returns
    /// `true` if the device is lost and the context is unusable
    pub fn is_device_lost(&self) -> bool {
        unsafe { sb::C_Context_isDeviceLost(self.native()) }
    }

    /// Synchronously read pixels from a Graphite-backed `surface`.
    ///
    /// Graphite is a deferred backend, so [`Surface::read_pixels`] does not work;
    /// this drives `Context::asyncRescaleAndReadPixels` to completion with a
    /// synchronous submit (it blocks until the GPU work and the readback copy
    /// have finished). This is the supported path for a screenshot / golden-image
    /// capture.
    ///
    /// Draw operations still pending on the surface's `Recorder` are **not**
    /// flushed by this call — the readback path never consults the recorder, so
    /// unsnapped work silently yields stale (typically blank) pixels. Snap and
    /// insert the recording first:
    ///
    /// ```no_run
    /// # use skia_safe::graphite;
    /// # fn f(context: &mut graphite::Context, recorder: &mut graphite::Recorder,
    /// #      surface: &mut skia_safe::Surface) {
    /// // ... draw to surface.canvas() ...
    /// let mut recording = recorder.snap().expect("snap");
    /// context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
    /// let info = surface.image_info();
    /// let mut pixels = vec![0u8; info.min_row_bytes() * info.height() as usize];
    /// let ok = context.read_pixels(surface, &info, &mut pixels, info.min_row_bytes(), (0, 0));
    /// # }
    /// ```
    ///
    /// `dst_pixels` receives `dst_info.height()` rows of `dst_row_bytes` each and
    /// must be at least `dst_row_bytes * dst_info.height()` bytes; `dst_row_bytes`
    /// must be at least `dst_info.min_row_bytes()`. `src` is the top-left source
    /// pixel to start reading from.
    ///
    /// # Returns
    /// `true` if the pixels were read back successfully.
    ///
    /// [`Surface::read_pixels`]: crate::Surface::read_pixels
    pub fn read_pixels(
        &mut self,
        surface: &mut Surface,
        dst_info: &ImageInfo,
        dst_pixels: &mut [u8],
        dst_row_bytes: usize,
        src: impl Into<IPoint>,
    ) -> bool {
        let src = src.into();
        if dst_row_bytes < dst_info.min_row_bytes() {
            return false;
        }
        match dst_row_bytes.checked_mul(dst_info.height() as usize) {
            Some(required) if dst_pixels.len() >= required => {}
            _ => return false,
        }

        unsafe {
            sb::C_Context_readPixels(
                self.native_mut(),
                surface.native_mut(),
                dst_info.native(),
                dst_pixels.as_mut_ptr() as *mut std::ffi::c_void,
                dst_row_bytes,
                src.x,
                src.y,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_debug() {
        // We can't easily create a Context without platform-specific setup,
        // but we can test that the debug implementation compiles
        let context: Option<Context> = None;
        assert!(context.is_none());
    }
}
