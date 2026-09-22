use std::error::Error;
use std::fmt;
use std::ptr;

use crate::gpu;
use crate::gpu::GpuStatsFlags;
use crate::prelude::NativeSliceAccess;
use skia_bindings as sb;

/// Possible 3D APIs that may be used by Ganesh.
pub use skia_bindings::GrBackendApi as BackendApi;
variant_name!(BackendApi::OpenGL);

#[deprecated(since = "0.80.0", note = "use BackendApi")]
pub use BackendApi as BackendAPI;

/// Legacy constant for [`BackendApi::Metal`].
pub const METAL_BACKEND: BackendApi = BackendApi::Metal;
/// Legacy constant for [`BackendApi::Vulkan`].
pub const VULKAN_BACKEND: BackendApi = BackendApi::Vulkan;
/// Legacy constant for [`BackendApi::Mock`].
pub const MOCK_BACKEND: BackendApi = BackendApi::Mock;

/// Is a texture renderable or not
pub use gpu::Renderable;

/// Is the data protected on the GPU or not.
pub use gpu::Protected;

/// GPU [`crate::Image`] and [`crate::Surface`]s can be stored such that (0, 0) in texture space
/// may correspond to either the top-left or bottom-left content pixel.
pub use skia_bindings::GrSurfaceOrigin as SurfaceOrigin;
variant_name!(SurfaceOrigin::BottomLeft);

// Note: BackendState is in gl/types.rs/

/// Struct to supply options to flush calls.
///
/// After issuing all commands, the semaphores set via [`FlushInfo::set_signal_semaphores()`]
/// will be signaled by the gpu. The client passes in an array of
/// [`crate::gpu::BackendSemaphore`]s. In general these can be either initialized or not. If they
/// are initialized, the backend uses the passed in semaphore. If it is not initialized, a new
/// semaphore is created and the [`crate::gpu::BackendSemaphore`] object is initialized with
/// that semaphore. The semaphores are not sent to the GPU until the next submit call is made.
/// See [`crate::gpu::DirectContext::submit()`] for more information.
///
/// The client will own and be responsible for deleting the underlying semaphores that are
/// stored and returned in initialized [`crate::gpu::BackendSemaphore`] objects. The
/// [`crate::gpu::BackendSemaphore`] objects themselves can be deleted as soon as the flush
/// call returns.
#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct FlushInfo {
    // TODO: wrap access to the following fields in a safe way:
    num_semaphores: usize,
    gpu_stats_flags: GpuStatsFlags,
    signal_semaphores: *mut sb::GrBackendSemaphore,
    finished_proc: sb::GrGpuFinishedProc,
    finished_with_stats_proc: sb::GrGpuFinishedWithStatsProc,
    finished_context: sb::GrGpuFinishedContext,
    submitted_proc: sb::GrGpuSubmittedProc,
    submitted_context: sb::GrGpuSubmittedContext,
}

impl Default for FlushInfo {
    fn default() -> Self {
        Self {
            num_semaphores: 0,
            gpu_stats_flags: GpuStatsFlags::NONE,
            signal_semaphores: ptr::null_mut(),
            finished_proc: None,
            finished_with_stats_proc: None,
            finished_context: ptr::null_mut(),
            submitted_proc: None,
            submitted_context: ptr::null_mut(),
        }
    }
}

native_transmutable!(sb::GrFlushInfo, FlushInfo);

impl FlushInfo {
    /// Sets the signal-semaphore array Skia will signal when work
    /// submitted by the next flush call has executed on the GPU. Each
    /// entry is treated as in/out by Skia: initialized semaphores are
    /// signaled directly, uninitialized entries are filled with a
    /// freshly-created semaphore.
    ///
    /// # Safety
    /// `semaphores` must outlive any flush call that consumes this
    /// [`FlushInfo`]. The flush operation may write back into the slice,
    /// so the caller must keep it borrowed mutably until the flush
    /// returns.
    pub unsafe fn set_signal_semaphores(
        &mut self,
        semaphores: &mut [gpu::BackendSemaphore],
    ) -> &mut Self {
        self.num_semaphores = semaphores.len();
        self.signal_semaphores = semaphores.native_mut().as_mut_ptr();
        self
    }
}

/// Enum used as return value when flush is called with semaphores so the client knows whether
/// the valid semaphores will be submitted on the next [`crate::gpu::DirectContext::submit()`]
/// call.
pub use sb::GrSemaphoresSubmitted as SemaphoresSubmitted;
variant_name!(SemaphoresSubmitted::Yes);

/// Result of a Ganesh flush call. A flush can be successful with or without
/// any semaphores being flushed. In some circumstances an unsuccessful flush
/// can still have flushed the semaphores, but the rendering results should be
/// discarded.
#[repr(C)]
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct FlushResult {
    /// Did the flush succeed.
    pub success: bool,
    /// Whether any semaphores were submitted during the flush process.
    /// Will be [`SemaphoresSubmitted::No`] if no semaphores were specified.
    pub submitted: SemaphoresSubmitted,
}

native_transmutable!(sb::GrDirectContext_FlushResult, FlushResult);

/// A Ganesh flush failed.
///
/// This type is not part of Skia's C++ API. The native `GrDirectContext::FlushResult` reports
/// whether the flush succeeded and whether semaphores were submitted as two independent fields,
/// which makes a failure easy to overlook: inspecting it reads like an ordinary status check, and
/// nothing draws attention to the flush having failed. The Rust flush methods therefore map a
/// failed flush to [`Err`] so that the failure has to be handled explicitly, and this type carries
/// the one piece of information that still has to be acted on.
///
/// A flush fails in the following situations:
///
/// - The context was abandoned, for example after a device loss or a call to
///   [`crate::gpu::DirectContext::abandon()`].
/// - The flush is reentrant: it was issued from an on-flush callback or a finished proc while
///   another flush was still in progress.
/// - The flushed [`crate::Image`] or [`crate::Surface`] is not backed by this context, is not
///   GPU-backed (a CPU-backed surface has no effect), or is null.
/// - An on-flush callback's `preFlush` failed, in which case the render tasks were never executed.
/// - Uploading pending data to the GPU failed, for example because a vertex, index or draw-indirect
///   buffer could not be unmapped.
/// - A GPU resource could not be allocated or instantiated, for example when the device ran out of
///   memory.
/// - A render task failed, for example because a stencil buffer could not be attached, a render
///   pass could not be created, or a pixel transfer failed.
/// - An intermediate submit issued while flushing failed.
/// - For the `flush_and_submit*` methods, the submit that follows the flush failed.
///
/// Handling a failure is not optional:
///
/// - The rendering results of a failed flush are undefined and must be discarded. Presenting them
///   or reading them back yields undefined output.
/// - The semaphores reported by [`Self::submitted`] may still have been submitted to the GPU, so
///   the client must still keep them alive and wait on them (or perform an equivalent global
///   synchronization) before deleting them. Ignoring them leads to a use-after-free or a hang.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FlushError {
    /// Whether any semaphores were submitted during the flush process.
    ///
    /// Will be [`SemaphoresSubmitted::No`] if no semaphores were specified. If it is
    /// [`SemaphoresSubmitted::Yes`], the semaphores were submitted even though the flush failed,
    /// so they must be waited on before they are deleted.
    pub submitted: SemaphoresSubmitted,
}

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.submitted {
            SemaphoresSubmitted::Yes => f.write_str(
                "flush failed; the rendering results must be discarded, but the semaphores were \
                 submitted and must be waited on before deletion",
            ),
            SemaphoresSubmitted::No => {
                f.write_str("flush failed; the rendering results must be discarded")
            }
        }
    }
}

impl Error for FlushError {}

pub use sb::GrPurgeResourceOptions as PurgeResourceOptions;
variant_name!(PurgeResourceOptions::AllResources);

pub use sb::GrSyncCpu as SyncCpu;
variant_name!(SyncCpu::Yes);

pub use sb::GrMarkFrameBoundary as MarkFrameBoundary;
variant_name!(MarkFrameBoundary::Yes);

/// Options passed to [`crate::gpu::DirectContext::submit()`].
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubmitInfo {
    /// If [`SyncCpu::Yes`], submit will return once the gpu has finished with all submitted
    /// work.
    pub sync: SyncCpu,
    /// If [`MarkFrameBoundary::Yes`] and the GPU supports a way to be notified about frame
    /// boundaries, the GPU will be notified about the frame boundary during/after the
    /// submission of work.
    pub mark_boundary: MarkFrameBoundary,
    /// A frame ID that is passed to the GPU when marking a boundary. Ideally this value should
    /// be unique for each frame.
    pub frame_id: u64,
}
native_transmutable!(sb::GrSubmitInfo, SubmitInfo);

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
