//! Command encoders, command buffers, render and compute passes, and query sets.
//!
//! A `wgpu` pass borrows its encoder until the pass ends, while WebGPU gives Skia independent
//! handles to both. `forget_lifetime` removes the borrow, and `wgpu` then checks at runtime that
//! the encoder is not used while a pass is open, as WebGPU requires anyway.

use std::sync::Mutex;

use skia_bindings as sb;

use super::binding::BindGroup;
use super::buffer::Buffer;
use super::device::Device;
use super::handle::{Handle, get_optional, handles};
use super::pipeline::{ComputePipeline, RenderPipeline};
use super::texture::{Texture, TextureView};
use super::{compat, convert, wgpu};

// Each `Option` turns `None` when Finish, End, or Submit consumes the `wgpu` object, while the
// WebGPU handle lives on until Release.

/// Behind a `WGPUCommandEncoder`: the encoder until `wgpuCommandEncoderFinish` takes it.
#[derive(Debug)]
pub struct CommandEncoder {
    encoder: Mutex<Option<wgpu::CommandEncoder>>,
}

/// Behind a `WGPUCommandBuffer`: the command buffer until `wgpuQueueSubmit` takes it.
#[derive(Debug)]
pub struct CommandBuffer {
    buffer: Mutex<Option<wgpu::CommandBuffer>>,
}

/// Behind a `WGPURenderPassEncoder`: the pass until `wgpuRenderPassEncoderEnd` ends it.
#[derive(Debug)]
pub struct RenderPassEncoder {
    pass: Mutex<Option<compat::RenderPass>>,
}

/// Behind a `WGPUComputePassEncoder`: the pass until `wgpuComputePassEncoderEnd` ends it.
#[derive(Debug)]
pub struct ComputePassEncoder {
    pass: Mutex<Option<wgpu::ComputePass<'static>>>,
}

/// Behind a `WGPUQuerySet`.
#[derive(Debug)]
pub struct QuerySet {
    set: wgpu::QuerySet,
}

handles! {
    CommandEncoder => WGPUCommandEncoder:
        wgpuCommandEncoderAddRef: WGPUProcCommandEncoderAddRef,
        wgpuCommandEncoderRelease: WGPUProcCommandEncoderRelease;
    CommandBuffer => WGPUCommandBuffer:
        wgpuCommandBufferAddRef: WGPUProcCommandBufferAddRef,
        wgpuCommandBufferRelease: WGPUProcCommandBufferRelease;
    RenderPassEncoder => WGPURenderPassEncoder:
        wgpuRenderPassEncoderAddRef: WGPUProcRenderPassEncoderAddRef,
        wgpuRenderPassEncoderRelease: WGPUProcRenderPassEncoderRelease;
    ComputePassEncoder => WGPUComputePassEncoder:
        wgpuComputePassEncoderAddRef: WGPUProcComputePassEncoderAddRef,
        wgpuComputePassEncoderRelease: WGPUProcComputePassEncoderRelease;
    QuerySet => WGPUQuerySet:
        wgpuQuerySetAddRef: WGPUProcQuerySetAddRef,
        wgpuQuerySetRelease: WGPUProcQuerySetRelease;
}

impl CommandBuffer {
    /// Takes the command buffer for submission. WebGPU submits a command buffer only once.
    pub fn take(&self) -> wgpu::CommandBuffer {
        self.buffer
            .lock()
            .unwrap()
            .take()
            .expect("command buffer submitted twice")
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateCommandEncoder(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUCommandEncoderDescriptor,
) -> sb::WGPUCommandEncoder {
    let device = unsafe { Device::get(device) };
    let label = unsafe { descriptor.as_ref() }.and_then(|d| {
        unsafe { convert::no_chain(d.nextInChain, "WGPUCommandEncoderDescriptor") };
        unsafe { convert::label(d.label) }
    });
    let encoder = device
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    CommandEncoder {
        encoder: Mutex::new(Some(encoder)),
    }
    .into_handle()
}
const _: sb::WGPUProcDeviceCreateCommandEncoder = Some(wgpuDeviceCreateCommandEncoder);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateQuerySet(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUQuerySetDescriptor,
) -> sb::WGPUQuerySet {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUQuerySetDescriptor") };
    let set = device.device.create_query_set(&wgpu::QuerySetDescriptor {
        label: unsafe { convert::label(d.label) },
        ty: convert::query_type(d.type_),
        count: d.count,
    });
    QuerySet { set }.into_handle()
}
const _: sb::WGPUProcDeviceCreateQuerySet = Some(wgpuDeviceCreateQuerySet);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderFinish(
    encoder: sb::WGPUCommandEncoder,
    descriptor: *const sb::WGPUCommandBufferDescriptor,
) -> sb::WGPUCommandBuffer {
    if let Some(d) = unsafe { descriptor.as_ref() } {
        unsafe { convert::no_chain(d.nextInChain, "WGPUCommandBufferDescriptor") };
    }
    let encoder = unsafe { CommandEncoder::get(encoder) }
        .encoder
        .lock()
        .unwrap()
        .take()
        .expect("command encoder finished twice");
    CommandBuffer {
        buffer: Mutex::new(Some(encoder.finish())),
    }
    .into_handle()
}
const _: sb::WGPUProcCommandEncoderFinish = Some(wgpuCommandEncoderFinish);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderBeginRenderPass(
    encoder: sb::WGPUCommandEncoder,
    descriptor: *const sb::WGPURenderPassDescriptor,
) -> sb::WGPURenderPassEncoder {
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPURenderPassDescriptor") };
    let color_attachments: Vec<Option<wgpu::RenderPassColorAttachment>> =
        unsafe { convert::slice(d.colorAttachments, d.colorAttachmentCount) }
            .iter()
            .map(|attachment| unsafe { color_attachment(attachment) })
            .collect();
    let depth_stencil_attachment = unsafe { d.depthStencilAttachment.as_ref() }
        .map(|attachment| unsafe { depth_stencil_attachment(attachment) });
    let timestamp_writes = unsafe { d.timestampWrites.as_ref() }.map(|writes| {
        let (query_set, beginning, end) = unsafe { timestamp_writes(writes) };
        wgpu::RenderPassTimestampWrites {
            query_set,
            beginning_of_pass_write_index: beginning,
            end_of_pass_write_index: end,
        }
    });
    let occlusion_query_set =
        unsafe { get_optional::<QuerySet, _>(d.occlusionQuerySet) }.map(|q| &q.set);
    let pass = unsafe { CommandEncoder::get(encoder) }.with(|encoder| {
        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: unsafe { convert::label(d.label) },
                color_attachments: &color_attachments,
                depth_stencil_attachment,
                timestamp_writes,
                occlusion_query_set,
                multiview_mask: None,
            })
            .forget_lifetime()
    });
    RenderPassEncoder {
        pass: Mutex::new(Some(pass.into())),
    }
    .into_handle()
}
const _: sb::WGPUProcCommandEncoderBeginRenderPass = Some(wgpuCommandEncoderBeginRenderPass);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderBeginComputePass(
    encoder: sb::WGPUCommandEncoder,
    descriptor: *const sb::WGPUComputePassDescriptor,
) -> sb::WGPUComputePassEncoder {
    let d = unsafe { descriptor.as_ref() };
    if let Some(d) = d {
        unsafe { convert::no_chain(d.nextInChain, "WGPUComputePassDescriptor") };
    }
    let timestamp_writes = d
        .and_then(|d| unsafe { d.timestampWrites.as_ref() })
        .map(|writes| {
            let (query_set, beginning, end) = unsafe { timestamp_writes(writes) };
            wgpu::ComputePassTimestampWrites {
                query_set,
                beginning_of_pass_write_index: beginning,
                end_of_pass_write_index: end,
            }
        });
    let label = d.and_then(|d| unsafe { convert::label(d.label) });
    let pass = unsafe { CommandEncoder::get(encoder) }.with(|encoder| {
        encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label,
                timestamp_writes,
            })
            .forget_lifetime()
    });
    ComputePassEncoder {
        pass: Mutex::new(Some(pass)),
    }
    .into_handle()
}
const _: sb::WGPUProcCommandEncoderBeginComputePass = Some(wgpuCommandEncoderBeginComputePass);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderClearBuffer(
    encoder: sb::WGPUCommandEncoder,
    buffer: sb::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let buffer = &unsafe { Buffer::get(buffer) }.buffer;
    unsafe { CommandEncoder::get(encoder) }
        .with(|encoder| encoder.clear_buffer(buffer, offset, convert::size(size)));
}
const _: sb::WGPUProcCommandEncoderClearBuffer = Some(wgpuCommandEncoderClearBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToBuffer(
    encoder: sb::WGPUCommandEncoder,
    source: sb::WGPUBuffer,
    source_offset: u64,
    destination: sb::WGPUBuffer,
    destination_offset: u64,
    size: u64,
) {
    let source = &unsafe { Buffer::get(source) }.buffer;
    let destination = &unsafe { Buffer::get(destination) }.buffer;
    unsafe { CommandEncoder::get(encoder) }.with(|encoder| {
        encoder.copy_buffer_to_buffer(
            source,
            source_offset,
            destination,
            destination_offset,
            convert::size(size),
        )
    });
}
const _: sb::WGPUProcCommandEncoderCopyBufferToBuffer = Some(wgpuCommandEncoderCopyBufferToBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    encoder: sb::WGPUCommandEncoder,
    source: *const sb::WGPUTexelCopyBufferInfo,
    destination: *const sb::WGPUTexelCopyTextureInfo,
    copy_size: *const sb::WGPUExtent3D,
) {
    let source = unsafe { buffer_copy(&*source) };
    let destination = unsafe { texture_copy(&*destination) };
    let size = convert::extent(unsafe { &*copy_size });
    unsafe { CommandEncoder::get(encoder) }
        .with(|encoder| encoder.copy_buffer_to_texture(source, destination, size));
}
const _: sb::WGPUProcCommandEncoderCopyBufferToTexture =
    Some(wgpuCommandEncoderCopyBufferToTexture);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    encoder: sb::WGPUCommandEncoder,
    source: *const sb::WGPUTexelCopyTextureInfo,
    destination: *const sb::WGPUTexelCopyBufferInfo,
    copy_size: *const sb::WGPUExtent3D,
) {
    let source = unsafe { texture_copy(&*source) };
    let destination = unsafe { buffer_copy(&*destination) };
    let size = convert::extent(unsafe { &*copy_size });
    unsafe { CommandEncoder::get(encoder) }
        .with(|encoder| encoder.copy_texture_to_buffer(source, destination, size));
}
const _: sb::WGPUProcCommandEncoderCopyTextureToBuffer =
    Some(wgpuCommandEncoderCopyTextureToBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderCopyTextureToTexture(
    encoder: sb::WGPUCommandEncoder,
    source: *const sb::WGPUTexelCopyTextureInfo,
    destination: *const sb::WGPUTexelCopyTextureInfo,
    copy_size: *const sb::WGPUExtent3D,
) {
    let source = unsafe { texture_copy(&*source) };
    let destination = unsafe { texture_copy(&*destination) };
    let size = convert::extent(unsafe { &*copy_size });
    unsafe { CommandEncoder::get(encoder) }
        .with(|encoder| encoder.copy_texture_to_texture(source, destination, size));
}
const _: sb::WGPUProcCommandEncoderCopyTextureToTexture =
    Some(wgpuCommandEncoderCopyTextureToTexture);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderResolveQuerySet(
    encoder: sb::WGPUCommandEncoder,
    query_set: sb::WGPUQuerySet,
    first_query: u32,
    query_count: u32,
    destination: sb::WGPUBuffer,
    destination_offset: u64,
) {
    let query_set = &unsafe { QuerySet::get(query_set) }.set;
    let destination = &unsafe { Buffer::get(destination) }.buffer;
    unsafe { CommandEncoder::get(encoder) }.with(|encoder| {
        encoder.resolve_query_set(
            query_set,
            first_query..first_query + query_count,
            destination,
            destination_offset,
        )
    });
}
const _: sb::WGPUProcCommandEncoderResolveQuerySet = Some(wgpuCommandEncoderResolveQuerySet);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuCommandEncoderWriteTimestamp(
    encoder: sb::WGPUCommandEncoder,
    query_set: sb::WGPUQuerySet,
    query_index: u32,
) {
    let query_set = &unsafe { QuerySet::get(query_set) }.set;
    unsafe { CommandEncoder::get(encoder) }
        .with(|encoder| encoder.write_timestamp(query_set, query_index));
}
const _: sb::WGPUProcCommandEncoderWriteTimestamp = Some(wgpuCommandEncoderWriteTimestamp);

//
// Render passes
//

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderEnd(pass: sb::WGPURenderPassEncoder) {
    // Dropping the `wgpu` pass ends it.
    let pass = unsafe { RenderPassEncoder::get(pass) }
        .pass
        .lock()
        .unwrap()
        .take();
    drop(pass.expect("render pass ended twice"));
}
const _: sb::WGPUProcRenderPassEncoderEnd = Some(wgpuRenderPassEncoderEnd);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetPipeline(
    pass: sb::WGPURenderPassEncoder,
    pipeline: sb::WGPURenderPipeline,
) {
    let pipeline = &unsafe { RenderPipeline::get(pipeline) }.pipeline;
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| pass.set_pipeline(pipeline));
}
const _: sb::WGPUProcRenderPassEncoderSetPipeline = Some(wgpuRenderPassEncoderSetPipeline);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    pass: sb::WGPURenderPassEncoder,
    group_index: u32,
    group: sb::WGPUBindGroup,
    dynamic_offset_count: usize,
    dynamic_offsets: *const u32,
) {
    let group = unsafe { get_optional::<BindGroup, _>(group) }.map(|g| &g.group);
    let offsets = unsafe { convert::slice(dynamic_offsets, dynamic_offset_count) };
    unsafe { RenderPassEncoder::get(pass) }
        .with(|pass| pass.set_bind_group(group_index, group, offsets));
}
const _: sb::WGPUProcRenderPassEncoderSetBindGroup = Some(wgpuRenderPassEncoderSetBindGroup);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    pass: sb::WGPURenderPassEncoder,
    slot: u32,
    buffer: sb::WGPUBuffer,
    offset: u64,
    size: u64,
) {
    let buffer = unsafe { get_optional::<Buffer, _>(buffer) };
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| {
        pass.set_vertex_buffer(slot, buffer.map(|b| buffer_slice(&b.buffer, offset, size)))
    });
}
const _: sb::WGPUProcRenderPassEncoderSetVertexBuffer = Some(wgpuRenderPassEncoderSetVertexBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    pass: sb::WGPURenderPassEncoder,
    buffer: sb::WGPUBuffer,
    format: sb::WGPUIndexFormat,
    offset: u64,
    size: u64,
) {
    let buffer = &unsafe { Buffer::get(buffer) }.buffer;
    let format = convert::index_format(format).expect("index buffer without format");
    unsafe { RenderPassEncoder::get(pass) }
        .with(|pass| pass.set_index_buffer(buffer_slice(buffer, offset, size), format));
}
const _: sb::WGPUProcRenderPassEncoderSetIndexBuffer = Some(wgpuRenderPassEncoderSetIndexBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetViewport(
    pass: sb::WGPURenderPassEncoder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
) {
    unsafe { RenderPassEncoder::get(pass) }
        .with(|pass| pass.set_viewport(x, y, width, height, min_depth, max_depth));
}
const _: sb::WGPUProcRenderPassEncoderSetViewport = Some(wgpuRenderPassEncoderSetViewport);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    pass: sb::WGPURenderPassEncoder,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| pass.set_scissor_rect(x, y, width, height));
}
const _: sb::WGPUProcRenderPassEncoderSetScissorRect = Some(wgpuRenderPassEncoderSetScissorRect);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    pass: sb::WGPURenderPassEncoder,
    color: *const sb::WGPUColor,
) {
    let color = convert::color(unsafe { &*color });
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| pass.set_blend_constant(color));
}
const _: sb::WGPUProcRenderPassEncoderSetBlendConstant =
    Some(wgpuRenderPassEncoderSetBlendConstant);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderSetImmediates(
    pass: sb::WGPURenderPassEncoder,
    offset: u32,
    data: *const std::ffi::c_void,
    size: usize,
) {
    let data = unsafe { convert::slice(data as *const u8, size) };
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| pass.set_immediates(offset, data));
}
const _: sb::WGPUProcRenderPassEncoderSetImmediates = Some(wgpuRenderPassEncoderSetImmediates);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderDraw(
    pass: sb::WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| {
        pass.draw(
            first_vertex..first_vertex + vertex_count,
            first_instance..first_instance + instance_count,
        )
    });
}
const _: sb::WGPUProcRenderPassEncoderDraw = Some(wgpuRenderPassEncoderDraw);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexed(
    pass: sb::WGPURenderPassEncoder,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) {
    unsafe { RenderPassEncoder::get(pass) }.with(|pass| {
        pass.draw_indexed(
            first_index..first_index + index_count,
            base_vertex,
            first_instance..first_instance + instance_count,
        )
    });
}
const _: sb::WGPUProcRenderPassEncoderDrawIndexed = Some(wgpuRenderPassEncoderDrawIndexed);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndirect(
    pass: sb::WGPURenderPassEncoder,
    indirect_buffer: sb::WGPUBuffer,
    indirect_offset: u64,
) {
    let buffer = &unsafe { Buffer::get(indirect_buffer) }.buffer;
    unsafe { RenderPassEncoder::get(pass) }
        .with(|pass| pass.draw_indirect(buffer, indirect_offset));
}
const _: sb::WGPUProcRenderPassEncoderDrawIndirect = Some(wgpuRenderPassEncoderDrawIndirect);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    pass: sb::WGPURenderPassEncoder,
    indirect_buffer: sb::WGPUBuffer,
    indirect_offset: u64,
) {
    let buffer = &unsafe { Buffer::get(indirect_buffer) }.buffer;
    unsafe { RenderPassEncoder::get(pass) }
        .with(|pass| pass.draw_indexed_indirect(buffer, indirect_offset));
}
const _: sb::WGPUProcRenderPassEncoderDrawIndexedIndirect =
    Some(wgpuRenderPassEncoderDrawIndexedIndirect);

//
// Compute passes
//

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuComputePassEncoderEnd(pass: sb::WGPUComputePassEncoder) {
    let pass = unsafe { ComputePassEncoder::get(pass) }
        .pass
        .lock()
        .unwrap()
        .take();
    drop(pass.expect("compute pass ended twice"));
}
const _: sb::WGPUProcComputePassEncoderEnd = Some(wgpuComputePassEncoderEnd);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuComputePassEncoderSetPipeline(
    pass: sb::WGPUComputePassEncoder,
    pipeline: sb::WGPUComputePipeline,
) {
    let pipeline = &unsafe { ComputePipeline::get(pipeline) }.pipeline;
    unsafe { ComputePassEncoder::get(pass) }.with(|pass| pass.set_pipeline(pipeline));
}
const _: sb::WGPUProcComputePassEncoderSetPipeline = Some(wgpuComputePassEncoderSetPipeline);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuComputePassEncoderSetBindGroup(
    pass: sb::WGPUComputePassEncoder,
    group_index: u32,
    group: sb::WGPUBindGroup,
    dynamic_offset_count: usize,
    dynamic_offsets: *const u32,
) {
    let group = unsafe { get_optional::<BindGroup, _>(group) }.map(|g| &g.group);
    let offsets = unsafe { convert::slice(dynamic_offsets, dynamic_offset_count) };
    unsafe { ComputePassEncoder::get(pass) }
        .with(|pass| pass.set_bind_group(group_index, group, offsets));
}
const _: sb::WGPUProcComputePassEncoderSetBindGroup = Some(wgpuComputePassEncoderSetBindGroup);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroups(
    pass: sb::WGPUComputePassEncoder,
    x: u32,
    y: u32,
    z: u32,
) {
    unsafe { ComputePassEncoder::get(pass) }.with(|pass| pass.dispatch_workgroups(x, y, z));
}
const _: sb::WGPUProcComputePassEncoderDispatchWorkgroups =
    Some(wgpuComputePassEncoderDispatchWorkgroups);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuComputePassEncoderDispatchWorkgroupsIndirect(
    pass: sb::WGPUComputePassEncoder,
    indirect_buffer: sb::WGPUBuffer,
    indirect_offset: u64,
) {
    let buffer = &unsafe { Buffer::get(indirect_buffer) }.buffer;
    unsafe { ComputePassEncoder::get(pass) }
        .with(|pass| pass.dispatch_workgroups_indirect(buffer, indirect_offset));
}
const _: sb::WGPUProcComputePassEncoderDispatchWorkgroupsIndirect =
    Some(wgpuComputePassEncoderDispatchWorkgroupsIndirect);

impl CommandEncoder {
    /// Runs `f` with the encoder, which exists until `wgpuCommandEncoderFinish`.
    fn with<R>(&self, f: impl FnOnce(&mut wgpu::CommandEncoder) -> R) -> R {
        let mut encoder = self.encoder.lock().unwrap();
        f(encoder.as_mut().expect("command encoder used after Finish"))
    }
}

/// Converts a color attachment. An attachment without a view is an unused slot.
unsafe fn color_attachment<'a>(
    c: &sb::WGPURenderPassColorAttachment,
) -> Option<wgpu::RenderPassColorAttachment<'a>> {
    unsafe { convert::no_chain(c.nextInChain, "WGPURenderPassColorAttachment") };
    let view = unsafe { get_optional::<TextureView, _>(c.view) }?;
    Some(wgpu::RenderPassColorAttachment {
        view: &view.view,
        depth_slice: convert::count(c.depthSlice),
        resolve_target: unsafe { get_optional::<TextureView, _>(c.resolveTarget) }.map(|v| &v.view),
        ops: operations(c.loadOp, c.storeOp, convert::color(&c.clearValue))
            .expect("color attachment without load and store operations"),
    })
}

unsafe fn depth_stencil_attachment<'a>(
    c: &sb::WGPURenderPassDepthStencilAttachment,
) -> wgpu::RenderPassDepthStencilAttachment<'a> {
    unsafe { convert::no_chain(c.nextInChain, "WGPURenderPassDepthStencilAttachment") };
    // A read-only aspect, or one the format lacks, has undefined operations.
    let depth_ops = if convert::bool(c.depthReadOnly) {
        None
    } else {
        operations(c.depthLoadOp, c.depthStoreOp, c.depthClearValue)
    };
    let stencil_ops = if convert::bool(c.stencilReadOnly) {
        None
    } else {
        operations(c.stencilLoadOp, c.stencilStoreOp, c.stencilClearValue)
    };
    wgpu::RenderPassDepthStencilAttachment {
        view: &unsafe { TextureView::get(c.view) }.view,
        depth_ops,
        stencil_ops,
    }
}

/// Converts load and store operations. Both undefined means the attachment aspect is not used.
fn operations<V>(
    load: sb::WGPULoadOp,
    store: sb::WGPUStoreOp,
    clear: V,
) -> Option<wgpu::Operations<V>> {
    use sb::WGPULoadOp as L;
    use sb::WGPUStoreOp as S;
    if load == L::WGPULoadOp_Undefined && store == S::WGPUStoreOp_Undefined {
        return None;
    }
    let load = match load {
        L::WGPULoadOp_Load => wgpu::LoadOp::Load,
        L::WGPULoadOp_Clear => wgpu::LoadOp::Clear(clear),
        // `ExpandResolveTexture` needs Dawn's `DawnLoadResolveTexture`, which the shim reports as
        // unsupported.
        other @ (L::WGPULoadOp_Undefined
        | L::WGPULoadOp_ExpandResolveTexture
        | L::WGPULoadOp_Force32) => panic!("unsupported {other:?}"),
    };
    let store = match store {
        S::WGPUStoreOp_Store => wgpu::StoreOp::Store,
        S::WGPUStoreOp_Discard => wgpu::StoreOp::Discard,
        other @ (S::WGPUStoreOp_Undefined | S::WGPUStoreOp_Force32) => {
            panic!("unsupported {other:?}")
        }
    };
    Some(wgpu::Operations { load, store })
}

unsafe fn timestamp_writes<'a>(
    c: &sb::WGPUPassTimestampWrites,
) -> (&'a wgpu::QuerySet, Option<u32>, Option<u32>) {
    unsafe { convert::no_chain(c.nextInChain, "WGPUPassTimestampWrites") };
    (
        &unsafe { QuerySet::get(c.querySet) }.set,
        convert::count(c.beginningOfPassWriteIndex),
        convert::count(c.endOfPassWriteIndex),
    )
}

unsafe fn buffer_copy<'a>(c: &sb::WGPUTexelCopyBufferInfo) -> wgpu::TexelCopyBufferInfo<'a> {
    wgpu::TexelCopyBufferInfo {
        buffer: &unsafe { Buffer::get(c.buffer) }.buffer,
        layout: wgpu::TexelCopyBufferLayout {
            offset: c.layout.offset,
            bytes_per_row: convert::count(c.layout.bytesPerRow),
            rows_per_image: convert::count(c.layout.rowsPerImage),
        },
    }
}

unsafe fn texture_copy<'a>(c: &sb::WGPUTexelCopyTextureInfo) -> wgpu::TexelCopyTextureInfo<'a> {
    wgpu::TexelCopyTextureInfo {
        texture: &unsafe { Texture::get(c.texture) }.texture,
        mip_level: c.mipLevel,
        origin: wgpu::Origin3d {
            x: c.origin.x,
            y: c.origin.y,
            z: c.origin.z,
        },
        aspect: convert::texture_aspect(c.aspect),
    }
}

impl RenderPassEncoder {
    fn with<R>(&self, f: impl FnOnce(&mut compat::RenderPass) -> R) -> R {
        let mut pass = self.pass.lock().unwrap();
        f(pass.as_mut().expect("render pass used after End"))
    }
}

fn buffer_slice(buffer: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BufferSlice<'_> {
    match convert::size(size) {
        Some(size) => buffer.slice(offset..offset + size),
        None => buffer.slice(offset..),
    }
}

impl ComputePassEncoder {
    fn with<R>(&self, f: impl FnOnce(&mut wgpu::ComputePass<'static>) -> R) -> R {
        let mut pass = self.pass.lock().unwrap();
        f(pass.as_mut().expect("compute pass used after End"))
    }
}
