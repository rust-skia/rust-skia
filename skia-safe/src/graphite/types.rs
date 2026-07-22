use skia_bindings as sb;

use crate::prelude::*;
// Re-export the shared `skgpu` types through `gpu` rather than aliasing the
// raw bindings again: `BackendApi` / `Mipmapped` are the same native types, and
// `Budgeted` gets `gpu`'s bool-newtype wrapper instead of a second, name-clashing
// incompatible type.
pub use crate::gpu::{BackendApi, Budgeted, Mipmapped};

/// Status of recording insertion (`skgpu::graphite::InsertStatus::V`).
///
/// [`Success`](InsertStatus::Success) is the only success value; every other
/// variant describes why the insertion failed (see the Skia documentation —
/// `AddCommandsFailed` and `AsyncShaderCompilesFailed` leave the context in an
/// unrecoverable state).
pub use sb::skgpu_graphite_InsertStatus_V as InsertStatus;
variant_name!(InsertStatus::Success);

/// Configuration for recorder creation
pub type RecorderOptions = Handle<sb::skgpu_graphite_RecorderOptions>;

impl NativeDrop for sb::skgpu_graphite_RecorderOptions {
    fn drop(&mut self) {
        unsafe { sb::C_RecorderOptions_destruct(self) }
    }
}

impl std::fmt::Debug for RecorderOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecorderOptions").finish()
    }
}

impl Default for RecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl RecorderOptions {
    /// Create new recorder options with the C++ defaults (e.g. a 256 MiB GPU
    /// budget). Placement-constructed rather than zero-initialized, because
    /// `RecorderOptions` has a non-trivial constructor and members (an `sk_sp`,
    /// a `std::optional`, and a non-zero default budget).
    pub fn new() -> Self {
        Self::construct(|options| unsafe { sb::C_RecorderOptions_Construct(options) })
    }
}

/// Information for inserting a recording into the context.
///
/// Borrows the [`Recording`](crate::graphite::Recording) it references (it stores
/// a raw `fRecording` pointer), so the borrow checker keeps the `Recording` alive
/// for as long as this info — and any `insert_recording` call using it — is in use.
#[derive(Debug)]
pub struct InsertRecordingInfo<'a> {
    inner: std::ptr::NonNull<sb::skgpu_graphite_InsertRecordingInfo>,
    _recording: std::marker::PhantomData<&'a mut crate::graphite::Recording>,
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
    pub fn new(recording: &'a mut crate::graphite::Recording) -> Self {
        // `InsertRecordingInfo` is not POD — `fSimulatedStatus` is an
        // `InsertStatus` holding a `std::string` — and a libstdc++
        // `std::string` in SSO state points into itself, so the struct cannot
        // live in (and be moved with) Rust-owned storage. It is kept at a
        // stable heap address instead (`new`/`delete` on the C++ side).
        let inner = std::ptr::NonNull::new(unsafe { sb::C_InsertRecordingInfo_new() })
            .expect("C_InsertRecordingInfo_new returned null");
        unsafe { (*inner.as_ptr()).fRecording = recording.native_mut() };
        Self {
            inner,
            _recording: std::marker::PhantomData,
        }
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
    /// (`SyncToCpu::No`, `MarkFrameBoundary::kNo`, `0`, null procs), so
    /// zero-init is a valid default here.
    pub fn new() -> Self {
        // Every field's zero value equals its C++ default, so a zeroed struct is
        // a valid default `SubmitInfo`.
        let inner = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self { inner }
    }

    /// Submit info whose `fSync` is set so that `Context::submit` blocks until
    /// the submitted GPU work has completed (`SyncToCpu::Yes` when `sync`).
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
    fn test_recorder_options_creation() {
        let options = RecorderOptions::new();
        let _default_options = RecorderOptions::default();
        // Should not panic and should create valid options
        let _ = format!("{:?}", options);
    }

    #[test]
    fn test_submit_info_creation() {
        let info = SubmitInfo::new();
        let _default_info = SubmitInfo::default();
        // Should not panic and should create valid info
        let _ = format!("{:?}", info);
    }
}
