use super::{DebugAttributes, FeInput, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgFeBlendMode = sb::SkSVGFeBlend_Mode;
pub type FeBlend = RCHandle<sb::SkSVGFeBlend>;

impl NativeRefCountedBase for sb::SkSVGFeBlend {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeBlend {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeBlend {
    const NAME: &'static str = "FeBlend";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("input2", self.get_input2())
                .field("mode", self.get_mode()),
        );
    }
}

impl FeBlend {
    skia_macros::attrs! {
        SkSVGFeBlend[native, native_mut] => {
            "in2" as input2: FeInput [get(value) => FeInput::from_native_ref(value), set(value) => value.into_native()],
            mode: SvgFeBlendMode [get(value) => value, set(value) => value]
        }
    }
}
