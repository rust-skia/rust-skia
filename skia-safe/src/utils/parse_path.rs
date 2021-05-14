use crate::{
    interop,
    prelude::{IfBoolSome, NativeAccess},
    Path,
};
use skia_bindings as sb;
use std::ffi::CString;

pub fn from_svg(svg: impl AsRef<str>) -> Option<Path> {
    let str = CString::new(svg.as_ref()).unwrap();
    let mut path = Path::default();
    unsafe { sb::SkParsePath_FromSVGString(str.as_ptr(), path.native_mut()) }.if_true_some(path)
}

pub fn to_svg(path: &Path) -> String {
    let mut svg = interop::String::default();
    unsafe { sb::SkParsePath_ToSVGString(path.native(), svg.native_mut()) };

    svg.as_str().into()
}

impl Path {
    pub fn from_svg(svg: impl AsRef<str>) -> Option<Path> {
        from_svg(svg)
    }

    pub fn to_svg(&self) -> String {
        to_svg(self)
    }
}
