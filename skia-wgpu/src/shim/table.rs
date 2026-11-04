//! Dawn's function table, filled with the shim's functions, and its installation.

use std::sync::Once;

use skia_bindings as sb;

use super::{binding, buffer, device, encoder, instance, pipeline, queue, texture};

/// Installs the shim's function table for the whole process, once.
///
/// `dawnProcSetProcs` checks that the table's revision stamp matches the one that `dawn_proc.cpp`
/// was generated with, so the table takes the stamp from `dawnProcGetVersion`.
pub fn install() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| unsafe { sb::dawnProcSetProcs(&table()) });
}

/// The table of the functions that Skia's Dawn backend calls.
///
/// The entries of all other functions stay null, and a call to one of them crashes. ADR 0003
/// replaces this with a struct literal that names every entry, so that a function added to Dawn's
/// header fails to compile until the shim handles it.
fn table() -> sb::DawnProcTable {
    let mut t: sb::DawnProcTable = unsafe { std::mem::zeroed() };
    let version = unsafe { std::slice::from_raw_parts(sb::dawnProcGetVersion(), t.version.len()) };
    t.version.copy_from_slice(version);

    t.instanceAddRef = Some(instance::instance_add_ref);
    t.instanceRelease = Some(instance::instance_release);
    t.instanceWaitAny = Some(instance::instance_wait_any);
    t.instanceProcessEvents = Some(instance::instance_process_events);
    t.adapterAddRef = Some(instance::adapter_add_ref);
    t.adapterRelease = Some(instance::adapter_release);
    t.adapterGetInfo = Some(instance::adapter_get_info);
    t.adapterInfoFreeMembers = Some(instance::adapter_info_free_members);

    t.deviceAddRef = Some(device::device_add_ref);
    t.deviceRelease = Some(device::device_release);
    t.deviceGetAdapter = Some(device::device_get_adapter);
    t.deviceGetLimits = Some(device::device_get_limits);
    t.deviceHasFeature = Some(device::device_has_feature);
    t.devicePushErrorScope = Some(device::device_push_error_scope);
    t.devicePopErrorScope = Some(device::device_pop_error_scope);
    t.deviceTick = Some(device::device_tick);
    t.deviceCreateBuffer = Some(buffer::device_create_buffer);
    t.deviceCreateTexture = Some(texture::device_create_texture);
    t.deviceCreateSampler = Some(texture::device_create_sampler);
    t.deviceCreateBindGroupLayout = Some(binding::device_create_bind_group_layout);
    t.deviceCreateBindGroup = Some(binding::device_create_bind_group);
    t.deviceCreatePipelineLayout = Some(binding::device_create_pipeline_layout);
    t.deviceCreateShaderModule = Some(pipeline::device_create_shader_module);
    t.deviceCreateRenderPipeline = Some(pipeline::device_create_render_pipeline);
    t.deviceCreateRenderPipelineAsync = Some(pipeline::device_create_render_pipeline_async);
    t.deviceCreateComputePipeline = Some(pipeline::device_create_compute_pipeline);
    t.deviceCreateCommandEncoder = Some(encoder::device_create_command_encoder);
    t.deviceCreateQuerySet = Some(encoder::device_create_query_set);

    t.queueAddRef = Some(queue::queue_add_ref);
    t.queueRelease = Some(queue::queue_release);
    t.queueSubmit = Some(queue::queue_submit);
    t.queueWriteBuffer = Some(queue::queue_write_buffer);
    t.queueOnSubmittedWorkDone = Some(queue::queue_on_submitted_work_done);

    t.bufferAddRef = Some(buffer::buffer_add_ref);
    t.bufferRelease = Some(buffer::buffer_release);
    t.bufferMapAsync = Some(buffer::buffer_map_async);
    t.bufferGetMappedRange = Some(buffer::buffer_get_mapped_range);
    t.bufferGetConstMappedRange = Some(buffer::buffer_get_const_mapped_range);
    t.bufferUnmap = Some(buffer::buffer_unmap);
    t.bufferGetMapState = Some(buffer::buffer_get_map_state);
    t.bufferGetSize = Some(buffer::buffer_get_size);
    t.bufferGetUsage = Some(buffer::buffer_get_usage);
    t.bufferDestroy = Some(buffer::buffer_destroy);
    t.bufferSetLabel = Some(buffer::buffer_set_label);

    t.textureAddRef = Some(texture::texture_add_ref);
    t.textureRelease = Some(texture::texture_release);
    t.textureCreateView = Some(texture::texture_create_view);
    t.textureGetFormat = Some(texture::texture_get_format);
    t.textureGetWidth = Some(texture::texture_get_width);
    t.textureGetHeight = Some(texture::texture_get_height);
    t.textureGetMipLevelCount = Some(texture::texture_get_mip_level_count);
    t.textureGetSampleCount = Some(texture::texture_get_sample_count);
    t.textureGetUsage = Some(texture::texture_get_usage);
    t.textureDestroy = Some(texture::texture_destroy);
    t.textureSetLabel = Some(texture::texture_set_label);
    t.textureViewAddRef = Some(texture::texture_view_add_ref);
    t.textureViewRelease = Some(texture::texture_view_release);
    t.textureViewSetLabel = Some(texture::texture_view_set_label);
    t.samplerAddRef = Some(texture::sampler_add_ref);
    t.samplerRelease = Some(texture::sampler_release);

    t.bindGroupLayoutAddRef = Some(binding::bind_group_layout_add_ref);
    t.bindGroupLayoutRelease = Some(binding::bind_group_layout_release);
    t.bindGroupAddRef = Some(binding::bind_group_add_ref);
    t.bindGroupRelease = Some(binding::bind_group_release);
    t.pipelineLayoutAddRef = Some(binding::pipeline_layout_add_ref);
    t.pipelineLayoutRelease = Some(binding::pipeline_layout_release);

    t.shaderModuleAddRef = Some(pipeline::shader_module_add_ref);
    t.shaderModuleRelease = Some(pipeline::shader_module_release);
    t.shaderModuleGetCompilationInfo = Some(pipeline::shader_module_get_compilation_info);
    t.renderPipelineAddRef = Some(pipeline::render_pipeline_add_ref);
    t.renderPipelineRelease = Some(pipeline::render_pipeline_release);
    t.renderPipelineGetBindGroupLayout = Some(pipeline::render_pipeline_get_bind_group_layout);
    t.computePipelineAddRef = Some(pipeline::compute_pipeline_add_ref);
    t.computePipelineRelease = Some(pipeline::compute_pipeline_release);

    t.commandEncoderAddRef = Some(encoder::command_encoder_add_ref);
    t.commandEncoderRelease = Some(encoder::command_encoder_release);
    t.commandEncoderFinish = Some(encoder::command_encoder_finish);
    t.commandEncoderBeginRenderPass = Some(encoder::command_encoder_begin_render_pass);
    t.commandEncoderBeginComputePass = Some(encoder::command_encoder_begin_compute_pass);
    t.commandEncoderClearBuffer = Some(encoder::command_encoder_clear_buffer);
    t.commandEncoderCopyBufferToBuffer = Some(encoder::command_encoder_copy_buffer_to_buffer);
    t.commandEncoderCopyBufferToTexture = Some(encoder::command_encoder_copy_buffer_to_texture);
    t.commandEncoderCopyTextureToBuffer = Some(encoder::command_encoder_copy_texture_to_buffer);
    t.commandEncoderCopyTextureToTexture = Some(encoder::command_encoder_copy_texture_to_texture);
    t.commandEncoderResolveQuerySet = Some(encoder::command_encoder_resolve_query_set);
    t.commandEncoderWriteTimestamp = Some(encoder::command_encoder_write_timestamp);
    t.commandBufferAddRef = Some(encoder::command_buffer_add_ref);
    t.commandBufferRelease = Some(encoder::command_buffer_release);
    t.querySetAddRef = Some(encoder::query_set_add_ref);
    t.querySetRelease = Some(encoder::query_set_release);

    t.renderPassEncoderAddRef = Some(encoder::render_pass_encoder_add_ref);
    t.renderPassEncoderRelease = Some(encoder::render_pass_encoder_release);
    t.renderPassEncoderEnd = Some(encoder::render_pass_encoder_end);
    t.renderPassEncoderSetPipeline = Some(encoder::render_pass_encoder_set_pipeline);
    t.renderPassEncoderSetBindGroup = Some(encoder::render_pass_encoder_set_bind_group);
    t.renderPassEncoderSetVertexBuffer = Some(encoder::render_pass_encoder_set_vertex_buffer);
    t.renderPassEncoderSetIndexBuffer = Some(encoder::render_pass_encoder_set_index_buffer);
    t.renderPassEncoderSetViewport = Some(encoder::render_pass_encoder_set_viewport);
    t.renderPassEncoderSetScissorRect = Some(encoder::render_pass_encoder_set_scissor_rect);
    t.renderPassEncoderSetBlendConstant = Some(encoder::render_pass_encoder_set_blend_constant);
    t.renderPassEncoderSetImmediates = Some(encoder::render_pass_encoder_set_immediates);
    t.renderPassEncoderDraw = Some(encoder::render_pass_encoder_draw);
    t.renderPassEncoderDrawIndexed = Some(encoder::render_pass_encoder_draw_indexed);
    t.renderPassEncoderDrawIndirect = Some(encoder::render_pass_encoder_draw_indirect);
    t.renderPassEncoderDrawIndexedIndirect =
        Some(encoder::render_pass_encoder_draw_indexed_indirect);

    t.computePassEncoderAddRef = Some(encoder::compute_pass_encoder_add_ref);
    t.computePassEncoderRelease = Some(encoder::compute_pass_encoder_release);
    t.computePassEncoderEnd = Some(encoder::compute_pass_encoder_end);
    t.computePassEncoderSetPipeline = Some(encoder::compute_pass_encoder_set_pipeline);
    t.computePassEncoderSetBindGroup = Some(encoder::compute_pass_encoder_set_bind_group);
    t.computePassEncoderDispatchWorkgroups =
        Some(encoder::compute_pass_encoder_dispatch_workgroups);
    t.computePassEncoderDispatchWorkgroupsIndirect =
        Some(encoder::compute_pass_encoder_dispatch_workgroups_indirect);

    t
}
