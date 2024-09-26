use super::SvgContainer;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgLength},
};
use skia_bindings as sb;

pub type SvgStop = Inherits<sb::SkSVGStop, SvgContainer>;

impl DebugAttributes for SvgStop {
    const NAME: &'static str = "Stop";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("offset", &self.get_offset()));
    }
}

impl NativeRefCountedBase for sb::SkSVGStop {
    type Base = sb::SkRefCntBase;
}

impl SvgStop {
    pub fn from_ptr(node: *mut sb::SkSVGStop) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGStop) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGStop[native, native_mut] => {
            offset: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
