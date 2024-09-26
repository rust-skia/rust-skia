use super::{DebugAttributes, Inherits, SvgContainer};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgG = Inherits<sb::SkSVGG, SvgContainer>;

impl DebugAttributes for SvgG {
    const NAME: &'static str = "G";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGG {
    type Base = sb::SkRefCntBase;
}

impl SvgG {
    pub fn from_ptr(node: *mut sb::SkSVGG) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGG) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }
}
