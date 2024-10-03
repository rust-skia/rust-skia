use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type Ellipse = RCHandle<sb::SkSVGEllipse>;

impl NativeRefCountedBase for sb::SkSVGEllipse {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGEllipse {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Ellipse {
    const NAME: &'static str = "Ellipse";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("rx", &self.get_rx())
                .field("ry", &self.get_ry()),
        );
    }
}

impl Ellipse {
    skia_macros::attrs! {
        SkSVGEllipse => {
            cx: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            cy: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            rx?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            ry?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()]
        }
    }
}
