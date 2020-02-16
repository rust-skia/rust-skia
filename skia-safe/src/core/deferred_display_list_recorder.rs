use crate::prelude::*;
use crate::{Canvas, SurfaceCharacterization};
use skia_bindings as sb;
use skia_bindings::SkDeferredDisplayListRecorder;

pub use private::DeferredDisplayList;

pub type DeferredDisplayListRecorder = Handle<SkDeferredDisplayListRecorder>;
unsafe impl Send for DeferredDisplayListRecorder {}

impl NativeDrop for SkDeferredDisplayListRecorder {
    fn drop(&mut self) {
        unsafe { sb::C_SkDeferredDisplayListRecorder_destruct(self) }
    }
}

impl Handle<SkDeferredDisplayListRecorder> {
    pub fn new(characterization: &SurfaceCharacterization) -> Self {
        DeferredDisplayListRecorder::from_native(unsafe {
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

    // TODO: makePromiseTexture()?
    // TODO: makeYUVAPromiseTexture()?
}

pub(crate) mod private {
    use crate::prelude::*;
    use skia_bindings as sb;
    use skia_bindings::SkDeferredDisplayList;

    pub type DeferredDisplayList = RefHandle<SkDeferredDisplayList>;
unsafe impl Send for DeferredDisplayList {}

    impl NativeDrop for SkDeferredDisplayList {
        fn drop(&mut self) {
            unsafe { sb::C_SkDeferredDisplayList_delete(self) }
        }
    }
}
