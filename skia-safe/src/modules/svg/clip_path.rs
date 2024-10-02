use super::{BoundingBoxUnits, DebugAttributes, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type ClipPath = RCHandle<sb::SkSVGClipPath>;

impl DebugAttributes for ClipPath {
    const NAME: &'static str = "ClipPath";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("clip_path_units", self.get_clip_path_units()));
    }
}

impl NativeRefCountedBase for sb::SkSVGClipPath {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGClipPath {
    type Base = sb::SkSVGContainer;
}

impl ClipPath {
    skia_macros::attrs! {
        SkSVGClipPath[native, native_mut] => {
            clip_path_units: BoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
