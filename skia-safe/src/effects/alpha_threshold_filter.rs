use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect, Region};
use skia_bindings::C_SkAlphaThresholdFilter_Make;

pub enum AlphaThresholdFilter {}

impl AlphaThresholdFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        region: &Region,
        inner_min: scalar,
        outer_max: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkAlphaThresholdFilter_Make(
                region.native(),
                inner_min,
                outer_max,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}
