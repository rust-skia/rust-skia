mod linear;
mod radial;

pub use self::{linear::SvgLinearGradient, radial::SvgRadialGradient};

use super::{
    DebugAttributes, Inherits, SvgBoundingBoxUnits, SvgContainer, SvgIri, SvgSpreadMethod,
};
use crate::{prelude::*, Matrix};
use skia_bindings as sb;

pub type SvgGradient = Inherits<sb::SkSVGGradient, SvgContainer>;

impl DebugAttributes for SvgGradient {
    const NAME: &'static str = "Gradient";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("href", self.get_href())
                .field("gradient_transform", self.get_gradient_transform())
                .field("spread_method", self.get_spread_method())
                .field("gradient_units", self.get_gradient_units()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGGradient {
    type Base = sb::SkRefCntBase;
}

impl SvgGradient {
    pub fn from_ptr(node: *mut sb::SkSVGGradient) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGGradient) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGGradient[native, native_mut] => {
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()],
            gradient_transform: Matrix [get(value) => Matrix::from_native_ref(value), set(value) => value.into_native()],
            spread_method: SvgSpreadMethod [get(value) => &value.fType, set(value) => sb::SkSVGSpreadMethod { fType: value }],
            gradient_units: SvgBoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
