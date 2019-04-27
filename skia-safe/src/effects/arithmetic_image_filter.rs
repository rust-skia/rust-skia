use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkArithmeticImageFilter_Make, SkImageFilter};

pub enum ArithmeticImageFilter {}

impl ArithmeticImageFilter {
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::too_many_arguments)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        k1: f32,
        k2: f32,
        k3: f32,
        k4: f32,
        enforce_pm_color: bool,
        background: &ImageFilter,
        foreground: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkArithmeticImageFilter_Make(
                k1,
                k2,
                k3,
                k4,
                enforce_pm_color,
                background.shared_native(),
                foreground.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn arithmetic<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        k1: f32,
        k2: f32,
        k3: f32,
        k4: f32,
        enforce_pm_color: bool,
        background: &Self,
        foreground: &Self,
        crop_rect: CR,
    ) -> Option<Self> {
        ArithmeticImageFilter::new(
            k1,
            k2,
            k3,
            k4,
            enforce_pm_color,
            background,
            foreground,
            crop_rect,
        )
    }
}
