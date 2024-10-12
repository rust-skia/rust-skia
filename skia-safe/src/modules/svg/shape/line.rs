use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type Line = RCHandle<sb::SkSVGLine>;

impl NativeRefCountedBase for sb::SkSVGLine {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGLine {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Line {
    const NAME: &'static str = "Line";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x1", &self.get_x1())
                .field("y1", &self.get_y1())
                .field("x2", &self.get_x2())
                .field("y2", &self.get_y2()),
        );
    }
}

impl Line {
    skia_macros::attrs! {
        SkSVGLine => {
            x1: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y1: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            x2: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y2: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
