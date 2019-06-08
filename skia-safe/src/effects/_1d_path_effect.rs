use crate::{scalar, Path, PathEffect};
use crate::prelude::*;
use skia_bindings::{C_SkPath1DPathEffect_Make, SkPath1DPathEffect_Style, SkPathEffect};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Path1DPathEffectStyle {
    Translate = SkPath1DPathEffect_Style::kTranslate_Style as _,
    Rotate = SkPath1DPathEffect_Style::kRotate_Style as _,
    Morph = SkPath1DPathEffect_Style::kMorph_Style as _,
}

impl NativeTransmutable<SkPath1DPathEffect_Style> for Path1DPathEffectStyle {}
#[test]
fn test_path_1d_path_effect_style_layout() {
    Path1DPathEffectStyle::test_layout()
}

pub enum Path1DPathEffect {}

impl Path1DPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        path: &Path,
        advance: scalar,
        phase: scalar,
        style: Path1DPathEffectStyle,
    ) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkPath1DPathEffect_Make(path.native(), advance, phase, style.into_native())
        })
    }
}

impl RCHandle<SkPathEffect> {
    pub fn path_1d(
        path: &Path,
        advance: scalar,
        phase: scalar,
        style: Path1DPathEffectStyle,
    ) -> Option<PathEffect> {
        Path1DPathEffect::new(path, advance, phase, style)
    }
}
