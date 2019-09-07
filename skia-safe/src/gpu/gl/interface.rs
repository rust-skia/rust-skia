use crate::prelude::{NativeRefCountedBase, RCHandle};
use skia_bindings as sb;
use skia_bindings::{GrGLInterface, SkRefCntBase};

pub type Interface = RCHandle<GrGLInterface>;

impl NativeRefCountedBase for GrGLInterface {
    type Base = SkRefCntBase;
}

impl RCHandle<GrGLInterface> {
    pub fn new_native() -> Option<Interface> {
        Self::from_ptr(unsafe { sb::C_GrGLInterface_MakeNativeInterface() as _ })
    }
}
