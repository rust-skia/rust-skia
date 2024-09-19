use std::fmt;

use super::node::*;
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgCircle = SvgNode<sb::SkSVGCircle>;

impl Tagged for sb::SkSVGCircle {
    const TAG: NodeTag = NodeTag::Circle;
}

impl TaggedDebug for SvgCircle {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("cx", &self.get_cx())
            .field("cy", &self.get_cy())
            .field("r", &self.get_r());
    }
}

impl NativeRefCountedBase for sb::SkSVGCircle {
    type Base = sb::SkRefCntBase;
}

impl SvgCircle {
    skia_macros::attrs! {
        SkSVGCircle[native, native_mut] => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            r: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
