use crate::skia::{scalar, PathEffect};
use skia_bindings::C_SkCornerPathEffect_Make;

pub enum CornerPathEffect {}

impl CornerPathEffect {
    pub fn new(radius: scalar) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkCornerPathEffect_Make(radius)
        })
    }
}
