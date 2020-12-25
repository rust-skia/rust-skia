use crate::prelude::*;
use crate::{image_filters, ColorFilter, IRect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn color_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        cf: impl Into<ColorFilter>,
    ) -> Option<Self> {
        image_filters::color_filter(cf, self, crop_rect.into().map(|r| r.into()))
    }
}
