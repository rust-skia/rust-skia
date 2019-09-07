use crate::prelude::*;
use crate::{image_filter::CropRect, scalar, ImageFilter, Region};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn alpha_threshold<'a>(
        self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        region: &Region,
        inner_min: scalar,
        outer_max: scalar,
    ) -> Option<Self> {
        new(region, inner_min, outer_max, self, crop_rect)
    }
}

#[deprecated(since = "m78", note = "use image_filters::alpha_threshold()")]
pub fn new<'a>(
    region: &Region,
    inner_min: scalar,
    outer_max: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkAlphaThresholdFilter_Make(
            region.native(),
            inner_min,
            outer_max,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
