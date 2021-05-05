use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::SkDeferredDisplayList;

// TODO: complete wrapper
pub type DeferredDisplayList = RCHandle<SkDeferredDisplayList>;
unsafe impl Send for DeferredDisplayList {}
unsafe impl Sync for DeferredDisplayList {}

impl NativeRefCounted for SkDeferredDisplayList {
    fn _ref(&self) {
        unsafe { sb::C_SkDeferredDisplayList_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkDeferredDisplayList_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkDeferredDisplayList_unique(self) }
    }
}
