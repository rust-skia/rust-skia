use super::{DebugAttributes, HasBase, Input};
use crate::prelude::*;
use skia_bindings as sb;

pub type MergeNode = RCHandle<sb::SkSVGFeMergeNode>;

impl NativeRefCountedBase for sb::SkSVGFeMergeNode {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeMergeNode {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for MergeNode {
    const NAME: &'static str = "FeMergeNode";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("input", self.get_input()));
    }
}

impl MergeNode {
    skia_svg_macros::attrs! {
        SkSVGFeMergeNode => {
            "in" as input: Input [get(value) => Input::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
