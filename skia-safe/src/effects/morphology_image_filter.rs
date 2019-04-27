use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkDilateImageFilter_Make, C_SkErodeImageFilter_Make};

pub enum DilateImageFilter {}

impl DilateImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        (radius_x, radius_y): (i32, i32),
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkDilateImageFilter_Make(
                radius_x,
                radius_y,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

pub enum ErodeImageFilter {}

impl ErodeImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        (radius_x, radius_y): (i32, i32),
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkErodeImageFilter_Make(
                radius_x,
                radius_y,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}
