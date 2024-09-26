use super::{DebugAttributes, Inherits, SvgContainer, SvgFeInput};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgFeMergeNode = Inherits<sb::SkSVGFeMergeNode, SvgContainer>;

impl DebugAttributes for SvgFeMergeNode {
    const NAME: &'static str = "FeMergeNode";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("input", self.get_input()));
    }
}

impl NativeRefCountedBase for sb::SkSVGFeMergeNode {
    type Base = sb::SkRefCntBase;
}

impl SvgFeMergeNode {
    pub fn from_ptr(node: *mut sb::SkSVGFeMergeNode) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeMergeNode) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeMergeNode[native, native_mut] => {
            "in" as input: SvgFeInput [get(value) => SvgFeInput::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
