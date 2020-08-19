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
pub use skia_bindings::GrMipMapped as MipMapped;
#[test]
fn test_mip_mapped_naming() {
    let _ = MipMapped::Yes;
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

// i32 on Windows, u32 on macOS, so we'd prefer to mal it to unsigned type in Rust.
bitflags! {
    pub struct FlushFlags: u32 {
        const NONE = sb::GrFlushFlags_kNone_GrFlushFlags as _;
        const SYNC_CPU = sb::GrFlushFlags_kSyncCpu_GrFlushFlag as _;
    }
}

#[allow(dead_code)]
pub struct FlushInfo {
    pub flags: FlushFlags,
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
            flags: FlushFlags::NONE,
            num_semaphores: 0,
            signal_semaphores: ptr::null_mut(),
            finished_proc: None,
            finished_context: ptr::null_mut(),
            submitted_proc: None,
            submitted_context: ptr::null_mut(),
        }
    }
}

impl From<FlushFlags> for FlushInfo {
    fn from(flags: FlushFlags) -> Self {
        Self {
            flags,
            ..Default::default()
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
