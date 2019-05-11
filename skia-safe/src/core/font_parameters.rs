use crate::prelude::*;
use crate::FourByteTag;
use skia_bindings::{C_SkFontParameters_Variation_Axis_isHidden, SkFontParameters_Variation_Axis};

#[derive(Clone, PartialEq, Default, Debug)]
pub struct VariationAxis {
    pub tag: FourByteTag,
    pub min: f32,
    pub def: f32,
    pub max: f32,
    flags: u16,
}

impl NativeTransmutable<SkFontParameters_Variation_Axis> for VariationAxis {}
#[test]
fn test_variation_axis_layout() {
    VariationAxis::test_layout()
}

impl VariationAxis {
    pub fn hidden(&self) -> bool {
        unsafe {
            // does not link:
            // self.native().isHidden()
            C_SkFontParameters_Variation_Axis_isHidden(self.native())
        }
    }

    pub fn set_hidden(&mut self, hidden: bool) -> &mut Self {
        unsafe {
            self.native_mut().setHidden(hidden);
        };
        self
    }
}
