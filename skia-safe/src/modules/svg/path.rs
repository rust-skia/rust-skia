use std::fmt;

use super::{NodeTag, SvgNode, Tagged, TaggedDebug};
use crate::{prelude::*, Path};
use skia_bindings as sb;

pub type SvgPath = SvgNode<sb::SkSVGPath>;

impl Tagged for sb::SkSVGPath {
    const TAG: NodeTag = NodeTag::Path;
}

impl TaggedDebug for SvgPath {
    fn _dbg(&self, f: &mut fmt::DebugStruct) {
        f.field("path", &self.get_path());
    }
}

impl NativeRefCountedBase for sb::SkSVGPath {
    type Base = sb::SkRefCntBase;
}

impl SvgPath {
    skia_macros::attrs! {
        SkSVGPath[native, native_mut] => {
            path: Path [get(value) => Path::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
