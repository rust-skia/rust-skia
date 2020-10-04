#[cfg(feature = "d3d")]
use super::d3d;
#[cfg(feature = "gl")]
use super::gl;
#[cfg(feature = "vulkan")]
use super::vk;
use crate::gpu::{
    BackendFormat, BackendRenderTarget, BackendSurfaceMutableState, BackendTexture, DirectContext,
    FlushInfo, Mipmapped, SemaphoresSubmitted,
};
use crate::prelude::*;
use crate::{image, Data, Image};
use skia_bindings as sb;
use skia_bindings::{GrContext, GrDirectContext, GrRecordingContext, SkRefCntBase};
use std::{
    ops::{Deref, DerefMut},
    ptr,
    time::Duration,
};

pub type Context = RCHandle<GrContext>;

impl NativeRefCountedBase for GrContext {
    type Base = SkRefCntBase;
}

impl From<RCHandle<GrDirectContext>> for RCHandle<GrContext> {
    fn from(direct_context: RCHandle<GrDirectContext>) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}

impl Deref for RCHandle<GrContext> {
    type Target = RCHandle<GrRecordingContext>;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for RCHandle<GrContext> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ResourceCacheLimits {
    pub max_resources: usize,
    pub max_resource_bytes: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ResourceCacheUsage {
    pub resource_count: usize,
    pub resource_bytes: usize,
}

impl RCHandle<GrContext> {
    #[cfg(feature = "gl")]
    pub fn new_gl(interface: impl Into<Option<gl::Interface>>) -> Option<Context> {
        DirectContext::new_gl(interface, None).map(|c| c.into())
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(backend_context: &vk::BackendContext) -> Option<Context> {
        DirectContext::new_vulkan(backend_context, None).map(|c| c.into())
    }

    /// # Safety
    /// This function is unsafe because `device` and `queue` are untyped handles which need to exceed the
    /// lifetime of the context returned.
    #[cfg(feature = "metal")]
    pub unsafe fn new_metal(
        device: *mut std::ffi::c_void,
        queue: *mut std::ffi::c_void,
    ) -> Option<Context> {
        DirectContext::new_metal(device, queue, None).map(|c| c.into())
    }

    // TODO: support variant with GrContextOptions
    #[cfg(feature = "d3d")]
    pub unsafe fn new_d3d(backend_context: &d3d::BackendContext) -> Option<Context> {
        DirectContext::new_d3d(backend_context, None).map(|c| c.into())
    }

    // TODO: threadSafeProxy()

    pub fn reset(&mut self, backend_state: Option<u32>) -> &mut Self {
        unsafe {
            self.native_mut()
                .resetContext(backend_state.unwrap_or(sb::kAll_GrBackendState))
        }
        self
    }

    pub fn reset_gl_texture_bindings(&mut self) -> &mut Self {
        unsafe { self.native_mut().resetGLTextureBindings() }
        self
    }

    pub fn abandon(&mut self) -> &mut Self {
        unsafe {
            // self.native_mut().abandonContext()
            sb::GrContext_abandonContext(self.native_mut() as *mut _ as _)
        }
        self
    }

    pub fn oomed(&mut self) -> bool {
        unsafe { self.native_mut().oomed() }
    }

    pub fn release_resources_and_abandon(&mut self) -> &mut Self {
        unsafe { sb::GrContext_releaseResourcesAndAbandonContext(self.native_mut() as *mut _ as _) }
        self
    }

    pub fn resource_cache_limits(&self) -> ResourceCacheLimits {
        let mut resources = 0;
        let mut resource_bytes = 0;
        unsafe {
            self.native()
                .getResourceCacheLimits(&mut resources, &mut resource_bytes)
        }
        ResourceCacheLimits {
            max_resources: resources.try_into().unwrap(),
            max_resource_bytes: resource_bytes,
        }
    }

    pub fn resource_cache_limit(&self) -> usize {
        unsafe { self.native().getResourceCacheLimit() }
    }

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

    pub fn resource_cache_purgeable_bytes(&self) -> usize {
        unsafe { self.native().getResourceCachePurgeableBytes() }
    }

    pub fn set_resource_cache_limits(&mut self, limits: ResourceCacheLimits) {
        unsafe {
            self.native_mut().setResourceCacheLimits(
                limits.max_resources.try_into().unwrap(),
                limits.max_resource_bytes,
            )
        }
    }

    pub fn set_resource_cache_limit(&mut self, max_resource_bytes: usize) {
        unsafe { self.native_mut().setResourceCacheLimit(max_resource_bytes) }
    }

    pub fn free_gpu_resources(&mut self) -> &mut Self {
        unsafe { sb::GrContext_freeGpuResources(self.native_mut() as *mut _ as _) }
        self
    }

    pub fn perform_deferred_cleanup(&mut self, not_used: Duration) -> &mut Self {
        unsafe {
            sb::C_GrContext_performDeferredCleanup(
                self.native_mut(),
                not_used.as_millis().try_into().unwrap(),
            )
        }
        self
    }

    pub fn purge_unlocked_resources(
        &mut self,
        bytes_to_purge: Option<usize>,
        prefer_scratch_resources: bool,
    ) -> &mut Self {
        unsafe {
            match bytes_to_purge {
                Some(bytes_to_purge) => self
                    .native_mut()
                    .purgeUnlockedResources(bytes_to_purge, prefer_scratch_resources),
                None => self
                    .native_mut()
                    .purgeUnlockedResources1(prefer_scratch_resources),
            }
        }
        self
    }

    // TODO: wait()

    pub fn flush_and_submit(&mut self) -> &mut Self {
        unsafe { sb::C_GrContext_flushAndSubmit(self.native_mut()) }
        self
    }

    pub fn flush_with_info(&mut self, info: &FlushInfo) -> SemaphoresSubmitted {
        unsafe { self.native_mut().flush(info.native()) }
    }

    #[deprecated(since = "0.30.0", note = "use flush_and_submit()")]
    pub fn flush(&mut self) -> &mut Self {
        self.flush_and_submit()
    }

    // TODO: flush(GrFlushInfo, ..)

    pub fn submit(&mut self, sync_cpu: impl Into<Option<bool>>) -> bool {
        unsafe { self.native_mut().submit(sync_cpu.into().unwrap_or(false)) }
    }

    pub fn check_async_work_completion(&mut self) {
        unsafe { self.native_mut().checkAsyncWorkCompletion() }
    }

    pub fn supports_distance_field_text(&self) -> bool {
        unsafe { self.native().supportsDistanceFieldText() }
    }

    #[cfg(feature = "vulkan")]
    pub fn store_vk_pipeline_cache_data(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().storeVkPipelineCacheData();
        }
        self
    }

    pub fn compute_image_size(
        image: impl AsRef<Image>,
        mipmapped: Mipmapped,
        use_next_pow2: impl Into<Option<bool>>,
    ) -> usize {
        unsafe {
            sb::C_GrContext_ComputeImageSize(
                image.as_ref().clone().into_ptr(),
                mipmapped,
                use_next_pow2.into().unwrap_or_default(),
            )
        }
    }

    // TODO: wrap createBackendTexture (several variants)
    //       introduced in m76, m77, and m79
    //       extended in m84 with finishedProc and finishedContext

    // TODO: wrap updateBackendTexture (several variants)
    //       introduced in m84

    pub fn compressed_backend_format(&self, compression: image::CompressionType) -> BackendFormat {
        let mut backend_format = BackendFormat::default();
        unsafe {
            sb::C_GrContext_compressedBackendFormat(
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
    pub fn set_backend_texture_state(
        &mut self,
        backend_texture: &BackendTexture,
        state: &BackendSurfaceMutableState,
    ) -> bool {
        self.set_backend_texture_state_and_return_previous(backend_texture, state)
            .is_some()
    }

    pub fn set_backend_texture_state_and_return_previous(
        &mut self,
        backend_texture: &BackendTexture,
        state: &BackendSurfaceMutableState,
    ) -> Option<BackendSurfaceMutableState> {
        let mut previous = BackendSurfaceMutableState::default();
        unsafe {
            self.native_mut().setBackendTextureState(
                backend_texture.native(),
                state.native(),
                previous.native_mut(),
                None,
                ptr::null_mut(),
            )
        }
        .if_true_some(previous)
    }

    // TODO: add variant with GpuFinishedProc / GpuFinishedContext
    pub fn set_backend_render_target_state(
        &mut self,
        target: &BackendRenderTarget,
        state: &BackendSurfaceMutableState,
    ) -> bool {
        self.set_backend_render_target_state_and_return_previous(target, state)
            .is_some()
    }

    pub fn set_backend_render_target_state_and_return_previous(
        &mut self,
        target: &BackendRenderTarget,
        state: &BackendSurfaceMutableState,
    ) -> Option<BackendSurfaceMutableState> {
        let mut previous = BackendSurfaceMutableState::default();
        unsafe {
            self.native_mut().setBackendRenderTargetState(
                target.native(),
                state.native(),
                previous.native_mut(),
                None,
                ptr::null_mut(),
            )
        }
        .if_true_some(previous)
    }

    // TODO: wrap deleteBackendTexture(),

    pub fn precompile_shader(&mut self, key: &Data, data: &Data) -> bool {
        unsafe {
            self.native_mut()
                .precompileShader(key.native(), data.native())
        }
    }
}
