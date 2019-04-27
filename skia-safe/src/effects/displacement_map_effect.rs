use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect};
use skia_bindings::{
    C_SkDisplacementMapEffect_Make, SkDisplacementMapEffect_ChannelSelectorType, SkImageFilter,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum DisplacementMapChannelSelector {
    Unknown = SkDisplacementMapEffect_ChannelSelectorType::kUnknown_ChannelSelectorType as _,
    R = SkDisplacementMapEffect_ChannelSelectorType::kR_ChannelSelectorType as _,
    G = SkDisplacementMapEffect_ChannelSelectorType::kG_ChannelSelectorType as _,
    B = SkDisplacementMapEffect_ChannelSelectorType::kB_ChannelSelectorType as _,
    A = SkDisplacementMapEffect_ChannelSelectorType::kA_ChannelSelectorType as _,
}

impl NativeTransmutable<SkDisplacementMapEffect_ChannelSelectorType>
    for DisplacementMapChannelSelector
{
}
#[test]
fn test_channel_selector_type_layout() {
    DisplacementMapChannelSelector::test_layout();
}

pub enum DisplacementMapEffect {}

impl DisplacementMapEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        (x_channel_selector, y_channel_selector): (
            DisplacementMapChannelSelector,
            DisplacementMapChannelSelector,
        ),
        scale: scalar,
        displacement: &ImageFilter,
        color: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkDisplacementMapEffect_Make(
                x_channel_selector.into_native(),
                y_channel_selector.into_native(),
                scale,
                displacement.shared_native(),
                color.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn displacement_map_effect<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        channel_selectors: (
            DisplacementMapChannelSelector,
            DisplacementMapChannelSelector,
        ),
        scale: scalar,
        displacement: &ImageFilter,
        color: &ImageFilter,
        crop_rect: CR,
    ) -> Option<Self> {
        DisplacementMapEffect::new(channel_selectors, scale, displacement, color, crop_rect)
    }
}
