use std::fmt;

use super::{node::SvgLength, Node, NodeTag, SvgNode, Tagged, TaggedDebug};
use crate::{prelude::*, Rect};
use skia_bindings as sb;

pub type Svg = SvgNode<sb::SkSVGSVG>;

impl Tagged for sb::SkSVGSVG {
    const TAG: NodeTag = NodeTag::Svg;
}

impl TaggedDebug for Svg {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("x", &self.get_x())
            .field("y", &self.get_y())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("view_box", &self.get_view_box())
            .field("children", &self.children());
    }
}

impl NativeRefCountedBase for sb::SkSVGSVG {
    type Base = sb::SkRefCntBase;
}

impl Svg {
    pub fn append_child<N: Tagged + NativeRefCounted>(&mut self, node: SvgNode<N>) {
        unsafe { sb::C_SkSVGSVG_appendChild(self.native_mut(), node.into_node_ptr()) }
    }

    pub fn has_children(&self) -> bool {
        unsafe { sb::C_SkSVGSVG_hasChildren(self.native()) }
    }

    pub fn children_count(&self) -> usize {
        unsafe { usize::try_from(sb::C_SkSVGSVG_childrenCount(self.native())).unwrap_or_default() }
    }

    pub fn children(&self) -> Vec<Node> {
        unsafe {
            let value = safer::from_raw_parts(
                sb::C_SkSVGSVG_children(self.native()),
                self.children_count(),
            );

            value
                .iter()
                .map(|value| Node::from_unshared_ptr(value.fPtr).unwrap_unchecked())
                .collect()
        }
    }

    skia_macros::attrs! {
        SkSVGSVG[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            // PreserveAspectRatio: SkSVGPreserveAspectRatio [get(value) => value, set(value) => value],
            view_box?: Rect [get(value) => value.map(Rect::from_native_ref), set(value) => value.into_native()]
        }
    }
}
