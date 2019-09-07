use crate::prelude::*;
use crate::{image_filter, image_filters, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn arithmetic<'a>(
        inputs: impl Into<ArithmeticFPInputs>,
        background: Self,
        foreground: Self,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        let inputs = inputs.into();
        image_filters::arithmetic(
            inputs.k[0],
            inputs.k[1],
            inputs.k[2],
            inputs.k[3],
            inputs.enforce_pm_color,
            background,
            foreground,
            crop_rect,
        )
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
#[deprecated(since = "0.0.0", note = "use image_filters::arithmetic()")]
pub fn new<'a>(
    inputs: impl Into<ArithmeticFPInputs>,
    background: ImageFilter,
    foreground: ImageFilter,
    crop_rect: impl Into<Option<&'a image_filter::CropRect>>,
) -> Option<ImageFilter> {
    let inputs = inputs.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkArithmeticImageFilter_Make(
            inputs.k[0],
            inputs.k[1],
            inputs.k[2],
            inputs.k[3],
            inputs.enforce_pm_color,
            background.into_ptr(),
            foreground.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
