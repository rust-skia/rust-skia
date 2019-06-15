use crate::prelude::*;
use crate::{ImageFilter, Picture, Rect};
use skia_bindings::{C_SkPictureImageFilter_Make, SkImageFilter, SkPicture};

impl RCHandle<SkImageFilter> {
    pub fn from_picture<'a>(
        picture: &Picture,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<Self> {
        from_picture(picture, crop_rect)
    }
}

impl RCHandle<SkPicture> {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        from_picture(self, crop_rect)
    }
}

pub fn from_picture<'a>(
    picture: &Picture,
    crop_rect: impl Into<Option<&'a Rect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkPictureImageFilter_Make(
            picture.shared_native(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
