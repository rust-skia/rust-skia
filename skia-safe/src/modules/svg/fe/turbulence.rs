use super::{DebugAttributes, Inherits, SvgFe};
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
pub type SvgFeTurbulence = Inherits<sb::SkSVGFeTurbulence, SvgFe>;

impl DebugAttributes for SvgFeTurbulence {
    const NAME: &'static str = "FeTurbulence";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("base_frequency", self.get_base_frequency())
                .field("num_octaves", &self.get_num_octaves())
                .field("seed", &self.get_seed())
                .field("turbulence_type", self.get_turbulence_type()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeTurbulence {
    type Base = sb::SkRefCntBase;
}

impl SvgFeTurbulence {
    pub fn from_ptr(node: *mut sb::SkSVGFeTurbulence) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeTurbulence) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeTurbulence[native, native_mut] => {
            base_frequency: SvgFeTurbulenceBaseFrequency [get(value) => SvgFeTurbulenceBaseFrequency::from_native_ref(value), set(value) => value.into_native()],
            *num_octaves: i32 [get(value) => value, set(value) => value],
            *seed: scalar [get(value) => value, set(value) => value],
            turbulence_type: SvgFeTurbulenceType [get(value) => &value.fType, set(value) => sb::SkSVGFeTurbulenceType { fType: value }]
        }
    }
}
