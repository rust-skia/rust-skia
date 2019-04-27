use crate::prelude::*;
use crate::{BlendMode, ImageFilter, ImageFilterCropRect};
use skia_bindings::C_SkXfermodeImageFilter_Make;

pub enum XferModeImageFilter {}

impl XferModeImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>, FI: Into<Option<&'a ImageFilter>>>(
        blend_mode: BlendMode,
        background: &ImageFilter,
        foreground: FI,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkXfermodeImageFilter_Make(
                blend_mode.into_native(),
                background.shared_native(),
                foreground.into().shared_ptr(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}
