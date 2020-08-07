use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, IRect, ImageFilter, Rect};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn magnifier<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        src_rect: impl AsRef<Rect>,
        inset: scalar,
    ) -> Option<Self> {
        image_filters::magnifier(src_rect, inset, self, crop_rect)
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::magnifier")]
pub fn new<'a>(
    src_rect: impl AsRef<Rect>,
    inset: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkMagnifierImageFilter_Make(
            src_rect.as_ref().native(),
            inset,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
