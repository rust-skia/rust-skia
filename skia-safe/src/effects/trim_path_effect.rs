use crate::prelude::*;
use crate::{scalar, PathEffect};
use skia_bindings as sb;
use skia_bindings::{SkPathEffect, SkTrimPathEffect_Mode};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Normal = sb::SkTrimPathEffect_Mode::kNormal as _,
    Inverted = sb::SkTrimPathEffect_Mode::kInverted as _,
}

impl NativeTransmutable<SkTrimPathEffect_Mode> for Mode {}
#[test]
fn mode_layout() {
    Mode::test_layout()
}

impl RCHandle<SkPathEffect> {
    pub fn trim(
        start_t: scalar,
        stop_t: scalar,
        mode: impl Into<Option<Mode>>,
    ) -> Option<PathEffect> {
        new(start_t, stop_t, mode)
    }
}

pub fn new(start_t: scalar, stop_t: scalar, mode: impl Into<Option<Mode>>) -> Option<PathEffect> {
    PathEffect::from_ptr(unsafe {
        sb::C_SkTrimPathEffect_Make(
            start_t,
            stop_t,
            mode.into().unwrap_or(Mode::Normal).into_native(),
        )
    })
}
