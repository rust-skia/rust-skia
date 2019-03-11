use crate::prelude::*;
use skia_bindings::SkBlurStyle;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum BlurStyle {
    Normal = SkBlurStyle::kNormal_SkBlurStyle as _,
    Solid = SkBlurStyle::kSolid_SkBlurStyle as _,
    Outer = SkBlurStyle::kOuter_SkBlurStyle as _,
    Inner = SkBlurStyle::kInner_SkBlurStyle as _
}

impl NativeTransmutable<SkBlurStyle> for BlurStyle {}
#[test] fn test_blur_style_layout() { BlurStyle::test_layout() }
