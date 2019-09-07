use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, ColorFilter, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn color_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        cf: ColorFilter,
    ) -> Option<Self> {
        image_filters::color_filter(cf, self, crop_rect)
    }
}

#[deprecated(since = "0.0.0", note = "use image_filters::color_filter")]
pub fn new<'a>(
    cf: ColorFilter,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkColorFilterImageFilter_Make(
            cf.into_ptr(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
