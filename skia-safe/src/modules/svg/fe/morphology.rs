use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Radius {
    pub x: scalar,
    pub y: scalar,
}

impl Radius {
    pub fn new(x: scalar, y: scalar) -> Self {
        Self { x, y }
    }

    pub fn new_all(value: scalar) -> Self {
        Self { x: value, y: value }
    }
}

native_transmutable!(sb::SkSVGFeMorphology_Radius, Radius, svg_radius_layout);

pub type MorphologyOperator = sb::SkSVGFeMorphology_Operator;
pub type Morphology = RCHandle<sb::SkSVGFeMorphology>;

impl NativeRefCountedBase for sb::SkSVGFeMorphology {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeMorphology {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for Morphology {
    const NAME: &'static str = "FeMorphology";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("operator", self.get_operator())
                .field("radius", &self.get_radius()),
        );
    }
}

impl Morphology {
    skia_macros::attrs! {
        SkSVGFeMorphology => {
            operator: MorphologyOperator [get(value) => value, set(value) => value],
            radius: Radius [get(value) => Radius::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
