use crate::prelude::*;
use std::ffi::CStr;
use rust_skia::{
    SkBlendMode,
    SkBlendMode_Name
};

pub type BlendMode = EnumHandle<SkBlendMode>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkBlendMode> {

    pub const Clear: Self = Self(SkBlendMode::kClear);
    pub const Src: Self = Self(SkBlendMode::kSrc);
    pub const Dst: Self = Self(SkBlendMode::kDst);
    pub const SrcOver: Self = Self(SkBlendMode::kSrcOver);
    pub const DstOver: Self = Self(SkBlendMode::kDstOver);
    pub const SrcIn: Self = Self(SkBlendMode::kSrcIn);
    pub const DstIn: Self = Self(SkBlendMode::kDstIn);
    pub const SrcOut: Self = Self(SkBlendMode::kSrcOut);
    pub const DstOut: Self = Self(SkBlendMode::kDstOut);
    pub const SrcATop: Self = Self(SkBlendMode::kSrcATop);
    pub const DstATop: Self = Self(SkBlendMode::kDstATop);
    pub const Xor: Self = Self(SkBlendMode::kXor);
    pub const Plus: Self = Self(SkBlendMode::kPlus);
    pub const Modulate: Self = Self(SkBlendMode::kModulate);
    pub const Screen: Self = Self(SkBlendMode::kScreen);

    pub const Overlay: Self = Self(SkBlendMode::kOverlay);
    pub const Darken: Self = Self(SkBlendMode::kDarken);
    pub const Lighten: Self = Self(SkBlendMode::kLighten);
    pub const ColorDodge: Self = Self(SkBlendMode::kColorDodge);
    pub const ColorBurn: Self = Self(SkBlendMode::kColorBurn);
    pub const HardLight: Self = Self(SkBlendMode::kHardLight);
    pub const SoftLight: Self = Self(SkBlendMode::kSoftLight);
    pub const Difference: Self = Self(SkBlendMode::kDifference);
    pub const Exclusion: Self = Self(SkBlendMode::kExclusion);
    pub const Multiply: Self = Self(SkBlendMode::kMultiply);

    pub const Hue: Self = Self(SkBlendMode::kHue);
    pub const Saturation: Self = Self(SkBlendMode::kSaturation);
    pub const Color: Self = Self(SkBlendMode::kColor);
    pub const Luminosity: Self = Self(SkBlendMode::kLuminosity);

    pub const LastCoeffMode: Self = Self(SkBlendMode::kLastCoeffMode);
    pub const LastSeparableMode: Self = Self(SkBlendMode::kLastSeparableMode);
    pub const LastMode: Self = Self(SkBlendMode::kLastMode);

    pub fn name(&self) -> &'static str {
        unsafe {
            let name_ptr = SkBlendMode_Name(self.into_native());
            CStr::from_ptr(name_ptr).to_str().unwrap()
        }
    }
}

impl Default for EnumHandle<SkBlendMode> {
    fn default() -> Self {
        BlendMode::SrcOver
    }
}
