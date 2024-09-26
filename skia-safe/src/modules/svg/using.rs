use super::{DebugAttributes, Inherits, SvgIri, SvgLength, SvgTransformableNode};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgUse = Inherits<sb::SkSVGUse, SvgTransformableNode>;

impl DebugAttributes for SvgUse {
    const NAME: &'static str = "Use";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("href", &self.get_href()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGUse {
    type Base = sb::SkRefCntBase;
}

impl SvgUse {
    pub fn from_ptr(node: *mut sb::SkSVGUse) -> Option<Self> {
        let base = SvgTransformableNode::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGUse) -> Option<Self> {
        let base = SvgTransformableNode::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGUse[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
