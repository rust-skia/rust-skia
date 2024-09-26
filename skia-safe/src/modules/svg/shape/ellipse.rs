use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgEllipse = Inherits<sb::SkSVGEllipse, SvgShape>;

impl DebugAttributes for SvgEllipse {
    const NAME: &'static str = "Ellipse";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("rx", &self.get_rx())
                .field("ry", &self.get_ry()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGEllipse {
    type Base = sb::SkRefCntBase;
}

impl SvgEllipse {
    pub fn from_ptr(node: *mut sb::SkSVGEllipse) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGEllipse) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGEllipse[native, native_mut] => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            rx?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            ry?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
