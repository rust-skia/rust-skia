use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeColorMatrixKind = sb::SkSVGFeColorMatrixType;
pub type FeColorMatrix = RCHandle<sb::SkSVGFeColorMatrix>;

impl NativeRefCountedBase for sb::SkSVGFeColorMatrix {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeColorMatrix {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeColorMatrix {
    const NAME: &'static str = "FeColorMatrix";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("values", &self.get_values())
                .field("kind", self.get_kind()),
        );
    }
}

impl FeColorMatrix {
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
