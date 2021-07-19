#[cfg(feature = "d3d")]
use super::d3d;
#[cfg(feature = "gl")]
use super::gl;
#[cfg(feature = "vulkan")]
use super::vk;
use super::{
    BackendFormat, BackendRenderTarget, BackendSurfaceMutableState, BackendTexture, ContextOptions,
    FlushInfo, RecordingContext, SemaphoresSubmitted,
};
use crate::{image, prelude::*, Data};
use skia_bindings::{self as sb, GrDirectContext, GrDirectContext_DirectContextID, SkRefCntBase};
use std::{
    fmt,
    ops::{Deref, DerefMut},
    ptr,
    time::Duration,
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DirectContextId {
    id: u32,
}

native_transmutable!(
    GrDirectContext_DirectContextID,
    DirectContextId,
    direct_context_id_layout
);

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
pub struct ResourceCacheLimits {
    pub max_resources: usize,
    pub max_resource_bytes: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ResourceCacheUsage {
    pub resource_count: usize,
    pub resource_bytes: usize,
}

impl fmt::Debug for DirectContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirectContext")
            .field("base", self as &RecordingContext)
            .field("resource_cache_limits", &self.resource_cache_limits())
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
    #[cfg(feature = "gl")]
    pub fn new_gl<'a>(
        interface: impl Into<Option<gl::Interface>>,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrDirectContext_MakeGL(
                interface.into().into_ptr_or_null(),
                options.into().native_ptr_or_null(),
            )
        })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan<'a>(
        backend_context: &vk::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = DirectContext::from_ptr(sb::C_GrDirectContext_MakeVulkan(
                backend_context.native.as_ptr() as _,
                options.into().native_ptr_or_null(),
            ));
            drop(end_resolving);
            context
        }
    }

    #[cfg(feature = "metal")]
    pub fn new_metal<'a>(
        backend: &crate::gpu::mtl::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrContext_MakeMetal(backend.native(), options.into().native_ptr_or_null())
        })
    }

    #[cfg(feature = "d3d")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_d3d<'a>(
        backend_context: &d3d::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(sb::C_GrDirectContext_MakeDirect3D(
            backend_context.native(),
            options.into().native_ptr_or_null(),
        ))
    }

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
            sb::GrDirectContext_abandonContext(self.native_mut() as *mut _ as _)
        }
        self
    }

    // TODO: threadSafeProxy()

    pub fn oomed(&mut self) -> bool {
        unsafe { self.native_mut().oomed() }
    }

    pub fn release_resources_and_abandon(&mut self) -> &mut Self {
        unsafe {
            sb::GrDirectContext_releaseResourcesAndAbandonContext(self.native_mut() as *mut _ as _)
        }
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
        unsafe { sb::GrDirectContext_freeGpuResources(self.native_mut() as *mut _ as _) }
        self
    }

    pub fn perform_deferred_cleanup(
        &mut self,
        not_used: Duration,
        scratch_resources_only: impl Into<Option<bool>>,
    ) -> &mut Self {
        unsafe {
            sb::C_GrDirectContext_performDeferredCleanup(
                self.native_mut(),
                not_used.as_millis().try_into().unwrap(),
                scratch_resources_only.into().unwrap_or(false),
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
        unsafe { sb::C_GrDirectContext_flushAndSubmit(self.native_mut()) }
        self
    }

    pub fn flush_submit_and_sync_cpu(&mut self) -> &mut Self {
        self.flush(&FlushInfo::default());
        self.submit(true);
        self
    }

    #[deprecated(since = "0.37.0", note = "Use flush()")]
    pub fn flush_with_info(&mut self, info: &FlushInfo) -> SemaphoresSubmitted {
        self.flush(info)
    }

    pub fn flush<'a>(&mut self, info: impl Into<Option<&'a FlushInfo>>) -> SemaphoresSubmitted {
        let n = self.native_mut();
        if let Some(info) = info.into() {
            unsafe { n.flush(info.native()) }
        } else {
            let info = FlushInfo::default();
            unsafe { n.flush(info.native()) }
        }
    }

    pub fn submit(&mut self, sync_cpu: impl Into<Option<bool>>) -> bool {
        unsafe { self.native_mut().submit(sync_cpu.into().unwrap_or(false)) }
    }

    pub fn check_async_work_completion(&mut self) {
        unsafe { self.native_mut().checkAsyncWorkCompletion() }
    }

    // TODO: dumpMemoryStatistics()

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

    // TODO: wrap createBackendTexture (several variants)
    //       introduced in m76, m77, and m79
    //       extended in m84 with finishedProc and finishedContext

    // TODO: wrap updateBackendTexture (several variants)
    //       introduced in m84

    pub fn compressed_backend_format(&self, compression: image::CompressionType) -> BackendFormat {
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

    pub fn id(&self) -> DirectContextId {
        let mut id = DirectContextId { id: 0 };
        unsafe { sb::C_GrDirectContext_directContextId(self.native(), id.native_mut()) }
        id
    }
}
