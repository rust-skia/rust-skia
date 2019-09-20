use crate::prelude::*;
use crate::{Color, ColorFilter};
use skia_bindings as sb;
use skia_bindings::SkColorFilter;

impl RCHandle<SkColorFilter> {
    pub fn new_lighting(mul: impl Into<Color>, add: impl Into<Color>) -> Option<Self> {
        new_lighting(mul, add)
    }
}

pub fn new_lighting(mul: impl Into<Color>, add: impl Into<Color>) -> Option<ColorFilter> {
    ColorFilter::from_ptr(unsafe {
        sb::C_SkColorMatrixFilter_MakeLightingFilter(
            mul.into().into_native(),
            add.into().into_native(),
        )
    })
}
