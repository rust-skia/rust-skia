use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkOffsetImageFilter_Make, SkImageFilter};

pub enum OffsetImageFilter {}

impl OffsetImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        (dx, dy): (scalar, scalar),
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkOffsetImageFilter_Make(
                dx,
                dy,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn offset<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
        delta: (scalar, scalar),
    ) -> Option<Self> {
        OffsetImageFilter::new(delta, self, crop_rect)
    }
}
