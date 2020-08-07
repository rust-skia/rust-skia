use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, ColorChannel, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn displacement_map_effect<'a>(
        channel_selectors: (ColorChannel, ColorChannel),
        scale: scalar,
        displacement: impl Into<ImageFilter>,
        color: impl Into<ImageFilter>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        image_filters::displacement_map(
            channel_selectors,
            scale,
            displacement.into(),
            color,
            crop_rect,
        )
    }
}

#[deprecated(since = "0.19.0", note = "use skia_safe::ColorChannel")]
pub use skia_bindings::SkDisplacementMapEffect_ChannelSelectorType as ChannelSelector;
#[test]
#[allow(deprecated)]
fn test_channel_selector_type_naming() {
    let _ = ChannelSelector::B;
}

#[deprecated(since = "0.19.0", note = "use image_filters::displacement_map")]
#[allow(deprecated)]
pub fn new<'a>(
    (x_channel_selector, y_channel_selector): (ChannelSelector, ChannelSelector),
    scale: scalar,
    displacement: impl Into<ImageFilter>,
    color: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkDisplacementMapEffect_Make(
            x_channel_selector,
            y_channel_selector,
            scale,
            displacement.into().into_ptr(),
            color.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
