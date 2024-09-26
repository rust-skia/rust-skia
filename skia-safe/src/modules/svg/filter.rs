use super::{DebugAttributes, Inherits, SvgBoundingBoxUnits, SvgContainer, SvgLength};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgFilter = Inherits<sb::SkSVGFilter, SvgContainer>;

impl DebugAttributes for SvgFilter {
    const NAME: &'static str = "Filter";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("filter_units", self.get_filter_units())
                .field("primitive_units", self.get_primitive_units()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFilter {
    type Base = sb::SkRefCntBase;
}

impl SvgFilter {
    pub fn from_ptr(node: *mut sb::SkSVGFilter) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFilter) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFilter[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            filter_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }],
            primitive_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
