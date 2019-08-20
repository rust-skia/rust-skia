use crate::prelude::*;
use crate::{image_filter::CropRect, BlendMode, ImageFilter};
use skia_bindings::{C_SkXfermodeImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn xfer_mode<'a>(
        blend_mode: BlendMode,
        background: ImageFilter,
        foreground: impl Into<Option<ImageFilter>>,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<Self> {
        new(blend_mode, background, foreground, crop_rect)
    }
}

pub fn new<'a>(
    blend_mode: BlendMode,
    background: ImageFilter,
    foreground: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkXfermodeImageFilter_Make(
            blend_mode.into_native(),
            background.into_ptr(),
            foreground.into().into_ptr_or_null(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
