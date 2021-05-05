use crate::prelude::*;
use skia_bindings as sb;
use std::ptr;

pub use skia_bindings::GrBackendApi as BackendAPI;
#[test]
fn test_backend_api_layout() {
    let _ = BackendAPI::Dawn;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrMipmapped as Mipmapped;

#[deprecated(since = "0.35.0", note = "Use Mipmapped (with a lowercase 'm')")]
pub use skia_bindings::GrMipmapped as MipMapped;

#[test]
fn test_mipmapped_naming() {
    let _ = Mipmapped::Yes;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrRenderable as Renderable;
#[test]
fn test_renderable_naming() {
    let _ = Renderable::No;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrProtected as Protected;
#[test]
fn test_protected_naming() {
    let _ = Protected::Yes;
}

pub use skia_bindings::GrSurfaceOrigin as SurfaceOrigin;
#[test]
fn test_surface_origin_naming() {
    let _ = SurfaceOrigin::TopLeft;
}

// Note: BackendState is in gl/types.rs/

#[allow(dead_code)]
pub struct FlushInfo {
    // TODO: wrap access to the following fields in a safe way:
    num_semaphores: std::os::raw::c_int,
    signal_semaphores: *mut sb::GrBackendSemaphore,
    finished_proc: sb::GrGpuFinishedProc,
    finished_context: sb::GrGpuFinishedContext,
    submitted_proc: sb::GrGpuSubmittedProc,
    submitted_context: sb::GrGpuSubmittedContext,
}

impl Default for FlushInfo {
    fn default() -> Self {
        Self {
            num_semaphores: 0,
            signal_semaphores: ptr::null_mut(),
            finished_proc: None,
            finished_context: ptr::null_mut(),
            submitted_proc: None,
            submitted_context: ptr::null_mut(),
        }
    }
}

impl NativeTransmutable<sb::GrFlushInfo> for FlushInfo {}
#[test]
fn test_flush_info_layout() {
    FlushInfo::test_layout();
}

pub use sb::GrSemaphoresSubmitted as SemaphoresSubmitted;
#[test]
fn test_semaphores_submitted_naming() {
    let _ = SemaphoresSubmitted::Yes;
}

// TODO: wrap GrPrepareForExternalIORequests
