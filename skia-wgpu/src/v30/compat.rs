//! Where `wgpu` 30 deviates from WebGPU in ways that Skia runs into, and how the shim makes up for
//! it. `README.md` next to this file describes each deviation and its upstream fix. The shim
//! reaches this module as `compat`, so its source names no `wgpu` version (ADR 0003).

use std::ops::{Deref, DerefMut};

use super::wgpu;

/// Whether `wgpu` accepts the `f16` fragment outputs that Skia writes with `ShaderF16` for `f32`
/// color targets. `wgpu` 30 rejects them, so the shim does not report the feature (`README.md`).
pub const ACCEPTS_F16_FRAGMENT_OUTPUTS: bool = false;

/// A render pass that keeps its immediates across pipeline changes, as WebGPU does and `wgpu` 30
/// does not, because Skia sets them only once per pass (`README.md`).
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
