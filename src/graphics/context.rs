use rust_skia::{GrContext, C_GrContext_MakeVulkan};
use super::vulkan;

pub struct Context {
    pub(crate) native: *mut GrContext
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref(); }
    }
}

impl Context {

    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Context {
        Context {
            native:
                unsafe { C_GrContext_MakeVulkan(backend_context.native) }
        }
    }
}