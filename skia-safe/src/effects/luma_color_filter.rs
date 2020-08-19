use crate::prelude::*;
use crate::ColorFilter;
use skia_bindings as sb;
use skia_bindings::SkColorFilter;

impl RCHandle<SkColorFilter> {
    pub fn luma() -> ColorFilter {
        new()
    }
}

pub fn new() -> ColorFilter {
    ColorFilter::from_ptr(unsafe { sb::C_SkLumaColorFilter_Make() }).unwrap()
}
