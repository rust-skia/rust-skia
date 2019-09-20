use crate::high_contrast_config::InvertStyle;
use crate::prelude::*;
use crate::{scalar, ColorFilter};
use skia_bindings as sb;
use skia_bindings::{SkColorFilter, SkHighContrastConfig};

pub mod high_contrast_config {
    use crate::prelude::NativeTransmutable;
    use skia_bindings as sb;
    use skia_bindings::SkHighContrastConfig_InvertStyle;

    #[repr(i32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum InvertStyle {
        NoInvert = sb::SkHighContrastConfig_InvertStyle::kNoInvert as _,
        InvertBrightness = sb::SkHighContrastConfig_InvertStyle::kInvertBrightness as _,
        InvertLightness = sb::SkHighContrastConfig_InvertStyle::kInvertLightness as _,
    }

    impl NativeTransmutable<SkHighContrastConfig_InvertStyle> for InvertStyle {}
    #[test]
    fn invert_style_layout() {
        InvertStyle::test_layout();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct HighContrastConfig {
    pub grayscale: bool,
    pub invert_style: InvertStyle,
    pub contrast: scalar,
}

impl NativeTransmutable<SkHighContrastConfig> for HighContrastConfig {}
#[test]
fn high_contrast_config_layout() {
    HighContrastConfig::test_layout();
}

impl Default for HighContrastConfig {
    fn default() -> Self {
        Self {
            grayscale: false,
            invert_style: InvertStyle::NoInvert,
            contrast: 0.0,
        }
    }
}

impl HighContrastConfig {
    pub fn new(grayscale: bool, invert_style: InvertStyle, contrast: scalar) -> Self {
        Self {
            grayscale,
            invert_style,
            contrast,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.contrast >= -1.0 && self.contrast <= 1.0
    }
}

impl RCHandle<SkColorFilter> {
    pub fn high_contrast(config: &HighContrastConfig) -> Option<Self> {
        new(config)
    }
}

pub fn new(config: &HighContrastConfig) -> Option<ColorFilter> {
    ColorFilter::from_ptr(unsafe { sb::C_SkHighContrastFilter_Make(config.native()) })
}
