use std::fmt;

use super::{iri::SvgIri, node::*};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgImage = SvgNode<sb::SkSVGImage>;

impl Tagged for sb::SkSVGImage {
    const TAG: NodeTag = NodeTag::Image;
}

impl TaggedDebug for SvgImage {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("x", &self.get_x())
            .field("y", &self.get_y())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("href", &self.get_href());
    }
}

impl NativeRefCountedBase for sb::SkSVGImage {
    type Base = sb::SkRefCntBase;
}

impl SvgImage {
    skia_macros::attrs! {
        SkSVGImage[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()]
            // preserve_aspect_ratio: PreserveAspectRatio [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
        }
    }
}
