use crate::prelude::*;
use skia_bindings::{SkPath1DPathEffect_Style, C_SkPath1DPathEffect_Make};
use crate::skia::{PathEffect, Path, scalar};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Path1DPathEffectStyle {
    Translate = SkPath1DPathEffect_Style::kTranslate_Style as _,
    Rotate = SkPath1DPathEffect_Style::kRotate_Style as _,
    Morph = SkPath1DPathEffect_Style::kMorph_Style as _,
}

impl NativeTransmutable<SkPath1DPathEffect_Style> for Path1DPathEffectStyle {}
#[test] fn test_path_1d_path_effect_style_layout() { Path1DPathEffectStyle::test_layout() }

pub enum Path1DPathEffect {}

impl Path1DPathEffect {

    pub fn new(path: &Path, advance: scalar, phase: scalar, style: Path1DPathEffectStyle) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkPath1DPathEffect_Make(path.native(), advance, phase, style.into_native())
        })
    }
}
