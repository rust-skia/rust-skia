use super::SvgGradient;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgLinearGradient = Inherits<sb::SkSVGLinearGradient, SvgGradient>;

impl DebugAttributes for SvgLinearGradient {
    const NAME: &'static str = "LinearGradient";

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

impl NativeRefCountedBase for sb::SkSVGLinearGradient {
    type Base = sb::SkRefCntBase;
}

impl SvgLinearGradient {
    pub fn from_ptr(node: *mut sb::SkSVGLinearGradient) -> Option<Self> {
        let base = SvgGradient::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGLinearGradient) -> Option<Self> {
        let base = SvgGradient::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGLinearGradient[native, native_mut] => {
            x1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            x2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
