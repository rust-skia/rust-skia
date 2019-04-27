use crate::prelude::*;
use crate::{scalar, PathEffect};
use skia_bindings::{C_SkDiscretePathEffect_Make, SkPathEffect};

pub enum DiscretePathEffect {}

impl DiscretePathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<SA: Into<Option<u32>>>(
        seg_length: scalar,
        dev: scalar,
        seed_assist: SA,
    ) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkDiscretePathEffect_Make(seg_length, dev, seed_assist.into().unwrap_or(0))
        })
    }
}

impl RCHandle<SkPathEffect> {
    pub fn discrete<SA: Into<Option<u32>>>(
        seg_length: scalar,
        dev: scalar,
        seed_assist: SA,
    ) -> Option<Self> {
        DiscretePathEffect::new(seg_length, dev, seed_assist)
    }
}
