use super::{DebugAttributes, HasBase, Iri, Length};
use crate::{prelude::*, Matrix};
use skia_bindings as sb;

pub type Pattern = RCHandle<sb::SkSVGPattern>;

impl DebugAttributes for Pattern {
    const NAME: &'static str = "Pattern";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGPattern {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGPattern {
    type Base = sb::SkSVGContainer;
}

impl Pattern {
    skia_macros::attrs! {
        SkSVGPattern => {
            href: Iri [get(value) => Iri::from_native_ref(value), set(value) => value.into_native()],
            x?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            y?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            width?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            height?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            pattern_transform?: Matrix [get(value) => value.map(Matrix::from_native_ref), set(value) => value.into_native()]
        }
    }
}
