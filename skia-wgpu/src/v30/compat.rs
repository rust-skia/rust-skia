//! Where `wgpu` 30 deviates from WebGPU in ways that Skia runs into, and how the shim makes up for
//! it. `README.md` next to this file describes each deviation and its upstream fix. The shim
//! reaches this module as `compat`, so its source names no `wgpu` version (ADR 0003).

use std::ops::Range;

use super::wgpu;

/// Whether `wgpu` accepts the `f16` fragment outputs that Skia writes with `ShaderF16` for `f32`
/// color targets. `wgpu` 30 rejects them, so the shim does not report the feature (`README.md`).
pub const ACCEPTS_F16_FRAGMENT_OUTPUTS: bool = false;

/// A render pass that keeps its immediates across pipeline changes, as WebGPU does and `wgpu` 30
/// does not, because Skia sets them only once per pass (`README.md`).
///
/// The `wgpu` pass stays private, and each call that the shim makes on it is forwarded below, so no
/// shim code can set a pipeline or immediates past the kept bytes. A call that the shim starts to
/// make fails to compile until it is forwarded here.
#[derive(Debug)]
pub struct RenderPass {
    pass: wgpu::RenderPass<'static>,
    /// Every byte that Skia set in this pass, from offset 0.
    immediates: Vec<u8>,
    /// Whether `wgpu` takes immediates yet, which it does only once the pass has a pipeline.
    has_pipeline: bool,
}

impl From<wgpu::RenderPass<'static>> for RenderPass {
    fn from(pass: wgpu::RenderPass<'static>) -> Self {
        Self {
            pass,
            immediates: Vec::new(),
            has_pipeline: false,
        }
    }
}

impl RenderPass {
    pub fn set_pipeline(&mut self, pipeline: &wgpu::RenderPipeline) {
        self.pass.set_pipeline(pipeline);
        self.has_pipeline = true;
        if !self.immediates.is_empty() {
            self.pass.set_immediates(0, &self.immediates);
        }
    }

    pub fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let start = offset as usize;
        let end = start + data.len();
        if self.immediates.len() < end {
            self.immediates.resize(end, 0);
        }
        self.immediates[start..end].copy_from_slice(data);
        if self.has_pipeline {
            self.pass.set_immediates(offset, data);
        }
    }

    pub fn set_bind_group(
        &mut self,
        index: u32,
        group: Option<&wgpu::BindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        self.pass.set_bind_group(index, group, offsets);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: Option<wgpu::BufferSlice<'_>>) {
        self.pass.set_vertex_buffer(slot, buffer);
    }

    pub fn set_index_buffer(&mut self, buffer: wgpu::BufferSlice<'_>, format: wgpu::IndexFormat) {
        self.pass.set_index_buffer(buffer, format);
    }

    pub fn set_viewport(&mut self, x: f32, y: f32, w: f32, h: f32, min_depth: f32, max_depth: f32) {
        self.pass.set_viewport(x, y, w, h, min_depth, max_depth);
    }

    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.pass.set_scissor_rect(x, y, width, height);
    }

    pub fn set_blend_constant(&mut self, color: wgpu::Color) {
        self.pass.set_blend_constant(color);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.pass.draw(vertices, instances);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.pass.draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw_indirect(&mut self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress) {
        self.pass.draw_indirect(buffer, offset);
    }

    pub fn draw_indexed_indirect(&mut self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress) {
        self.pass.draw_indexed_indirect(buffer, offset);
    }
}
