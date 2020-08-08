use crate::prelude::*;
use crate::{image_filter, image_filters, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn arithmetic<'a>(
        inputs: impl Into<ArithmeticFPInputs>,
        background: impl Into<Option<Self>>,
        foreground: impl Into<Option<Self>>,
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

impl ArithmeticFPInputs {
    pub fn new(k0: f32, k1: f32, k2: f32, k3: f32, enforce_pm_color: bool) -> Self {
        Self {
            k: [k0, k1, k2, k3],
            enforce_pm_color,
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[deprecated(since = "0.19.0", note = "use image_filters::arithmetic()")]
pub fn new<'a>(
    inputs: impl Into<ArithmeticFPInputs>,
    background: impl Into<ImageFilter>,
    foreground: impl Into<ImageFilter>,
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
            background.into().into_ptr(),
            foreground.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
