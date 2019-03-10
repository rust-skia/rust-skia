use crate::prelude::*;
use std::ffi::CStr;
use skia_bindings::{
    SkBlendMode,
    SkBlendMode_Name
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum BlendMode {

    Clear = SkBlendMode::kClear as _,
    Src = SkBlendMode::kSrc as _,
    Dst = SkBlendMode::kDst as _,
    SrcOver = SkBlendMode::kSrcOver as _,
    DstOver = SkBlendMode::kDstOver as _,
    SrcIn = SkBlendMode::kSrcIn as _,
    DstIn = SkBlendMode::kDstIn as _,
    SrcOut = SkBlendMode::kSrcOut as _,
    DstOut = SkBlendMode::kDstOut as _,
    SrcATop = SkBlendMode::kSrcATop as _,
    DstATop = SkBlendMode::kDstATop as _,
    Xor = SkBlendMode::kXor as _,
    Plus = SkBlendMode::kPlus as _,
    Modulate = SkBlendMode::kModulate as _,
    Screen = SkBlendMode::kScreen as _,

    Overlay = SkBlendMode::kOverlay as _,
    Darken = SkBlendMode::kDarken as _,
    Lighten = SkBlendMode::kLighten as _,
    ColorDodge = SkBlendMode::kColorDodge as _,
    ColorBurn = SkBlendMode::kColorBurn as _,
    HardLight = SkBlendMode::kHardLight as _,
    SoftLight = SkBlendMode::kSoftLight as _,
    Difference = SkBlendMode::kDifference as _,
    Exclusion = SkBlendMode::kExclusion as _,
    Multiply = SkBlendMode::kMultiply as _,

    Hue = SkBlendMode::kHue as _,
    Saturation = SkBlendMode::kSaturation as _,
    Color = SkBlendMode::kColor as _,
    Luminosity = SkBlendMode::kLuminosity as _,
}

impl NativeTransmutable<SkBlendMode> for BlendMode {}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::SrcOver
    }
}

impl BlendMode {

    pub fn name(self) -> &'static str {
        unsafe {
            let name_ptr = SkBlendMode_Name(self.into_native());
            CStr::from_ptr(name_ptr).to_str().unwrap()
        }
    }
}
