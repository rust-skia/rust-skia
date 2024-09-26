use super::{DebugAttributes, Inherits, SvgFe};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeOffset = Inherits<sb::SkSVGFeOffset, SvgFe>;

impl DebugAttributes for SvgFeOffset {
    const NAME: &'static str = "FeOffset";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("dx", &self.get_dx())
                .field("dy", &self.get_dy()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeOffset {
    type Base = sb::SkRefCntBase;
}

impl SvgFeOffset {
    pub fn from_ptr(node: *mut sb::SkSVGFeOffset) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeOffset) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeOffset[native, native_mut] => {
            *dx: scalar [get(value) => value, set(value) => value],
            *dy: scalar [get(value) => value, set(value) => value]
        }
    }
}
