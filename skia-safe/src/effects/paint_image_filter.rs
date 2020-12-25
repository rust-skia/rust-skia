use crate::prelude::*;
use crate::{image_filters, IRect, ImageFilter, Paint};
use skia_bindings::{SkImageFilter, SkPaint};

impl RCHandle<SkImageFilter> {
    pub fn from_paint<'a>(paint: &Paint, crop_rect: impl Into<Option<&'a IRect>>) -> Option<Self> {
        image_filters::paint(paint, crop_rect.into().map(|r| r.into()))
    }
}

impl Handle<SkPaint> {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<ImageFilter> {
        image_filters::paint(self, crop_rect.into().map(|r| r.into()))
    }
}
