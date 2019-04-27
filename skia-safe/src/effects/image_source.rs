use crate::prelude::*;
use crate::{FilterQuality, Image, ImageFilter, Rect};
use skia_bindings::{C_SkImageSource_Make, C_SkImageSource_Make2};

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
