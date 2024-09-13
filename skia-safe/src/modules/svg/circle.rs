use std::fmt;

use super::node::*;
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgCircle = RCHandle<sb::SkSVGCircle>;

impl NativeBase<sb::SkRefCnt> for sb::SkSVGCircle {}
impl NativeRefCounted for sb::SkSVGCircle {
    fn _ref(&self) {
        self.base()._base._ref();
    }

    fn _unref(&self) {
        self.base()._base._unref();
    }

    fn unique(&self) -> bool {
        self.base()._base.unique()
    }
}

impl fmt::Debug for SvgCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgCircle")
            .field("cx", &self.get_cx())
            .field("cy", &self.get_cy())
            .field("r", &self.get_r())
            .finish()
    }
}

impl SvgCircle {
    skia_macros::attrs! {
        SkSVGCircle => {
            cx: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            cy: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            r: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}

impl Tagged for SvgCircle {
    const TAG: NodeTag = NodeTag::Circle;
}
