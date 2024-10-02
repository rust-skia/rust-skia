use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type FeOffset = RCHandle<sb::SkSVGFeOffset>;

impl NativeRefCountedBase for sb::SkSVGFeOffset {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeOffset {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeOffset {
    const NAME: &'static str = "FeOffset";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("dx", &self.get_dx())
                .field("dy", &self.get_dy()),
        );
    }
}

impl FeOffset {
    skia_macros::attrs! {
        SkSVGFeOffset[native, native_mut] => {
            *dx: scalar [get(value) => value, set(value) => value],
            *dy: scalar [get(value) => value, set(value) => value]
        }
    }
}
