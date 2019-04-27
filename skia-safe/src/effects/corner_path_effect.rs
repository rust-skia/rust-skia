use crate::core::{scalar, PathEffect};
use skia_bindings::C_SkCornerPathEffect_Make;

pub enum CornerPathEffect {}

impl CornerPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(radius: scalar) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe { C_SkCornerPathEffect_Make(radius) })
    }
}
