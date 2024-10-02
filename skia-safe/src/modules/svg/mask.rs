use super::{BoundingBoxUnits, DebugAttributes, HasBase, Length};
use crate::prelude::*;
use skia_bindings as sb;

pub type Mask = RCHandle<sb::SkSVGMask>;

impl HasBase for sb::SkSVGMask {
    type Base = sb::SkSVGContainer;
}

impl NativeRefCountedBase for sb::SkSVGMask {
    type Base = sb::SkRefCntBase;
}

impl DebugAttributes for Mask {
    const NAME: &'static str = "Mask";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
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

impl Mask {
    skia_macros::attrs! {
        SkSVGMask[native, native_mut] => {
            x: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            width: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            height: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            mask_units: BoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }],
            mask_content_units: BoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
