use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase, Length},
};
use skia_bindings as sb;

pub type RadialGradient = RCHandle<sb::SkSVGRadialGradient>;

impl NativeRefCountedBase for sb::SkSVGRadialGradient {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGRadialGradient {
    type Base = sb::SkSVGGradient;
}

impl DebugAttributes for RadialGradient {
    const NAME: &'static str = "RadialGradient";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("cx", &self.get_cx())
                .field("cy", &self.get_cy())
                .field("r", &self.get_r()),
        );
    }
}

impl RadialGradient {
    skia_macros::attrs! {
        SkSVGRadialGradient[native, native_mut] => {
            cx: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            cy: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            r: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            fx?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            fy?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()]
        }
    }
}
