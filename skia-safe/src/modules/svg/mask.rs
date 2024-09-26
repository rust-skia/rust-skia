use super::{DebugAttributes, Inherits, SvgBoundingBoxUnits, SvgContainer, SvgLength};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgMask = Inherits<sb::SkSVGMask, SvgContainer>;

impl DebugAttributes for SvgMask {
    const NAME: &'static str = "Mask";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("mask_units", self.get_mask_units())
                .field("mask_content_units", self.get_mask_content_units()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGMask {
    type Base = sb::SkRefCntBase;
}

impl SvgMask {
    pub fn from_ptr(node: *mut sb::SkSVGMask) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGMask) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGMask[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            mask_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }],
            mask_content_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
