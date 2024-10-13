use super::{DebugAttributes, HasBase, Input};
use crate::prelude::*;
use skia_bindings as sb;

pub type BlendMode = sb::SkSVGFeBlend_Mode;
variant_name!(BlendMode::Multiply);

pub type Blend = RCHandle<sb::SkSVGFeBlend>;

impl NativeRefCountedBase for sb::SkSVGFeBlend {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeBlend {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for Blend {
    const NAME: &'static str = "FeBlend";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("input2", self.input2())
                .field("mode", self.mode()),
        );
    }
}

impl Blend {
    skia_svg_macros::attrs! {
        SkSVGFeBlend => {
            "in2" as input2: Input [get(value) => Input::from_native_ref(value), set(value) => value.into_native()],
            mode: BlendMode [get(value) => value, set(value) => value]
        }
    }
}
