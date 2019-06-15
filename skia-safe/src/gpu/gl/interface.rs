use crate::prelude::{NativeRefCountedBase, RCHandle};
use skia_bindings::{C_GrGLInterface_MakeNativeInterface, GrGLInterface, SkRefCntBase};

pub type Interface = RCHandle<GrGLInterface>;

impl NativeRefCountedBase for GrGLInterface {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl RCHandle<GrGLInterface> {
    pub fn new_native() -> Option<Interface> {
        Self::from_ptr(unsafe { C_GrGLInterface_MakeNativeInterface() as _ })
    }
}
