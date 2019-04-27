use crate::prelude::*;
use crate::{ImageFilter, Picture, Rect};
use skia_bindings::{C_SkPictureImageFilter_Make, SkImageFilter, SkPicture};

pub enum PictureImageFilter {}

impl PictureImageFilter {
    pub fn from_picture<'a, CR: Into<Option<&'a Rect>>>(
        picture: &Picture,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkPictureImageFilter_Make(
                picture.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn from_picture<'a, CR: Into<Option<&'a Rect>>>(
        picture: &Picture,
        crop_rect: CR,
    ) -> Option<Self> {
        PictureImageFilter::from_picture(picture, crop_rect)
    }
}

impl RCHandle<SkPicture> {
    pub fn as_image_filter<'a, CR: Into<Option<&'a Rect>>>(
        &self,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        PictureImageFilter::from_picture(self, crop_rect)
    }
}
