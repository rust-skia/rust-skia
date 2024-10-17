use skia_bindings as sb;
use std::ptr;

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Protected as Protected;
variant_name!(Protected::Yes);

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Renderable as Renderable;
variant_name!(Renderable::No);

pub use skia_bindings::skgpu_BackendApi as BackendApi;
variant_name!(BackendApi::Metal);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Budgeted(bool);

native_transmutable!(sb::skgpu_Budgeted, Budgeted, budgeted_layout);

#[allow(non_upper_case_globals)]
impl Budgeted {
    #[deprecated(since = "0.29.0", note = "use No")]
    pub const NO: Budgeted = Budgeted(false);
    #[deprecated(since = "0.29.0", note = "use Yes")]
    pub const YES: Budgeted = Budgeted(true);

    // we want this look like enum case names.
    pub const No: Budgeted = Budgeted(false);
    pub const Yes: Budgeted = Budgeted(true);
}

// TODO: CallbackResult

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Mipmapped as Mipmapped;

pub use skia_bindings::skgpu_Origin as Origin;
variant_name!(Origin::TopLeft);

pub use skia_bindings::GrBackendApi as BackendAPI;
variant_name!(BackendAPI::OpenGL);

pub use skia_bindings::GrSurfaceOrigin as SurfaceOrigin;
variant_name!(SurfaceOrigin::BottomLeft);

// Note: BackendState is in gl/types.rs/

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct FlushInfo {
    // TODO: wrap access to the following fields in a safe way:
    num_semaphores: usize,
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

native_transmutable!(sb::GrFlushInfo, FlushInfo, flush_info_layout);

pub use sb::GrSemaphoresSubmitted as SemaphoresSubmitted;
variant_name!(SemaphoresSubmitted::Yes);

pub use sb::GrPurgeResourceOptions as PurgeResourceOptions;
variant_name!(PurgeResourceOptions::AllResources);

pub use sb::GrSyncCpu as SyncCpu;
variant_name!(SyncCpu::Yes);

pub use sb::GrMarkFrameBoundary as MarkFrameBoundary;
variant_name!(MarkFrameBoundary::Yes);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub sync: SyncCpu,
    pub mark_boundary: MarkFrameBoundary,
    pub frame_id: u64,
}
native_transmutable!(sb::GrSubmitInfo, SubmitInfo, submit_info_layout);

impl Default for SubmitInfo {
    fn default() -> Self {
        Self {
            sync: SyncCpu::No,
            mark_boundary: MarkFrameBoundary::No,
            frame_id: 0,
        }
    }
}

impl From<SyncCpu> for SubmitInfo {
    fn from(sync: SyncCpu) -> Self {
        Self {
            sync,
            ..Self::default()
        }
    }
}

impl From<Option<SyncCpu>> for SubmitInfo {
    fn from(sync_cpu: Option<SyncCpu>) -> Self {
        match sync_cpu {
            Some(sync_cpu) => sync_cpu.into(),
            None => Self::default(),
        }
    }
}
