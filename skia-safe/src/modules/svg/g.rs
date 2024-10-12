use super::{DebugAttributes, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type G = RCHandle<sb::SkSVGG>;

impl DebugAttributes for G {
    const NAME: &'static str = "G";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGG {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGG {
    type Base = sb::SkSVGContainer;
}
