use crate::prelude::*;
use crate::{image_filters, IRect, Vector};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn offset<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        delta: impl Into<Vector>,
    ) -> Option<Self> {
        image_filters::offset(delta, self, crop_rect.into().map(|r| r.into()))
    }
}
