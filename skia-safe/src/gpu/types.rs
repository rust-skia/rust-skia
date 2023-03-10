use skia_bindings as sb;
use std::ptr;

pub use skia_bindings::GrBackendApi as BackendAPI;
variant_name!(BackendAPI::Dawn);

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

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrMipmapped as Mipmapped;

#[deprecated(since = "0.35.0", note = "Use Mipmapped (with a lowercase 'm')")]
pub use skia_bindings::GrMipmapped as MipMapped;
variant_name!(Mipmapped::Yes);

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrRenderable as Renderable;
variant_name!(Renderable::No);

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrProtected as Protected;
variant_name!(Protected::Yes);

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

// TODO: wrap GrPrepareForExternalIORequests
