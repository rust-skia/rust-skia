use super::{DebugAttributes, Inherits, SvgFe};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StdDeviation {
    x: scalar,
    y: scalar,
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

pub type SvgFeGaussianBlur = Inherits<sb::SkSVGFeGaussianBlur, SvgFe>;

impl DebugAttributes for SvgFeGaussianBlur {
    const NAME: &'static str = "FeGaussianBlur";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base
            ._dbg(builder.field("std_deviation", self.get_std_deviation()));
    }
}

impl NativeRefCountedBase for sb::SkSVGFeGaussianBlur {
    type Base = sb::SkRefCntBase;
}

impl SvgFeGaussianBlur {
    pub fn from_ptr(node: *mut sb::SkSVGFeGaussianBlur) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeGaussianBlur) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeGaussianBlur[native, native_mut] => {
            std_deviation: StdDeviation [get(value) => StdDeviation::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
