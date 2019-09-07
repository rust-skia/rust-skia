use crate::prelude::*;
use crate::{image_filter::CropRect, scalar, ImageFilter};
use skia_bindings as sb;
use skia_bindings::{SkDisplacementMapEffect_ChannelSelectorType, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn displacement_map_effect<'a>(
        channel_selectors: (ChannelSelector, ChannelSelector),
        scale: scalar,
        displacement: ImageFilter,
        color: ImageFilter,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<Self> {
        new(channel_selectors, scale, displacement, color, crop_rect)
    }
}

#[deprecated(since = "m78", note = "use skia_safe::ColorChannel")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ChannelSelector {
    Unknown = SkDisplacementMapEffect_ChannelSelectorType::kUnknown_ChannelSelectorType as _,
    R = SkDisplacementMapEffect_ChannelSelectorType::kR_ChannelSelectorType as _,
    G = SkDisplacementMapEffect_ChannelSelectorType::kG_ChannelSelectorType as _,
    B = SkDisplacementMapEffect_ChannelSelectorType::kB_ChannelSelectorType as _,
    A = SkDisplacementMapEffect_ChannelSelectorType::kA_ChannelSelectorType as _,
}

impl NativeTransmutable<SkDisplacementMapEffect_ChannelSelectorType> for ChannelSelector {}
#[test]
fn test_channel_selector_type_layout() {
    ChannelSelector::test_layout();
}

#[deprecated(since = "m78", note = "use color_filters::displacement_map")]
pub fn new<'a>(
    (x_channel_selector, y_channel_selector): (ChannelSelector, ChannelSelector),
    scale: scalar,
    displacement: ImageFilter,
    color: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkDisplacementMapEffect_Make(
            x_channel_selector.into_native(),
            y_channel_selector.into_native(),
            scale,
            displacement.into_ptr(),
            color.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
