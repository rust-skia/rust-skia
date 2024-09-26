use super::{DebugAttributes, Inherits, SvgContainer};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgDefs = Inherits<sb::SkSVGDefs, SvgContainer>;

impl DebugAttributes for SvgDefs {
    const NAME: &'static str = "Defs";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGDefs {
    type Base = sb::SkRefCntBase;
}

impl SvgDefs {
    pub fn from_ptr(node: *mut sb::SkSVGDefs) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGDefs) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }
}
