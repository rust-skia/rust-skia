use std::fmt;

use super::node::*;
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgEllipse = SvgNode<sb::SkSVGEllipse>;

impl Tagged for sb::SkSVGEllipse {
    const TAG: NodeTag = NodeTag::Ellipse;
}

impl TaggedDebug for SvgEllipse {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("cx", &self.get_cx())
            .field("cy", &self.get_cy())
            .field("rx", &self.get_rx())
            .field("ry", &self.get_ry());
    }
}

impl NativeRefCountedBase for sb::SkSVGEllipse {
    type Base = sb::SkRefCntBase;
}

impl SvgEllipse {
    skia_macros::attrs! {
        SkSVGEllipse[native, native_mut] => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            rx?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            ry?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
