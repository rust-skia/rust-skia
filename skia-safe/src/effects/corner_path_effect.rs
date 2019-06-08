use crate::{scalar, PathEffect};
use crate::prelude::*;
use skia_bindings::{C_SkCornerPathEffect_Make, SkPathEffect};

pub enum CornerPathEffect {}

impl CornerPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(radius: scalar) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe { C_SkCornerPathEffect_Make(radius) })
    }
}

impl RCHandle<SkPathEffect> {
    pub fn corner_path(radius: scalar) -> Option<Self> {
        CornerPathEffect::new(radius)
    }
}
