use crate::prelude::*;
use skia_bindings::{SkDocument, SkRefCntBase};

pub type Document = RCHandle<SkDocument>;

impl NativeRefCountedBase for SkDocument {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

