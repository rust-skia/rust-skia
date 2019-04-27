use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect, Rect};
use skia_bindings::C_SkMagnifierImageFilter_Make;

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
