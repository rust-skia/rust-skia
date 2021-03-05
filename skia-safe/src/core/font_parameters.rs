pub use variation::Axis as VariationAxis;

pub mod variation {
    use crate::prelude::*;
    use crate::FourByteTag;
    use skia_bindings as sb;
    use skia_bindings::SkFontParameters_Variation_Axis;

    #[derive(Clone, PartialEq, Default, Debug)]
    pub struct Axis {
        pub tag: FourByteTag,
        pub min: f32,
        pub def: f32,
        pub max: f32,
        flags: u16,
    }

    impl NativeTransmutable<SkFontParameters_Variation_Axis> for Axis {}
    #[test]
    fn test_variation_axis_layout() {
        Axis::test_layout()
    }

    impl Axis {
        pub const fn new(tag: FourByteTag, min: f32, def: f32, max: f32, hidden: bool) -> Self {
            Axis {
                tag,
                min,
                def,
                max,
                flags: if hidden { 1 } else { 0 },
            }
        }

        pub fn is_hidden(&self) -> bool {
            unsafe { sb::C_SkFontParameters_Variation_Axis_isHidden(self.native()) }
        }

        pub fn set_hidden(&mut self, hidden: bool) -> &mut Self {
            unsafe {
                sb::C_SkFontParameters_Variation_Axis_setHidden(self.native_mut(), hidden);
            }
            self
        }
    }
}
