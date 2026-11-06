//! The device: limits, features, error scopes, and polling. The `wgpuDeviceCreate*` functions
//! live with the types they create.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use skia_bindings as sb;

use super::convert::{self, CallbackString};
use super::future::{Userdata, block_on};
use super::handle::{Handle, handles};
use super::instance::{self, Instance};
use super::{compat, wgpu};

/// Behind a `WGPUDevice`: the application's `wgpu` device and the instance that polls it.
#[derive(Debug)]
pub struct Device {
    pub device: wgpu::Device,
    pub instance: Arc<Instance>,
}

handles! {
    Device => WGPUDevice:
        wgpuDeviceAddRef: WGPUProcDeviceAddRef,
        wgpuDeviceRelease: WGPUProcDeviceRelease;
}

impl Device {
    pub fn new(device: wgpu::Device, instance: Arc<Instance>) -> Self {
        Self { device, instance }
    }

    /// Runs `create` in error scopes for all three error filters and returns its result and the
    /// error it raised, so that the creation functions can report it through their callbacks.
    ///
    /// `wgpu` passes an error that no scope of its filter captures to the device's handler, which
    /// panics by default. A pipeline whose shader `wgpu` cannot translate for its backend raises an
    /// internal error, not a validation error, so a validation scope alone would let it panic.
    pub fn capture_errors<T>(&self, create: impl FnOnce() -> T) -> (T, Option<wgpu::Error>) {
        let validation = self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let out_of_memory = self.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let internal = self.device.push_error_scope(wgpu::ErrorFilter::Internal);
        let result = create();
        // `wgpu` closes the scopes of a thread innermost first.
        let internal = block_on(internal.pop());
        let out_of_memory = block_on(out_of_memory.pop());
        let validation = block_on(validation.pop());
        (result, validation.or(internal).or(out_of_memory))
    }
}

/// Completes the buffer maps that are still pending when Skia releases its last device handle.
///
/// `wgpu` holds the callback of a pending map until a poll completes it, and Skia's map callback
/// holds Skia's buffer, which holds the `wgpu` buffer. Without this poll, the buffers leak if the
/// application never polls or submits on its device again (ADR 0003). Skia's map callbacks fire
/// here, while Skia destroys its shared context, which they do not use. The poll waits for all
/// work submitted to the device, also the application's (ADR 0003).
impl Drop for Device {
    fn drop(&mut self) {
        // An incomplete wait on WebGL leaves the maps pending and leaks them (ADR 0003).
        let _ = self.instance.poll(true);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceGetAdapter(device: sb::WGPUDevice) -> sb::WGPUAdapter {
    instance::adapter_handle(&unsafe { Device::get(device) }.device)
}
const _: sb::WGPUProcDeviceGetAdapter = Some(wgpuDeviceGetAdapter);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceTick(device: sb::WGPUDevice) {
    unsafe { Device::get(device) }.instance.poll(false);
}
const _: sb::WGPUProcDeviceTick = Some(wgpuDeviceTick);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceGetLimits(
    device: sb::WGPUDevice,
    limits: *mut sb::WGPULimits,
) -> sb::WGPUStatus {
    let w = unsafe { Device::get(device) }.device.limits();
    let c = unsafe { &mut *limits };
    // Skia chains the compatibility mode limits. Without compatibility mode they equal the core
    // limits of each shader stage.
    let mut chain = c.nextInChain;
    while let Some(chained) = unsafe { chain.as_mut() } {
        match chained.sType {
            sb::WGPUSType::WGPUSType_CompatibilityModeLimits => {
                let compat = unsafe { &mut *(chain as *mut sb::WGPUCompatibilityModeLimits) };
                compat.maxStorageBuffersInVertexStage = w.max_storage_buffers_per_shader_stage;
                compat.maxStorageTexturesInVertexStage = w.max_storage_textures_per_shader_stage;
                compat.maxStorageBuffersInFragmentStage = w.max_storage_buffers_per_shader_stage;
                compat.maxStorageTexturesInFragmentStage = w.max_storage_textures_per_shader_stage;
            }
            other => panic!("WGPULimits chains the unsupported extension {other:?}"),
        }
        chain = chained.next;
    }
    c.maxTextureDimension1D = w.max_texture_dimension_1d;
    c.maxTextureDimension2D = w.max_texture_dimension_2d;
    c.maxTextureDimension3D = w.max_texture_dimension_3d;
    c.maxTextureArrayLayers = w.max_texture_array_layers;
    c.maxBindGroups = w.max_bind_groups;
    c.maxBindGroupsPlusVertexBuffers = w.max_bind_groups_plus_vertex_buffers;
    c.maxBindingsPerBindGroup = w.max_bindings_per_bind_group;
    c.maxDynamicUniformBuffersPerPipelineLayout = w.max_dynamic_uniform_buffers_per_pipeline_layout;
    c.maxDynamicStorageBuffersPerPipelineLayout = w.max_dynamic_storage_buffers_per_pipeline_layout;
    c.maxSampledTexturesPerShaderStage = w.max_sampled_textures_per_shader_stage;
    c.maxSamplersPerShaderStage = w.max_samplers_per_shader_stage;
    c.maxStorageBuffersPerShaderStage = w.max_storage_buffers_per_shader_stage;
    c.maxStorageTexturesPerShaderStage = w.max_storage_textures_per_shader_stage;
    c.maxUniformBuffersPerShaderStage = w.max_uniform_buffers_per_shader_stage;
    c.maxUniformBufferBindingSize = w.max_uniform_buffer_binding_size;
    c.maxStorageBufferBindingSize = w.max_storage_buffer_binding_size;
    c.minUniformBufferOffsetAlignment = w.min_uniform_buffer_offset_alignment;
    c.minStorageBufferOffsetAlignment = w.min_storage_buffer_offset_alignment;
    c.maxVertexBuffers = w.max_vertex_buffers;
    c.maxBufferSize = w.max_buffer_size;
    c.maxVertexAttributes = w.max_vertex_attributes;
    c.maxVertexBufferArrayStride = w.max_vertex_buffer_array_stride;
    c.maxInterStageShaderVariables = w.max_inter_stage_shader_variables;
    c.maxColorAttachments = w.max_color_attachments;
    c.maxColorAttachmentBytesPerSample = w.max_color_attachment_bytes_per_sample;
    c.maxComputeWorkgroupStorageSize = w.max_compute_workgroup_storage_size;
    c.maxComputeInvocationsPerWorkgroup = w.max_compute_invocations_per_workgroup;
    c.maxComputeWorkgroupSizeX = w.max_compute_workgroup_size_x;
    c.maxComputeWorkgroupSizeY = w.max_compute_workgroup_size_y;
    c.maxComputeWorkgroupSizeZ = w.max_compute_workgroup_size_z;
    c.maxComputeWorkgroupsPerDimension = w.max_compute_workgroups_per_dimension;
    c.maxImmediateSize = w.max_immediate_size;
    sb::WGPUStatus::WGPUStatus_Success
}
const _: sb::WGPUProcDeviceGetLimits = Some(wgpuDeviceGetLimits);

/// Reports a WebGPU feature as supported when the device has the `wgpu` feature that implements
/// it. The shim reports every other feature as unsupported, so Skia takes its fallback paths for
/// them. The crate's `FEATURES.md` gives the reason for each.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceHasFeature(
    device: sb::WGPUDevice,
    feature: sb::WGPUFeatureName,
) -> sb::WGPUBool {
    use sb::WGPUFeatureName as F;
    use wgpu::Features as W;
    let device = &unsafe { Device::get(device) }.device;
    let features = device.features();
    let required = match feature {
        // `wgpu` devices are never in WebGPU's compatibility mode.
        F::WGPUFeatureName_CoreFeaturesAndLimits => W::empty(),
        F::WGPUFeatureName_DepthClipControl => W::DEPTH_CLIP_CONTROL,
        F::WGPUFeatureName_Depth32FloatStencil8 => W::DEPTH32FLOAT_STENCIL8,
        F::WGPUFeatureName_TextureCompressionBC => W::TEXTURE_COMPRESSION_BC,
        F::WGPUFeatureName_TextureCompressionETC2 => W::TEXTURE_COMPRESSION_ETC2,
        F::WGPUFeatureName_TextureCompressionASTC => W::TEXTURE_COMPRESSION_ASTC,
        // On Metal, Skia writes timestamps at the beginning and end of render passes, which
        // WebGPU's timestamp queries cover. On every other backend, it writes them on the command
        // encoder, which `wgpu` allows only with a native feature on top of them (`FEATURES.md`).
        F::WGPUFeatureName_TimestampQuery
            if device.adapter_info().backend == wgpu::Backend::Metal =>
        {
            W::TIMESTAMP_QUERY
        }
        F::WGPUFeatureName_TimestampQuery => {
            W::TIMESTAMP_QUERY | W::TIMESTAMP_QUERY_INSIDE_ENCODERS
        }
        F::WGPUFeatureName_IndirectFirstInstance => W::INDIRECT_FIRST_INSTANCE,
        // With this feature, Skia writes `f16` fragment outputs, which some `wgpu` versions reject
        // (`src/vNN/README.md`).
        F::WGPUFeatureName_ShaderF16 if compat::ACCEPTS_F16_FRAGMENT_OUTPUTS => W::SHADER_F16,
        F::WGPUFeatureName_RG11B10UfloatRenderable => W::RG11B10UFLOAT_RENDERABLE,
        F::WGPUFeatureName_BGRA8UnormStorage => W::BGRA8UNORM_STORAGE,
        F::WGPUFeatureName_Float32Filterable => W::FLOAT32_FILTERABLE,
        F::WGPUFeatureName_DualSourceBlending => W::DUAL_SOURCE_BLENDING,
        F::WGPUFeatureName_ClipDistances => W::CLIP_DISTANCES,
        F::WGPUFeatureName_Float32Blendable => W::FLOAT32_BLENDABLE,
        // Mapping buffers that the GPU reads directly only pays off on GPUs that share memory with
        // the CPU, so the application decides by requesting the feature for its device
        // (`FEATURES.md`).
        F::WGPUFeatureName_BufferMapExtendedUsages => W::MAPPABLE_PRIMARY_BUFFERS,
        // Features without a `wgpu` counterpart, and those that the shim leaves off on purpose.
        // Listing every variant makes a feature added to Dawn's header a compile error.
        // `FEATURES.md` explains why `TransientAttachments`, `Unorm16TextureFormats`, and
        // `DawnAllowUndefinedLoadStoreOp` stay unreported although `wgpu` has a counterpart.
        // `ShaderF16` lands here on a `wgpu` version that rejects Skia's `f16` outputs.
        F::WGPUFeatureName_ShaderF16
        | F::WGPUFeatureName_TextureCompressionBCSliced3D
        | F::WGPUFeatureName_TextureCompressionASTCSliced3D
        | F::WGPUFeatureName_Subgroups
        | F::WGPUFeatureName_TextureFormatsTier1
        | F::WGPUFeatureName_TextureFormatsTier2
        | F::WGPUFeatureName_PrimitiveIndex
        | F::WGPUFeatureName_TextureComponentSwizzle
        | F::WGPUFeatureName_SubgroupSizeControl
        | F::WGPUFeatureName_TextureCompressionUnaligned
        | F::WGPUFeatureName_AtomicVec2uMinMax
        | F::WGPUFeatureName_DawnInternalUsages
        | F::WGPUFeatureName_DawnMultiPlanarFormats
        | F::WGPUFeatureName_DawnNative
        | F::WGPUFeatureName_ChromiumExperimentalTimestampQueryInsidePasses
        | F::WGPUFeatureName_ImplicitDeviceSynchronization
        | F::WGPUFeatureName_TransientAttachments
        | F::WGPUFeatureName_MSAARenderToSingleSampled
        | F::WGPUFeatureName_D3D11MultithreadProtected
        | F::WGPUFeatureName_ANGLETextureSharing
        | F::WGPUFeatureName_PixelLocalStorageCoherent
        | F::WGPUFeatureName_PixelLocalStorageNonCoherent
        | F::WGPUFeatureName_Unorm16TextureFormats
        | F::WGPUFeatureName_MultiPlanarFormatExtendedUsages
        | F::WGPUFeatureName_MultiPlanarFormatP010
        | F::WGPUFeatureName_HostMappedPointer
        | F::WGPUFeatureName_MultiPlanarRenderTargets
        | F::WGPUFeatureName_MultiPlanarFormatNv12a
        | F::WGPUFeatureName_FramebufferFetch
        | F::WGPUFeatureName_AdapterPropertiesMemoryHeaps
        | F::WGPUFeatureName_AdapterPropertiesD3D
        | F::WGPUFeatureName_AdapterPropertiesVk
        | F::WGPUFeatureName_DawnFormatCapabilities
        | F::WGPUFeatureName_DawnDrmFormatCapabilities
        | F::WGPUFeatureName_MultiPlanarFormatNv16
        | F::WGPUFeatureName_MultiPlanarFormatNv24
        | F::WGPUFeatureName_MultiPlanarFormatP210
        | F::WGPUFeatureName_MultiPlanarFormatP410
        | F::WGPUFeatureName_SharedTextureMemoryVkDedicatedAllocation
        | F::WGPUFeatureName_SharedTextureMemoryAHardwareBuffer
        | F::WGPUFeatureName_SharedTextureMemoryDmaBuf
        | F::WGPUFeatureName_SharedTextureMemoryOpaqueFD
        | F::WGPUFeatureName_SharedTextureMemoryZirconHandle
        | F::WGPUFeatureName_SharedTextureMemoryDXGISharedHandle
        | F::WGPUFeatureName_SharedTextureMemoryD3D11Texture2D
        | F::WGPUFeatureName_SharedTextureMemoryIOSurface
        | F::WGPUFeatureName_SharedTextureMemoryEGLImage
        | F::WGPUFeatureName_SharedFenceVkSemaphoreOpaqueFD
        | F::WGPUFeatureName_SharedFenceSyncFD
        | F::WGPUFeatureName_SharedFenceVkSemaphoreZirconHandle
        | F::WGPUFeatureName_SharedFenceDXGISharedHandle
        | F::WGPUFeatureName_SharedFenceMTLSharedEvent
        | F::WGPUFeatureName_SharedBufferMemoryD3D12Resource
        | F::WGPUFeatureName_StaticSamplers
        | F::WGPUFeatureName_YCbCrVulkanSamplers
        | F::WGPUFeatureName_ShaderModuleCompilationOptions
        | F::WGPUFeatureName_DawnLoadResolveTexture
        | F::WGPUFeatureName_DawnPartialLoadResolveTexture
        | F::WGPUFeatureName_MultiDrawIndirect
        | F::WGPUFeatureName_DawnTexelCopyBufferRowAlignment
        | F::WGPUFeatureName_FlexibleTextureViews
        | F::WGPUFeatureName_ChromiumExperimentalSubgroupMatrix
        | F::WGPUFeatureName_SharedFenceEGLSync
        | F::WGPUFeatureName_DawnDeviceAllocatorControl
        | F::WGPUFeatureName_AdapterPropertiesWGPU
        | F::WGPUFeatureName_SharedBufferMemoryFromWindowsHandle
        | F::WGPUFeatureName_SharedTextureMemoryD3D12Resource
        | F::WGPUFeatureName_ChromiumExperimentalSamplingResourceTable
        | F::WGPUFeatureName_Unorm16FormatsForExternalTexture
        | F::WGPUFeatureName_OpaqueYCbCrAndroidForExternalTexture
        | F::WGPUFeatureName_Unorm16Filterable
        | F::WGPUFeatureName_RenderPassRenderArea
        | F::WGPUFeatureName_AdapterPropertiesDrm
        | F::WGPUFeatureName_DawnAllowUndefinedLoadStoreOp
        | F::WGPUFeatureName_BufferMapWriteExtendedUsages
        | F::WGPUFeatureName_SharedBufferMemoryHostPointer
        | F::WGPUFeatureName_Force32 => {
            return convert::c_bool(false);
        }
    };
    convert::c_bool(features.contains(required))
}
const _: sb::WGPUProcDeviceHasFeature = Some(wgpuDeviceHasFeature);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDevicePushErrorScope(
    device: sb::WGPUDevice,
    filter: sb::WGPUErrorFilter,
) {
    use sb::WGPUErrorFilter as F;
    let filter = match filter {
        F::WGPUErrorFilter_Validation => wgpu::ErrorFilter::Validation,
        F::WGPUErrorFilter_OutOfMemory => wgpu::ErrorFilter::OutOfMemory,
        F::WGPUErrorFilter_Internal => wgpu::ErrorFilter::Internal,
        F::WGPUErrorFilter_Force32 => panic!("unsupported {filter:?}"),
    };
    let scope = unsafe { Device::get(device) }
        .device
        .push_error_scope(filter);
    ERROR_SCOPES.with_borrow_mut(|scopes| scopes.entry(device as usize).or_default().push(scope));
}
const _: sb::WGPUProcDevicePushErrorScope = Some(wgpuDevicePushErrorScope);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDevicePopErrorScope(
    device: sb::WGPUDevice,
    callback_info: sb::WGPUPopErrorScopeCallbackInfo,
) -> sb::WGPUFuture {
    use sb::WGPUErrorType as E;
    use sb::WGPUPopErrorScopeStatus as S;
    unsafe { convert::no_chain(callback_info.nextInChain, "WGPUPopErrorScopeCallbackInfo") };
    let callback = callback_info
        .callback
        .expect("null WGPUPopErrorScopeCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    let call = move |result: Option<(S, E, Option<CallbackString>)>| {
        let userdata = userdata;
        let (status, error_type, message) = result.unwrap_or((
            S::WGPUPopErrorScopeStatus_CallbackCancelled,
            E::WGPUErrorType_NoError,
            None,
        ));
        let message = message
            .as_ref()
            .map_or_else(convert::empty_string, CallbackString::view);
        unsafe { callback(status, error_type, message, userdata.0, userdata.1) };
    };
    let (future, completion) = unsafe { Device::get(device) }
        .instance
        .futures
        .register(callback_info.mode, call);
    let scope = ERROR_SCOPES.with_borrow_mut(|scopes| {
        let stack = scopes.get_mut(&(device as usize))?;
        let scope = stack.pop();
        if stack.is_empty() {
            scopes.remove(&(device as usize));
        }
        scope
    });
    let (status, error_type, message) = match scope {
        None => (
            S::WGPUPopErrorScopeStatus_Error,
            E::WGPUErrorType_NoError,
            None,
        ),
        Some(scope) => match block_on(scope.pop()) {
            None => (
                S::WGPUPopErrorScopeStatus_Success,
                E::WGPUErrorType_NoError,
                None,
            ),
            Some(error) => {
                let error_type = match &error {
                    wgpu::Error::OutOfMemory { .. } => E::WGPUErrorType_OutOfMemory,
                    wgpu::Error::Validation { .. } => E::WGPUErrorType_Validation,
                    wgpu::Error::Internal { .. } => E::WGPUErrorType_Internal,
                };
                let message = CallbackString::new(&error.to_string());
                (
                    S::WGPUPopErrorScopeStatus_Success,
                    error_type,
                    Some(message),
                )
            }
        },
    };
    completion.complete((status, error_type, message));
    future
}
const _: sb::WGPUProcDevicePopErrorScope = Some(wgpuDevicePopErrorScope);

thread_local! {
    /// The scopes that `wgpuDevicePushErrorScope` opened on this thread, by device handle,
    /// innermost last. `wgpu` keeps error scopes per thread and closes one through its guard,
    /// which must stay on that thread, while WebGPU closes the innermost scope of the device.
    static ERROR_SCOPES: RefCell<HashMap<usize, Vec<wgpu::ErrorScopeGuard>>> = RefCell::default();
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    use skia_bindings as sb;

    use super::super::api::BackendHandles;
    use super::super::{testing, wgpu};
    use super::{wgpuDeviceHasFeature, wgpuDevicePopErrorScope, wgpuDevicePushErrorScope};

    /// `TimestampQuery` also requires `TIMESTAMP_QUERY_INSIDE_ENCODERS`, because Skia writes
    /// timestamps outside passes, except on Metal (`FEATURES.md`).
    #[test]
    fn timestamp_query_alone_is_reported_only_on_metal() {
        let Some((device, queue)) = testing::device_with(wgpu::Features::TIMESTAMP_QUERY) else {
            return;
        };
        let handles = BackendHandles::new(&device, &queue);

        let reported = unsafe {
            wgpuDeviceHasFeature(
                handles.device(),
                sb::WGPUFeatureName::WGPUFeatureName_TimestampQuery,
            )
        };

        let metal = device.adapter_info().backend == wgpu::Backend::Metal;
        assert_eq!(reported != 0, metal);
    }

    /// `wgpu` keeps error scopes per thread and panics when a thread pops a scope that another
    /// thread pushed, unless both scopes happen to have the same nesting depth. Skia's scoped error
    /// checks nest three scopes on every thread that records, so a pop must close the innermost
    /// scope of its own thread, also while another thread has scopes open at another depth.
    #[test]
    fn a_pop_closes_the_scope_of_its_own_thread() {
        let Some((device, queue)) = testing::device() else {
            return;
        };
        let handles = BackendHandles::new(&device, &queue);
        // Raw pointers are not `Send`. The handle stays valid while `handles` lives.
        let device = handles.device() as usize;

        push(device);
        push(device);
        let (pushed, other_pushed) = mpsc::channel();
        let (popped, main_popped) = mpsc::channel::<()>();
        let other = thread::spawn(move || {
            push(device);
            pushed.send(()).unwrap();
            main_popped.recv().unwrap();
            pop(device);
        });
        other_pushed.recv().unwrap();
        pop(device);
        popped.send(()).unwrap();
        other.join().unwrap();
        pop(device);
    }

    fn push(device: usize) {
        unsafe {
            wgpuDevicePushErrorScope(
                device as sb::WGPUDevice,
                sb::WGPUErrorFilter::WGPUErrorFilter_Validation,
            )
        };
    }

    fn pop(device: usize) {
        unsafe extern "C" fn ignore(
            _: sb::WGPUPopErrorScopeStatus,
            _: sb::WGPUErrorType,
            _: sb::WGPUStringView,
            _: *mut std::ffi::c_void,
            _: *mut std::ffi::c_void,
        ) {
        }
        let callback_info = sb::WGPUPopErrorScopeCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: sb::WGPUCallbackMode::WGPUCallbackMode_AllowSpontaneous,
            callback: Some(ignore),
            userdata1: std::ptr::null_mut(),
            userdata2: std::ptr::null_mut(),
        };
        unsafe { wgpuDevicePopErrorScope(device as sb::WGPUDevice, callback_info) };
    }
}
