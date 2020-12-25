use crate::prelude::*;
use crate::{image_filters, scalar, ColorChannel, IRect, ImageFilter};
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
            crop_rect.into().map(|r| r.into()),
        )
    }
}
