use super::{DebugAttributes, Inherits, Node, SvgTransformableNode};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgContainer = Inherits<sb::SkSVGContainer, SvgTransformableNode>;

impl DebugAttributes for SvgContainer {
    const NAME: &'static str = "Container";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("children", &self.children()));
    }
}

impl NativeRefCountedBase for sb::SkSVGContainer {
    type Base = sb::SkRefCntBase;
}

impl SvgContainer {
    pub fn from_ptr(node: *mut sb::SkSVGContainer) -> Option<Self> {
        let base = SvgTransformableNode::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGContainer) -> Option<Self> {
        let base = SvgTransformableNode::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn append_child<N: NativeRefCounted, B>(&mut self, node: Inherits<N, B>) {
        unsafe { sb::C_SkSVGContainer_appendChild(self.native_mut(), node.into_ptr() as *mut _) }
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
