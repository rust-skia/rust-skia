use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type Linear = RCHandle<sb::SkSVGLinearGradient>;

impl NativeRefCountedBase for sb::SkSVGLinearGradient {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGLinearGradient {
    type Base = sb::SkSVGGradient;
}

impl DebugAttributes for Linear {
    const NAME: &'static str = "LinearGradient";

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

impl Linear {
    skia_svg_macros::attrs! {
        SkSVGLinearGradient => {
            x1: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y1: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            x2: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y2: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
