use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StdDeviation {
    pub x: scalar,
    pub y: scalar,
}

impl StdDeviation {
    pub fn new(x: scalar, y: scalar) -> Self {
        Self { x, y }
    }

    pub fn new_all(value: scalar) -> Self {
        Self { x: value, y: value }
    }
}

native_transmutable!(
    sb::SkSVGFeGaussianBlur_StdDeviation,
    StdDeviation,
    std_deviation_layout
);

pub type GaussianBlur = RCHandle<sb::SkSVGFeGaussianBlur>;

impl NativeRefCountedBase for sb::SkSVGFeGaussianBlur {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeGaussianBlur {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for GaussianBlur {
    const NAME: &'static str = "FeGaussianBlur";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("std_deviation", self.std_deviation()));
    }
}

impl GaussianBlur {
    skia_svg_macros::attrs! {
        SkSVGFeGaussianBlur => {
            std_deviation: StdDeviation [get(value) => StdDeviation::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
