use crate::prelude::*;
use skia_bindings::{GrContext, GrDirectContext, SkRefCntBase};
use std::ops::{Deref, DerefMut};

pub type DirectContext = RCHandle<GrDirectContext>;

impl NativeRefCountedBase for GrDirectContext {
    type Base = SkRefCntBase;
}

impl Deref for RCHandle<GrDirectContext> {
    type Target = RCHandle<GrContext>;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for RCHandle<GrDirectContext> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}
