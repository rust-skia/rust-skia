use crate::prelude::*;
use crate::{image_filters, scalar, IRect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn dilate<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        image_filters::dilate(radii, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn erode<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        image_filters::erode(radii, self, crop_rect.into().map(|r| r.into()))
    }
}
