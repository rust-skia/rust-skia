use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, IRect, ImageFilter, Paint};
use skia_bindings as sb;
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

#[deprecated(since = "0.19.0", note = "use image_filters::paint")]
pub fn from_paint<'a>(
    paint: &Paint,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkPaintImageFilter_Make(paint.native(), crop_rect.into().native_ptr_or_null())
    })
}
