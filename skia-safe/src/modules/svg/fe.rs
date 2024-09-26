mod blend;
mod color_matrix;
mod composite;
mod displacement_map;
mod func;
mod gaussian_blur;
mod image;
mod light_source;
mod lighting;
mod merge;
mod morphology;
mod offset;
mod turbulence;

pub use self::{
    blend::*, color_matrix::*, composite::*, displacement_map::*, func::*, gaussian_blur::*,
    image::*, light_source::*, lighting::*, merge::*, morphology::*, offset::*, turbulence::*,
};

use super::{DebugAttributes, Inherits, SvgContainer, SvgFeInput, SvgLength};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgFe = Inherits<sb::SkSVGFe, SvgContainer>;
pub type SvgFeComponentTransfer = SvgFe;
pub type SvgFeFlood = SvgFe;
pub type SvgFeMerge = SvgFe;

impl DebugAttributes for SvgFe {
    const NAME: &'static str = "Fe";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("input", self.get_input())
                .field("result", self.get_result())
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFe {
    type Base = sb::SkRefCntBase;
}

impl SvgFe {
    pub fn from_ptr(node: *mut sb::SkSVGFe) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFe) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFe[native, native_mut] => {
            "in" as input: SvgFeInput [get(value) => SvgFeInput::from_native_ref(value), set(value) => value.into_native()],
            result: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()],
            x?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            y?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            width?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            height?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()]
        }
    }
}
