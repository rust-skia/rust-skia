use crate::prelude::*;
use crate::{image_filters, ImageFilter, Picture, Rect};
use skia_bindings::{SkImageFilter, SkPicture};

impl RCHandle<SkImageFilter> {
    pub fn from_picture<'a>(
        picture: impl Into<Picture>,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<Self> {
        image_filters::picture(picture, crop_rect)
    }
}

impl RCHandle<SkPicture> {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        self.clone().into_image_filter(crop_rect)
    }

    pub fn into_image_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        image_filters::picture(self, crop_rect)
    }
}
