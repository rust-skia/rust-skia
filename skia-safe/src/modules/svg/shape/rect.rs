use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type Rect = RCHandle<sb::SkSVGRect>;

impl NativeRefCountedBase for sb::SkSVGRect {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGRect {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Rect {
    const NAME: &'static str = "Rect";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("rx", &self.get_rx())
                .field("ry", &self.get_ry()),
        );
    }
}

impl Rect {
    skia_macros::attrs! {
        SkSVGRect[native, native_mut] => {
            x: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            width: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            height: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            rx?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            ry?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()]
        }
    }
}
