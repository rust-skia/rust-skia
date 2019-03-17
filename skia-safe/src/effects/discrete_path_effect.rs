use crate::skia::{scalar, PathEffect};
use skia_bindings::C_SkDiscretePathEffect_Make;

pub enum DiscretePathEffect {}

impl DiscretePathEffect {

    pub fn new(seg_length: scalar, dev: scalar, seed_assist: Option<u32>) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkDiscretePathEffect_Make(seg_length, dev, seed_assist.unwrap_or(0))
        })
    }
}
