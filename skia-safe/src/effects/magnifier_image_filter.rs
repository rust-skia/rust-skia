use crate::prelude::*;
use crate::{scalar, ImageFilter, image_filter::CropRect, Rect};
use skia_bindings::{C_SkMagnifierImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn magnifier<'a>(
        &self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        src_rect: impl AsRef<Rect>,
        inset: scalar,
    ) -> Option<Self> {
        new(src_rect, inset, self, crop_rect)
    }
}

pub fn new<'a>(
    src_rect: impl AsRef<Rect>,
    inset: scalar,
    input: &ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkMagnifierImageFilter_Make(
            src_rect.as_ref().native(),
            inset,
            input.shared_native(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
