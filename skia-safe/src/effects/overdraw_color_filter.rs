use crate::prelude::*;
use crate::{ColorFilter, PMColor};
use skia_bindings as sb;
use skia_bindings::SkColorFilter;

pub const NUM_COLORS: usize = 6;

impl RCHandle<SkColorFilter> {
    pub fn overdraw(colors: &[PMColor; NUM_COLORS]) -> ColorFilter {
        new(colors)
    }
}

pub fn new(colors: &[PMColor; NUM_COLORS]) -> ColorFilter {
    ColorFilter::from_ptr(unsafe { sb::C_SkOverdrawColorFilter_Make(colors.as_ptr()) }).unwrap()
}
