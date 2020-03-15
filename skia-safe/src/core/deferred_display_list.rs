use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::SkDeferredDisplayList;

// TODO: complete wrapper
pub type DeferredDisplayList = RefHandle<SkDeferredDisplayList>;

impl NativeDrop for SkDeferredDisplayList {
    fn drop(&mut self) {
        unsafe { sb::C_SkDeferredDisplayList_delete(self) }
    }
}
