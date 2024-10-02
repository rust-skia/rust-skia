use super::{DebugAttributes, FeInput, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeCompositeOperator = sb::SkSVGFeCompositeOperator;
pub type FeComposite = RCHandle<sb::SkSVGFeComposite>;

impl NativeRefCountedBase for sb::SkSVGFeComposite {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeComposite {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeComposite {
    const NAME: &'static str = "FeComposite";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("input2", self.get_input2())
                .field("k1", &self.get_k1())
                .field("k2", &self.get_k2())
                .field("k3", &self.get_k3())
                .field("k4", &self.get_k4())
                .field("operator", self.get_operator()),
        );
    }
}

impl FeComposite {
    skia_macros::attrs! {
        SkSVGFeComposite[native, native_mut] => {
            "in2" as input2: FeInput [get(value) => FeInput::from_native_ref(value), set(value) => value.into_native()],
            *k1: scalar [get(value) => value, set(value) => value],
            *k2: scalar [get(value) => value, set(value) => value],
            *k3: scalar [get(value) => value, set(value) => value],
            *k4: scalar [get(value) => value, set(value) => value],
            operator: SvgFeCompositeOperator [get(value) => value, set(value) => value]
        }
    }
}
