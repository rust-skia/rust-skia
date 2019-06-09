use crate::prelude::*;
use crate::{scalar, PathEffect};
use skia_bindings::{C_SkDashPathEffect_Make, SkPathEffect};

impl RCHandle<SkPathEffect> {
    pub fn dash(intervals: &[scalar], phase: scalar) -> Option<Self> {
        new(intervals, phase)
    }
}

pub fn new(intervals: &[scalar], phase: scalar) -> Option<PathEffect> {
    PathEffect::from_ptr(unsafe {
        C_SkDashPathEffect_Make(
            intervals.as_ptr(),
            intervals.len().try_into().unwrap(),
            phase,
        )
    })
}
