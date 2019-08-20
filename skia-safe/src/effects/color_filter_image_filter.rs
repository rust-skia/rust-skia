use crate::prelude::*;
use crate::{image_filter::CropRect, ColorFilter, ImageFilter};
use skia_bindings::{C_SkColorFilterImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn color_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        cf: ColorFilter,
    ) -> Option<Self> {
        new(cf, self, crop_rect)
    }
}

pub fn new<'a>(
    cf: ColorFilter,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkColorFilterImageFilter_Make(
            cf.into_ptr(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
