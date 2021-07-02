use crate::{prelude::*, Canvas, DeferredDisplayList, SurfaceCharacterization};
use skia_bindings::{self as sb, SkDeferredDisplayListRecorder};
use std::fmt;

pub type DeferredDisplayListRecorder = Handle<SkDeferredDisplayListRecorder>;

impl NativeDrop for SkDeferredDisplayListRecorder {
    fn drop(&mut self) {
        unsafe { sb::C_SkDeferredDisplayListRecorder_destruct(self) }
    }
}

impl fmt::Debug for DeferredDisplayListRecorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeferredDisplayListRecorder").finish()
    }
}

impl DeferredDisplayListRecorder {
    pub fn new(characterization: &SurfaceCharacterization) -> Self {
        DeferredDisplayListRecorder::from_native_c(unsafe {
            SkDeferredDisplayListRecorder::new(characterization.native())
        })
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        Canvas::borrow_from_native_mut(unsafe { &mut *self.native_mut().getCanvas() })
    }

    pub fn detach(mut self) -> Option<DeferredDisplayList> {
        DeferredDisplayList::from_ptr(unsafe {
            sb::C_SkDeferredDisplayListRecorder_detach(self.native_mut())
        })
    }
}
