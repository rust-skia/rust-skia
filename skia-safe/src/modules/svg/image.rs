use super::{DebugAttributes, HasBase, Iri, Length, PreserveAspectRatio};
use crate::prelude::*;
use skia_bindings as sb;

pub type Image = RCHandle<sb::SkSVGImage>;

impl NativeRefCountedBase for sb::SkSVGImage {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGImage {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for Image {
    const NAME: &'static str = "Image";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("href", &self.get_href())
                .field("preserve_aspect_ratio", self.get_preserve_aspect_ratio()),
        );
    }
}

impl Image {
    skia_svg_macros::attrs! {
        SkSVGImage => {
            x: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            width: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            height: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            href: Iri [get(value) => Iri::from_native_ref(value), set(value) => value.into_native()],
            preserve_aspect_ratio: PreserveAspectRatio [get(value) => PreserveAspectRatio::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
