use rust_skia::{GrContext, C_GrContext_MakeVulkan};
use super::vulkan;
use crate::prelude::*;

pub type Context = RCHandle<GrContext>;

impl NativeRefCounted for GrContext {
    fn _ref(&self) {
        unsafe { self._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { self._base._base.unref(); }
    }
}

impl Context {
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       Context::from_ptr(unsafe { C_GrContext_MakeVulkan(backend_context.native) })
    }
}
