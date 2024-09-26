use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgLine = Inherits<sb::SkSVGLine, SvgShape>;

impl DebugAttributes for SvgLine {
    const NAME: &'static str = "Line";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x1", &self.get_x1())
                .field("y1", &self.get_y1())
                .field("x2", &self.get_x2())
                .field("y2", &self.get_y2()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGLine {
    type Base = sb::SkRefCntBase;
}

impl SvgLine {
    pub fn from_ptr(node: *mut sb::SkSVGLine) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGLine) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGLine[native, native_mut] => {
            x1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            x2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
