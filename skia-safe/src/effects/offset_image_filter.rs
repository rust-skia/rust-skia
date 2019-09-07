use crate::prelude::*;
use crate::{image_filter::CropRect, ImageFilter, Vector};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn offset<'a>(
        self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        delta: impl Into<Vector>,
    ) -> Option<Self> {
        new(delta, self, crop_rect)
    }
}

#[deprecated(since = "m78", note = "use color_filters::offset")]
pub fn new<'a>(
    delta: impl Into<Vector>,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkOffsetImageFilter_Make(
            delta.x,
            delta.y,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
