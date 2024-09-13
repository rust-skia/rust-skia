use std::fmt;

use super::{NodeTag, SvgNode, Tagged};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgTextLiteral = RCHandle<sb::SkSVGTextLiteral>;

impl NativeBase<sb::SkRefCnt> for sb::SkSVGTextLiteral {}
impl NativeRefCounted for sb::SkSVGTextLiteral {
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

impl fmt::Debug for SvgTextLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgTextLiteral")
            .field("text", &self.get_text())
            .finish()
    }
}

impl SvgTextLiteral {
    skia_macros::attrs! {
        SkSVGTextLiteral => {
            text: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}

impl Tagged for SvgTextLiteral {
    const TAG: NodeTag = NodeTag::TextLiteral;
}
