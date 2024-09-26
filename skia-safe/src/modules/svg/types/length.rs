use skia_bindings as sb;

use crate::scalar;

pub type SvgUnit = sb::SkSVGLength_Unit;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SvgLength {
    pub value: scalar,
    pub unit: SvgUnit,
}

native_transmutable!(sb::SkSVGLength, SvgLength, svg_length_layout);

impl SvgLength {
    pub fn new(value: scalar, unit: SvgUnit) -> Self {
        Self { value, unit }
    }
}
