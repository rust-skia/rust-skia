mod blend;
mod color_matrix;
mod composite;
mod displacement_map;
mod func;
mod gaussian_blur;
mod image;
mod light_source;
pub mod lighting;
mod merge_node;
pub mod morphology;
mod offset;
mod turbulence;
mod types;

pub use self::{
    blend::*, color_matrix::*, composite::*, displacement_map::*, func::*, gaussian_blur::*,
    image::*, light_source::*, lighting::Diffuse as DiffuseLighting, lighting::Lighting,
    lighting::Specular as SpecularLighting, merge_node::*, morphology::Morphology, offset::*,
    turbulence::*, types::*,
};

use super::{DebugAttributes, HasBase, Length};
use crate::prelude::*;
use skia_bindings as sb;

pub type Fe = RCHandle<sb::SkSVGFe>;
pub type ComponentTransfer = Fe;
pub type Flood = Fe;
pub type Merge = Fe;

impl NativeRefCountedBase for sb::SkSVGFe {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFe {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for Fe {
    const NAME: &'static str = "Fe";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
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

impl Fe {
    skia_macros::attrs! {
        SkSVGFe => {
            "in" as input: Input [get(value) => Input::from_native_ref(value), set(value) => value.into_native()],
            result: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()],
            x?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            y?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            width?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            height?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()]
        }
    }
}
