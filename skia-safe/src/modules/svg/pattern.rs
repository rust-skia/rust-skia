use super::{DebugAttributes, Inherits, SvgContainer, SvgIri, SvgLength};
use crate::{prelude::*, Matrix};
use skia_bindings as sb;

pub type SvgPattern = Inherits<sb::SkSVGPattern, SvgContainer>;

impl DebugAttributes for SvgPattern {
    const NAME: &'static str = "Pattern";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGPattern {
    type Base = sb::SkRefCntBase;
}

impl SvgPattern {
    pub fn from_ptr(node: *mut sb::SkSVGPattern) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGPattern) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGPattern[native, native_mut] => {
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()],
            x?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            y?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            width?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            height?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            pattern_transform?: Matrix [get(value) => value.map(Matrix::from_native_ref), set(value) => value.into_native()]
        }
    }
}
