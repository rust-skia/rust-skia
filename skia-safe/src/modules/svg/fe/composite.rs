use super::{DebugAttributes, Inherits, SvgFe, SvgFeInput};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeCompositeOperator = sb::SkSVGFeCompositeOperator;
pub type SvgFeComposite = Inherits<sb::SkSVGFeComposite, SvgFe>;

impl DebugAttributes for SvgFeComposite {
    const NAME: &'static str = "FeComposite";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("input2", self.get_input2())
                .field("k1", &self.get_k1())
                .field("k2", &self.get_k2())
                .field("k3", &self.get_k3())
                .field("k4", &self.get_k4())
                .field("operator", self.get_operator()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeComposite {
    type Base = sb::SkRefCntBase;
}

impl SvgFeComposite {
    pub fn from_ptr(node: *mut sb::SkSVGFeComposite) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeComposite) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeComposite[native, native_mut] => {
            "in2" as input2: SvgFeInput [get(value) => SvgFeInput::from_native_ref(value), set(value) => value.into_native()],
            *k1: scalar [get(value) => value, set(value) => value],
            *k2: scalar [get(value) => value, set(value) => value],
            *k3: scalar [get(value) => value, set(value) => value],
            *k4: scalar [get(value) => value, set(value) => value],
            operator: SvgFeCompositeOperator [get(value) => value, set(value) => value]
        }
    }
}
