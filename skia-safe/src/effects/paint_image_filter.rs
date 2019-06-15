use crate::prelude::*;
use crate::{image_filter::CropRect, ImageFilter, Paint};
use skia_bindings::{C_SkPaintImageFilter_Make, SkImageFilter, SkPaint};

impl RCHandle<SkImageFilter> {
    pub fn from_paint<'a>(
        paint: &Paint,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<Self> {
        from_paint(paint, crop_rect)
    }
}

impl Handle<SkPaint> {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<ImageFilter> {
        from_paint(self, crop_rect)
    }
}

pub fn from_paint<'a>(
    paint: &Paint,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkPaintImageFilter_Make(paint.native(), crop_rect.into().native_ptr_or_null())
    })
}
