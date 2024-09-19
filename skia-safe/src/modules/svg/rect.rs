use std::fmt;

use super::node::*;
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgRect = SvgNode<sb::SkSVGRect>;

impl Tagged for sb::SkSVGRect {
    const TAG: NodeTag = NodeTag::Rect;
}

impl TaggedDebug for SvgRect {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("x", &self.get_x())
            .field("y", &self.get_y())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("rx", &self.get_rx())
            .field("ry", &self.get_ry());
    }
}

impl NativeRefCountedBase for sb::SkSVGRect {
    type Base = sb::SkRefCntBase;
}

impl SvgRect {
    skia_macros::attrs! {
        SkSVGRect[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            rx?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            ry?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
