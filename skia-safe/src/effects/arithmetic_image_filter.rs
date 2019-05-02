use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkArithmeticImageFilter_Make, SkImageFilter};

pub enum ArithmeticImageFilter {}

#[derive(Clone, Debug)]
pub struct ArithmeticImageFilterFPInputs {
    pub k: [f32; 4],
    pub enforce_pm_color: bool,
}

impl From<([f32; 4], bool)> for ArithmeticImageFilterFPInputs {
    fn from((k, enforce_pm_color): ([f32; 4], bool)) -> Self {
        ArithmeticImageFilterFPInputs {
            k,
            enforce_pm_color,
        }
    }
}

impl ArithmeticImageFilter {
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::too_many_arguments)]
    pub fn new<
        'a,
        II: Into<ArithmeticImageFilterFPInputs>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        inputs: II,
        background: &ImageFilter,
        foreground: &ImageFilter,
        crop_rect: CR,
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
}

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn arithmetic<
        'a,
        II: Into<ArithmeticImageFilterFPInputs>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        inputs: II,
        background: &Self,
        foreground: &Self,
        crop_rect: CR,
    ) -> Option<Self> {
        ArithmeticImageFilter::new(inputs, background, foreground, crop_rect)
    }
}
