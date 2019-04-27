use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect, Rect};
use skia_bindings::{C_SkMagnifierImageFilter_Make, SkImageFilter};

pub enum MagnifierImageFilter {}

impl MagnifierImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, R: AsRef<Rect>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        src_rect: R,
        inset: scalar,
        input: &ImageFilter,
        crop_rect: CR,
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
}

impl RCHandle<SkImageFilter> {
    pub fn magnifier<'a, R: AsRef<Rect>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
        src_rect: R,
        inset: scalar,
    ) -> Option<Self> {
        MagnifierImageFilter::new(src_rect, inset, self, crop_rect)
    }
}
