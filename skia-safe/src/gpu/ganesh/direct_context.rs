use std::{
    fmt,
    ops::{Deref, DerefMut},
    ptr,
    time::Duration,
};

use crate::{
    Data, Image, Surface, TextureCompressionType,
    gpu::{
        BackendFormat, BackendRenderTarget, BackendTexture, FlushError, FlushInfo, FlushResult,
        GpuStatsFlags, MutableTextureState, PurgeResourceOptions, RecordingContext,
        SemaphoresSubmitted, SubmitInfo, SyncCpu,
    },
    prelude::*,
    surfaces,
};
use skia_bindings::{self as sb, GrDirectContext, GrDirectContext_DirectContextID, SkRefCntBase};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// A unique ID associated with a [`DirectContext`].
pub struct DirectContextId {
    id: u32,
}

native_transmutable!(GrDirectContext_DirectContextID, DirectContextId);

pub type DirectContext = RCHandle<GrDirectContext>;

impl NativeRefCountedBase for GrDirectContext {
    type Base = SkRefCntBase;
}

impl Deref for DirectContext {
    type Target = RecordingContext;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for DirectContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// The maximum number of resources and total number of bytes of video memory that can be held
/// in the GPU resource cache.
pub struct ResourceCacheLimits {
    pub max_resources: usize,
    pub max_resource_bytes: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// The current GPU resource cache usage.
pub struct ResourceCacheUsage {
    pub resource_count: usize,
    pub resource_bytes: usize,
}

impl fmt::Debug for DirectContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirectContext")
            .field("base", self as &RecordingContext)
            .field("resource_cache_limit", &self.resource_cache_limit())
            .field("resource_cache_usage", &self.resource_cache_usage())
            .field(
                "resource_cache_purgeable_bytes",
                &self.resource_cache_purgeable_bytes(),
            )
            .field(
                "supports_distance_field_text",
                &self.supports_distance_field_text(),
            )
            .finish()
    }
}

impl DirectContext {
    /// The context normally assumes that no outsider is setting state
    /// within the underlying 3D API's context/device/whatever. This call informs
    /// the context that the state was modified and it should resend. Shouldn't
    /// be called frequently for good performance.
    /// The flag bits, `backend_state`, is dependent on which backend is used by the
    /// context, either GL or D3D (possible in future).
    ///
    /// - `backend_state` flag bits to reset, or `None` to reset all backend state
    pub fn reset(&mut self, backend_state: Option<u32>) -> &mut Self {
        unsafe {
            self.native_mut()
                .resetContext(backend_state.unwrap_or(sb::kAll_GrBackendState))
        }
        self
    }

    /// If the backend is [`crate::gpu::ganesh::BackendApi::OpenGL`], then all texture
    /// unit/target combinations for
    /// which the context has modified the bound texture will have texture id 0 bound. This does
    /// not flush the context. Calling [`DirectContext::reset()`] does not change the set that
    /// will be bound to texture id 0 on the next call to
    /// [`DirectContext::reset_gl_texture_bindings()`]. After this is called all unit/target
    /// combinations are considered to have unmodified bindings until the context subsequently
    /// modifies them (meaning if this is called twice in a row with no intervening context usage
    /// then the second call is a no-op.)
    pub fn reset_gl_texture_bindings(&mut self) -> &mut Self {
        unsafe { self.native_mut().resetGLTextureBindings() }
        self
    }

    /// Abandons all GPU resources and assumes the underlying backend 3D API context is no
    /// longer usable. Call this if you have lost the associated GPU context, and thus internal
    /// texture, buffer, etc. references/IDs are now invalid. Calling this ensures that the
    /// destructors of the context and any of its created resource objects will not make backend
    /// 3D API calls. Content rendered but not previously flushed may be lost. After this
    /// function is called all subsequent calls on the context will fail or be no-ops.
    ///
    /// The typical use case for this function is that the underlying 3D context was lost and
    /// further API calls may crash.
    ///
    /// This call is not valid to be made inside release procs passed into [`crate::Surface`] or
    /// [`crate::Image`]. The call will simply fail (and assert in debug) if it is called while
    /// inside a release proc.
    ///
    /// For Vulkan, even if the device becomes lost, the VkQueue, VkDevice, or VkInstance used
    /// to create the context must be kept alive even after abandoning the context. Those
    /// objects must live for the lifetime of the context object itself. The reason for this is
    /// so that we can continue to delete any outstanding [`BackendTexture`]s/RenderTargets
    /// which must be cleaned up even in a device lost state.
    pub fn abandon(&mut self) -> &mut Self {
        unsafe {
            // self.native_mut().abandonContext()
            sb::GrDirectContext_abandonContext(self.native_mut() as *mut _ as _)
        }
        self
    }

    /// Returns true if the context was abandoned or if the backend specific context has gotten
    /// into an unrecoverarble, lost state (e.g. in Vulkan backend if we've gotten a
    /// VK_ERROR_DEVICE_LOST). If the backend context is lost, this call will also abandon this
    /// context.
    pub fn is_device_lost(&mut self) -> bool {
        unsafe { self.native_mut().isDeviceLost() }
    }

    // TODO: threadSafeProxy()

    /// Returns true if the backend specific context has gotten into an unrecoverarble, lost
    /// state (e.g. in Vulkan backend if we've gotten a VK_ERROR_DEVICE_LOST). If the backend
    /// context is lost, this call will also abandon this context.
    pub fn oomed(&mut self) -> bool {
        unsafe { self.native_mut().oomed() }
    }

    /// This is similar to [`DirectContext::abandon()`] however the underlying 3D context is
    /// not yet lost and the context will cleanup all allocated resources before returning.
    /// After returning it will assume that the underlying context may no longer be valid.
    ///
    /// The typical use case for this function is that the client is going to destroy the 3D
    /// context but can't guarantee that context will be destroyed first (perhaps because it may
    /// be ref'ed elsewhere by either the client or Skia objects).
    ///
    /// For Vulkan, even if the device becomes lost, the VkQueue, VkDevice, or VkInstance used
    /// to create the context must be alive before calling
    /// [`DirectContext::release_resources_and_abandon()`].
    pub fn release_resources_and_abandon(&mut self) -> &mut Self {
        unsafe {
            sb::GrDirectContext_releaseResourcesAndAbandonContext(self.native_mut() as *mut _ as _)
        }
        self
    }

    /// Return the current GPU resource cache limit in bytes.
    pub fn resource_cache_limit(&self) -> usize {
        unsafe { self.native().getResourceCacheLimit() }
    }

    /// Gets the current GPU resource cache usage.
    pub fn resource_cache_usage(&self) -> ResourceCacheUsage {
        let mut resource_count = 0;
        let mut resource_bytes = 0;
        unsafe {
            self.native()
                .getResourceCacheUsage(&mut resource_count, &mut resource_bytes)
        }
        ResourceCacheUsage {
            resource_count: resource_count.try_into().unwrap(),
            resource_bytes,
        }
    }

    /// Gets the number of bytes in the cache consumed by purgeable (e.g. unlocked) resources.
    pub fn resource_cache_purgeable_bytes(&self) -> usize {
        unsafe { self.native().getResourceCachePurgeableBytes() }
    }

    /// Specify the GPU resource cache limits. If the current cache exceeds the
    /// `max_resource_bytes` limit, it will be purged (LRU) to keep the cache within the limit.
    pub fn set_resource_cache_limits(&mut self, limits: ResourceCacheLimits) {
        unsafe {
            self.native_mut().setResourceCacheLimits(
                limits.max_resources.try_into().unwrap(),
                limits.max_resource_bytes,
            )
        }
    }

    /// Specify the GPU resource cache limit. If the cache currently exceeds this limit,
    /// it will be purged (LRU) to keep the cache within the limit.
    pub fn set_resource_cache_limit(&mut self, max_resource_bytes: usize) {
        unsafe { self.native_mut().setResourceCacheLimit(max_resource_bytes) }
    }

    /// Frees GPU created by the context. Can be called to reduce GPU memory
    /// pressure.
    pub fn free_gpu_resources(&mut self) -> &mut Self {
        unsafe { sb::GrDirectContext_freeGpuResources(self.native_mut() as *mut _ as _) }
        self
    }

    /// Purge GPU resources that haven't been used in the past `not_used` milliseconds or are
    /// otherwise marked for deletion, regardless of whether the context is under budget.
    ///
    /// - `not_used` only unlocked resources not used in these last milliseconds will be cleaned
    ///   up
    /// - `opts` specify which resources should be cleaned up. If [`PurgeResourceOptions::
    ///   ScratchResourcesOnly`] then, all unlocked scratch resources older than `not_used` will
    ///   be purged but the unlocked resources with persistent data will remain.
    pub fn perform_deferred_cleanup(
        &mut self,
        not_used: Duration,
        opts: impl Into<Option<PurgeResourceOptions>>,
    ) -> &mut Self {
        unsafe {
            sb::C_GrDirectContext_performDeferredCleanup(
                self.native_mut(),
                not_used.as_millis().try_into().unwrap(),
                opts.into().unwrap_or(PurgeResourceOptions::AllResources),
            )
        }
        self
    }

    /// Purge unlocked resources from the cache until the provided byte count has been reached
    /// or we have purged all unlocked resources. The default policy is to purge in LRU order,
    /// but can be overridden to prefer purging scratch resources (in LRU order) prior to
    /// purging other resource types.
    ///
    /// - `bytes_to_purge` the desired number of bytes to be purged
    /// - `prefer_scratch_resources` if true scratch resources will be purged prior to other
    ///   resource types
    pub fn purge_unlocked_resource_bytes(
        &mut self,
        bytes_to_purge: usize,
        prefer_scratch_resources: bool,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .purgeUnlockedResources(bytes_to_purge, prefer_scratch_resources)
        }
        self
    }

    /// This entry point is intended for instances where an app has been backgrounded or
    /// suspended.
    /// If `opts` is [`PurgeResourceOptions::ScratchResourcesOnly`] all unlocked scratch
    /// resources will be purged but the unlocked resources with persistent data will remain.
    /// If it is [`PurgeResourceOptions::AllResources`] then all unlocked resources will be
    /// purged.
    /// In either case, after the unlocked resources are purged a separate pass will be made to
    /// ensure that resource usage is under budget (i.e., even if scratch resources only are
    /// purged some resources with persistent data may be purged to be under budget).
    pub fn purge_unlocked_resources(&mut self, opts: PurgeResourceOptions) -> &mut Self {
        unsafe { self.native_mut().purgeUnlockedResources1(opts) }
        self
    }

    /// Gets the types of GPU stats supported by this context.
    pub fn supported_gpu_stats(&self) -> GpuStatsFlags {
        GpuStatsFlags::from_bits_truncate(unsafe { self.native().supportedGpuStats() })
    }

    // TODO: wait()

    /// Call to ensure all drawing to the context has been flushed and submitted to the
    /// underlying 3D API. This is equivalent to calling [`DirectContext::flush()`] with a
    /// default [`FlushInfo`] followed by [`DirectContext::submit()`].
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush or the submit failed.
    pub fn flush_and_submit(&mut self) -> Result<SemaphoresSubmitted, FlushError> {
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flushAndSubmit(self.native_mut(), SyncCpu::No, result)
        }))
    }

    /// Version of [`DirectContext::flush()`] that uses a default [`FlushInfo`] and submits with
    /// [`SyncCpu::Yes`], so it returns once the gpu has finished with all submitted work.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush or the submit failed.
    pub fn flush_submit_and_sync_cpu(&mut self) -> Result<SemaphoresSubmitted, FlushError> {
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flushAndSubmit(self.native_mut(), SyncCpu::Yes, result)
        }))
    }

    /// Call to ensure all drawing to the context has been flushed to underlying 3D API
    /// specific objects. A call to [`DirectContext::submit()`] is always required to ensure
    /// work is actually sent to the gpu. Some specific API details:
    ///
    /// GL: Commands are actually sent to the driver, but glFlush is never called. Thus some
    /// sync objects from the flush will not be valid until a submission occurs.
    ///
    /// Vulkan/Metal/D3D/Dawn: Commands are recorded to the backend APIs corresponding command
    /// buffer or encoder objects. However, these objects are not sent to the gpu until a
    /// submission occurs.
    ///
    /// If the returned [`SemaphoresSubmitted`] is [`crate::gpu::SemaphoresSubmitted::Yes`], only
    /// initialized [`crate::gpu::BackendSemaphore`]s will be submitted to the gpu during the next
    /// submit call (it is possible Skia failed to create a subset of the semaphores). The client
    /// should not wait on these semaphores until after submit has been called, and must keep them
    /// alive until then. If it is [`crate::gpu::SemaphoresSubmitted::No`], the GPU backend will
    /// not submit any semaphores to be signaled on the GPU. Thus the client should not have the
    /// GPU wait on any of the semaphores passed in with the [`FlushInfo`]. Regardless of whether
    /// semaphores were submitted to the GPU or not, the client is still responsible for
    /// deleting any initialized semaphores.
    /// Regardless of semaphore submission the context will still be flushed. It should be
    /// emphasized that a [`crate::gpu::SemaphoresSubmitted::No`] value does not mean the flush did
    /// not happen. It simply means there were no semaphores submitted to the GPU. A caller should
    /// only take this as a failure if they passed in semaphores to be submitted.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush failed.
    ///
    /// - `info` flush options, or `None` for default flush options
    pub fn flush<'a>(
        &mut self,
        info: impl Into<Option<&'a FlushInfo>>,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        let default_info = FlushInfo::default();
        let info = info.into().unwrap_or(&default_info).native();
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flush(self.native_mut(), info, result)
        }))
    }

    /// Flushes any pending uses of texture-backed images in the GPU backend. If the image is
    /// not texture-backed (including promise texture images) or if the [`DirectContext`] does
    /// not have the same context ID as the context backing the image then this is a no-op.
    /// If the image was not used in any non-culled draws in the current queue of work for the
    /// passed [`DirectContext`] then this is a no-op unless the [`FlushInfo`] contains
    /// semaphores or a finish proc. Those are respected even when the image has not been used.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush failed.
    ///
    /// - `image` the image to flush
    /// - `info` flush options
    pub fn flush_image_with_info(
        &mut self,
        image: &Image,
        info: &FlushInfo,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flushImageWithInfo(
                self.native_mut(),
                image.clone().into_ptr(),
                info.native(),
                result,
            )
        }))
    }

    /// Version of [`DirectContext::flush_image_with_info()`] that uses a default
    /// [`FlushInfo`].
    pub fn flush_image(&mut self, image: &Image) -> Result<SemaphoresSubmitted, FlushError> {
        self.flush_image_with_info(image, &FlushInfo::default())
    }

    /// Version of [`DirectContext::flush()`] that uses a default [`FlushInfo`]. Also submits
    /// the flushed image work to the GPU.
    ///
    /// The submit is issued even when the flush failed, so the semaphores reported by
    /// [`FlushError::submitted`] may still be signaled by it.
    pub fn flush_and_submit_image(
        &mut self,
        image: &Image,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        let result = self.flush_image_with_info(image, &FlushInfo::default());
        let submitted = self.submit(SubmitInfo::default());
        match result {
            Err(err) => Err(err),
            Ok(semaphores) if submitted => Ok(semaphores),
            Ok(semaphores) => Err(FlushError {
                submitted: semaphores,
            }),
        }
    }

    /// Issues pending [`Surface`] commands to the GPU-backed API objects and resolves any
    /// [`Surface`] MSAA. A call to [`DirectContext::submit()`] is always required to ensure
    /// work is actually sent to the gpu. Some specific API details:
    ///
    /// GL: Commands are actually sent to the driver, but glFlush is never called. Thus some
    /// sync objects from the flush will not be valid until a submission occurs.
    ///
    /// Vulkan/Metal/D3D/Dawn: Commands are recorded to the backend APIs corresponding command
    /// buffer or encoder objects. However, these objects are not sent to the gpu until a
    /// submission occurs.
    ///
    /// The work that is submitted to the GPU will be dependent on the `access` that is passed
    /// in.
    ///
    /// If [`surfaces::BackendSurfaceAccess::NoAccess`] is passed in all commands will be issued
    /// to the GPU.
    ///
    /// If [`surfaces::BackendSurfaceAccess::Present`] is passed in and the backend API is not
    /// Vulkan, it is treated the same as `NoAccess`. If the backend API is Vulkan, the VkImage
    /// that backs the [`Surface`] will be transferred back to its original queue. If the
    /// [`Surface`] was created by wrapping a VkImage, the queue will be set to the queue which
    /// was originally passed in on the GrVkImageInfo. Additionally, if the original queue was
    /// not external or foreign the layout of the VkImage will be set to
    /// VK_IMAGE_LAYOUT_PRESENT_SRC_KHR.
    ///
    /// The [`FlushInfo`] describes additional options to flush. Please see documentation at
    /// [`FlushInfo`] for more info.
    ///
    /// If the returned [`SemaphoresSubmitted`] is [`crate::gpu::SemaphoresSubmitted::Yes`], only
    /// initialized [`crate::gpu::BackendSemaphore`]s will be submitted to the gpu during the next
    /// submit call (it is possible Skia failed to create a subset of the semaphores). The client
    /// should not wait on these semaphores until after submit has been called, but must keep them
    /// alive until then. If a submit flag was passed in with the flush these valid semaphores can
    /// be waited on immediately. If it is [`crate::gpu::SemaphoresSubmitted::No`], the GPU backend
    /// will not submit any semaphores to be signaled on the GPU. Thus the client should not have
    /// the GPU wait on any of the semaphores passed in with the [`FlushInfo`]. Regardless of
    /// whether semaphores were submitted to the GPU or not, the client is still responsible for
    /// deleting any initialized semaphores.
    /// Regardless of semaphore submission the context will still be flushed. It should be
    /// emphasized that a [`crate::gpu::SemaphoresSubmitted::No`] value does not mean the flush did
    /// not happen. It simply means there were no semaphores submitted to the GPU. A caller should
    /// only take this as a failure if they passed in semaphores to be submitted.
    ///
    /// Pending surface commands are flushed regardless of the return result.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush failed.
    ///
    /// - `surface` the GPU backed surface to be flushed. Has no effect on a CPU-backed surface
    /// - `access` type of access the call will do on the backend object after flush
    /// - `info` flush options
    pub fn flush_surface_with_access(
        &mut self,
        surface: &mut Surface,
        access: surfaces::BackendSurfaceAccess,
        info: &FlushInfo,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flushSurfaceWithAccess(
                self.native_mut(),
                surface.native_mut(),
                access,
                info.native(),
                result,
            )
        }))
    }

    /// Same as [`DirectContext::flush_surface_with_access()`] except:
    ///
    /// If a [`MutableTextureState`] is passed in, at the end of the flush we will transition
    /// the surface to be in the state requested by the [`MutableTextureState`]. If the surface
    /// (or [`crate::Image`] or backend object wrapping the same backend object) is used again
    /// after this flush the state may be changed and no longer match what is requested here.
    /// This is often used if the surface will be used for presenting or external use and the
    /// client wants the backend object to be prepped for that use. A finished proc or semaphore
    /// on the [`FlushInfo`] will also include the work for any requested state change.
    ///
    /// If the backend API is Vulkan, the caller can set the [`MutableTextureState`]'s
    /// VkImageLayout to VK_IMAGE_LAYOUT_UNDEFINED or queueFamilyIndex to VK_QUEUE_FAMILY_IGNORED
    /// to tell Skia to not change those respective states.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush failed.
    ///
    /// - `surface` the GPU backed surface to be flushed. Has no effect on a CPU-backed surface
    /// - `info` flush options
    /// - `new_state` optional state change request after flush
    pub fn flush_surface_with_texture_state(
        &mut self,
        surface: &mut Surface,
        info: &FlushInfo,
        new_state: Option<&MutableTextureState>,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        flush_result(FlushResult::construct(|result| unsafe {
            sb::C_GrDirectContext_flushSurfaceWithTextureState(
                self.native_mut(),
                surface.native_mut(),
                info.native(),
                new_state.native_ptr_or_null(),
                result,
            )
        }))
    }

    /// Call to ensure all reads/writes of the surface have been issued to the underlying 3D
    /// API. Skia will correctly order its own draws and pixel operations. This must be used to
    /// ensure correct ordering when the surface backing store is accessed outside Skia (e.g.
    /// direct use of the 3D API or a windowing system). This is equivalent to calling
    /// [`DirectContext::flush_surface()`] with a default [`FlushInfo`] followed by
    /// [`DirectContext::submit()`].
    ///
    /// Has no effect on a CPU-backed surface.
    ///
    /// The submit is issued even when the flush failed, so the semaphores reported by
    /// [`FlushError::submitted`] may still be signaled by it.
    ///
    /// Returns [`Ok`] with the [`SemaphoresSubmitted`] value of the flush, or [`Err`] with a
    /// [`FlushError`] if the flush or the submit failed.
    ///
    /// - `sync_cpu` whether submit should return once the GPU has finished with all submitted
    ///   work, defaults to [`SyncCpu::No`]
    pub fn flush_and_submit_surface(
        &mut self,
        surface: &mut Surface,
        sync_cpu: impl Into<Option<SyncCpu>>,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        let result = self.flush_surface_with_access(
            surface,
            surfaces::BackendSurfaceAccess::NoAccess,
            &FlushInfo::default(),
        );
        let submitted = self.submit(sync_cpu.into().unwrap_or(SyncCpu::No));
        match result {
            Err(err) => Err(err),
            Ok(semaphores) if submitted => Ok(semaphores),
            Ok(semaphores) => Err(FlushError {
                submitted: semaphores,
            }),
        }
    }

    /// Flushes the given surface with the default [`FlushInfo`].
    ///
    /// Has no effect on a CPU-backed surface.
    pub fn flush_surface(
        &mut self,
        surface: &mut Surface,
    ) -> Result<SemaphoresSubmitted, FlushError> {
        self.flush_surface_with_access(
            surface,
            surfaces::BackendSurfaceAccess::NoAccess,
            &FlushInfo::default(),
        )
    }

    /// Submit outstanding work to the gpu from all previously un-submitted flushes. The return
    /// value of the submit will indicate whether or not the submission to the GPU was
    /// successful.
    ///
    /// If the call returns true, all previously passed in semaphores in flush calls will have
    /// been submitted to the GPU and they can safely be waited on. The caller should wait on
    /// those semaphores or perform some other global synchronization before deleting the
    /// semaphores.
    ///
    /// If it returns false, then those same semaphores will not have been submitted and we will
    /// not try to submit them again. The caller is free to delete the semaphores at any time.
    ///
    /// If [`SubmitInfo::sync`] is [`SyncCpu::Yes`], this function will return once the gpu has
    /// finished with all submitted work.
    ///
    /// If [`SubmitInfo::mark_boundary`] is
    /// [`crate::gpu::ganesh::MarkFrameBoundary::Yes`] and the GPU supports a
    /// way to be notified about frame boundaries, then we will notify the GPU during/after the
    /// submission of work to the GPU. [`SubmitInfo::frame_id`] is a frame ID that is passed to
    /// the GPU when marking a boundary. Ideally this value should be unique for each frame.
    /// Currently marking frame boundaries is only supported with the Vulkan backend and only if
    /// the VK_EXT_frame_boundary extension is available.
    pub fn submit(&mut self, submit_info: impl Into<SubmitInfo>) -> bool {
        unsafe { self.native_mut().submit(&submit_info.into().into_native()) }
    }

    /// Checks whether any asynchronous work is complete and if so calls related callbacks.
    pub fn check_async_work_completion(&mut self) {
        unsafe { self.native_mut().checkAsyncWorkCompletion() }
    }

    // TODO: dumpMemoryStatistics()

    /// Returns true if this context supports rendering glyphs as distance fields.
    pub fn supports_distance_field_text(&self) -> bool {
        unsafe { self.native().supportsDistanceFieldText() }
    }
}

#[cfg(feature = "vulkan")]
impl DirectContext {
    /// Returns true if the underlying Vulkan implementation can accurately detect when the data
    /// in the pipeline cache changes. Returns false on non-Vulkan implementations.
    ///
    /// When this is false, the return value of [`DirectContext::has_new_vk_pipeline_cache_data()`]
    /// will occasionally issue a false positive.
    pub fn can_detect_new_vk_pipeline_cache_data(&self) -> bool {
        unsafe { self.native().canDetectNewVkPipelineCacheData() }
    }

    /// For Vulkan implementations, returns true if the data in the pipeline cache could have
    /// changed since the last call to [`DirectContext::store_vk_pipeline_cache_data()`]. Always
    /// returns true on non-Vulkan implementations.
    ///
    /// Pipeline cache changes are detected when creating new pipelines, however this will
    /// occasionally result in a false positive. When VK_EXT_pipeline_creation_cache_control is
    /// enabled, we additionally know when a pipeline creation does not change the cache, thus
    /// eliminating false-positives.
    ///
    /// Check [`DirectContext::can_detect_new_vk_pipeline_cache_data()`] to see whether
    /// VK_EXT_pipeline_creation_cache_control is available and enabled.
    pub fn has_new_vk_pipeline_cache_data(&self) -> bool {
        unsafe { self.native().hasNewVkPipelineCacheData() }
    }

    /// Stores the Vulkan pipeline cache data so that it can be retrieved via
    /// [`DirectContext::has_new_vk_pipeline_cache_data()`] and reused in a later session.
    pub fn store_vk_pipeline_cache_data(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().storeVkPipelineCacheData();
        }
        self
    }

    /// Same as [`DirectContext::store_vk_pipeline_cache_data()`], but with a maximum size in
    /// bytes for the stored pipeline cache data.
    pub fn store_vk_pipeline_cache_data_with_max_size(&mut self, max_size: usize) -> &mut Self {
        unsafe {
            self.native_mut().storeVkPipelineCacheData1(max_size);
        }
        self
    }
}

impl DirectContext {
    // TODO: wrap createBackendTexture (several variants)
    //       introduced in m76, m77, and m79
    //       extended in m84 with finishedProc and finishedContext
    //       extended in m107 with label

    // TODO: wrap updateBackendTexture (several variants)
    //       introduced in m84

    /// Retrieve the [`BackendFormat`] for a given [`TextureCompressionType`]. This is
    /// guaranteed to match the backend format used by the compressed backend texture creation
    /// methods that take a [`TextureCompressionType`].
    ///
    /// The caller should check that the returned format is valid.
    pub fn compressed_backend_format(&self, compression: TextureCompressionType) -> BackendFormat {
        let mut backend_format = BackendFormat::new_invalid();
        unsafe {
            sb::C_GrDirectContext_compressedBackendFormat(
                self.native(),
                compression,
                backend_format.native_mut(),
            )
        };
        backend_format
    }

    // TODO: wrap createCompressedBackendTexture (several variants)
    //       introduced in m81
    //       extended in m84 with finishedProc and finishedContext

    // TODO: wrap updateCompressedBackendTexture (two variants)
    //       introduced in m86

    // TODO: add variant with GpuFinishedProc / GpuFinishedContext
    /// Updates the state of the [`BackendTexture`] to have the passed in
    /// [`MutableTextureState`]. All objects that wrap the backend surface (i.e. [`Surface`]s
    /// and [`crate::Image`]s) will also be aware of this state change. This call does not
    /// submit the state change to the gpu, but requires the client to call
    /// [`DirectContext::submit()`] to send it to the GPU. The work for this call is ordered
    /// linearly with all other calls that require submit to be called (e.g. flush).
    ///
    /// See [`MutableTextureState`] to see what state can be set via this call.
    ///
    /// If the backend API is Vulkan, the caller can set the [`MutableTextureState`]'s
    /// VkImageLayout to VK_IMAGE_LAYOUT_UNDEFINED or queueFamilyIndex to VK_QUEUE_FAMILY_IGNORED
    /// to tell Skia to not change those respective states.
    ///
    /// Returns whether the state was successfully updated.
    pub fn set_backend_texture_state(
        &mut self,
        backend_texture: &BackendTexture,
        state: &MutableTextureState,
    ) -> bool {
        self.set_backend_texture_state_and_return_previous(backend_texture, state)
            .is_some()
    }

    /// Same as [`DirectContext::set_backend_texture_state()`], but also returns the previous
    /// state of the [`BackendTexture`] if the state was successfully updated, or `None`
    /// otherwise.
    pub fn set_backend_texture_state_and_return_previous(
        &mut self,
        backend_texture: &BackendTexture,
        state: &MutableTextureState,
    ) -> Option<MutableTextureState> {
        let mut previous = MutableTextureState::default();
        unsafe {
            self.native_mut().setBackendTextureState(
                backend_texture.native(),
                state.native(),
                previous.native_mut(),
                None,
                ptr::null_mut(),
            )
        }
        .then_some(previous)
    }

    // TODO: add variant with GpuFinishedProc / GpuFinishedContext
    /// Updates the state of the [`BackendRenderTarget`] to have the passed in
    /// [`MutableTextureState`], see [`DirectContext::set_backend_texture_state()`] for details.
    ///
    /// Returns whether the state was successfully updated.
    pub fn set_backend_render_target_state(
        &mut self,
        target: &BackendRenderTarget,
        state: &MutableTextureState,
    ) -> bool {
        self.set_backend_render_target_state_and_return_previous(target, state)
            .is_some()
    }

    /// Same as [`DirectContext::set_backend_render_target_state()`], but also returns the
    /// previous state of the [`BackendRenderTarget`] if the state was successfully updated, or
    /// `None` otherwise.
    pub fn set_backend_render_target_state_and_return_previous(
        &mut self,
        target: &BackendRenderTarget,
        state: &MutableTextureState,
    ) -> Option<MutableTextureState> {
        let mut previous = MutableTextureState::default();
        unsafe {
            self.native_mut().setBackendRenderTargetState(
                target.native(),
                state.native(),
                previous.native_mut(),
                None,
                ptr::null_mut(),
            )
        }
        .then_some(previous)
    }

    /// Deletes a [`BackendTexture`] that was created by Skia via the explicitly allocated
    /// backend texture API. It is the client's responsibility to delete all these objects
    /// before deleting the context used to create them. If the backend is Vulkan, the textures
    /// must be deleted before abandoning the context as well. Additionally, clients should only
    /// delete these objects on the thread for which that context is active.
    pub fn delete_backend_texture(&mut self, texture: &BackendTexture) {
        unsafe { self.native_mut().deleteBackendTexture(texture.native()) }
    }

    /// Pre-compiles a shader and populates the runtime program cache. The `key` and `data`
    /// blobs should be the ones passed to the persistent cache, in SkSL format.
    ///
    /// To use this API, create a [`DirectContext`] as normal, but set the persistent cache on
    /// [`crate::gpu::ContextOptions`] to something that will save the cached shader blobs, with
    /// a shader
    /// cache strategy of [`crate::gpu::ganesh::context_options::ShaderCacheStrategy::SkSL`]
    /// to ensure
    /// the blobs are SkSL and are
    /// suitable for pre-compilation. Then run the application and save all of the key/data
    /// pairs that are fed to the cache. At startup (or any convenient time), call this function
    /// for each key/data pair to compile the SkSL and populate the runtime cache.
    ///
    /// This is only guaranteed to work if the context/device used when the shader blobs were
    /// created is created in the same way as the one used here, and the same
    /// [`crate::gpu::ContextOptions`]
    /// are specified. Using cached shader blobs on a different device or driver is undefined.
    pub fn precompile_shader(&mut self, key: &Data, data: &Data) -> bool {
        unsafe {
            self.native_mut()
                .precompileShader(key.native(), data.native())
        }
    }

    /// Returns the unique [`DirectContextId`] of this context.
    pub fn id(&self) -> DirectContextId {
        let mut id = DirectContextId { id: 0 };
        unsafe { sb::C_GrDirectContext_directContextId(self.native(), id.native_mut()) }
        id
    }
}

/// Maps a native [`FlushResult`] to the result returned by the [`DirectContext`] flush methods.
fn flush_result(result: FlushResult) -> Result<SemaphoresSubmitted, FlushError> {
    if result.success {
        Ok(result.submitted)
    } else {
        Err(FlushError {
            submitted: result.submitted,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DirectContext;
    use crate::gpu::{SubmitInfo, SyncCpu};

    #[allow(unused)]
    fn submit_invocation(direct_context: &mut DirectContext) {
        direct_context.submit(SyncCpu::Yes);
        direct_context.submit(None);
        direct_context.submit(Some(SyncCpu::Yes));
        direct_context.submit(SubmitInfo::default());
    }
}
