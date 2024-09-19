use std::fmt;

use super::{NodeTag, SvgNode, Tagged, TaggedDebug};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgTextLiteral = SvgNode<sb::SkSVGTextLiteral>;

impl Tagged for sb::SkSVGTextLiteral {
    const TAG: NodeTag = NodeTag::TextLiteral;
}

impl TaggedDebug for SvgTextLiteral {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("text", &self.get_text());
    }
}

impl NativeRefCountedBase for sb::SkSVGTextLiteral {
    type Base = sb::SkRefCntBase;
}

impl SvgTextLiteral {
    skia_macros::attrs! {
        SkSVGTextLiteral[native, native_mut] => {
            text: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
