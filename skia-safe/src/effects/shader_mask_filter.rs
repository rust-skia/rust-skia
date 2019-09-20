use crate::prelude::*;
use crate::{MaskFilter, Shader};
use skia_bindings as sb;
use skia_bindings::SkMaskFilter;

impl RCHandle<SkMaskFilter> {
    pub fn from_shader(shader: Shader) -> MaskFilter {
        new(shader)
    }
}

pub fn new(shader: Shader) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::C_SkShaderMaskFilter_Make(shader.into_ptr()) }).unwrap()
}
