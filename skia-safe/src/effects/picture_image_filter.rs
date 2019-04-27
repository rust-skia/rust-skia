use crate::prelude::*;
use crate::{ImageFilter, Picture, Rect};
use skia_bindings::C_SkPictureImageFilter_Make;

pub enum PictureImageFilter {}

impl PictureImageFilter {
    pub fn from_picture<'a, CR: Into<Option<&Rect>>>(
        picture: &Picture,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkPictureImageFilter_Make(
                picture.shared_native(),
                crop_rect.into().native_ptr_or_null()
            )
        })
    }
}
