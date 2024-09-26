use super::{DebugAttributes, Inherits, SvgFe};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeColorMatrixKind = sb::SkSVGFeColorMatrixType;
pub type SvgFeColorMatrix = Inherits<sb::SkSVGFeColorMatrix, SvgFe>;

impl DebugAttributes for SvgFeColorMatrix {
    const NAME: &'static str = "FeColorMatrix";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("values", &self.get_values())
                .field("kind", self.get_kind()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeColorMatrix {
    type Base = sb::SkRefCntBase;
}

impl SvgFeColorMatrix {
    pub fn from_ptr(node: *mut sb::SkSVGFeColorMatrix) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeColorMatrix) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn get_values(&self) -> &[scalar] {
        unsafe {
            safer::from_raw_parts(
                sb::C_SkSVGFeColorMatrix_getValues(self.native()),
                self.get_values_count(),
            )
        }
    }

    pub fn get_values_count(&self) -> usize {
        unsafe { sb::C_SkSVGFeColorMatrix_getValuesCount(self.native()) }
    }

    skia_macros::attrs! {
        SkSVGFeColorMatrix[native, native_mut] => {
            "type" as kind: SvgFeColorMatrixKind [get(value) => value, set(value) => value]
        }
    }
}
