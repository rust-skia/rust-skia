use super::{DebugAttributes, HasBase, Iri, Length};
use crate::prelude::*;
use skia_bindings as sb;

pub type Use = RCHandle<sb::SkSVGUse>;

impl NativeRefCountedBase for sb::SkSVGUse {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGUse {
    type Base = sb::SkSVGTransformableNode;
}

impl DebugAttributes for Use {
    const NAME: &'static str = "Use";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("href", &self.get_href()),
        );
    }
}

impl Use {
    skia_macros::attrs! {
        SkSVGUse => {
            x: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            href: Iri [get(value) => Iri::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
