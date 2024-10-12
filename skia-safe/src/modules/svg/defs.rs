use super::{DebugAttributes, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type Defs = RCHandle<sb::SkSVGDefs>;

impl NativeRefCountedBase for sb::SkSVGDefs {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGDefs {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for Defs {
    const NAME: &'static str = "Defs";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}
