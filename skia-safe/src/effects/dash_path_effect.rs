use crate::prelude::*;
use crate::skia::{scalar, PathEffect};
use skia_bindings::C_SkDashPathEffect_Make;

pub enum DashPathEffect {}

impl DashPathEffect {

    pub fn new(intervals: &[scalar], phase: scalar) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkDashPathEffect_Make(intervals.as_ptr(), intervals.len().try_into().unwrap(), phase)
        })
    }
}
