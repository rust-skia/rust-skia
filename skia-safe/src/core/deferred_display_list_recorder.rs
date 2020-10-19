use crate::prelude::*;
use crate::{Canvas, DeferredDisplayList, SurfaceCharacterization};
use skia_bindings as sb;
use skia_bindings::SkDeferredDisplayListRecorder;

pub type DeferredDisplayListRecorder = Handle<SkDeferredDisplayListRecorder>;

impl NativeDrop for SkDeferredDisplayListRecorder {
    fn drop(&mut self) {
        unsafe { sb::C_SkDeferredDisplayListRecorder_destruct(self) }
    }
}

impl Handle<SkDeferredDisplayListRecorder> {
    pub fn new(characterization: &SurfaceCharacterization) -> Self {
        DeferredDisplayListRecorder::from_native_c(unsafe {
            SkDeferredDisplayListRecorder::new(characterization.native())
        })
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        Canvas::borrow_from_native(unsafe { &mut *self.native_mut().getCanvas() })
    }

    pub fn detach(mut self) -> Option<DeferredDisplayList> {
        DeferredDisplayList::from_ptr(unsafe {
            sb::C_SkDeferredDisplayListRecorder_detach(self.native_mut())
        })
    }

    // TODO: makePromiseTexture()
    // TODO: makeYUVAPromiseTexture()
}
