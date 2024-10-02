use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvgFeTurbulenceBaseFrequency {
    x: scalar,
    y: scalar,
}

impl SvgFeTurbulenceBaseFrequency {
    pub fn new(x: scalar, y: scalar) -> Self {
        Self { x, y }
    }

    pub fn new_all(value: scalar) -> Self {
        Self { x: value, y: value }
    }
}

native_transmutable!(
    sb::SkSVGFeTurbulenceBaseFrequency,
    SvgFeTurbulenceBaseFrequency,
    svg_fe_turbulence_base_frequency_layout
);

pub type SvgFeTurbulenceType = sb::SkSVGFeTurbulenceType_Type;
pub type FeTurbulence = RCHandle<sb::SkSVGFeTurbulence>;

impl NativeRefCountedBase for sb::SkSVGFeTurbulence {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeTurbulence {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeTurbulence {
    const NAME: &'static str = "FeTurbulence";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("base_frequency", self.get_base_frequency())
                .field("num_octaves", &self.get_num_octaves())
                .field("seed", &self.get_seed())
                .field("turbulence_type", self.get_turbulence_type()),
        );
    }
}

impl FeTurbulence {
    skia_macros::attrs! {
        SkSVGFeTurbulence[native, native_mut] => {
            base_frequency: SvgFeTurbulenceBaseFrequency [get(value) => SvgFeTurbulenceBaseFrequency::from_native_ref(value), set(value) => value.into_native()],
            *num_octaves: i32 [get(value) => value, set(value) => value],
            *seed: scalar [get(value) => value, set(value) => value],
            turbulence_type: SvgFeTurbulenceType [get(value) => &value.fType, set(value) => sb::SkSVGFeTurbulenceType { fType: value }]
        }
    }
}
