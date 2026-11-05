use skia_bindings as sb;

use crate::{IRect, IVector, Surface, gpu::MutableTextureState, prelude::*};

/// Status of recording insertion (`skgpu::graphite::InsertStatus::V`).
///
/// [`Success`](InsertStatus::Success) is the only success value; every other
/// variant describes why the insertion failed (see the Skia documentation —
/// `AddCommandsFailed` and `AsyncShaderCompilesFailed` leave the context in an
/// unrecoverable state).
pub use sb::skgpu_graphite_InsertStatus_V as InsertStatus;
variant_name!(InsertStatus::Success);

/// Information for inserting a recording into the context.
///
/// Borrows the [`Recording`](super::Recording) it references (it stores
/// a raw `fRecording` pointer), so the borrow checker keeps the `Recording` alive
/// for as long as this info — and any `insert_recording` call using it — is in use.
/// The target surface and texture state set through the builder methods are
/// borrowed for the same lifetime.
#[derive(Debug)]
pub struct InsertRecordingInfo<'a> {
    inner: std::ptr::NonNull<sb::skgpu_graphite_InsertRecordingInfo>,
    _borrows: std::marker::PhantomData<(
        &'a mut super::Recording,
        &'a mut Surface,
        &'a MutableTextureState,
    )>,
}

impl Drop for InsertRecordingInfo<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_InsertRecordingInfo_delete(self.inner.as_ptr()) }
    }
}

impl<'a> InsertRecordingInfo<'a> {
    /// Create insert recording info for a recording.
    ///
    /// Takes the recording as `&mut` because `Context::insert_recording`
    /// mutates it through the stored pointer (it sets failure results for
    /// finished procs and deinstantiates volatile lazy proxies).
    pub fn new(recording: &'a mut super::Recording) -> Self {
        // `InsertRecordingInfo` is not POD — `fSimulatedStatus` is an
        // `InsertStatus` holding a `std::string` — and a libstdc++
        // `std::string` in SSO state points into itself, so the struct cannot
        // live in (and be moved with) Rust-owned storage. It is kept at a
        // stable heap address instead (`new`/`delete` on the C++ side).
        let inner = std::ptr::NonNull::new(unsafe { sb::C_InsertRecordingInfo_new() })
            .expect("C_InsertRecordingInfo_new returned null");
        unsafe { (*inner.as_ptr()).fRecording = recording.native_ptr() };
        Self {
            inner,
            _borrows: std::marker::PhantomData,
        }
    }

    /// Sets the surface that the draws of a deferred canvas replay into.
    ///
    /// A recording that contains draws on a canvas from
    /// [`Recorder::make_deferred_canvas`](super::Recorder::make_deferred_canvas) needs a target
    /// surface, or [`Context::insert_recording`](super::Context::insert_recording) fails. A
    /// recording without such draws replays nothing into the surface, but can still leave its
    /// texture in a new state through
    /// [`target_surface_with_state`](Self::target_surface_with_state). The surface must be
    /// Graphite-backed, or the insert fails.
    ///
    /// The texture of the surface must match the [`TextureInfo`](super::TextureInfo) that the
    /// deferred canvas was made with. Skia checks this only in debug builds, so release builds
    /// replay the draws into a mismatched texture.
    ///
    /// A mipmapped deferred canvas has stricter rules: the surface must have the same dimensions,
    /// and the insert fails with any [translation](Self::target_translation) or
    /// [clip](Self::target_clip).
    pub fn target_surface(self, surface: &'a mut Surface) -> Self {
        unsafe { (*self.inner.as_ptr()).fTargetSurface = surface.native_mut() };
        self
    }

    /// Offsets the draws that replay into the [target surface](Self::target_surface).
    pub fn target_translation(self, translation: impl Into<IVector>) -> Self {
        unsafe { (*self.inner.as_ptr()).fTargetTranslation = translation.into().into_native() };
        self
    }

    /// Clips the draws that replay into the [target surface](Self::target_surface).
    ///
    /// The clip is in the deferred canvas's coordinates, and Skia moves it by the
    /// [translation](Self::target_translation) together with the draws. Without a clip, the draws
    /// are only clipped to the target surface's bounds. An empty clip is the same as no clip.
    pub fn target_clip(self, clip: impl AsRef<IRect>) -> Self {
        unsafe { (*self.inner.as_ptr()).fTargetClip = *clip.as_ref().native() };
        self
    }

    /// Sets the [target surface](Self::target_surface) together with the state, such as the
    /// Vulkan image layout, that its texture is left in after the recording.
    ///
    /// The state is set only together with a surface, because Skia applies it to the target
    /// surface without checking that there is one. Only the Vulkan backend applies the state, and
    /// the other backends ignore it.
    pub fn target_surface_with_state(
        self,
        surface: &'a mut Surface,
        state: &'a MutableTextureState,
    ) -> Self {
        let this = self.target_surface(surface);
        // Skia only reads the state, but the field is declared as a mutable pointer.
        unsafe {
            (*this.inner.as_ptr()).fTargetTextureState =
                state.native() as *const _ as *mut sb::skgpu_MutableTextureState
        };
        this
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_InsertRecordingInfo {
        unsafe { self.inner.as_ref() }
    }
}

/// Information for submitting work to the GPU
#[derive(Debug)]
pub struct SubmitInfo {
    inner: sb::skgpu_graphite_SubmitInfo,
}

impl Default for SubmitInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SubmitInfo {
    /// Create new submit info with default settings (no CPU sync).
    ///
    /// Every `SubmitInfo` field's zero value equals its C++ default
    /// ([`SyncToCpu::No`], no frame boundary marked, `0`, null procs), so
    /// zero-init is a valid default here.
    pub fn new() -> Self {
        // Every field's zero value equals its C++ default, so a zeroed struct is
        // a valid default `SubmitInfo`.
        let inner = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self { inner }
    }

    /// Submit info whose `fSync` is set so that `Context::submit` blocks until
    /// the submitted GPU work has completed ([`SyncToCpu::Yes`] when `sync`).
    pub fn with_sync_to_cpu(sync: bool) -> Self {
        let mut info = Self::new();
        info.inner.fSync = if sync { SyncToCpu::Yes } else { SyncToCpu::No };
        info
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_SubmitInfo {
        &self.inner
    }
}

/// Synchronization mode for GPU operations
pub use sb::skgpu_graphite_SyncToCpu as SyncToCpu;
variant_name!(SyncToCpu::Yes);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_info_creation() {
        let info = SubmitInfo::new();
        let _default_info = SubmitInfo::default();
        // Should not panic and should create valid info
        let _ = format!("{:?}", info);
    }
}
