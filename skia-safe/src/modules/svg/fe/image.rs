use super::SvgFe;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits, SvgIri, SvgPreserveAspectRatio},
};
use skia_bindings as sb;

pub type SvgFeImage = Inherits<sb::SkSVGFeImage, SvgFe>;

impl DebugAttributes for SvgFeImage {
    const NAME: &'static str = "FeImage";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("href", &self.get_href())
                .field("preserve_aspect_ratio", self.get_preserve_aspect_ratio()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeImage {
    type Base = sb::SkRefCntBase;
}

impl SvgFeImage {
    pub fn from_ptr(node: *mut sb::SkSVGFeImage) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeImage) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeImage[native, native_mut] => {
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()],
            preserve_aspect_ratio: SvgPreserveAspectRatio [get(value) => SvgPreserveAspectRatio::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
