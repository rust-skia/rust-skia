use skia_bindings::{GrGLInterface, SkRefCntBase, C_GrGLInterface_MakeNativeInterface};
use crate::prelude::{RCHandle, NativeRefCountedBase};

pub type Interface = RCHandle<GrGLInterface>;

impl NativeRefCountedBase for GrGLInterface {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl RCHandle<GrGLInterface> {
    pub fn native() -> Option<Interface> {
        Self::from_ptr(unsafe {
            C_GrGLInterface_MakeNativeInterface() as _
        })
    }
}