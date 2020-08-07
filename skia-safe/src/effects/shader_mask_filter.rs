use crate::prelude::*;
use crate::{MaskFilter, Shader};
use skia_bindings as sb;
use skia_bindings::SkMaskFilter;

impl RCHandle<SkMaskFilter> {
    pub fn from_shader(shader: impl AsOwned<Shader>) -> MaskFilter {
        new(shader)
    }
}

pub fn new(shader: impl AsOwned<Shader>) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::C_SkShaderMaskFilter_Make(shader.as_owned().into_ptr()) })
        .unwrap()
}
