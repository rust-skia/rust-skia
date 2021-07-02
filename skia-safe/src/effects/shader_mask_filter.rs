use crate::{prelude::*, MaskFilter, Shader};
use skia_bindings as sb;

impl MaskFilter {
    pub fn from_shader(shader: impl Into<Shader>) -> MaskFilter {
        new(shader)
    }
}

pub fn new(shader: impl Into<Shader>) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::C_SkShaderMaskFilter_Make(shader.into().into_ptr()) })
        .unwrap()
}
