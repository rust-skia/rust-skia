use super::{DebugAttributes, Inherits, SvgBoundingBoxUnits, SvgContainer};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgClipPath = Inherits<sb::SkSVGClipPath, SvgContainer>;

impl DebugAttributes for SvgClipPath {
    const NAME: &'static str = "ClipPath";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base
            ._dbg(builder.field("clip_path_units", self.get_clip_path_units()));
    }
}

impl NativeRefCountedBase for sb::SkSVGClipPath {
    type Base = sb::SkRefCntBase;
}

impl SvgClipPath {
    pub fn from_ptr(node: *mut sb::SkSVGClipPath) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGClipPath) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGClipPath[native, native_mut] => {
            clip_path_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
