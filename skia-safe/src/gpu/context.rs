#[cfg(feature = "d3d")]
use super::d3d;
#[cfg(feature = "gl")]
use super::gl;
#[cfg(feature = "vulkan")]
use super::vk;
use super::{BackendRenderTarget, BackendSurfaceMutableState, BackendTexture};
use crate::gpu::{BackendFormat, MipMapped, Renderable};
use crate::prelude::*;
use crate::{image, ColorType, Data, Image};
use skia_bindings as sb;
use skia_bindings::{GrContext, SkRefCntBase};
use std::{ptr, time::Duration};

pub type Context = RCHandle<GrContext>;

impl NativeRefCountedBase for GrContext {
    type Base = SkRefCntBase;
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
    // TODO: support variant with GrContextOptions
    #[cfg(feature = "gl")]
    pub fn new_gl(interface: impl Into<Option<gl::Interface>>) -> Option<Context> {
        Context::from_ptr(unsafe { sb::C_GrContext_MakeGL(interface.into().into_ptr_or_null()) })
    }

    // TODO: support variant with GrContextOptions
    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(backend_context: &vk::BackendContext) -> Option<Context> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = Context::from_ptr(sb::C_GrContext_MakeVulkan(
                backend_context.native.as_ptr() as _,
            ));
            drop(end_resolving);
            context
        }
    }

    // TODO: support variant with GrContextOptions
    /// # Safety
    /// This function is unsafe because `device` and `queue` are untyped handles which need to exceed the
    /// lifetime of the context returned.
    #[cfg(feature = "metal")]
    pub unsafe fn new_metal(
        device: *mut std::ffi::c_void,
        queue: *mut std::ffi::c_void,
    ) -> Option<Context> {
        Context::from_ptr(sb::C_GrContext_MakeMetal(device, queue))
    }

    // TODO: support variant with GrContextOptions
    #[cfg(feature = "d3d")]
    pub unsafe fn new_d3d(backend_context: &d3d::BackendContext) -> Option<Context> {
        Context::from_ptr(sb::C_GrContext_MakeDirect3D(backend_context.native()))
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

    // TODO: is_...?
    pub fn abandoned(&mut self) -> bool {
        unsafe { sb::C_GrContext_abandoned(self.native_mut()) }
    }

    // TODO: is_...?
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

    pub fn max_texture_size(&self) -> i32 {
        unsafe { self.native().maxTextureSize() }
    }

    pub fn max_render_target_size(&self) -> i32 {
        unsafe { self.native().maxRenderTargetSize() }
    }

    // TODO: is_...?
    pub fn color_type_supported_as_image(&self, color_type: ColorType) -> bool {
        unsafe {
            self.native()
                .colorTypeSupportedAsImage(color_type.into_native())
        }
    }

    // TODO: is_...?
    pub fn color_type_supported_as_surface(&self, color_type: ColorType) -> bool {
        unsafe {
            sb::C_GrContext_colorTypeSupportedAsSurface(self.native(), color_type.into_native())
        }
    }

    pub fn max_surface_sample_count_for_color_type(&self, color_type: ColorType) -> usize {
        unsafe {
            self.native()
                .maxSurfaceSampleCountForColorType(color_type.into_native())
                .try_into()
                .unwrap()
        }
    }

    // TODO: wait()

    #[deprecated(since = "0.30.0", note = "use flush_and_submit()")]
    pub fn flush(&mut self) -> &mut Self {
        self.flush_and_submit()
    }

    pub fn flush_and_submit(&mut self) -> &mut Self {
        unsafe { sb::C_GrContext_flushAndSubmit(self.native_mut()) }
        self
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
        mip_mapped: MipMapped,
        use_next_pow2: impl Into<Option<bool>>,
    ) -> usize {
        unsafe {
            sb::C_GrContext_ComputeImageSize(
                image.as_ref().clone().into_ptr(),
                mip_mapped,
                use_next_pow2.into().unwrap_or_default(),
            )
        }
    }

    pub fn default_backend_format(&self, ct: ColorType, renderable: Renderable) -> BackendFormat {
        let mut format = BackendFormat::default();
        unsafe {
            sb::C_GrContext_defaultBackendFormat(
                self.native(),
                ct.into_native(),
                renderable,
                format.native_mut(),
            )
        };
        format
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

    // TODO: add variant with GpuFinishedProc / GpuFinishedContext
    pub fn set_backend_texture_state(
        &mut self,
        backend_texture: &BackendTexture,
        state: &BackendSurfaceMutableState,
    ) -> bool {
        unsafe {
            self.native_mut().setBackendTextureState(
                backend_texture.native(),
                state.native(),
                None,
                ptr::null_mut(),
            )
        }
    }

    // TODO: add variant with GpuFinishedProc / GpuFinishedContext
    pub fn set_backend_render_target_state(
        &mut self,
        target: &BackendRenderTarget,
        state: &BackendSurfaceMutableState,
    ) -> bool {
        unsafe {
            self.native_mut().setBackendRenderTargetState(
                target.native(),
                state.native(),
                None,
                ptr::null_mut(),
            )
        }
    }

    // TODO: wrap deleteBackendTexture(),

    pub fn precompile_shader(&mut self, key: &Data, data: &Data) -> bool {
        unsafe {
            self.native_mut()
                .precompileShader(key.native(), data.native())
        }
    }
}
