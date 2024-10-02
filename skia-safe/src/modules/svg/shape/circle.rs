use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type Circle = RCHandle<sb::SkSVGCircle>;

impl NativeRefCountedBase for sb::SkSVGCircle {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGCircle {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Circle {
    const NAME: &'static str = "Circle";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("r", &self.get_r()),
        );
    }
}

impl Circle {
    skia_macros::attrs! {
        SkSVGCircle[native, native_mut] => {
            cx: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            cy: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            r: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
