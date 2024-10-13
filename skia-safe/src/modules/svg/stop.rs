use super::HasBase;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Length},
};
use skia_bindings as sb;

pub type Stop = RCHandle<sb::SkSVGStop>;

impl NativeRefCountedBase for sb::SkSVGStop {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGStop {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for Stop {
    const NAME: &'static str = "Stop";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder.field("offset", &self.offset()));
    }
}

impl Stop {
    skia_svg_macros::attrs! {
        SkSVGStop => {
            offset: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
