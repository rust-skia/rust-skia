use std::fmt;

use super::{NodeTag, Tagged};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgTextLiteral = RCHandle<sb::SkSVGTextLiteral>;

impl NativeRefCountedBase for sb::SkSVGTextLiteral {
    type Base = sb::SkRefCntBase;
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
