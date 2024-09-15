use std::fmt;

use super::{node::SvgLength, NodeTag, SvgNode, Tagged};
use crate::{prelude::*, Rect};
use skia_bindings as sb;

pub type Svg = RCHandle<sb::SkSVGSVG>;

impl NativeRefCountedBase for sb::SkSVGSVG {
    type Base = sb::SkRefCntBase;
}

impl fmt::Debug for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Svg")
            .field("x", &self.get_x())
            .field("y", &self.get_y())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("view_box", &self.get_view_box())
            .field("children", &self.children())
            .finish()
    }
}

impl Svg {
    pub fn append_child(&mut self, node: SvgNode) {
        unsafe { sb::C_SkSVGSVG_appendChild(self.native_mut(), node.into_ptr()) }
    }

    pub fn has_children(&self) -> bool {
        unsafe { sb::C_SkSVGSVG_hasChildren(self.native()) }
    }

    pub fn children_count(&self) -> usize {
        unsafe { usize::try_from(sb::C_SkSVGSVG_childrenCount(self.native())).unwrap_or_default() }
    }

    pub fn children(&self) -> Vec<SvgNode> {
        unsafe {
            let value = safer::from_raw_parts(
                sb::C_SkSVGSVG_children(self.native()),
                self.children_count(),
            );

            value
                .into_iter()
                .map(|value| SvgNode::from_unshared_ptr(value.fPtr).unwrap_unchecked())
                .collect()
        }
    }

    skia_macros::attrs! {
        SkSVGSVG => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            // PreserveAspectRatio: SkSVGPreserveAspectRatio [get(value) => value, set(value) => value],
            view_box?: Rect [get(value) => value.map(Rect::from_native_ref), set(value) => value.into_native()]
        }
    }
}

impl Tagged for Svg {
    const TAG: NodeTag = NodeTag::Svg;
}
