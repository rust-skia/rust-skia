use crate::prelude::*;
use skia_bindings::{self as sb, SkDeferredDisplayList};
use std::fmt;

// TODO: complete wrapper
pub type DeferredDisplayList = RCHandle<SkDeferredDisplayList>;
unsafe_send_sync!(DeferredDisplayList);

impl fmt::Debug for DeferredDisplayList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeferredDisplayList").finish()
    }
}

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
