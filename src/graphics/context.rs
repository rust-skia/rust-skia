use rust_skia::{GrContext, C_GrContext_MakeVulkan};
use super::vulkan;
use crate::prelude::*;

#[derive(RCCloneDrop)]
pub struct Context(pub(crate) *mut GrContext);

impl RefCounted for Context {
    fn _ref(&self) {
        unsafe { (*self.0)._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { (*self.0)._base._base.unref(); }
    }
}

impl Context {
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       unsafe { C_GrContext_MakeVulkan(backend_context.native) }
           .to_option()
           .map(Context)
    }
}
