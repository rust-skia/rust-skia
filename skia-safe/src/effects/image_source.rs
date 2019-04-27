use crate::prelude::*;
use crate::{FilterQuality, Image, ImageFilter, Rect};
use skia_bindings::{C_SkImageSource_Make, C_SkImageSource_Make2, SkImage, SkImageFilter};

pub enum ImageSource {}

impl ImageSource {
    pub fn from_image(image: &Image) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe { C_SkImageSource_Make(image.shared_native()) })
    }

    // TODO: improve naming of that function?
    pub fn from_image_partial<SR: AsRef<Rect>, DR: AsRef<Rect>>(
        image: &Image,
        src_rect: SR,
        dst_rect: DR,
        filter_quality: FilterQuality,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkImageSource_Make2(
                image.shared_native(),
                src_rect.as_ref().native(),
                dst_rect.as_ref().native(),
                filter_quality.into_native(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn from_image(image: &Image) -> Option<Self> {
        ImageSource::from_image(image)
    }

    // TODO: improve naming of that function?
    pub fn from_image_partial<SR: AsRef<Rect>, DR: AsRef<Rect>>(
        image: &Image,
        src_rect: SR,
        dst_rect: DR,
        filter_quality: FilterQuality,
    ) -> Option<Self> {
        ImageSource::from_image_partial(image, src_rect, dst_rect, filter_quality)
    }
}

impl RCHandle<SkImage> {
    pub fn as_filter(&self) -> Option<ImageFilter> {
        ImageSource::from_image(self)
    }

    // TODO: improve naming of that function?
    pub fn as_filter_partial<SR: AsRef<Rect>, DR: AsRef<Rect>>(
        &self,
        src_rect: SR,
        dst_rect: DR,
        filter_quality: FilterQuality,
    ) -> Option<ImageFilter> {
        ImageSource::from_image_partial(self, src_rect, dst_rect, filter_quality)
    }
}
