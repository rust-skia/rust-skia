use crate::prelude::*;
use crate::{Canvas, SurfaceCharacterization};
use skia_bindings::{
    C_SkDeferredDisplayListRecorder_destruct, C_SkDeferredDisplayListRecorder_detach,
    SkDeferredDisplayListRecorder,
};

pub use private::DeferredDisplayList;

pub type DeferredDisplayListRecorder = Handle<SkDeferredDisplayListRecorder>;

impl NativeDrop for SkDeferredDisplayListRecorder {
    fn drop(&mut self) {
        unsafe { C_SkDeferredDisplayListRecorder_destruct(self) }
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
        let ptr = unsafe { C_SkDeferredDisplayListRecorder_detach(self.native_mut()) };
        if ptr.is_null() {
            return None;
        }
        Some(DeferredDisplayList(ptr))
    }

    // TODO: makePromiseTexture()?
    // TODO: makeYUVAPromiseTexture()?
}

pub(crate) mod private {
    use crate::prelude::*;
    use skia_bindings::{C_SkDeferredDisplayList_delete, SkDeferredDisplayList};

    #[repr(transparent)]
    pub struct DeferredDisplayList(pub(crate) *mut SkDeferredDisplayList);

    impl NativeAccess<SkDeferredDisplayList> for DeferredDisplayList {
        fn native(&self) -> &SkDeferredDisplayList {
            unsafe { &*self.0 }
        }

        fn native_mut(&mut self) -> &mut SkDeferredDisplayList {
            unsafe { &mut *self.0 }
        }
    }

    impl Drop for DeferredDisplayList {
        fn drop(&mut self) {
            unsafe { C_SkDeferredDisplayList_delete(self.0) }
        }
    }
}
