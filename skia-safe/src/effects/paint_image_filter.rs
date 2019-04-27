use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect, Paint};
use skia_bindings::{C_SkPaintImageFilter_Make, SkImageFilter, SkPaint};

pub enum PaintImageFilter {}

impl PaintImageFilter {
    pub fn from_paint<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        paint: &Paint,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkPaintImageFilter_Make(paint.native(), crop_rect.into().native_ptr_or_null())
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn from_paint<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        paint: &Paint,
        crop_rect: CR,
    ) -> Option<Self> {
        PaintImageFilter::from_paint(paint, crop_rect)
    }
}

impl Handle<SkPaint> {
    pub fn as_image_filter<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        PaintImageFilter::from_paint(self, crop_rect)
    }
}
