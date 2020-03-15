use crate::prelude::*;
use crate::{Color, ColorFilter};
use skia_bindings as sb;
use skia_bindings::SkColorFilter;

pub const NUM_COLORS: usize = 6;

impl RCHandle<SkColorFilter> {
    pub fn overdraw(colors: &[Color; NUM_COLORS]) -> ColorFilter {
        new(colors)
    }
}

pub fn new(colors: &[Color; NUM_COLORS]) -> ColorFilter {
    ColorFilter::from_ptr(unsafe {
        sb::C_SkOverdrawColorFilter_MakeWithSkColors(colors.native().as_ptr())
    })
    .unwrap()
}
