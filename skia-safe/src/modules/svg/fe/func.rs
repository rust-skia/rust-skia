use super::{DebugAttributes, Inherits, SvgFe};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeFuncKind = sb::SkSVGFeFuncType;
pub type SvgFeFunc = Inherits<sb::SkSVGFeFunc, SvgFe>;

impl DebugAttributes for SvgFeFunc {
    const NAME: &'static str = "FeFunc";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("amplitude", &self.get_amplitude())
                .field("exponent", &self.get_exponent())
                .field("intercept", &self.get_intercept())
                .field("offset", &self.get_offset())
                .field("slope", &self.get_slope())
                .field("table_values", &self.get_table_values())
                .field("kind", self.get_kind()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeFunc {
    type Base = sb::SkRefCntBase;
}

impl SvgFeFunc {
    pub fn from_ptr(node: *mut sb::SkSVGFeFunc) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeFunc) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn get_table_values(&self) -> &[scalar] {
        unsafe {
            safer::from_raw_parts(
                sb::C_SkSVGFeFunc_getTableValues(self.native()),
                self.get_table_values_count(),
            )
        }
    }

    pub fn get_table_values_count(&self) -> usize {
        unsafe { sb::C_SkSVGFeFunc_getTableValuesCount(self.native()) }
    }

    skia_macros::attrs! {
        SkSVGFeFunc[native, native_mut] => {
            *amplitude: scalar [get(value) => value, set(value) => value],
            *exponent: scalar [get(value) => value, set(value) => value],
            *intercept: scalar [get(value) => value, set(value) => value],
            *offset: scalar [get(value) => value, set(value) => value],
            *slope: scalar [get(value) => value, set(value) => value],
            "type" as kind: SvgFeFuncKind [get(value) => value, set(value) => value]
        }
    }
}
