use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits},
    Path,
};
use skia_bindings as sb;

pub type SvgPath = Inherits<sb::SkSVGPath, SvgShape>;

impl DebugAttributes for SvgPath {
    const NAME: &'static str = "Path";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("path", &self.get_path()));
    }
}

impl NativeRefCountedBase for sb::SkSVGPath {
    type Base = sb::SkRefCntBase;
}

impl SvgPath {
    pub fn from_ptr(node: *mut sb::SkSVGPath) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGPath) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGPath[native, native_mut] => {
            path: Path [get(value) => Path::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
