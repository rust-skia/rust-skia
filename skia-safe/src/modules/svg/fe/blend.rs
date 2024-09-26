use super::{DebugAttributes, Inherits, SvgFe, SvgFeInput};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgFeBlendMode = sb::SkSVGFeBlend_Mode;
pub type SvgFeBlend = Inherits<sb::SkSVGFeBlend, SvgFe>;

impl DebugAttributes for SvgFeBlend {
    const NAME: &'static str = "FeBlend";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("input2", self.get_input2())
                .field("mode", self.get_mode()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeBlend {
    type Base = sb::SkRefCntBase;
}

impl SvgFeBlend {
    pub fn from_ptr(node: *mut sb::SkSVGFeBlend) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeBlend) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeBlend[native, native_mut] => {
            "in2" as input2: SvgFeInput [get(value) => SvgFeInput::from_native_ref(value), set(value) => value.into_native()],
            mode: SvgFeBlendMode [get(value) => value, set(value) => value]
        }
    }
}
