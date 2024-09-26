use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgCircle = Inherits<sb::SkSVGCircle, SvgShape>;

impl DebugAttributes for SvgCircle {
    const NAME: &'static str = "Circle";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("r", &self.get_r()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGCircle {
    type Base = sb::SkRefCntBase;
}

impl SvgCircle {
    pub fn from_ptr(node: *mut sb::SkSVGCircle) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGCircle) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGCircle[native, native_mut] => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            r: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
