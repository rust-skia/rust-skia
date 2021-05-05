use crate::prelude::*;
use crate::{image_filters, IRect};
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
            crop_rect.into().map(|r| r.into()),
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
