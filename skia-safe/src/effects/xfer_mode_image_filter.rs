use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, BlendMode, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn xfer_mode<'a>(
        blend_mode: BlendMode,
        background: impl Into<Option<ImageFilter>>,
        foreground: impl Into<Option<ImageFilter>>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        image_filters::xfermode(blend_mode, background, foreground, crop_rect)
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::xfermode()")]
pub fn new<'a>(
    blend_mode: BlendMode,
    background: impl Into<ImageFilter>,
    foreground: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkXfermodeImageFilter_Make(
            blend_mode,
            background.into().into_ptr(),
            foreground.into().into_ptr_or_null(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
