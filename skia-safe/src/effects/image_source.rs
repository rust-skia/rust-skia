use crate::prelude::*;
use crate::{image_filters, FilterQuality, Image, ImageFilter, Rect};
use skia_bindings::{SkImage, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn from_image(image: impl Into<Image>) -> Option<Self> {
        image_filters::image(image, None, None, None)
    }

    pub fn from_image_rect(
        image: impl Into<Image>,
        src_rect: impl AsRef<Rect>,
        dst_rect: impl AsRef<Rect>,
        filter_quality: FilterQuality,
    ) -> Option<Self> {
        image_filters::image(image, src_rect.as_ref(), dst_rect.as_ref(), filter_quality)
    }
}

impl RCHandle<SkImage> {
    pub fn as_filter(&self) -> Option<ImageFilter> {
        self.clone().into_filter()
    }

    pub fn into_filter(self) -> Option<ImageFilter> {
        image_filters::image(self, None, None, None)
    }

    pub fn as_filter_rect(
        &self,
        src_rect: impl AsRef<Rect>,
        dst_rect: impl AsRef<Rect>,
        filter_quality: FilterQuality,
    ) -> Option<ImageFilter> {
        self.clone()
            .into_filter_rect(src_rect, dst_rect, filter_quality)
    }

    pub fn into_filter_rect(
        self,
        src_rect: impl AsRef<Rect>,
        dst_rect: impl AsRef<Rect>,
        filter_quality: FilterQuality,
    ) -> Option<ImageFilter> {
        image_filters::image(self, src_rect.as_ref(), dst_rect.as_ref(), filter_quality)
    }
}
