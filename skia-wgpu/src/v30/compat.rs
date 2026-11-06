//! Where `wgpu` 30 deviates from WebGPU in ways that Skia runs into, and how the shim makes up for
//! it (ADR 0003). The shim reaches this module as `compat`, so its source names no `wgpu` version.

use std::ops::{Deref, DerefMut};

use super::wgpu;

/// Whether `wgpu` accepts `f16` fragment outputs for color targets of `f32` formats, such as
/// `BGRA8Unorm`. Skia writes them when the device reports `ShaderF16`, and `wgpu` 30 rejects them
/// because it compares the scalar width. gfx-rs/wgpu#9999 compares only the scalar kind.
pub const ACCEPTS_F16_FRAGMENT_OUTPUTS: bool = false;

/// A render pass that keeps its immediates as WebGPU does.
///
/// Skia sets its intrinsic uniforms as immediates once per pass, before the first pipeline.
/// `wgpu` 30 rejects immediates before the first pipeline and zeroes them whenever the pipeline
/// layout changes. This pass therefore keeps them and sets them again after each pipeline.
/// gfx-rs/wgpu#9597 makes `wgpu` keep them itself.
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
}

/// Every other render pass call goes to `wgpu` unchanged.
impl Deref for RenderPass {
    type Target = wgpu::RenderPass<'static>;

    fn deref(&self) -> &Self::Target {
        &self.pass
    }
}

impl DerefMut for RenderPass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pass
    }
}
