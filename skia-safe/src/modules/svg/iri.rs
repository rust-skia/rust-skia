use std::fmt;

use crate::{interop::AsStr, prelude::*};
use skia_bindings as sb;

pub type SvgIriKind = sb::SkSVGIRI_Type;
pub type SvgIriFuncKind = sb::SkSVGFuncIRI_Type;

pub type SvgIri = Handle<sb::SkSVGIRI>;

unsafe_send_sync!(SvgIri);

impl NativeDrop for sb::SkSVGIRI {
    fn drop(&mut self) {
        unsafe { sb::C_SkSVGIRI_destruct(self) }
    }
}

impl Default for SvgIri {
    fn default() -> Self {
        Self::construct(|uninitialized| unsafe { sb::C_SkSVGIRI_new(uninitialized) })
    }
}

impl fmt::Debug for SvgIri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgIri")
            .field("data", &self.data())
            .finish()
    }
}

impl SvgIri {
    pub fn data(&self) -> &str {
        self.native().fIRI.as_str()
    }

    pub fn new<T: AsRef<str>>(value: T, kind: SvgIriKind) -> Self {
        Self::construct(|uninitialized| unsafe {
            let iri = crate::interop::String::from_str(value.as_ref());

            sb::C_SkSVGIRI_new1(uninitialized, kind, iri.native())
        })
    }
}

pub type SvgIriFunc = Handle<sb::SkSVGFuncIRI>;

impl NativeDrop for sb::SkSVGFuncIRI {
    fn drop(&mut self) {
        unsafe { sb::C_SkSVGFuncIRI_destruct(self) }
    }
}

impl fmt::Debug for SvgIriFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgIriFunc")
            .field("kind", &self.kind())
            .field("iri", &self.iri())
            .finish()
    }
}

impl SvgIriFunc {
    pub fn iri(&self) -> Option<&SvgIri> {
        let func = self.native();

        if matches!(func.fType, SvgIriFuncKind::IRI) {
            Some(SvgIri::from_native_ref(&self.native().fIRI))
        } else {
            None
        }
    }

    pub fn kind(&self) -> SvgIriFuncKind {
        self.native().fType
    }

    pub fn from_iri(value: SvgIri) -> Self {
        Self::construct(|uninitialized| unsafe {
            sb::C_SkSVGFuncIRI_IRI(uninitialized, value.native())
        })
    }

    pub fn none() -> Self {
        Self::construct(|uninitialized| unsafe { sb::C_SkSVGFuncIRI_None(uninitialized) })
    }
}
