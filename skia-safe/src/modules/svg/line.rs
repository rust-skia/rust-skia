use std::fmt;

use super::node::*;
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgLine = SvgNode<sb::SkSVGLine>;

impl Tagged for sb::SkSVGLine {
    const TAG: NodeTag = NodeTag::Line;
}

impl TaggedDebug for SvgLine {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("x1", &self.get_x1())
            .field("y1", &self.get_y1())
            .field("x2", &self.get_x2())
            .field("y2", &self.get_y2());
    }
}

impl NativeRefCountedBase for sb::SkSVGLine {
    type Base = sb::SkRefCntBase;
}

impl SvgLine {
    skia_macros::attrs! {
        SkSVGLine[native, native_mut] => {
            x1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y1: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            x2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y2: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
