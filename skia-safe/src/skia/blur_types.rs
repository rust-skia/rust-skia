use crate::prelude::*;
use skia_bindings::SkBlurStyle;

pub type BlurStyle = EnumHandle<SkBlurStyle>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkBlurStyle> {
    pub const Normal: Self = Self(SkBlurStyle::kNormal_SkBlurStyle);
    pub const Solid: Self = Self(SkBlurStyle::kSolid_SkBlurStyle);
    pub const Outer: Self = Self(SkBlurStyle::kOuter_SkBlurStyle);
    pub const Inner: Self = Self(SkBlurStyle::kInner_SkBlurStyle);
}
