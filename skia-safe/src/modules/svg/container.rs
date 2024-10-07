use super::{DebugAttributes, HasBase, Node, SvgNode};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgContainer = RCHandle<sb::SkSVGContainer>;

impl HasBase for sb::SkSVGContainer {
    type Base = sb::SkSVGTransformableNode;
}

impl NativeRefCountedBase for sb::SkSVGContainer {
    type Base = sb::SkRefCntBase;
}

impl DebugAttributes for SvgContainer {
    const NAME: &'static str = "Container";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("children", &self.children()));
    }
}

impl SvgContainer {
    pub fn append_child(&mut self, node: SvgNode) {
        unsafe { sb::C_SkSVGContainer_appendChild(self.native_mut(), node.into_ptr()) }
    }

    pub fn has_children(&self) -> bool {
        unsafe { sb::C_SkSVGContainer_hasChildren(self.native()) }
    }

    pub fn children_count(&self) -> usize {
        unsafe {
            usize::try_from(sb::C_SkSVGContainer_childrenCount(self.native())).unwrap_or_default()
        }
    }

    pub fn children(&self) -> Vec<Node> {
        unsafe {
            let value = safer::from_raw_parts(
                sb::C_SkSVGContainer_children(self.native()),
                self.children_count(),
            );

            value
                .iter()
                .map(|value| Node::from_unshared_ptr(value.fPtr).unwrap_unchecked())
                .collect()
        }
    }
}
