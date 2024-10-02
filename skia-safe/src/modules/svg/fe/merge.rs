use super::{DebugAttributes, FeInput, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type FeMergeNode = RCHandle<sb::SkSVGFeMergeNode>;

impl NativeRefCountedBase for sb::SkSVGFeMergeNode {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeMergeNode {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for FeMergeNode {
    const NAME: &'static str = "FeMergeNode";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("input", self.get_input()));
    }
}

impl FeMergeNode {
    skia_macros::attrs! {
        SkSVGFeMergeNode[native, native_mut] => {
            "in" as input: FeInput [get(value) => FeInput::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
