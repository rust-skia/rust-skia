//! Shader modules and render and compute pipelines.

use std::borrow::Cow;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use skia_bindings as sb;

use super::binding::{BindGroupLayout, PipelineLayout};
use super::convert::{self, CallbackString};
use super::device::Device;
use super::future::{Userdata, ready_or_report_late};
use super::handle::{Handle, get_optional, handles};
use super::instance::Instance;
use super::wgpu;

/// Behind a `WGPUShaderModule`.
#[derive(Debug)]
pub struct ShaderModule {
    pub module: wgpu::ShaderModule,
    label: Option<String>,
    /// The error of the module's creation. Skia learns about invalid WGSL only through the
    /// compilation info, while `wgpu` reports it as an error of the creation.
    creation_error: Option<String>,
    /// The instance that owns the futures of `wgpuShaderModuleGetCompilationInfo`.
    instance: Arc<Instance>,
}

/// Behind a `WGPURenderPipeline`.
#[derive(Debug)]
pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

/// Behind a `WGPUComputePipeline`.
#[derive(Debug)]
pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

handles! {
    ShaderModule => WGPUShaderModule:
        wgpuShaderModuleAddRef: WGPUProcShaderModuleAddRef,
        wgpuShaderModuleRelease: WGPUProcShaderModuleRelease;
    RenderPipeline => WGPURenderPipeline:
        wgpuRenderPipelineAddRef: WGPUProcRenderPipelineAddRef,
        wgpuRenderPipelineRelease: WGPUProcRenderPipelineRelease;
    ComputePipeline => WGPUComputePipeline:
        wgpuComputePipelineAddRef: WGPUProcComputePipelineAddRef,
        wgpuComputePipelineRelease: WGPUProcComputePipelineRelease;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateShaderModule(
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
    // Skia does not open an error scope around the creation, so without these, `wgpu` would pass
    // a WGSL error to its default handler, which panics.
    let label = unsafe { convert::label(d.label) };
    let (module, creation_error) = device.capture_errors("shader module", || {
        device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
            })
    });
    ShaderModule {
        module,
        label: label.map(str::to_owned),
        creation_error: creation_error.map(|error| error.to_string()),
        instance: device.instance.clone(),
    }
    .into_handle()
}
const _: sb::WGPUProcDeviceCreateShaderModule = Some(wgpuDeviceCreateShaderModule);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuShaderModuleGetCompilationInfo(
    shader_module: sb::WGPUShaderModule,
    callback_info: sb::WGPUCompilationInfoCallbackInfo,
) -> sb::WGPUFuture {
    unsafe { convert::no_chain(callback_info.nextInChain, "WGPUCompilationInfoCallbackInfo") };
    let module = unsafe { ShaderModule::get(shader_module) };
    let info = {
        let label = module.label.clone();
        let module = module.module.clone();
        ready_or_report_late(
            async move { module.get_compilation_info().await },
            move |info| late_compilation_errors(label.as_deref(), &info),
        )
    };
    let mut messages: Vec<Message> = info
        .map(|info| info.messages.iter().map(Message::new).collect())
        .unwrap_or_default();
    let has_error = messages.iter().any(|m| {
        m.message_type == sb::WGPUCompilationMessageType::WGPUCompilationMessageType_Error
    });
    if let (Some(error), false) = (&module.creation_error, has_error) {
        messages.push(Message::error(error));
    }
    let callback = callback_info
        .callback
        .expect("null WGPUCompilationInfoCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    let (future, completion) = module
        .instance
        .futures
        .register(callback_info.mode, move |messages| unsafe {
            call_compilation_info_callback(callback, messages, userdata)
        });
    completion.complete(messages);
    future
}
const _: sb::WGPUProcShaderModuleGetCompilationInfo = Some(wgpuShaderModuleGetCompilationInfo);

/// The message for the errors in `info`, for when the browser reports them after Skia has used
/// the shader module `label`.
fn late_compilation_errors(label: Option<&str>, info: &wgpu::CompilationInfo) -> Option<String> {
    let errors: Vec<String> = info
        .messages
        .iter()
        .filter(|m| m.message_type == wgpu::CompilationMessageType::Error)
        .map(|m| match m.location {
            Some(l) => format!("line {}:{} {}", l.line_number, l.line_position, m.message),
            None => m.message.clone(),
        })
        .collect();
    let module = match label {
        Some(label) if !label.is_empty() => format!("the shader module {label:?}"),
        _ => "a shader module".to_owned(),
    };
    (!errors.is_empty()).then(|| {
        format!(
            "skia-wgpu: {module} has errors, after Skia continued with it:\n{}",
            errors.join("\n")
        )
    })
}

/// The C callback of `wgpuShaderModuleGetCompilationInfo`.
type CompilationInfoCallback = unsafe extern "C" fn(
    sb::WGPUCompilationInfoRequestStatus,
    *const sb::WGPUCompilationInfo,
    *mut c_void,
    *mut c_void,
);

/// Calls `callback` with `messages`, or with the status `CallbackCancelled` for `None`.
unsafe fn call_compilation_info_callback(
    callback: CompilationInfoCallback,
    messages: Option<Vec<Message>>,
    userdata: Userdata,
) {
    use sb::WGPUCompilationInfoRequestStatus as S;
    let Some(messages) = messages else {
        let status = S::WGPUCompilationInfoRequestStatus_CallbackCancelled;
        unsafe { callback(status, ptr::null(), userdata.0, userdata.1) };
        return;
    };
    let c_messages: Vec<sb::WGPUCompilationMessage> = messages.iter().map(Message::to_c).collect();
    let info = sb::WGPUCompilationInfo {
        nextInChain: ptr::null_mut(),
        messageCount: c_messages.len(),
        messages: c_messages.as_ptr(),
    };
    let status = S::WGPUCompilationInfoRequestStatus_Success;
    unsafe { callback(status, &info, userdata.0, userdata.1) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipeline(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPURenderPipelineDescriptor,
) -> sb::WGPURenderPipeline {
    let device = unsafe { Device::get(device) };
    let pipeline = unsafe { create_render_pipeline(device, &*descriptor) };
    RenderPipeline { pipeline }.into_handle()
}
const _: sb::WGPUProcDeviceCreateRenderPipeline = Some(wgpuDeviceCreateRenderPipeline);

/// Creates the pipeline at once and reports the result through the callback, with the pipeline's
/// error if there is one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateRenderPipelineAsync(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPURenderPipelineDescriptor,
    callback_info: sb::WGPUCreateRenderPipelineAsyncCallbackInfo,
) -> sb::WGPUFuture {
    unsafe {
        convert::no_chain(
            callback_info.nextInChain,
            "WGPUCreateRenderPipelineAsyncCallbackInfo",
        )
    };
    let device = unsafe { Device::get(device) };
    let (pipeline, error) = device.capture_errors("render pipeline", || unsafe {
        create_render_pipeline(device, &*descriptor)
    });
    let result = match error {
        None => Ok(RenderPipeline { pipeline }),
        Some(error) => Err(error),
    };
    let callback = callback_info
        .callback
        .expect("null WGPUCreateRenderPipelineAsyncCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    let (future, completion) = device
        .instance
        .futures
        .register(callback_info.mode, move |result| unsafe {
            call_render_pipeline_callback(callback, result, userdata)
        });
    completion.complete(result);
    future
}
const _: sb::WGPUProcDeviceCreateRenderPipelineAsync = Some(wgpuDeviceCreateRenderPipelineAsync);

/// The C callback of `wgpuDeviceCreateRenderPipelineAsync`.
type RenderPipelineCallback = unsafe extern "C" fn(
    sb::WGPUCreatePipelineAsyncStatus,
    sb::WGPURenderPipeline,
    sb::WGPUStringView,
    *mut c_void,
    *mut c_void,
);

/// Calls `callback` with the pipeline or the creation error, or with the status `CallbackCancelled`
/// for `None`. The pipeline becomes a handle only here, so a cancelled callback leaks no handle.
unsafe fn call_render_pipeline_callback(
    callback: RenderPipelineCallback,
    result: Option<Result<RenderPipeline, wgpu::Error>>,
    userdata: Userdata,
) {
    use sb::WGPUCreatePipelineAsyncStatus as S;
    let (status, pipeline, message) = match result {
        Some(Ok(pipeline)) => (
            S::WGPUCreatePipelineAsyncStatus_Success,
            pipeline.into_handle(),
            None,
        ),
        Some(Err(error)) => {
            // WebGPU reports an out-of-memory error of a pipeline creation as an internal error.
            let status = match error {
                wgpu::Error::Validation { .. } => S::WGPUCreatePipelineAsyncStatus_ValidationError,
                wgpu::Error::Internal { .. } | wgpu::Error::OutOfMemory { .. } => {
                    S::WGPUCreatePipelineAsyncStatus_InternalError
                }
            };
            let message = CallbackString::new(&error.to_string());
            (status, ptr::null_mut(), Some(message))
        }
        None => (
            S::WGPUCreatePipelineAsyncStatus_CallbackCancelled,
            ptr::null_mut(),
            None,
        ),
    };
    let message = message
        .as_ref()
        .map_or_else(convert::empty_string, CallbackString::view);
    unsafe { callback(status, pipeline, message, userdata.0, userdata.1) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateComputePipeline(
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
const _: sb::WGPUProcDeviceCreateComputePipeline = Some(wgpuDeviceCreateComputePipeline);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPipelineGetBindGroupLayout(
    pipeline: sb::WGPURenderPipeline,
    group_index: u32,
) -> sb::WGPUBindGroupLayout {
    let pipeline = unsafe { RenderPipeline::get(pipeline) };
    BindGroupLayout {
        layout: pipeline.pipeline.get_bind_group_layout(group_index),
    }
    .into_handle()
}
const _: sb::WGPUProcRenderPipelineGetBindGroupLayout = Some(wgpuRenderPipelineGetBindGroupLayout);

/// A compilation message that the callback can read during its call.
#[derive(Debug)]
struct Message {
    text: CallbackString,
    message_type: sb::WGPUCompilationMessageType,
    location: wgpu::SourceLocation,
}

impl Message {
    /// The location of a message that refers to no part of the source.
    const NO_LOCATION: wgpu::SourceLocation = wgpu::SourceLocation {
        line_number: 0,
        line_position: 0,
        offset: 0,
        length: 0,
    };

    fn new(message: &wgpu::CompilationMessage) -> Self {
        use sb::WGPUCompilationMessageType as T;
        Self {
            text: CallbackString::new(&message.message),
            message_type: match message.message_type {
                wgpu::CompilationMessageType::Error => T::WGPUCompilationMessageType_Error,
                wgpu::CompilationMessageType::Warning => T::WGPUCompilationMessageType_Warning,
                wgpu::CompilationMessageType::Info => T::WGPUCompilationMessageType_Info,
            },
            location: message.location.unwrap_or(Self::NO_LOCATION),
        }
    }

    /// An error without a source location.
    fn error(text: &str) -> Self {
        Self {
            text: CallbackString::new(text),
            message_type: sb::WGPUCompilationMessageType::WGPUCompilationMessageType_Error,
            location: Self::NO_LOCATION,
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

#[cfg(test)]
mod tests {
    use super::{late_compilation_errors, wgpu};

    fn message(
        message_type: wgpu::CompilationMessageType,
        message: &str,
        line: Option<(u32, u32)>,
    ) -> wgpu::CompilationMessage {
        wgpu::CompilationMessage {
            message: message.to_owned(),
            message_type,
            location: line.map(|(line_number, line_position)| wgpu::SourceLocation {
                line_number,
                line_position,
                offset: 0,
                length: 0,
            }),
        }
    }

    #[test]
    fn late_compilation_errors_reports_only_errors() {
        use wgpu::CompilationMessageType as T;
        let info = wgpu::CompilationInfo {
            messages: vec![
                message(T::Warning, "unused variable", Some((1, 2))),
                message(T::Error, "unknown type", Some((3, 4))),
                message(T::Error, "no location", None),
            ],
        };
        assert_eq!(
            late_compilation_errors(Some("fs"), &info).unwrap(),
            "skia-wgpu: the shader module \"fs\" has errors, after Skia continued with it:\n\
             line 3:4 unknown type\n\
             no location"
        );
    }

    #[test]
    fn late_compilation_errors_names_no_label_that_is_empty() {
        use wgpu::CompilationMessageType as T;
        let info = wgpu::CompilationInfo {
            messages: vec![message(T::Error, "unknown type", None)],
        };
        let expected = "skia-wgpu: a shader module has errors, after Skia continued with it:\n\
                        unknown type";
        assert_eq!(late_compilation_errors(Some(""), &info).unwrap(), expected);
        assert_eq!(late_compilation_errors(None, &info).unwrap(), expected);
    }

    #[test]
    fn late_compilation_errors_reports_nothing_without_errors() {
        use wgpu::CompilationMessageType as T;
        let info = wgpu::CompilationInfo {
            messages: vec![message(T::Warning, "unused variable", None)],
        };
        assert_eq!(late_compilation_errors(None, &info), None);
    }
}
