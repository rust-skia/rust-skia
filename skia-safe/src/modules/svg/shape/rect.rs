use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgRect = Inherits<sb::SkSVGRect, SvgShape>;

impl DebugAttributes for SvgRect {
    const NAME: &'static str = "Rect";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("rx", &self.get_rx())
                .field("ry", &self.get_ry()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGRect {
    type Base = sb::SkRefCntBase;
}

impl SvgRect {
    pub fn from_ptr(node: *mut sb::SkSVGRect) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGRect) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGRect[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            rx?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            ry?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
