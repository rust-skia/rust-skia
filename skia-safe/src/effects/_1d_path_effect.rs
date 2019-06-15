use crate::prelude::*;
use skia_bindings::SkPathEffect;
use crate::{Path, scalar, PathEffect};

impl RCHandle<SkPathEffect> {
    pub fn path_1d(
        path: &Path,
        advance: scalar,
        phase: scalar,
        style: path_1d_path_effect::Style,
    ) -> Option<PathEffect> {
        path_1d_path_effect::new(path, advance, phase, style)
    }
}

pub mod path_1d_path_effect {
    use crate::prelude::*;
    use crate::{scalar, Path, PathEffect};
    use skia_bindings::{C_SkPath1DPathEffect_Make, SkPath1DPathEffect_Style};

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(i32)]
    pub enum Style {
        Translate = SkPath1DPathEffect_Style::kTranslate_Style as _,
        Rotate = SkPath1DPathEffect_Style::kRotate_Style as _,
        Morph = SkPath1DPathEffect_Style::kMorph_Style as _,
    }

    impl NativeTransmutable<SkPath1DPathEffect_Style> for Style {}
    #[test]
    fn test_path_1d_path_effect_style_layout() {
        Style::test_layout()
    }

    pub fn new(
        path: &Path,
        advance: scalar,
        phase: scalar,
        style: Style,
    ) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkPath1DPathEffect_Make(path.native(), advance, phase, style.into_native())
        })
    }
}
