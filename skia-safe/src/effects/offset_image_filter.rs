use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect, Vector};
use skia_bindings::{C_SkOffsetImageFilter_Make, SkImageFilter};

pub enum OffsetImageFilter {}

impl OffsetImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, IV: Into<Vector>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        delta: IV,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        let delta = delta.into();
        ImageFilter::from_ptr(unsafe {
            C_SkOffsetImageFilter_Make(
                delta.x,
                delta.y,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn offset<'a, IV: Into<Vector>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
        delta: IV,
    ) -> Option<Self> {
        OffsetImageFilter::new(delta, self, crop_rect)
    }
}
