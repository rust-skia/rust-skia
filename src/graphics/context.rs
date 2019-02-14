use rust_skia::{GrContext, C_GrContext_MakeVulkan};
use super::vulkan;
use crate::prelude::*;

pub struct Context {
    pub(crate) native: *mut GrContext
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref(); }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        unsafe { (*self.native)._base._base.ref_() }
        Context { native: self.native }
    }
}

impl Context {
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       unsafe { C_GrContext_MakeVulkan(backend_context.native) }
           .to_option()
           .map(|native| Context{ native })
    }
}
