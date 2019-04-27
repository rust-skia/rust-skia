use crate::core::{scalar, PathEffect};
use crate::prelude::*;
use skia_bindings::C_SkDashPathEffect_Make;

pub enum DashPathEffect {}

impl DashPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(intervals: &[scalar], phase: scalar) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkDashPathEffect_Make(
                intervals.as_ptr(),
                intervals.len().try_into().unwrap(),
                phase,
            )
        })
    }
}
