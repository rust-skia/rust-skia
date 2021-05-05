use crate::prelude::*;
use crate::{image_filters, scalar, IRect, Region};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn alpha_threshold<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        region: &Region,
        inner_min: scalar,
        outer_max: scalar,
    ) -> Option<Self> {
        image_filters::alpha_threshold(
            region,
            inner_min,
            outer_max,
            self,
            crop_rect.into().map(|r| r.into()),
        )
    }
}
