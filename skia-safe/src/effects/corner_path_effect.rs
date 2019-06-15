use crate::prelude::*;
use crate::{scalar, PathEffect};
use skia_bindings::{C_SkCornerPathEffect_Make, SkPathEffect};

impl RCHandle<SkPathEffect> {
    pub fn corner_path(radius: scalar) -> Option<Self> {
        new(radius)
    }
}

pub fn new(radius: scalar) -> Option<PathEffect> {
    PathEffect::from_ptr(unsafe { C_SkCornerPathEffect_Make(radius) })
}
