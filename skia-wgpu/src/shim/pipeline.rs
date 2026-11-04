//! Shader modules and render and compute pipelines.

use std::borrow::Cow;
use std::ptr;
use std::sync::Arc;

use skia_bindings as sb;

use super::binding::{BindGroupLayout, PipelineLayout};
use super::convert::{self, CallbackString};
use super::device::Device;
use super::future::{Userdata, block_on};
use super::handle::{Handle, get_optional, handles};
use super::instance::Instance;
use super::wgpu;

pub struct ShaderModule {
    pub module: wgpu::ShaderModule,
    /// The instance that owns the futures of `wgpuShaderModuleGetCompilationInfo`.
    instance: Arc<Instance>,
}

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

handles! {
    ShaderModule => WGPUShaderModule: shader_module_add_ref, shader_module_release;
    RenderPipeline => WGPURenderPipeline: render_pipeline_add_ref, render_pipeline_release;
    ComputePipeline => WGPUComputePipeline: compute_pipeline_add_ref, compute_pipeline_release;
}

pub unsafe extern "C" fn device_create_shader_module(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUShaderModuleDescriptor,
) -> sb::WGPUShaderModule {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    // Skia passes its shaders as WGSL, in a `ShaderSourceWGSL` chained to the descriptor.
    let mut code = None;
    let mut chain = d.nextInChain as *const sb::WGPUChainedStruct;
    while let Some(chained) = unsafe { chain.as_ref() } {
        match chained.sType {
            sb::WGPUSType::WGPUSType_ShaderSourceWGSL => {
                let source = unsafe { &*(chain as *const sb::WGPUShaderSourceWGSL) };
                code = unsafe { convert::str(source.code) };
            }
            other => {
                panic!("WGPUShaderModuleDescriptor chains the unsupported extension {other:?}")
            }
        }
        chain = chained.next;
    }
    let code = code.expect("WGPUShaderModuleDescriptor without WGSL source");
    let module = device
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: unsafe { convert::label(d.label) },
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
        });
    ShaderModule {
        module,
        instance: device.instance.clone(),
    }
    .into_handle()
}

pub unsafe extern "C" fn shader_module_get_compilation_info(
    shader_module: sb::WGPUShaderModule,
    callback_info: sb::WGPUCompilationInfoCallbackInfo,
) -> sb::WGPUFuture {
    unsafe { convert::no_chain(callback_info.nextInChain, "WGPUCompilationInfoCallbackInfo") };
    let module = unsafe { ShaderModule::get(shader_module) };
    let messages: Vec<Message> = block_on(module.module.get_compilation_info())
        .messages
        .iter()
        .map(Message::new)
        .collect();
    let (future, completion) = module.instance.futures.register(callback_info.mode);
    let callback = callback_info
        .callback
        .expect("null WGPUCompilationInfoCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    completion.complete(move || {
        let userdata = userdata;
        let c_messages: Vec<sb::WGPUCompilationMessage> =
            messages.iter().map(Message::to_c).collect();
        let info = sb::WGPUCompilationInfo {
            nextInChain: ptr::null_mut(),
            messageCount: c_messages.len(),
            messages: c_messages.as_ptr(),
        };
        unsafe {
            callback(
                sb::WGPUCompilationInfoRequestStatus::WGPUCompilationInfoRequestStatus_Success,
                &info,
                userdata.0,
                userdata.1,
            )
        };
    });
    future
}

pub unsafe extern "C" fn device_create_render_pipeline(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPURenderPipelineDescriptor,
) -> sb::WGPURenderPipeline {
    let device = unsafe { Device::get(device) };
    let pipeline = unsafe { create_render_pipeline(device, &*descriptor) };
    RenderPipeline { pipeline }.into_handle()
}

/// Creates the pipeline at once and reports the result through the callback, with the pipeline's
/// validation error if there is one.
pub unsafe extern "C" fn device_create_render_pipeline_async(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPURenderPipelineDescriptor,
    callback_info: sb::WGPUCreateRenderPipelineAsyncCallbackInfo,
) -> sb::WGPUFuture {
    use sb::WGPUCreatePipelineAsyncStatus as S;
    unsafe {
        convert::no_chain(
            callback_info.nextInChain,
            "WGPUCreateRenderPipelineAsyncCallbackInfo",
        )
    };
    let device = unsafe { Device::get(device) };
    let (pipeline, error) =
        device.capture_validation(|| unsafe { create_render_pipeline(device, &*descriptor) });
    let (status, pipeline, message) = match error {
        None => (
            S::WGPUCreatePipelineAsyncStatus_Success,
            SendHandle(RenderPipeline { pipeline }.into_handle()),
            None,
        ),
        Some(error) => (
            S::WGPUCreatePipelineAsyncStatus_ValidationError,
            SendHandle(ptr::null_mut()),
            Some(CallbackString::new(&error)),
        ),
    };
    let (future, completion) = device.instance.futures.register(callback_info.mode);
    let callback = callback_info
        .callback
        .expect("null WGPUCreateRenderPipelineAsyncCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    completion.complete(move || {
        let (pipeline, userdata) = (pipeline, userdata);
        let message = message
            .as_ref()
            .map_or_else(convert::empty_string, CallbackString::view);
        unsafe { callback(status, pipeline.0, message, userdata.0, userdata.1) };
    });
    future
}

pub unsafe extern "C" fn device_create_compute_pipeline(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUComputePipelineDescriptor,
) -> sb::WGPUComputePipeline {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUComputePipelineDescriptor") };
    let c = &d.compute;
    unsafe { convert::no_chain(c.nextInChain, "WGPUComputeState") };
    let constants = unsafe { constants(c.constants, c.constantCount) };
    let pipeline = device
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: unsafe { convert::label(d.label) },
            layout: unsafe { get_optional::<PipelineLayout, _>(d.layout) }.map(|l| &l.layout),
            module: &unsafe { ShaderModule::get(c.module) }.module,
            entry_point: unsafe { convert::label(c.entryPoint) },
            compilation_options: compilation_options(&constants),
            cache: None,
        });
    ComputePipeline { pipeline }.into_handle()
}

pub unsafe extern "C" fn render_pipeline_get_bind_group_layout(
    pipeline: sb::WGPURenderPipeline,
    group_index: u32,
) -> sb::WGPUBindGroupLayout {
    let pipeline = unsafe { RenderPipeline::get(pipeline) };
    BindGroupLayout {
        layout: pipeline.pipeline.get_bind_group_layout(group_index),
    }
    .into_handle()
}

/// A compilation message that the callback can read during its call.
struct Message {
    text: CallbackString,
    message_type: sb::WGPUCompilationMessageType,
    location: wgpu::SourceLocation,
}

impl Message {
    fn new(message: &wgpu::CompilationMessage) -> Self {
        use sb::WGPUCompilationMessageType as T;
        Self {
            text: CallbackString::new(&message.message),
            message_type: match message.message_type {
                wgpu::CompilationMessageType::Error => T::WGPUCompilationMessageType_Error,
                wgpu::CompilationMessageType::Warning => T::WGPUCompilationMessageType_Warning,
                wgpu::CompilationMessageType::Info => T::WGPUCompilationMessageType_Info,
            },
            location: message.location.unwrap_or(wgpu::SourceLocation {
                line_number: 0,
                line_position: 0,
                offset: 0,
                length: 0,
            }),
        }
    }

    fn to_c(&self) -> sb::WGPUCompilationMessage {
        sb::WGPUCompilationMessage {
            nextInChain: ptr::null_mut(),
            message: self.text.view(),
            type_: self.message_type,
            lineNum: self.location.line_number.into(),
            linePos: self.location.line_position.into(),
            offset: self.location.offset.into(),
            length: self.location.length.into(),
        }
    }
}

/// A pipeline handle on its way to the callback, which takes over its reference.
struct SendHandle(sb::WGPURenderPipeline);

// The handle points to a `RenderPipeline`, which `wgpu` makes safe to send.
unsafe impl Send for SendHandle {}

unsafe fn create_render_pipeline(
    device: &Device,
    d: &sb::WGPURenderPipelineDescriptor,
) -> wgpu::RenderPipeline {
    unsafe { convert::no_chain(d.nextInChain, "WGPURenderPipelineDescriptor") };
    let layout = unsafe { get_optional::<PipelineLayout, _>(d.layout) }.map(|l| &l.layout);

    let v = &d.vertex;
    unsafe { convert::no_chain(v.nextInChain, "WGPUVertexState") };
    let vertex_constants = unsafe { constants(v.constants, v.constantCount) };
    let c_buffers = unsafe { convert::slice(v.buffers, v.bufferCount) };
    let attributes: Vec<Vec<wgpu::VertexAttribute>> = c_buffers
        .iter()
        .map(|buffer| {
            unsafe { convert::no_chain(buffer.nextInChain, "WGPUVertexBufferLayout") };
            unsafe { convert::slice(buffer.attributes, buffer.attributeCount) }
                .iter()
                .map(|attribute| {
                    unsafe { convert::no_chain(attribute.nextInChain, "WGPUVertexAttribute") };
                    wgpu::VertexAttribute {
                        format: convert::vertex_format(attribute.format),
                        offset: attribute.offset,
                        shader_location: attribute.shaderLocation,
                    }
                })
                .collect()
        })
        .collect();
    // WebGPU marks an unused slot with an undefined step mode and no attributes
    // (`DawnGraphicsPipeline.cpp` does so for render steps without vertex or instance data).
    let buffers: Vec<Option<wgpu::VertexBufferLayout>> = c_buffers
        .iter()
        .zip(&attributes)
        .map(|(buffer, attributes)| {
            let unused = buffer.stepMode == sb::WGPUVertexStepMode::WGPUVertexStepMode_Undefined
                && attributes.is_empty();
            (!unused).then(|| wgpu::VertexBufferLayout {
                array_stride: buffer.arrayStride,
                step_mode: convert::vertex_step_mode(buffer.stepMode),
                attributes,
            })
        })
        .collect();

    let p = &d.primitive;
    unsafe { convert::no_chain(p.nextInChain, "WGPUPrimitiveState") };
    let primitive = wgpu::PrimitiveState {
        topology: convert::primitive_topology(p.topology),
        strip_index_format: convert::index_format(p.stripIndexFormat),
        front_face: convert::front_face(p.frontFace),
        cull_mode: convert::cull_mode(p.cullMode),
        unclipped_depth: convert::bool(p.unclippedDepth),
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
    };

    let depth_stencil = unsafe { d.depthStencil.as_ref() }.map(|s| unsafe { depth_stencil(s) });

    let m = &d.multisample;
    unsafe { convert::no_chain(m.nextInChain, "WGPUMultisampleState") };
    let multisample = wgpu::MultisampleState {
        count: m.count,
        mask: m.mask.into(),
        alpha_to_coverage_enabled: convert::bool(m.alphaToCoverageEnabled),
    };

    let fragment = unsafe { d.fragment.as_ref() };
    let fragment_constants = fragment
        .map(|f| unsafe { constants(f.constants, f.constantCount) })
        .unwrap_or_default();
    let targets: Vec<Option<wgpu::ColorTargetState>> = fragment
        .map(|f| {
            unsafe { convert::no_chain(f.nextInChain, "WGPUFragmentState") };
            unsafe { convert::slice(f.targets, f.targetCount) }
                .iter()
                .map(|target| unsafe { color_target(target) })
                .collect()
        })
        .unwrap_or_default();

    device
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: unsafe { convert::label(d.label) },
            layout,
            vertex: wgpu::VertexState {
                module: &unsafe { ShaderModule::get(v.module) }.module,
                entry_point: unsafe { convert::label(v.entryPoint) },
                compilation_options: compilation_options(&vertex_constants),
                buffers: &buffers,
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: fragment.map(|f| wgpu::FragmentState {
                module: &unsafe { ShaderModule::get(f.module) }.module,
                entry_point: unsafe { convert::label(f.entryPoint) },
                compilation_options: compilation_options(&fragment_constants),
                targets: &targets,
            }),
            multiview_mask: None,
            cache: None,
        })
}

unsafe fn depth_stencil(c: &sb::WGPUDepthStencilState) -> wgpu::DepthStencilState {
    unsafe { convert::no_chain(c.nextInChain, "WGPUDepthStencilState") };
    use sb::WGPUOptionalBool as B;
    let face = |f: &sb::WGPUStencilFaceState| wgpu::StencilFaceState {
        compare: convert::compare_function(f.compare).unwrap_or(wgpu::CompareFunction::Always),
        fail_op: convert::stencil_operation(f.failOp),
        depth_fail_op: convert::stencil_operation(f.depthFailOp),
        pass_op: convert::stencil_operation(f.passOp),
    };
    wgpu::DepthStencilState {
        format: convert::required_texture_format(c.format),
        depth_write_enabled: match c.depthWriteEnabled {
            B::WGPUOptionalBool_False => Some(false),
            B::WGPUOptionalBool_True => Some(true),
            B::WGPUOptionalBool_Undefined => None,
            B::WGPUOptionalBool_Force32 => panic!("unsupported {:?}", c.depthWriteEnabled),
        },
        depth_compare: convert::compare_function(c.depthCompare),
        stencil: wgpu::StencilState {
            front: face(&c.stencilFront),
            back: face(&c.stencilBack),
            read_mask: c.stencilReadMask,
            write_mask: c.stencilWriteMask,
        },
        bias: wgpu::DepthBiasState {
            constant: c.depthBias,
            slope_scale: c.depthBiasSlopeScale,
            clamp: c.depthBiasClamp,
        },
    }
}

/// Converts a color target. A target with an undefined format is an unused attachment slot.
unsafe fn color_target(c: &sb::WGPUColorTargetState) -> Option<wgpu::ColorTargetState> {
    unsafe { convert::no_chain(c.nextInChain, "WGPUColorTargetState") };
    let format = convert::texture_format(c.format)?;
    let component = |b: &sb::WGPUBlendComponent| wgpu::BlendComponent {
        src_factor: convert::blend_factor(b.srcFactor, wgpu::BlendFactor::One),
        dst_factor: convert::blend_factor(b.dstFactor, wgpu::BlendFactor::Zero),
        operation: convert::blend_operation(b.operation),
    };
    Some(wgpu::ColorTargetState {
        format,
        blend: unsafe { c.blend.as_ref() }.map(|blend| wgpu::BlendState {
            color: component(&blend.color),
            alpha: component(&blend.alpha),
        }),
        write_mask: convert::color_writes(c.writeMask),
    })
}

unsafe fn constants<'a>(ptr: *const sb::WGPUConstantEntry, count: usize) -> Vec<(&'a str, f64)> {
    unsafe { convert::slice(ptr, count) }
        .iter()
        .map(|entry| {
            unsafe { convert::no_chain(entry.nextInChain, "WGPUConstantEntry") };
            let key = unsafe { convert::str(entry.key) }.expect("pipeline constant without key");
            (key, entry.value)
        })
        .collect()
}

fn compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
) -> wgpu::PipelineCompilationOptions<'a> {
    wgpu::PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory: true,
    }
}
