use crate::prelude::*;
use crate::{ImageFilter, image_filter};
use skia_bindings::{C_SkArithmeticImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn arithmetic<'a>(
        inputs: impl Into<ArithmeticFPInputs>,
        background: &Self,
        foreground: &Self,
        crop_rect: impl Into<Option<&'a image_filter::CropRect>>,
    ) -> Option<Self> {
        new(inputs, background, foreground, crop_rect)
    }
}

#[derive(Clone, Debug)]
pub struct ArithmeticFPInputs {
    pub k: [f32; 4],
    pub enforce_pm_color: bool,
}

impl From<([f32; 4], bool)> for ArithmeticFPInputs {
    fn from((k, enforce_pm_color): ([f32; 4], bool)) -> Self {
        ArithmeticFPInputs {
            k,
            enforce_pm_color,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn new<'a>(
    inputs: impl Into<ArithmeticFPInputs>,
    background: &ImageFilter,
    foreground: &ImageFilter,
    crop_rect: impl Into<Option<&'a image_filter::CropRect>>,
) -> Option<ImageFilter> {
    let inputs = inputs.into();
    ImageFilter::from_ptr(unsafe {
        C_SkArithmeticImageFilter_Make(
            inputs.k[0],
            inputs.k[1],
            inputs.k[2],
            inputs.k[3],
            inputs.enforce_pm_color,
            background.shared_native(),
            foreground.shared_native(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
