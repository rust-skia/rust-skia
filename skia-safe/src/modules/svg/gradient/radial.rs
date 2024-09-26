use super::SvgGradient;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgRadialGradient = Inherits<sb::SkSVGRadialGradient, SvgGradient>;

impl DebugAttributes for SvgRadialGradient {
    const NAME: &'static str = "RadialGradient";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("r", &self.get_r()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGRadialGradient {
    type Base = sb::SkRefCntBase;
}

impl SvgRadialGradient {
    pub fn from_ptr(node: *mut sb::SkSVGRadialGradient) -> Option<Self> {
        let base = SvgGradient::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGRadialGradient) -> Option<Self> {
        let base = SvgGradient::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGRadialGradient[native, native_mut] => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            r: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            fx?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            fy?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
