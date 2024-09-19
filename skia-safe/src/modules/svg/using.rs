use std::fmt;

use super::{iri::SvgIri, node::*};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgUse = SvgNode<sb::SkSVGUse>;

impl Tagged for sb::SkSVGUse {
    const TAG: NodeTag = NodeTag::Use;
}

impl TaggedDebug for SvgUse {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("x", &self.get_x())
            .field("y", &self.get_y())
            .field("href", &self.get_href());
    }
}

impl NativeRefCountedBase for sb::SkSVGUse {
    type Base = sb::SkRefCntBase;
}

impl SvgUse {
    skia_macros::attrs! {
        SkSVGUse[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
