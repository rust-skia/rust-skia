use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase},
    Path as SkPath,
};
use skia_bindings as sb;

pub type Path = RCHandle<sb::SkSVGPath>;

impl NativeRefCountedBase for sb::SkSVGPath {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGPath {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Path {
    const NAME: &'static str = "Path";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder.field("path", &self.get_path()));
    }
}

impl Path {
    skia_macros::attrs! {
        SkSVGPath[native, native_mut] => {
            path: SkPath [get(value) => SkPath::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
